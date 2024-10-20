use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_game_base_types(registry: &mut TypeRegistry) {
    registry.register_type(UIIMSCREENDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIIMSCREENDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UISCREENSAMPLERSETTINGS_TYPE_INFO);
    registry.register_type(UISCREENSAMPLERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(UIIMSCREENSTATICSTATE_TYPE_INFO);
    registry.register_type(UIIMSCREENSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIIMREVERSEHANDLE_TYPE_INFO);
    registry.register_type(UIIMREVERSEHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(UIIMCOMMANDHANDLE_TYPE_INFO);
    registry.register_type(UIIMCOMMANDHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(UIIMSCREENHANDLE_TYPE_INFO);
    registry.register_type(UIIMSCREENHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(UIIMTEXTCOMMANDCONFIG_TYPE_INFO);
    registry.register_type(UIIMTEXTCOMMANDCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(UITEXTUREMAPPINGASSETBINDING_TYPE_INFO);
    registry.register_type(UITEXTUREMAPPINGASSETBINDING_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTFONTEFFECTBASEASSET_TYPE_INFO);
    registry.register_type(UIELEMENTFONTEFFECTBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTFONTSTYLEBASEASSET_TYPE_INFO);
    registry.register_type(UIELEMENTFONTSTYLEBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(UITEXTUREMAPPINGBASEASSET_TYPE_INFO);
    registry.register_type(UITEXTUREMAPPINGBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(UISCREENPROJECTIONMODE_TYPE_INFO);
    registry.register_type(UISCREENPROJECTIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTALIGNMENT_TYPE_INFO);
    registry.register_type(UIELEMENTALIGNMENT_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct UIImScreenDynamicState {
    pub enabled: bool,
    pub commands: Vec<UIImCommandHandle>,
    pub enable_depth_culling: bool,
    pub global_fade_value: f32,
    pub render_pass: i32,
    pub update_order: i32,
    pub render_target: super::render_base::TextureBaseAsset,
    pub create_render_target_stencil: bool,
    pub render_target_on_trigger: bool,
    pub render_target_trigger_count: u32,
    pub clear_render_target: bool,
    pub viewport_size: super::core::Vec2,
    pub display_rect: super::render_base::ViewportRect,
    pub preferred_rect: super::render_base::ViewportRect,
    pub virtual_screen_size: super::core::Vec2,
    pub screen_layout: super::core::Vec2,
    pub transform: super::state_stream::TransformSpaceHandle,
    pub scale: f32,
    pub center_screen: bool,
    pub projection_mode: UIScreenProjectionMode,
    pub use_game_view_projection: bool,
    pub normalized_mouse_position: super::core::Vec2,
    pub texture_bindings: Vec<UITextureMappingAssetBinding>,
    pub unmapped_textures: Vec<super::render_base::TextureResourceHandle>,
    pub screen_sampler_settings: UIScreenSamplerSettings,
    pub field_flag_changed0: u32,
}

pub const UIIMSCREENDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, enabled),
            },
            FieldInfoData {
                name: "Commands",
                flags: MemberInfoFlags::new(144),
                field_type: UIIMCOMMANDHANDLE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, commands),
            },
            FieldInfoData {
                name: "EnableDepthCulling",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, enable_depth_culling),
            },
            FieldInfoData {
                name: "GlobalFadeValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, global_fade_value),
            },
            FieldInfoData {
                name: "RenderPass",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, render_pass),
            },
            FieldInfoData {
                name: "UpdateOrder",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, update_order),
            },
            FieldInfoData {
                name: "RenderTarget",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, render_target),
            },
            FieldInfoData {
                name: "CreateRenderTargetStencil",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, create_render_target_stencil),
            },
            FieldInfoData {
                name: "RenderTargetOnTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, render_target_on_trigger),
            },
            FieldInfoData {
                name: "RenderTargetTriggerCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, render_target_trigger_count),
            },
            FieldInfoData {
                name: "ClearRenderTarget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, clear_render_target),
            },
            FieldInfoData {
                name: "ViewportSize",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, viewport_size),
            },
            FieldInfoData {
                name: "DisplayRect",
                flags: MemberInfoFlags::new(0),
                field_type: VIEWPORTRECT_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, display_rect),
            },
            FieldInfoData {
                name: "PreferredRect",
                flags: MemberInfoFlags::new(0),
                field_type: VIEWPORTRECT_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, preferred_rect),
            },
            FieldInfoData {
                name: "VirtualScreenSize",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, virtual_screen_size),
            },
            FieldInfoData {
                name: "ScreenLayout",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, screen_layout),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, transform),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, scale),
            },
            FieldInfoData {
                name: "CenterScreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, center_screen),
            },
            FieldInfoData {
                name: "ProjectionMode",
                flags: MemberInfoFlags::new(0),
                field_type: UISCREENPROJECTIONMODE_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, projection_mode),
            },
            FieldInfoData {
                name: "UseGameViewProjection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, use_game_view_projection),
            },
            FieldInfoData {
                name: "NormalizedMousePosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, normalized_mouse_position),
            },
            FieldInfoData {
                name: "TextureBindings",
                flags: MemberInfoFlags::new(144),
                field_type: UITEXTUREMAPPINGASSETBINDING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, texture_bindings),
            },
            FieldInfoData {
                name: "UnmappedTextures",
                flags: MemberInfoFlags::new(144),
                field_type: TEXTURERESOURCEHANDLE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, unmapped_textures),
            },
            FieldInfoData {
                name: "ScreenSamplerSettings",
                flags: MemberInfoFlags::new(0),
                field_type: UISCREENSAMPLERSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, screen_sampler_settings),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIIMSCREENDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIImScreenDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIIMSCREENDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIIMSCREENDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImScreenDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIScreenSamplerSettings {
    pub min_filter: super::render::TextureFilter,
    pub mag_filter: super::render::TextureFilter,
    pub mip_filter: super::render::TextureFilter,
    pub anisotropy_degree: i32,
}

