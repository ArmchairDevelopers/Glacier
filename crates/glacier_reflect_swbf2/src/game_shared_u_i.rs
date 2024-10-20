use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_game_shared_u_i_types(registry: &mut TypeRegistry) {
    registry.register_type(UIGEOMETRYASSET_TYPE_INFO);
    registry.register_type(UIGEOMETRYASSET_ARRAY_TYPE_INFO);
    registry.register_type(UIAUTOMAPPEDTEXTURE_TYPE_INFO);
    registry.register_type(UIAUTOMAPPEDTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(UICPPSCREENDATA_TYPE_INFO);
    registry.register_type(UICPPSCREENDATA_ARRAY_TYPE_INFO);
    registry.register_type(UICPPSCREENSTATESTREAMLANETYPE_TYPE_INFO);
    registry.register_type(UICPPSCREENSTATESTREAMLANETYPE_ARRAY_TYPE_INFO);
    registry.register_type(UISCREENRENDERTARGETENTITYDATA_TYPE_INFO);
    registry.register_type(UISCREENRENDERTARGETENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UISCREENRENDERENTITYDATA_TYPE_INFO);
    registry.register_type(UISCREENRENDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UISCREENRENDERINGPASS_TYPE_INFO);
    registry.register_type(UISCREENRENDERINGPASS_ARRAY_TYPE_INFO);
    registry.register_type(UIIMSETTINGSASSET_TYPE_INFO);
    registry.register_type(UIIMSETTINGSASSET_ARRAY_TYPE_INFO);
    registry.register_type(UIAUTOSCROLLTEXTSETTINGS_TYPE_INFO);
    registry.register_type(UIAUTOSCROLLTEXTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTWIDGETREFERENCEENTITYDATA_TYPE_INFO);
    registry.register_type(UIELEMENTWIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTBITMAPDISTANCEFIELDPARAMS_TYPE_INFO);
    registry.register_type(UIELEMENTBITMAPDISTANCEFIELDPARAMS_ARRAY_TYPE_INFO);
    registry.register_type(UIMASKINGCONTAINERENTITYDATA_TYPE_INFO);
    registry.register_type(UIMASKINGCONTAINERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UICONTAINERENTITYDATA_TYPE_INFO);
    registry.register_type(UICONTAINERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(UIELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UIWIDGETENTITYDATA_TYPE_INFO);
    registry.register_type(UIWIDGETENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTLAYERENTITYDATA_TYPE_INFO);
    registry.register_type(UIELEMENTLAYERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTINCLUSIONSETTINGS_TYPE_INFO);
    registry.register_type(UIELEMENTINCLUSIONSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(UIBLENDMODE_TYPE_INFO);
    registry.register_type(UIBLENDMODE_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTGRADIENT_TYPE_INFO);
    registry.register_type(UIELEMENTGRADIENT_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTANCHOR_TYPE_INFO);
    registry.register_type(UIELEMENTANCHOR_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTSIZE_TYPE_INFO);
    registry.register_type(UIELEMENTSIZE_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTOFFSET_TYPE_INFO);
    registry.register_type(UIELEMENTOFFSET_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTRECTEXPANSION_TYPE_INFO);
    registry.register_type(UIELEMENTRECTEXPANSION_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTFONTSTYLE_TYPE_INFO);
    registry.register_type(UIELEMENTFONTSTYLE_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTFONTEFFECT_TYPE_INFO);
    registry.register_type(UIELEMENTFONTEFFECT_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTFONTDEFINITION_TYPE_INFO);
    registry.register_type(UIELEMENTFONTDEFINITION_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTFONTDEFINITIONOVERRIDE_TYPE_INFO);
    registry.register_type(UIELEMENTFONTDEFINITIONOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTTEXTFILTERGLOW_TYPE_INFO);
    registry.register_type(UIELEMENTTEXTFILTERGLOW_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTTEXTFILTERDROPSHADOW_TYPE_INFO);
    registry.register_type(UIELEMENTTEXTFILTERDROPSHADOW_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTTEXTFILTERBLUR_TYPE_INFO);
    registry.register_type(UIELEMENTTEXTFILTERBLUR_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTTEXTFILTER_TYPE_INFO);
    registry.register_type(UIELEMENTTEXTFILTER_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTCOLOR_TYPE_INFO);
    registry.register_type(UIELEMENTCOLOR_ARRAY_TYPE_INFO);
    registry.register_type(UITEXTUREMAPPINGASSET_TYPE_INFO);
    registry.register_type(UITEXTUREMAPPINGASSET_ARRAY_TYPE_INFO);
    registry.register_type(UILAYOUTMODE_TYPE_INFO);
    registry.register_type(UILAYOUTMODE_ARRAY_TYPE_INFO);
    registry.register_type(UITEXTUREMAPPINGCOMPARTMENT_TYPE_INFO);
    registry.register_type(UITEXTUREMAPPINGCOMPARTMENT_ARRAY_TYPE_INFO);
    registry.register_type(UITEXTUREMAPPINGOUTPUTENTRY_TYPE_INFO);
    registry.register_type(UITEXTUREMAPPINGOUTPUTENTRY_ARRAY_TYPE_INFO);
    registry.register_type(UIWIDGETBLUEPRINT_TYPE_INFO);
    registry.register_type(UIWIDGETBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(UIIMMEDIATEMODEFONTCONFIGURATIONASSET_TYPE_INFO);
    registry.register_type(UIIMMEDIATEMODEFONTCONFIGURATIONASSET_ARRAY_TYPE_INFO);
    registry.register_type(UIIMMEDIATEMODEFONTLOOKUP_TYPE_INFO);
    registry.register_type(UIIMMEDIATEMODEFONTLOOKUP_ARRAY_TYPE_INFO);
    registry.register_type(UIIMMEDIATEMODEFONTBUNDLE_TYPE_INFO);
    registry.register_type(UIIMMEDIATEMODEFONTBUNDLE_ARRAY_TYPE_INFO);
    registry.register_type(UITTFASSET_TYPE_INFO);
    registry.register_type(UITTFASSET_ARRAY_TYPE_INFO);
    registry.register_type(UIINPUTENTITYDATA_TYPE_INFO);
    registry.register_type(UIINPUTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOVIETRACKDATA_TYPE_INFO);
    registry.register_type(MOVIETRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOVIETRACKKEYFRAME_TYPE_INFO);
    registry.register_type(MOVIETRACKKEYFRAME_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIGeometryAsset {
}

pub const UIGEOMETRYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIGeometryAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIGEOMETRYASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIGeometryAsset {
    fn type_info() -> &'static TypeInfo {
        UIGEOMETRYASSET_TYPE_INFO
    }
}


pub const UIGEOMETRYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIGeometryAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIGeometryAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIAutoMappedTexture {
    pub id: i32,
    pub texture_ref: super::core::ResourceRef,
}

pub const UIAUTOMAPPEDTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoMappedTexture",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIAutoMappedTexture, id),
            },
            FieldInfoData {
                name: "TextureRef",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(UIAutoMappedTexture, texture_ref),
            },
        ],
    }),
    array_type: Some(UIAUTOMAPPEDTEXTURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIAutoMappedTexture {
    fn type_info() -> &'static TypeInfo {
        UIAUTOMAPPEDTEXTURE_TYPE_INFO
    }
}


pub const UIAUTOMAPPEDTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoMappedTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIAutoMappedTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UICppScreenData {
    pub state_stream_enabled: bool,
    pub state_stream_lane: UICppScreenStateStreamLaneType,
    pub field_of_view: f32,
    pub scale_up_and_keep_aspect_ratio: bool,
    pub flash_compatibility_mode: bool,
    pub screen_layout_width: f32,
    pub screen_layout_height: f32,
    pub allow_input: bool,
    pub eat_all_input: bool,
    pub layout_with_safe_zone: bool,
    pub screen_sampler_settings: super::game_base::UIScreenSamplerSettings,
}

pub const UICPPSCREENDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StateStreamEnabled",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, state_stream_enabled),
            },
            FieldInfoData {
                name: "StateStreamLane",
                flags: MemberInfoFlags::new(0),
                field_type: UICPPSCREENSTATESTREAMLANETYPE_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, state_stream_lane),
            },
            FieldInfoData {
                name: "FieldOfView",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, field_of_view),
            },
            FieldInfoData {
                name: "ScaleUpAndKeepAspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, scale_up_and_keep_aspect_ratio),
            },
            FieldInfoData {
                name: "FlashCompatibilityMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, flash_compatibility_mode),
            },
            FieldInfoData {
                name: "ScreenLayoutWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, screen_layout_width),
            },
            FieldInfoData {
                name: "ScreenLayoutHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, screen_layout_height),
            },
            FieldInfoData {
                name: "AllowInput",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, allow_input),
            },
            FieldInfoData {
                name: "EatAllInput",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, eat_all_input),
            },
            FieldInfoData {
                name: "LayoutWithSafeZone",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, layout_with_safe_zone),
            },
            FieldInfoData {
                name: "ScreenSamplerSettings",
                flags: MemberInfoFlags::new(0),
                field_type: UISCREENSAMPLERSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(UICppScreenData, screen_sampler_settings),
            },
        ],
    }),
    array_type: Some(UICPPSCREENDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UICppScreenData {
    fn type_info() -> &'static TypeInfo {
        UICPPSCREENDATA_TYPE_INFO
    }
}


