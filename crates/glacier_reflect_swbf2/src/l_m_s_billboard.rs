use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_l_m_s_billboard_types(registry: &mut TypeRegistry) {
    registry.register_type(LINEARMEDIABILLBOARDSETTINGS_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDCLIENTENTITYDATA_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDCLIENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDOVERRIDEFEEDENTITYDATA_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDOVERRIDEFEEDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDFEEDENTITYDATA_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDFEEDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIALODCODES_TYPE_INFO);
    registry.register_type(LINEARMEDIALODCODES_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDPROVIDERENTITYDATA_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDPROVIDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LMSBILLBOARDASSET_TYPE_INFO);
    registry.register_type(LMSBILLBOARDASSET_ARRAY_TYPE_INFO);
    registry.register_type(LODDIMENSION_TYPE_INFO);
    registry.register_type(LODDIMENSION_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDDEFS_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDDEFS_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDPROVIDERENTITY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDOVERRIDEFEEDENTITY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDOVERRIDEFEEDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDFEEDENTITY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDFEEDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDCLIENTENTITY_TYPE_INFO);
    registry.register_type(LINEARMEDIABILLBOARDCLIENTENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct LinearMediaBillboardSettings {
    pub debug_entity_logic_update_enable: bool,
    pub streaming_enable: bool,
    pub skip_l_o_d: i32,
    pub max_active_feeds: i32,
    pub support_texture2_enable: bool,
    pub support_emissive_enable: bool,
    pub l_o_d_screen_surface_scale: f32,
    pub force_lowest_mip_dynamic: bool,
}

pub const LINEARMEDIABILLBOARDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardSettings",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DebugEntityLogicUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardSettings, debug_entity_logic_update_enable),
            },
            FieldInfoData {
                name: "StreamingEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardSettings, streaming_enable),
            },
            FieldInfoData {
                name: "SkipLOD",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardSettings, skip_l_o_d),
            },
            FieldInfoData {
                name: "MaxActiveFeeds",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardSettings, max_active_feeds),
            },
            FieldInfoData {
                name: "SupportTexture2Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardSettings, support_texture2_enable),
            },
            FieldInfoData {
                name: "SupportEmissiveEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardSettings, support_emissive_enable),
            },
            FieldInfoData {
                name: "LODScreenSurfaceScale",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardSettings, l_o_d_screen_surface_scale),
            },
            FieldInfoData {
                name: "ForceLowestMipDynamic",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardSettings, force_lowest_mip_dynamic),
            },
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaBillboardSettings {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIABILLBOARDSETTINGS_TYPE_INFO
    }
}


pub const LINEARMEDIABILLBOARDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LinearMediaBillboardClientEntityData {
    pub receiving_channel: i32,
    pub is_dynamic_object: bool,
    pub size_in_meter_sqr: f32,
    pub disable_l_o_d: bool,
    pub use_second_texture_stream: bool,
    pub enabled_on_startup: bool,
    pub fallback_texture: super::render::TextureAsset,
}

pub const LINEARMEDIABILLBOARDCLIENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardClientEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ReceivingChannel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, receiving_channel),
            },
            FieldInfoData {
                name: "isDynamicObject",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, is_dynamic_object),
            },
            FieldInfoData {
                name: "SizeInMeterSqr",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, size_in_meter_sqr),
            },
            FieldInfoData {
                name: "disableLOD",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, disable_l_o_d),
            },
            FieldInfoData {
                name: "useSecondTextureStream",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, use_second_texture_stream),
            },
            FieldInfoData {
                name: "EnabledOnStartup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, enabled_on_startup),
            },
            FieldInfoData {
                name: "FallbackTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, fallback_texture),
            },
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDCLIENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaBillboardClientEntityData {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIABILLBOARDCLIENTENTITYDATA_TYPE_INFO
    }
}


pub const LINEARMEDIABILLBOARDCLIENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardClientEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardClientEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LinearMediaBillboardOverrideFeedEntityData {
    pub channels_to_override: Vec<i32>,
}

pub const LINEARMEDIABILLBOARDOVERRIDEFEEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardOverrideFeedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINEARMEDIABILLBOARDFEEDENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ChannelsToOverride",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardOverrideFeedEntityData, channels_to_override),
            },
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDOVERRIDEFEEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaBillboardOverrideFeedEntityData {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIABILLBOARDOVERRIDEFEEDENTITYDATA_TYPE_INFO
    }
}


pub const LINEARMEDIABILLBOARDOVERRIDEFEEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardOverrideFeedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardOverrideFeedEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LinearMediaBillboardFeedEntityData {
    pub emitting_channel: i32,
    pub texture_codes: Vec<LinearMediaLODCodes>,
    pub linear_media_billboard_asset: LMSBillboardAsset,
    pub start_on_load: bool,
    pub r#loop: bool,
    pub external_time: f32,
}

pub const LINEARMEDIABILLBOARDFEEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardFeedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EmittingChannel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, emitting_channel),
            },
            FieldInfoData {
                name: "TextureCodes",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARMEDIALODCODES_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, texture_codes),
            },
            FieldInfoData {
                name: "LinearMediaBillboardAsset",
                flags: MemberInfoFlags::new(0),
                field_type: LMSBILLBOARDASSET_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, linear_media_billboard_asset),
            },
            FieldInfoData {
                name: "StartOnLoad",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, start_on_load),
            },
            FieldInfoData {
                name: "Loop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, r#loop),
            },
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, external_time),
            },
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDFEEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaBillboardFeedEntityData {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIABILLBOARDFEEDENTITYDATA_TYPE_INFO
    }
}


pub const LINEARMEDIABILLBOARDFEEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardFeedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardFeedEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaLODCodes {
    pub texture_out_code: u64,
    pub texture2_out_code: u64,
    pub emissive_texture_out_code: u64,
}

pub const LINEARMEDIALODCODES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaLODCodes",
    flags: MemberInfoFlags::new(36937),
    module: "LMSBillboard",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TextureOutCode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaLODCodes, texture_out_code),
            },
            FieldInfoData {
                name: "Texture2OutCode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaLODCodes, texture2_out_code),
            },
            FieldInfoData {
                name: "EmissiveTextureOutCode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaLODCodes, emissive_texture_out_code),
            },
        ],
    }),
    array_type: Some(LINEARMEDIALODCODES_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaLODCodes {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIALODCODES_TYPE_INFO
    }
}


pub const LINEARMEDIALODCODES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaLODCodes-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaLODCodes-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaBillboardProviderEntityData {
    pub fallbacktexture: super::render::TextureAsset,
    pub is_active: bool,
}

pub const LINEARMEDIABILLBOARDPROVIDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardProviderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Fallbacktexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardProviderEntityData, fallbacktexture),
            },
            FieldInfoData {
                name: "IsActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaBillboardProviderEntityData, is_active),
            },
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDPROVIDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaBillboardProviderEntityData {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIABILLBOARDPROVIDERENTITYDATA_TYPE_INFO
    }
}


pub const LINEARMEDIABILLBOARDPROVIDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardProviderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardProviderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LMSBillboardAsset {
    pub base_texture: super::render::TextureAsset,
}

pub const LMSBILLBOARDASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSBillboardAsset",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINEARMEDIAASSETDESC_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BaseTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(LMSBillboardAsset, base_texture),
            },
        ],
    }),
    array_type: Some(LMSBILLBOARDASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LMSBillboardAsset {
    fn type_info() -> &'static TypeInfo {
        LMSBILLBOARDASSET_TYPE_INFO
    }
}


pub const LMSBILLBOARDASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSBillboardAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LMSBillboardAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LODDimension {
    pub width: i32,
    pub height: i32,
}

pub const LODDIMENSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LODDimension",
    flags: MemberInfoFlags::new(36937),
    module: "LMSBillboard",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LODDimension, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LODDimension, height),
            },
        ],
    }),
    array_type: Some(LODDIMENSION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LODDimension {
    fn type_info() -> &'static TypeInfo {
        LODDIMENSION_TYPE_INFO
    }
}


pub const LODDIMENSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LODDimension-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LODDimension-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LinearMediaBillboardDefs {
    #[default]
    LMBBD_MaxSpeakersPerBillboardFeed = 4,
    LMBBD_MaxSupportedBillboardFeeds = 32,
}

pub const LINEARMEDIABILLBOARDDEFS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardDefs",
    flags: MemberInfoFlags::new(49429),
    module: "LMSBillboard",
    data: TypeInfoData::Enum,
    array_type: Some(LINEARMEDIABILLBOARDDEFS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LinearMediaBillboardDefs {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIABILLBOARDDEFS_TYPE_INFO
    }
}


pub const LINEARMEDIABILLBOARDDEFS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardDefs-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardDefs-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaBillboardProviderEntity {
}

pub const LINEARMEDIABILLBOARDPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaBillboardProviderEntity {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIABILLBOARDPROVIDERENTITY_TYPE_INFO
    }
}


pub const LINEARMEDIABILLBOARDPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardProviderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaBillboardOverrideFeedEntity {
}

pub const LINEARMEDIABILLBOARDOVERRIDEFEEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardOverrideFeedEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDOVERRIDEFEEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaBillboardOverrideFeedEntity {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIABILLBOARDOVERRIDEFEEDENTITY_TYPE_INFO
    }
}


pub const LINEARMEDIABILLBOARDOVERRIDEFEEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardOverrideFeedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardOverrideFeedEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaBillboardFeedEntity {
}

pub const LINEARMEDIABILLBOARDFEEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardFeedEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDFEEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaBillboardFeedEntity {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIABILLBOARDFEEDENTITY_TYPE_INFO
    }
}


pub const LINEARMEDIABILLBOARDFEEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardFeedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardFeedEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaBillboardClientEntity {
}

pub const LINEARMEDIABILLBOARDCLIENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardClientEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDCLIENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaBillboardClientEntity {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIABILLBOARDCLIENTENTITY_TYPE_INFO
    }
}


pub const LINEARMEDIABILLBOARDCLIENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardClientEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardClientEntity-Array"),
    array_type: None,
    alignment: 8,
};