pub const UISCREENSAMPLERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenSamplerSettings",
    flags: MemberInfoFlags::new(36937),
    module: "GameBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MinFilter",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREFILTER_TYPE_INFO,
                rust_offset: offset_of!(UIScreenSamplerSettings, min_filter),
            },
            FieldInfoData {
                name: "MagFilter",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREFILTER_TYPE_INFO,
                rust_offset: offset_of!(UIScreenSamplerSettings, mag_filter),
            },
            FieldInfoData {
                name: "MipFilter",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREFILTER_TYPE_INFO,
                rust_offset: offset_of!(UIScreenSamplerSettings, mip_filter),
            },
            FieldInfoData {
                name: "AnisotropyDegree",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIScreenSamplerSettings, anisotropy_degree),
            },
        ],
    }),
    array_type: Some(UISCREENSAMPLERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIScreenSamplerSettings {
    fn type_info() -> &'static TypeInfo {
        UISCREENSAMPLERSETTINGS_TYPE_INFO
    }
}


pub const UISCREENSAMPLERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenSamplerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIScreenSamplerSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIImScreenStaticState {
    pub view_index: u32,
    pub compartment: u16,
    pub field_of_view: f32,
    pub z_plane: f32,
    pub field_flag_changed0: u8,
}

pub const UIIMSCREENSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ViewIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenStaticState, view_index),
            },
            FieldInfoData {
                name: "Compartment",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenStaticState, compartment),
            },
            FieldInfoData {
                name: "FieldOfView",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenStaticState, field_of_view),
            },
            FieldInfoData {
                name: "ZPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenStaticState, z_plane),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIImScreenStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIIMSCREENSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIImScreenStaticState {
    fn type_info() -> &'static TypeInfo {
        UIIMSCREENSTATICSTATE_TYPE_INFO
    }
}


pub const UIIMSCREENSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImScreenStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIImReverseHandle {
}

pub const UIIMREVERSEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImReverseHandle",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(UIIMREVERSEHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIImReverseHandle {
    fn type_info() -> &'static TypeInfo {
        UIIMREVERSEHANDLE_TYPE_INFO
    }
}


pub const UIIMREVERSEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImReverseHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImReverseHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIImCommandHandle {
}

pub const UIIMCOMMANDHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImCommandHandle",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(UIIMCOMMANDHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIImCommandHandle {
    fn type_info() -> &'static TypeInfo {
        UIIMCOMMANDHANDLE_TYPE_INFO
    }
}


pub const UIIMCOMMANDHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImCommandHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImCommandHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIImScreenHandle {
}

pub const UIIMSCREENHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenHandle",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(UIIMSCREENHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIImScreenHandle {
    fn type_info() -> &'static TypeInfo {
        UIIMSCREENHANDLE_TYPE_INFO
    }
}


pub const UIIMSCREENHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImScreenHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIImTextCommandConfig {
    pub effect: UIElementFontEffectBaseAsset,
    pub style: UIElementFontStyleBaseAsset,
    pub horizontal_alignment: UIElementAlignment,
    pub vertical_alignment: UIElementAlignment,
    pub scale: f32,
    pub clip_to_rect: bool,
    pub wordwrap: bool,
    pub password_mode: bool,
    pub scrollable: bool,
    pub offset: f32,
}

pub const UIIMTEXTCOMMANDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImTextCommandConfig",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTFONTEFFECTBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(UIImTextCommandConfig, effect),
            },
            FieldInfoData {
                name: "Style",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTFONTSTYLEBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(UIImTextCommandConfig, style),
            },
            FieldInfoData {
                name: "HorizontalAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTALIGNMENT_TYPE_INFO,
                rust_offset: offset_of!(UIImTextCommandConfig, horizontal_alignment),
            },
            FieldInfoData {
                name: "VerticalAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTALIGNMENT_TYPE_INFO,
                rust_offset: offset_of!(UIImTextCommandConfig, vertical_alignment),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIImTextCommandConfig, scale),
            },
            FieldInfoData {
                name: "ClipToRect",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImTextCommandConfig, clip_to_rect),
            },
            FieldInfoData {
                name: "Wordwrap",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImTextCommandConfig, wordwrap),
            },
            FieldInfoData {
                name: "PasswordMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImTextCommandConfig, password_mode),
            },
            FieldInfoData {
                name: "Scrollable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIImTextCommandConfig, scrollable),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIImTextCommandConfig, offset),
            },
        ],
    }),
    array_type: Some(UIIMTEXTCOMMANDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIImTextCommandConfig {
    fn type_info() -> &'static TypeInfo {
        UIIMTEXTCOMMANDCONFIG_TYPE_INFO
    }
}


pub const UIIMTEXTCOMMANDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImTextCommandConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImTextCommandConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UITextureMappingAssetBinding {
    pub mapping: UITextureMappingBaseAsset,
    pub compartment: u16,
}

pub const UITEXTUREMAPPINGASSETBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingAssetBinding",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Mapping",
                flags: MemberInfoFlags::new(0),
                field_type: UITEXTUREMAPPINGBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(UITextureMappingAssetBinding, mapping),
            },
            FieldInfoData {
                name: "Compartment",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(UITextureMappingAssetBinding, compartment),
            },
        ],
    }),
    array_type: Some(UITEXTUREMAPPINGASSETBINDING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UITextureMappingAssetBinding {
    fn type_info() -> &'static TypeInfo {
        UITEXTUREMAPPINGASSETBINDING_TYPE_INFO
    }
}


pub const UITEXTUREMAPPINGASSETBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingAssetBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UITextureMappingAssetBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIElementFontEffectBaseAsset {
}

pub const UIELEMENTFONTEFFECTBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontEffectBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTFONTEFFECTBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementFontEffectBaseAsset {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTFONTEFFECTBASEASSET_TYPE_INFO
    }
}


pub const UIELEMENTFONTEFFECTBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontEffectBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIElementFontEffectBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIElementFontStyleBaseAsset {
}

pub const UIELEMENTFONTSTYLEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontStyleBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTFONTSTYLEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementFontStyleBaseAsset {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTFONTSTYLEBASEASSET_TYPE_INFO
    }
}


pub const UIELEMENTFONTSTYLEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontStyleBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIElementFontStyleBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UITextureMappingBaseAsset {
}

pub const UITEXTUREMAPPINGBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UITEXTUREMAPPINGBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UITextureMappingBaseAsset {
    fn type_info() -> &'static TypeInfo {
        UITEXTUREMAPPINGBASEASSET_TYPE_INFO
    }
}


pub const UITEXTUREMAPPINGBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UITextureMappingBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIScreenProjectionMode {
    #[default]
    UIScreenProjectionMode_Default = 0,
    UIScreenProjectionMode_Billboard = 1,
    UIScreenProjectionMode_BillboardCylindrical = 2,
    UIScreenProjectionMode_BillboardFixedSize = 3,
}

pub const UISCREENPROJECTIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionMode",
    flags: MemberInfoFlags::new(49429),
    module: "GameBase",
    data: TypeInfoData::Enum,
    array_type: Some(UISCREENPROJECTIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIScreenProjectionMode {
    fn type_info() -> &'static TypeInfo {
        UISCREENPROJECTIONMODE_TYPE_INFO
    }
}


pub const UISCREENPROJECTIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIScreenProjectionMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIElementAlignment {
    #[default]
    UIElementAlignment_Left = 0,
    UIElementAlignment_Right = 1,
    UIElementAlignment_Center = 2,
    UIElementAlignment_Top = 3,
    UIElementAlignment_Bottom = 4,
    UIElementAlignment_Justify = 5,
    UIElementAlignment_Count = 6,
}

pub const UIELEMENTALIGNMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementAlignment",
    flags: MemberInfoFlags::new(49429),
    module: "GameBase",
    data: TypeInfoData::Enum,
    array_type: Some(UIELEMENTALIGNMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIElementAlignment {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTALIGNMENT_TYPE_INFO
    }
}


pub const UIELEMENTALIGNMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementAlignment-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIElementAlignment-Array"),
    array_type: None,
    alignment: 8,
};