pub const UICPPSCREENDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UICppScreenData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UICppScreenStateStreamLaneType {
    #[default]
    UICppScreenStateStreamLaneType_Primary = 0,
    UICppScreenStateStreamLaneType_Secondary = 1,
}

pub const UICPPSCREENSTATESTREAMLANETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenStateStreamLaneType",
    flags: MemberInfoFlags::new(49429),
    module: "GameSharedUI",
    data: TypeInfoData::Enum,
    array_type: Some(UICPPSCREENSTATESTREAMLANETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UICppScreenStateStreamLaneType {
    fn type_info() -> &'static TypeInfo {
        UICPPSCREENSTATESTREAMLANETYPE_TYPE_INFO
    }
}


pub const UICPPSCREENSTATESTREAMLANETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenStateStreamLaneType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UICppScreenStateStreamLaneType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIScreenRenderTargetEntityData {
    pub render_target: super::render::RenderTextureAsset,
    pub generate_render_target: bool,
    pub create_render_target_stencil: bool,
    pub clear_render_target: bool,
    pub render_on_event: bool,
}

pub const UISCREENRENDERTARGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderTargetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UISCREENRENDERENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RenderTarget",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERTEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, render_target),
            },
            FieldInfoData {
                name: "GenerateRenderTarget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, generate_render_target),
            },
            FieldInfoData {
                name: "CreateRenderTargetStencil",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, create_render_target_stencil),
            },
            FieldInfoData {
                name: "ClearRenderTarget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, clear_render_target),
            },
            FieldInfoData {
                name: "RenderOnEvent",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, render_on_event),
            },
        ],
    }),
    array_type: Some(UISCREENRENDERTARGETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIScreenRenderTargetEntityData {
    fn type_info() -> &'static TypeInfo {
        UISCREENRENDERTARGETENTITYDATA_TYPE_INFO
    }
}


