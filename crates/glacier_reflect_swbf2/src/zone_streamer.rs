use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerZoneDestroyMessage {
}

pub const ZONESTREAMERZONEDESTROYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneDestroyMessage",
    flags: MemberInfoFlags::new(36937),
    module: "ZoneStreamer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ZoneStreamerZoneDestroyMessage {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERZONEDESTROYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerZoneInitMessage {
}

pub const ZONESTREAMERZONEINITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneInitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "ZoneStreamer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ZoneStreamerZoneInitMessage {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERZONEINITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerZoneChangedMessage {
}

pub const ZONESTREAMERZONECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "ZoneStreamer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ZoneStreamerZoneChangedMessage {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERZONECHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerShutdownMessage {
}

pub const ZONESTREAMERSHUTDOWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerShutdownMessage",
    flags: MemberInfoFlags::new(36937),
    module: "ZoneStreamer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ZoneStreamerShutdownMessage {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERSHUTDOWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerAnnounceMessage {
}

pub const ZONESTREAMERANNOUNCEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerAnnounceMessage",
    flags: MemberInfoFlags::new(36937),
    module: "ZoneStreamer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ZoneStreamerAnnounceMessage {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERANNOUNCEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerNotificationEntityData {
    pub control_entity: super::core::Guid,
    pub bundle_name: String,
}

pub const ZONESTREAMERNOTIFICATIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerNotificationEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ControlEntity",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerNotificationEntityData, control_entity),
            },
            FieldInfoData {
                name: "BundleName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerNotificationEntityData, bundle_name),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERNOTIFICATIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerNotificationEntityData {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERNOTIFICATIONENTITYDATA_TYPE_INFO
    }
}


pub const ZONESTREAMERNOTIFICATIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerNotificationEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerNotificationEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VistaZoneInfo {
    pub neighbours: Vec<i16>,
    pub objects: Vec<VistaZoneMeshInfo>,
}

pub const VISTAZONEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VistaZoneInfo",
    flags: MemberInfoFlags::new(73),
    module: "ZoneStreamer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Neighbours",
                flags: MemberInfoFlags::new(144),
                field_type: INT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VistaZoneInfo, neighbours),
            },
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: VISTAZONEMESHINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VistaZoneInfo, objects),
            },
        ],
    }),
    array_type: Some(VISTAZONEINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VistaZoneInfo {
    fn type_info() -> &'static TypeInfo {
        VISTAZONEINFO_TYPE_INFO
    }
}


pub const VISTAZONEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VistaZoneInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("VistaZoneInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VistaZoneMeshInfo {
    pub object: super::entity::ObjectBlueprint,
    pub transform: super::core::LinearTransform,
}

pub const VISTAZONEMESHINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VistaZoneMeshInfo",
    flags: MemberInfoFlags::new(73),
    module: "ZoneStreamer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Object",
                flags: MemberInfoFlags::new(0),
                field_type: OBJECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(VistaZoneMeshInfo, object),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(VistaZoneMeshInfo, transform),
            },
        ],
    }),
    array_type: Some(VISTAZONEMESHINFO_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VistaZoneMeshInfo {
    fn type_info() -> &'static TypeInfo {
        VISTAZONEMESHINFO_TYPE_INFO
    }
}


pub const VISTAZONEMESHINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VistaZoneMeshInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("VistaZoneMeshInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ZoneStreamerVistaEntityData {
    pub zone_infos: Vec<VistaZoneInfo>,
    pub control_entity: super::core::Guid,
}

pub const ZONESTREAMERVISTAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerVistaEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ZoneInfos",
                flags: MemberInfoFlags::new(144),
                field_type: VISTAZONEINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerVistaEntityData, zone_infos),
            },
            FieldInfoData {
                name: "ControlEntity",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerVistaEntityData, control_entity),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERVISTAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerVistaEntityData {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERVISTAENTITYDATA_TYPE_INFO
    }
}


pub const ZONESTREAMERVISTAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerVistaEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerVistaEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ZoneStreamerSubWorldRod {
}

pub const ZONESTREAMERSUBWORLDROD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerSubWorldRod",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERSUBWORLDROD_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ZoneStreamerSubWorldRod {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERSUBWORLDROD_TYPE_INFO
    }
}


pub const ZONESTREAMERSUBWORLDROD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerSubWorldRod-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerSubWorldRod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ZoneStreamerEntityData {
    pub client_side_only: bool,
    pub enable_default_focus: bool,
    pub info: ZoneStreamerInfo,
}

pub const ZONESTREAMERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ClientSideOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerEntityData, client_side_only),
            },
            FieldInfoData {
                name: "EnableDefaultFocus",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerEntityData, enable_default_focus),
            },
            FieldInfoData {
                name: "Info",
                flags: MemberInfoFlags::new(0),
                field_type: ZONESTREAMERINFO_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerEntityData, info),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ZoneStreamerEntityData {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERENTITYDATA_TYPE_INFO
    }
}