pub const UISCREENRENDERTARGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderTargetEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIScreenRenderTargetEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIScreenRenderEntityData {
    pub screen_data: UICppScreenData,
    pub use_game_view_projection: bool,
    pub scale: f32,
    pub enable_depth_culling: bool,
    pub projection_mode: super::game_base::UIScreenProjectionMode,
    pub render_pass: UIScreenRenderingPass,
    pub update_order: i32,
    pub center_screen: bool,
    pub view_id: super::render_base::LocalPlayerViewId,
    pub start_enabled: bool,
    pub color: super::core::Vec3,
    pub alpha: f32,
}

pub const UISCREENRENDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ScreenData",
                flags: MemberInfoFlags::new(0),
                field_type: UICPPSCREENDATA_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, screen_data),
            },
            FieldInfoData {
                name: "UseGameViewProjection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, use_game_view_projection),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, scale),
            },
            FieldInfoData {
                name: "EnableDepthCulling",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, enable_depth_culling),
            },
            FieldInfoData {
                name: "ProjectionMode",
                flags: MemberInfoFlags::new(0),
                field_type: UISCREENPROJECTIONMODE_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, projection_mode),
            },
            FieldInfoData {
                name: "RenderPass",
                flags: MemberInfoFlags::new(0),
                field_type: UISCREENRENDERINGPASS_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, render_pass),
            },
            FieldInfoData {
                name: "UpdateOrder",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, update_order),
            },
            FieldInfoData {
                name: "CenterScreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, center_screen),
            },
            FieldInfoData {
                name: "ViewId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERVIEWID_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, view_id),
            },
            FieldInfoData {
                name: "StartEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, start_enabled),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, color),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIScreenRenderEntityData, alpha),
            },
        ],
    }),
    array_type: Some(UISCREENRENDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIScreenRenderEntityData {
    fn type_info() -> &'static TypeInfo {
        UISCREENRENDERENTITYDATA_TYPE_INFO
    }
}


pub const UISCREENRENDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIScreenRenderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIScreenRenderingPass {
    #[default]
    UIScreenRenderingPass_RenderTarget = 0,
    UIScreenRenderingPass_AboveFlash = 1,
    UIScreenRenderingPass_BelowFlash = 2,
    UIScreenRenderingPass_Dialog = 3,
    UIScreenRenderingPass_Count = 4,
}

pub const UISCREENRENDERINGPASS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderingPass",
    flags: MemberInfoFlags::new(49429),
    module: "GameSharedUI",
    data: TypeInfoData::Enum,
    array_type: Some(UISCREENRENDERINGPASS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIScreenRenderingPass {
    fn type_info() -> &'static TypeInfo {
        UISCREENRENDERINGPASS_TYPE_INFO
    }
}


pub const UISCREENRENDERINGPASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderingPass-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIScreenRenderingPass-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIIMSettingsAsset {
    pub auto_scroll_settings: UIAutoScrollTextSettings,
}

pub const UIIMSETTINGSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIIMSettingsAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AutoScrollSettings",
                flags: MemberInfoFlags::new(0),
                field_type: UIAUTOSCROLLTEXTSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(UIIMSettingsAsset, auto_scroll_settings),
            },
        ],
    }),
    array_type: Some(UIIMSETTINGSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIIMSettingsAsset {
    fn type_info() -> &'static TypeInfo {
        UIIMSETTINGSASSET_TYPE_INFO
    }
}


pub const UIIMSETTINGSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIIMSettingsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIIMSettingsAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIAutoScrollTextSettings {
    pub no_scroll_wait_time: f32,
    pub fully_scrolled_wait_time: f32,
    pub max_scroll_time: f32,
    pub pixels_per_second: f32,
    pub scrollback_multiplier: f32,
}

pub const UIAUTOSCROLLTEXTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoScrollTextSettings",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "NoScrollWaitTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIAutoScrollTextSettings, no_scroll_wait_time),
            },
            FieldInfoData {
                name: "FullyScrolledWaitTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIAutoScrollTextSettings, fully_scrolled_wait_time),
            },
            FieldInfoData {
                name: "MaxScrollTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIAutoScrollTextSettings, max_scroll_time),
            },
            FieldInfoData {
                name: "PixelsPerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIAutoScrollTextSettings, pixels_per_second),
            },
            FieldInfoData {
                name: "ScrollbackMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIAutoScrollTextSettings, scrollback_multiplier),
            },
        ],
    }),
    array_type: Some(UIAUTOSCROLLTEXTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIAutoScrollTextSettings {
    fn type_info() -> &'static TypeInfo {
        UIAUTOSCROLLTEXTSETTINGS_TYPE_INFO
    }
}


pub const UIAUTOSCROLLTEXTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoScrollTextSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIAutoScrollTextSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementWidgetReferenceEntityData {
    pub instance_name: String,
    pub instance_name_hash: u32,
    pub inclusion_settings: UIElementInclusionSettings,
    pub transform_pivot: super::core::Vec3,
    pub use_element_size: bool,
    pub size: super::core::Vec2,
    pub layout_mode: UILayoutMode,
    pub offset: UIElementOffset,
    pub anchor: UIElementAnchor,
    pub position: UIElementOffset,
    pub expansion: UIElementRectExpansion,
    pub color: super::core::Vec3,
    pub alpha: f32,
    pub code_access_identifier: String,
}

pub const UIELEMENTWIDGETREFERENCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementWidgetReferenceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InstanceName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, instance_name),
            },
            FieldInfoData {
                name: "InstanceNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, instance_name_hash),
            },
            FieldInfoData {
                name: "InclusionSettings",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTINCLUSIONSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, inclusion_settings),
            },
            FieldInfoData {
                name: "TransformPivot",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, transform_pivot),
            },
            FieldInfoData {
                name: "UseElementSize",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, use_element_size),
            },
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, size),
            },
            FieldInfoData {
                name: "LayoutMode",
                flags: MemberInfoFlags::new(0),
                field_type: UILAYOUTMODE_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, layout_mode),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTOFFSET_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, offset),
            },
            FieldInfoData {
                name: "Anchor",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTANCHOR_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, anchor),
            },
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTOFFSET_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, position),
            },
            FieldInfoData {
                name: "Expansion",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTRECTEXPANSION_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, expansion),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, color),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, alpha),
            },
            FieldInfoData {
                name: "CodeAccessIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, code_access_identifier),
            },
        ],
    }),
    array_type: Some(UIELEMENTWIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementWidgetReferenceEntityData {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTWIDGETREFERENCEENTITYDATA_TYPE_INFO
    }
}


pub const UIELEMENTWIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementWidgetReferenceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementWidgetReferenceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementBitmapDistanceFieldParams {
    pub alpha_threshold: f32,
    pub distance_scale: f32,
    pub outline_inner: f32,
    pub outline_outer: f32,
    pub outline_color: UIElementColor,
}

pub const UIELEMENTBITMAPDISTANCEFIELDPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementBitmapDistanceFieldParams",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AlphaThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, alpha_threshold),
            },
            FieldInfoData {
                name: "DistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, distance_scale),
            },
            FieldInfoData {
                name: "OutlineInner",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, outline_inner),
            },
            FieldInfoData {
                name: "OutlineOuter",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, outline_outer),
            },
            FieldInfoData {
                name: "OutlineColor",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTCOLOR_TYPE_INFO,
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, outline_color),
            },
        ],
    }),
    array_type: Some(UIELEMENTBITMAPDISTANCEFIELDPARAMS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementBitmapDistanceFieldParams {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTBITMAPDISTANCEFIELDPARAMS_TYPE_INFO
    }
}


pub const UIELEMENTBITMAPDISTANCEFIELDPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementBitmapDistanceFieldParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementBitmapDistanceFieldParams-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIMaskingContainerEntityData {
    pub masks: Vec<super::entity::GameObjectData>,
    pub mask_threshold: f32,
}

pub const UIMASKINGCONTAINERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingContainerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UICONTAINERENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Masks",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIMaskingContainerEntityData, masks),
            },
            FieldInfoData {
                name: "MaskThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIMaskingContainerEntityData, mask_threshold),
            },
        ],
    }),
    array_type: Some(UIMASKINGCONTAINERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIMaskingContainerEntityData {
    fn type_info() -> &'static TypeInfo {
        UIMASKINGCONTAINERENTITYDATA_TYPE_INFO
    }
}


pub const UIMASKINGCONTAINERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingContainerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIMaskingContainerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIContainerEntityData {
    pub elements: Vec<super::entity::GameObjectData>,
}

pub const UICONTAINERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIContainerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Elements",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIContainerEntityData, elements),
            },
        ],
    }),
    array_type: Some(UICONTAINERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIContainerEntityData {
    fn type_info() -> &'static TypeInfo {
        UICONTAINERENTITYDATA_TYPE_INFO
    }
}


pub const UICONTAINERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIContainerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIContainerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementEntityData {
    pub instance_name: String,
    pub instance_name_hash: u32,
    pub transform_pivot: super::core::Vec3,
    pub size: super::core::Vec2,
    pub layout_mode: UILayoutMode,
    pub offset: UIElementOffset,
    pub anchor: UIElementAnchor,
    pub position: UIElementOffset,
    pub expansion: UIElementRectExpansion,
    pub visible: bool,
    pub color: super::core::Vec3,
    pub alpha: f32,
    pub transform: super::core::LinearTransform,
}

pub const UIELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InstanceName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, instance_name),
            },
            FieldInfoData {
                name: "InstanceNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, instance_name_hash),
            },
            FieldInfoData {
                name: "TransformPivot",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, transform_pivot),
            },
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, size),
            },
            FieldInfoData {
                name: "LayoutMode",
                flags: MemberInfoFlags::new(0),
                field_type: UILAYOUTMODE_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, layout_mode),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTOFFSET_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, offset),
            },
            FieldInfoData {
                name: "Anchor",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTANCHOR_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, anchor),
            },
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTOFFSET_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, position),
            },
            FieldInfoData {
                name: "Expansion",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTRECTEXPANSION_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, expansion),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, visible),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, color),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, alpha),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(UIElementEntityData, transform),
            },
        ],
    }),
    array_type: Some(UIELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIElementEntityData {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTENTITYDATA_TYPE_INFO
    }
}


pub const UIELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIWidgetEntityData {
    pub size: UIElementSize,
    pub layers: Vec<UIElementLayerEntityData>,
    pub texture_mappings: Vec<UITextureMappingAsset>,
    pub components: Vec<super::entity::GameObjectData>,
    pub visible: bool,
}

pub const UIWIDGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTSIZE_TYPE_INFO,
                rust_offset: offset_of!(UIWidgetEntityData, size),
            },
            FieldInfoData {
                name: "Layers",
                flags: MemberInfoFlags::new(144),
                field_type: UIELEMENTLAYERENTITYDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIWidgetEntityData, layers),
            },
            FieldInfoData {
                name: "TextureMappings",
                flags: MemberInfoFlags::new(144),
                field_type: UITEXTUREMAPPINGASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIWidgetEntityData, texture_mappings),
            },
            FieldInfoData {
                name: "Components",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIWidgetEntityData, components),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIWidgetEntityData, visible),
            },
        ],
    }),
    array_type: Some(UIWIDGETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIWidgetEntityData {
    fn type_info() -> &'static TypeInfo {
        UIWIDGETENTITYDATA_TYPE_INFO
    }
}