pub const ZONESTREAMERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerInfo {
    pub grid_resource: super::core::ResourceRef,
    pub sub_level_path: String,
    pub zone_infos: Vec<ZoneStreamerZoneInfo>,
    pub bundle_parents: Vec<i16>,
    pub bundle_names: Vec<String>,
}

pub const ZONESTREAMERINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerInfo",
    flags: MemberInfoFlags::new(73),
    module: "ZoneStreamer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "GridResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerInfo, grid_resource),
            },
            FieldInfoData {
                name: "SubLevelPath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerInfo, sub_level_path),
            },
            FieldInfoData {
                name: "ZoneInfos",
                flags: MemberInfoFlags::new(144),
                field_type: ZONESTREAMERZONEINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerInfo, zone_infos),
            },
            FieldInfoData {
                name: "BundleParents",
                flags: MemberInfoFlags::new(144),
                field_type: INT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerInfo, bundle_parents),
            },
            FieldInfoData {
                name: "BundleNames",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerInfo, bundle_names),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerInfo {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERINFO_TYPE_INFO
    }
}


pub const ZONESTREAMERINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerZoneInfo {
    pub neighbours: Vec<i16>,
}

pub const ZONESTREAMERZONEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneInfo",
    flags: MemberInfoFlags::new(73),
    module: "ZoneStreamer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Neighbours",
                flags: MemberInfoFlags::new(144),
                field_type: INT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerZoneInfo, neighbours),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERZONEINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerZoneInfo {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERZONEINFO_TYPE_INFO
    }
}


pub const ZONESTREAMERZONEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerZoneInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ZoneStreamerRasterNodeUsage {
    #[default]
    ZoneStreamerRasterNodeUsage_Default = 0,
    ZoneStreamerRasterNodeUsage_Disabled = 1,
    ZoneStreamerRasterNodeUsage_Persistent = 2,
    ZoneStreamerRasterNodeUsage_PersistentDedicatedServer = 3,
    ZoneStreamerRasterNodeUsage_Skipped = 4,
}

pub const ZONESTREAMERRASTERNODEUSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerRasterNodeUsage",
    flags: MemberInfoFlags::new(49429),
    module: "ZoneStreamer",
    data: TypeInfoData::Enum,
    array_type: Some(ZONESTREAMERRASTERNODEUSAGE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ZoneStreamerRasterNodeUsage {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERRASTERNODEUSAGE_TYPE_INFO
    }
}


pub const ZONESTREAMERRASTERNODEUSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerRasterNodeUsage-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerRasterNodeUsage-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ZoneStreamerSettings {
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

pub const ZONESTREAMERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerSettings",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TestZoneHeights",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, test_zone_heights),
            },
            FieldInfoData {
                name: "PinVisitedZones",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, pin_visited_zones),
            },
            FieldInfoData {
                name: "PauseAll",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, pause_all),
            },
            FieldInfoData {
                name: "DrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw_stats),
            },
            FieldInfoData {
                name: "Draw3dDebug",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw3d_debug),
            },
            FieldInfoData {
                name: "Draw3dNameScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw3d_name_scale),
            },
            FieldInfoData {
                name: "Draw2dDebug",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_debug),
            },
            FieldInfoData {
                name: "Draw2dScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_scale),
            },
            FieldInfoData {
                name: "Draw2dZones",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_zones),
            },
            FieldInfoData {
                name: "Draw2dRotate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_rotate),
            },
            FieldInfoData {
                name: "Draw2dZoneStates",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_zone_states),
            },
            FieldInfoData {
                name: "Draw2dCentroids",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_centroids),
            },
            FieldInfoData {
                name: "Draw2dPointSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_point_size),
            },
            FieldInfoData {
                name: "Draw2dBgAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_bg_alpha),
            },
            FieldInfoData {
                name: "Draw2dNames",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_names),
            },
            FieldInfoData {
                name: "DrawTerrainTiles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw_terrain_tiles),
            },
            FieldInfoData {
                name: "DrawTerrainTileLoadedOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw_terrain_tile_loaded_only),
            },
            FieldInfoData {
                name: "DrawTerrainTileToDraw",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, draw_terrain_tile_to_draw),
            },
            FieldInfoData {
                name: "SelectedStreamer",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerSettings, selected_streamer),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerSettings {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERSETTINGS_TYPE_INFO
    }
}


pub const ZONESTREAMERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ZoneStreamerFocusEntityData {
    pub focus_point: super::core::LinearTransform,
    pub auto_enabled: bool,
}

pub const ZONESTREAMERFOCUSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerFocusEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FocusPoint",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerFocusEntityData, focus_point),
            },
            FieldInfoData {
                name: "AutoEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerFocusEntityData, auto_enabled),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERFOCUSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ZoneStreamerFocusEntityData {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERFOCUSENTITYDATA_TYPE_INFO
    }
}


pub const ZONESTREAMERFOCUSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerFocusEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerFocusEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerZoneProxyEntityData {
    pub zone_and_region_names: Vec<String>,
}

pub const ZONESTREAMERZONEPROXYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneProxyEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ZoneAndRegionNames",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerZoneProxyEntityData, zone_and_region_names),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERZONEPROXYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerZoneProxyEntityData {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERZONEPROXYENTITYDATA_TYPE_INFO
    }
}


pub const ZONESTREAMERZONEPROXYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneProxyEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerZoneProxyEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerTransitionEntityData {
    pub auto_begin: bool,
}

pub const ZONESTREAMERTRANSITIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerTransitionEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AutoBegin",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerTransitionEntityData, auto_begin),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERTRANSITIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerTransitionEntityData {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERTRANSITIONENTITYDATA_TYPE_INFO
    }
}


pub const ZONESTREAMERTRANSITIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerTransitionEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerTransitionEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerControlEntityData {
    pub start_paused: bool,
}

pub const ZONESTREAMERCONTROLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerControlEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StartPaused",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerControlEntityData, start_paused),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERCONTROLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerControlEntityData {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERCONTROLENTITYDATA_TYPE_INFO
    }
}


pub const ZONESTREAMERCONTROLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerControlEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerControlEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerLogicEntityData {
    pub realm: super::core::Realm,
}

pub const ZONESTREAMERLOGICENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerLogicEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ZoneStreamerLogicEntityData, realm),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERLOGICENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerLogicEntityData {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERLOGICENTITYDATA_TYPE_INFO
    }
}


pub const ZONESTREAMERLOGICENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerLogicEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerLogicEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerZoneProxyEntity {
}

pub const ZONESTREAMERZONEPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneProxyEntity",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERZONEPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerZoneProxyEntity {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERZONEPROXYENTITY_TYPE_INFO
    }
}


pub const ZONESTREAMERZONEPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneProxyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerZoneProxyEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerVistaEntity {
}

pub const ZONESTREAMERVISTAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerVistaEntity",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERVISTAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerVistaEntity {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERVISTAENTITY_TYPE_INFO
    }
}


pub const ZONESTREAMERVISTAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerVistaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerVistaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerTransitionEntity {
}

pub const ZONESTREAMERTRANSITIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerTransitionEntity",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERTRANSITIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerTransitionEntity {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERTRANSITIONENTITY_TYPE_INFO
    }
}


pub const ZONESTREAMERTRANSITIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerTransitionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerTransitionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerNotificationEntity {
}

pub const ZONESTREAMERNOTIFICATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerNotificationEntity",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERNOTIFICATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerNotificationEntity {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERNOTIFICATIONENTITY_TYPE_INFO
    }
}


pub const ZONESTREAMERNOTIFICATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerNotificationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerNotificationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerLogicEntity {
}

pub const ZONESTREAMERLOGICENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerLogicEntity",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERLOGICENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerLogicEntity {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERLOGICENTITY_TYPE_INFO
    }
}


pub const ZONESTREAMERLOGICENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerLogicEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerLogicEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerGrid {
}

pub const ZONESTREAMERGRID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerGrid",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERGRID_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerGrid {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERGRID_TYPE_INFO
    }
}


pub const ZONESTREAMERGRID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerGrid-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerGrid-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerEntityBase {
}

pub const ZONESTREAMERENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerEntityBase {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERENTITYBASE_TYPE_INFO
    }
}


pub const ZONESTREAMERENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerEntity {
}

pub const ZONESTREAMERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntity",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerEntity {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERENTITY_TYPE_INFO
    }
}


pub const ZONESTREAMERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerControlEntity {
}

pub const ZONESTREAMERCONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerControlEntity",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERCONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerControlEntity {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERCONTROLENTITY_TYPE_INFO
    }
}


pub const ZONESTREAMERCONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerControlEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerControlEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RealmProxy {
}

pub const REALMPROXY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RealmProxy",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(REALMPROXY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RealmProxy {
    fn type_info() -> &'static TypeInfo {
        REALMPROXY_TYPE_INFO
    }
}


pub const REALMPROXY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RealmProxy-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("RealmProxy-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ZoneStreamerFocusEntity {
}

pub const ZONESTREAMERFOCUSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerFocusEntity",
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERFOCUSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerFocusEntity {
    fn type_info() -> &'static TypeInfo {
        ZONESTREAMERFOCUSENTITY_TYPE_INFO
    }
}


pub const ZONESTREAMERFOCUSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerFocusEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerFocusEntity-Array"),
    array_type: None,
    alignment: 8,
};