pub const UIWIDGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIWidgetEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIElementLayerEntityData {
    pub elements: Vec<super::entity::GameObjectData>,
    pub inclusion_settings: UIElementInclusionSettings,
    pub visible: bool,
    pub internal_layer_name: String,
}

pub const UIELEMENTLAYERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementLayerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Elements",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIElementLayerEntityData, elements),
            },
            FieldInfoData {
                name: "InclusionSettings",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTINCLUSIONSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(UIElementLayerEntityData, inclusion_settings),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementLayerEntityData, visible),
            },
            FieldInfoData {
                name: "InternalLayerName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIElementLayerEntityData, internal_layer_name),
            },
        ],
    }),
    array_type: Some(UIELEMENTLAYERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementLayerEntityData {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTLAYERENTITYDATA_TYPE_INFO
    }
}


pub const UIELEMENTLAYERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementLayerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementLayerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIElementInclusionSettings {
    pub is_singleplayer_layer: bool,
    pub is_multiplayer_layer: bool,
    pub custom_inclusion_critera: Vec<String>,
    pub is_s_d_layer: bool,
    pub is_h_d_layer: bool,
}

pub const UIELEMENTINCLUSIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementInclusionSettings",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "IsSingleplayerLayer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementInclusionSettings, is_singleplayer_layer),
            },
            FieldInfoData {
                name: "IsMultiplayerLayer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementInclusionSettings, is_multiplayer_layer),
            },
            FieldInfoData {
                name: "CustomInclusionCritera",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIElementInclusionSettings, custom_inclusion_critera),
            },
            FieldInfoData {
                name: "IsSDLayer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementInclusionSettings, is_s_d_layer),
            },
            FieldInfoData {
                name: "IsHDLayer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementInclusionSettings, is_h_d_layer),
            },
        ],
    }),
    array_type: Some(UIELEMENTINCLUSIONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementInclusionSettings {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTINCLUSIONSETTINGS_TYPE_INFO
    }
}


pub const UIELEMENTINCLUSIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementInclusionSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementInclusionSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIBlendMode {
    #[default]
    UIBlendMode_Solid = 0,
    UIBlendMode_AlphaBlend = 1,
    UIBlendMode_Lighten = 2,
    UIBlendMode_Darken = 3,
    UIBlendMode_Add = 4,
    UIBlendMode_Subtract = 5,
    UIBlendMode_Count = 6,
    UIBlendMode_Passthrough = 7,
}

pub const UIBLENDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIBlendMode",
    flags: MemberInfoFlags::new(49429),
    module: "GameSharedUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIBLENDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIBlendMode {
    fn type_info() -> &'static TypeInfo {
        UIBLENDMODE_TYPE_INFO
    }
}


pub const UIBLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIBlendMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIBlendMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementGradient {
    pub top_left_color: UIElementColor,
    pub top_right_color: UIElementColor,
    pub bottom_left_color: UIElementColor,
    pub bottom_right_color: UIElementColor,
}

pub const UIELEMENTGRADIENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementGradient",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TopLeftColor",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTCOLOR_TYPE_INFO,
                rust_offset: offset_of!(UIElementGradient, top_left_color),
            },
            FieldInfoData {
                name: "TopRightColor",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTCOLOR_TYPE_INFO,
                rust_offset: offset_of!(UIElementGradient, top_right_color),
            },
            FieldInfoData {
                name: "BottomLeftColor",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTCOLOR_TYPE_INFO,
                rust_offset: offset_of!(UIElementGradient, bottom_left_color),
            },
            FieldInfoData {
                name: "BottomRightColor",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTCOLOR_TYPE_INFO,
                rust_offset: offset_of!(UIElementGradient, bottom_right_color),
            },
        ],
    }),
    array_type: Some(UIELEMENTGRADIENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementGradient {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTGRADIENT_TYPE_INFO
    }
}


pub const UIELEMENTGRADIENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementGradient-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementGradient-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementAnchor {
    pub x: f32,
    pub y: f32,
}

pub const UIELEMENTANCHOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementAnchor",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementAnchor, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementAnchor, y),
            },
        ],
    }),
    array_type: Some(UIELEMENTANCHOR_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIElementAnchor {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTANCHOR_TYPE_INFO
    }
}


pub const UIELEMENTANCHOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementAnchor-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementAnchor-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementSize {
    pub x: f32,
    pub y: f32,
}

pub const UIELEMENTSIZE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementSize",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementSize, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementSize, y),
            },
        ],
    }),
    array_type: Some(UIELEMENTSIZE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIElementSize {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTSIZE_TYPE_INFO
    }
}


pub const UIELEMENTSIZE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementSize-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementSize-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementOffset {
    pub x: f32,
    pub y: f32,
}

pub const UIELEMENTOFFSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementOffset",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementOffset, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementOffset, y),
            },
        ],
    }),
    array_type: Some(UIELEMENTOFFSET_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIElementOffset {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTOFFSET_TYPE_INFO
    }
}


pub const UIELEMENTOFFSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementOffset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementOffset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementRectExpansion {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub const UIELEMENTRECTEXPANSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementRectExpansion",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementRectExpansion, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementRectExpansion, y),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementRectExpansion, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementRectExpansion, height),
            },
        ],
    }),
    array_type: Some(UIELEMENTRECTEXPANSION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIElementRectExpansion {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTRECTEXPANSION_TYPE_INFO
    }
}


pub const UIELEMENTRECTEXPANSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementRectExpansion-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementRectExpansion-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementFontStyle {
    pub hd: UIElementFontDefinition,
    pub sd: UIElementFontDefinition,
    pub color: UIElementColor,
    pub language_overrides: Vec<UIElementFontDefinitionOverride>,
}

pub const UIELEMENTFONTSTYLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontStyle",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTFONTSTYLEBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Hd",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTFONTDEFINITION_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontStyle, hd),
            },
            FieldInfoData {
                name: "Sd",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTFONTDEFINITION_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontStyle, sd),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTCOLOR_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontStyle, color),
            },
            FieldInfoData {
                name: "LanguageOverrides",
                flags: MemberInfoFlags::new(144),
                field_type: UIELEMENTFONTDEFINITIONOVERRIDE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontStyle, language_overrides),
            },
        ],
    }),
    array_type: Some(UIELEMENTFONTSTYLE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementFontStyle {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTFONTSTYLE_TYPE_INFO
    }
}


pub const UIELEMENTFONTSTYLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontStyle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontStyle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIElementFontEffect {
    pub effect_script: String,
}

pub const UIELEMENTFONTEFFECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontEffect",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTFONTEFFECTBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EffectScript",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontEffect, effect_script),
            },
        ],
    }),
    array_type: Some(UIELEMENTFONTEFFECT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementFontEffect {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTFONTEFFECT_TYPE_INFO
    }
}


pub const UIELEMENTFONTEFFECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontEffect-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontEffect-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementFontDefinition {
    pub font_lookup: Vec<UIImmediateModeFontLookup>,
    pub point_size: f32,
    pub letter_spacing: f32,
    pub row_spacing: i32,
}

pub const UIELEMENTFONTDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinition",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FontLookup",
                flags: MemberInfoFlags::new(144),
                field_type: UIIMMEDIATEMODEFONTLOOKUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontDefinition, font_lookup),
            },
            FieldInfoData {
                name: "PointSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontDefinition, point_size),
            },
            FieldInfoData {
                name: "LetterSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontDefinition, letter_spacing),
            },
            FieldInfoData {
                name: "RowSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontDefinition, row_spacing),
            },
        ],
    }),
    array_type: Some(UIELEMENTFONTDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementFontDefinition {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTFONTDEFINITION_TYPE_INFO
    }
}


pub const UIELEMENTFONTDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontDefinition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementFontDefinitionOverride {
    pub language: super::core::LanguageFormat,
    pub hd: UIElementFontDefinition,
    pub sd: UIElementFontDefinition,
}

pub const UIELEMENTFONTDEFINITIONOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinitionOverride",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: LANGUAGEFORMAT_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontDefinitionOverride, language),
            },
            FieldInfoData {
                name: "Hd",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTFONTDEFINITION_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontDefinitionOverride, hd),
            },
            FieldInfoData {
                name: "Sd",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTFONTDEFINITION_TYPE_INFO,
                rust_offset: offset_of!(UIElementFontDefinitionOverride, sd),
            },
        ],
    }),
    array_type: Some(UIELEMENTFONTDEFINITIONOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementFontDefinitionOverride {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTFONTDEFINITIONOVERRIDE_TYPE_INFO
    }
}


pub const UIELEMENTFONTDEFINITIONOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinitionOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontDefinitionOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementTextFilterGlow {
    pub knock_out: bool,
    pub hide_object: bool,
    pub fine_blur: bool,
    pub x: f32,
    pub y: f32,
    pub strength: f32,
    pub color: UIElementColor,
}

pub const UIELEMENTTEXTFILTERGLOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterGlow",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTTEXTFILTER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "KnockOut",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterGlow, knock_out),
            },
            FieldInfoData {
                name: "HideObject",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterGlow, hide_object),
            },
            FieldInfoData {
                name: "FineBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterGlow, fine_blur),
            },
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterGlow, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterGlow, y),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterGlow, strength),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTCOLOR_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterGlow, color),
            },
        ],
    }),
    array_type: Some(UIELEMENTTEXTFILTERGLOW_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementTextFilterGlow {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTTEXTFILTERGLOW_TYPE_INFO
    }
}


pub const UIELEMENTTEXTFILTERGLOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterGlow-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilterGlow-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementTextFilterDropShadow {
    pub knock_out: bool,
    pub hide_object: bool,
    pub fine_blur: bool,
    pub x: f32,
    pub y: f32,
    pub strength: f32,
    pub color: UIElementColor,
    pub angle: f32,
    pub distance: f32,
}

pub const UIELEMENTTEXTFILTERDROPSHADOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterDropShadow",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTTEXTFILTER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "KnockOut",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterDropShadow, knock_out),
            },
            FieldInfoData {
                name: "HideObject",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterDropShadow, hide_object),
            },
            FieldInfoData {
                name: "FineBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterDropShadow, fine_blur),
            },
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterDropShadow, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterDropShadow, y),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterDropShadow, strength),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTCOLOR_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterDropShadow, color),
            },
            FieldInfoData {
                name: "Angle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterDropShadow, angle),
            },
            FieldInfoData {
                name: "Distance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterDropShadow, distance),
            },
        ],
    }),
    array_type: Some(UIELEMENTTEXTFILTERDROPSHADOW_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementTextFilterDropShadow {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTTEXTFILTERDROPSHADOW_TYPE_INFO
    }
}


pub const UIELEMENTTEXTFILTERDROPSHADOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterDropShadow-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilterDropShadow-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementTextFilterBlur {
    pub x: f32,
    pub y: f32,
    pub strength: f32,
}

pub const UIELEMENTTEXTFILTERBLUR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterBlur",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTTEXTFILTER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterBlur, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterBlur, y),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementTextFilterBlur, strength),
            },
        ],
    }),
    array_type: Some(UIELEMENTTEXTFILTERBLUR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementTextFilterBlur {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTTEXTFILTERBLUR_TYPE_INFO
    }
}


pub const UIELEMENTTEXTFILTERBLUR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterBlur-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilterBlur-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIElementTextFilter {
}

pub const UIELEMENTTEXTFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilter",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTTEXTFILTER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementTextFilter {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTTEXTFILTER_TYPE_INFO
    }
}


pub const UIELEMENTTEXTFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIElementColor {
    pub rgb: super::core::Vec3,
    pub alpha: f32,
}

pub const UIELEMENTCOLOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementColor",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Rgb",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(UIElementColor, rgb),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIElementColor, alpha),
            },
        ],
    }),
    array_type: Some(UIELEMENTCOLOR_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementColor {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTCOLOR_TYPE_INFO
    }
}


pub const UIELEMENTCOLOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementColor-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementColor-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UITextureMappingAsset {
    pub compartment: UITextureMappingCompartment,
    pub output: Vec<UITextureMappingOutputEntry>,
    pub disable_atlas: bool,
    pub force_atlas: bool,
}

pub const UITEXTUREMAPPINGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UITEXTUREMAPPINGBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Compartment",
                flags: MemberInfoFlags::new(0),
                field_type: UITEXTUREMAPPINGCOMPARTMENT_TYPE_INFO,
                rust_offset: offset_of!(UITextureMappingAsset, compartment),
            },
            FieldInfoData {
                name: "Output",
                flags: MemberInfoFlags::new(144),
                field_type: UITEXTUREMAPPINGOUTPUTENTRY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UITextureMappingAsset, output),
            },
            FieldInfoData {
                name: "DisableAtlas",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UITextureMappingAsset, disable_atlas),
            },
            FieldInfoData {
                name: "ForceAtlas",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UITextureMappingAsset, force_atlas),
            },
        ],
    }),
    array_type: Some(UITEXTUREMAPPINGASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UITextureMappingAsset {
    fn type_info() -> &'static TypeInfo {
        UITEXTUREMAPPINGASSET_TYPE_INFO
    }
}


pub const UITEXTUREMAPPINGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITextureMappingAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UILayoutMode {
    #[default]
    UILayoutMode_AnchorOffset = 0,
    UILayoutMode_PositionExpansion = 1,
}

pub const UILAYOUTMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UILayoutMode",
    flags: MemberInfoFlags::new(49429),
    module: "GameSharedUI",
    data: TypeInfoData::Enum,
    array_type: Some(UILAYOUTMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UILayoutMode {
    fn type_info() -> &'static TypeInfo {
        UILAYOUTMODE_TYPE_INFO
    }
}


pub const UILAYOUTMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UILayoutMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UILayoutMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UITextureMappingCompartment {
    #[default]
    UITextureMappingCompartment_Default = 0,
    UITextureMappingCompartment_Static = 1,
}

pub const UITEXTUREMAPPINGCOMPARTMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingCompartment",
    flags: MemberInfoFlags::new(49429),
    module: "GameSharedUI",
    data: TypeInfoData::Enum,
    array_type: Some(UITEXTUREMAPPINGCOMPARTMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UITextureMappingCompartment {
    fn type_info() -> &'static TypeInfo {
        UITEXTUREMAPPINGCOMPARTMENT_TYPE_INFO
    }
}


pub const UITEXTUREMAPPINGCOMPARTMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingCompartment-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITextureMappingCompartment-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UITextureMappingOutputEntry {
    pub id: i32,
    pub texture_ref: super::core::ResourceRef,
    pub uv_rect: super::core::Vec4,
}

pub const UITEXTUREMAPPINGOUTPUTENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingOutputEntry",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UITextureMappingOutputEntry, id),
            },
            FieldInfoData {
                name: "TextureRef",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(UITextureMappingOutputEntry, texture_ref),
            },
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UITextureMappingOutputEntry, uv_rect),
            },
        ],
    }),
    array_type: Some(UITEXTUREMAPPINGOUTPUTENTRY_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UITextureMappingOutputEntry {
    fn type_info() -> &'static TypeInfo {
        UITEXTUREMAPPINGOUTPUTENTRY_TYPE_INFO
    }
}


pub const UITEXTUREMAPPINGOUTPUTENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingOutputEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITextureMappingOutputEntry-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIWidgetBlueprint {
}

pub const UIWIDGETBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBJECTBLUEPRINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIWIDGETBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIWidgetBlueprint {
    fn type_info() -> &'static TypeInfo {
        UIWIDGETBLUEPRINT_TYPE_INFO
    }
}


pub const UIWIDGETBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIWidgetBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIImmediateModeFontConfigurationAsset {
    pub font_bundles: Vec<UIImmediateModeFontBundle>,
    pub font_dpi: i32,
    pub glyph_cache_size: i32,
    pub glyph_cache_size_low_end: i32,
    pub glyph_cache_padding: i32,
    pub auto_hinting: bool,
}

pub const UIIMMEDIATEMODEFONTCONFIGURATIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontConfigurationAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIFONTCONFIGURATIONASSETBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FontBundles",
                flags: MemberInfoFlags::new(144),
                field_type: UIIMMEDIATEMODEFONTBUNDLE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, font_bundles),
            },
            FieldInfoData {
                name: "FontDpi",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, font_dpi),
            },
            FieldInfoData {
                name: "GlyphCacheSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, glyph_cache_size),
            },
            FieldInfoData {
                name: "GlyphCacheSizeLowEnd",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, glyph_cache_size_low_end),
            },
            FieldInfoData {
                name: "GlyphCachePadding",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, glyph_cache_padding),
            },
            FieldInfoData {
                name: "AutoHinting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, auto_hinting),
            },
        ],
    }),
    array_type: Some(UIIMMEDIATEMODEFONTCONFIGURATIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIImmediateModeFontConfigurationAsset {
    fn type_info() -> &'static TypeInfo {
        UIIMMEDIATEMODEFONTCONFIGURATIONASSET_TYPE_INFO
    }
}


pub const UIIMMEDIATEMODEFONTCONFIGURATIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontConfigurationAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIImmediateModeFontConfigurationAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIImmediateModeFontLookup {
    pub language: super::core::LanguageFormat,
    pub font_asset_path: String,
}

pub const UIIMMEDIATEMODEFONTLOOKUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontLookup",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: LANGUAGEFORMAT_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontLookup, language),
            },
            FieldInfoData {
                name: "FontAssetPath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontLookup, font_asset_path),
            },
        ],
    }),
    array_type: Some(UIIMMEDIATEMODEFONTLOOKUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIImmediateModeFontLookup {
    fn type_info() -> &'static TypeInfo {
        UIIMMEDIATEMODEFONTLOOKUP_TYPE_INFO
    }
}


pub const UIIMMEDIATEMODEFONTLOOKUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontLookup-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIImmediateModeFontLookup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIImmediateModeFontBundle {
    pub language: super::core::LanguageFormat,
    pub bundle_path: String,
    pub ttf_assets: Vec<UITtfAsset>,
}

pub const UIIMMEDIATEMODEFONTBUNDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontBundle",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: LANGUAGEFORMAT_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontBundle, language),
            },
            FieldInfoData {
                name: "BundlePath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontBundle, bundle_path),
            },
            FieldInfoData {
                name: "TtfAssets",
                flags: MemberInfoFlags::new(144),
                field_type: UITTFASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIImmediateModeFontBundle, ttf_assets),
            },
        ],
    }),
    array_type: Some(UIIMMEDIATEMODEFONTBUNDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIImmediateModeFontBundle {
    fn type_info() -> &'static TypeInfo {
        UIIMMEDIATEMODEFONTBUNDLE_TYPE_INFO
    }
}


pub const UIIMMEDIATEMODEFONTBUNDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontBundle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIImmediateModeFontBundle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UITtfAsset {
    pub font_family_name: String,
    pub italic: bool,
    pub bold: bool,
    pub font_resource: super::core::ResourceRef,
}

pub const UITTFASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITtfAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FontFamilyName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UITtfAsset, font_family_name),
            },
            FieldInfoData {
                name: "Italic",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UITtfAsset, italic),
            },
            FieldInfoData {
                name: "Bold",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UITtfAsset, bold),
            },
            FieldInfoData {
                name: "FontResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(UITtfAsset, font_resource),
            },
        ],
    }),
    array_type: Some(UITTFASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UITtfAsset {
    fn type_info() -> &'static TypeInfo {
        UITTFASSET_TYPE_INFO
    }
}


pub const UITTFASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITtfAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITtfAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIInputEntityData {
}

pub const UIINPUTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIINPUTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIInputEntityData {
    fn type_info() -> &'static TypeInfo {
        UIINPUTENTITYDATA_TYPE_INFO
    }
}


pub const UIINPUTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIInputEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MovieTrackData {
    pub keyframes: Vec<MovieTrackKeyframe>,
    pub volume: f32,
    pub expose_on_movie_started: bool,
    pub disable_world_renderer: bool,
}

pub const MOVIETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GUIDETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: MOVIETRACKKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MovieTrackData, keyframes),
            },
            FieldInfoData {
                name: "Volume",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MovieTrackData, volume),
            },
            FieldInfoData {
                name: "ExposeOnMovieStarted",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieTrackData, expose_on_movie_started),
            },
            FieldInfoData {
                name: "DisableWorldRenderer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieTrackData, disable_world_renderer),
            },
        ],
    }),
    array_type: Some(MOVIETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieTrackData {
    fn type_info() -> &'static TypeInfo {
        MOVIETRACKDATA_TYPE_INFO
    }
}


pub const MOVIETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("MovieTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MovieTrackKeyframe {
    pub time: f32,
    pub length: f32,
    pub movie: super::movie_base::MovieTextureBaseAsset,
    pub pause_on_ending: bool,
    pub renderable_count: u32,
    pub thread_count: u32,
}

pub const MOVIETRACKKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MovieTrackKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MovieTrackKeyframe, length),
            },
            FieldInfoData {
                name: "Movie",
                flags: MemberInfoFlags::new(0),
                field_type: MOVIETEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MovieTrackKeyframe, movie),
            },
            FieldInfoData {
                name: "PauseOnEnding",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieTrackKeyframe, pause_on_ending),
            },
            FieldInfoData {
                name: "RenderableCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MovieTrackKeyframe, renderable_count),
            },
            FieldInfoData {
                name: "ThreadCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MovieTrackKeyframe, thread_count),
            },
        ],
    }),
    array_type: Some(MOVIETRACKKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieTrackKeyframe {
    fn type_info() -> &'static TypeInfo {
        MOVIETRACKKEYFRAME_TYPE_INFO
    }
}


pub const MOVIETRACKKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("MovieTrackKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


