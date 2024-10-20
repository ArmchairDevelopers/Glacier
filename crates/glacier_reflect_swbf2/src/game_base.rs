use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct UIImScreenDynamicState {
    pub enabled: bool,
    pub commands: Vec<UIImCommandHandle>,
    pub enable_depth_culling: bool,
    pub global_fade_value: f32,
    pub render_pass: i32,
    pub update_order: i32,
    pub render_target: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
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

pub trait UIImScreenDynamicStateTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn commands(&self) -> &Vec<UIImCommandHandle>;
    fn enable_depth_culling(&self) -> &bool;
    fn global_fade_value(&self) -> &f32;
    fn render_pass(&self) -> &i32;
    fn update_order(&self) -> &i32;
    fn render_target(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn create_render_target_stencil(&self) -> &bool;
    fn render_target_on_trigger(&self) -> &bool;
    fn render_target_trigger_count(&self) -> &u32;
    fn clear_render_target(&self) -> &bool;
    fn viewport_size(&self) -> &super::core::Vec2;
    fn display_rect(&self) -> &super::render_base::ViewportRect;
    fn preferred_rect(&self) -> &super::render_base::ViewportRect;
    fn virtual_screen_size(&self) -> &super::core::Vec2;
    fn screen_layout(&self) -> &super::core::Vec2;
    fn transform(&self) -> &super::state_stream::TransformSpaceHandle;
    fn scale(&self) -> &f32;
    fn center_screen(&self) -> &bool;
    fn projection_mode(&self) -> &UIScreenProjectionMode;
    fn use_game_view_projection(&self) -> &bool;
    fn normalized_mouse_position(&self) -> &super::core::Vec2;
    fn texture_bindings(&self) -> &Vec<UITextureMappingAssetBinding>;
    fn unmapped_textures(&self) -> &Vec<super::render_base::TextureResourceHandle>;
    fn screen_sampler_settings(&self) -> &UIScreenSamplerSettings;
    fn field_flag_changed0(&self) -> &u32;
}

impl UIImScreenDynamicStateTrait for UIImScreenDynamicState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn commands(&self) -> &Vec<UIImCommandHandle> {
        &self.commands
    }
    fn enable_depth_culling(&self) -> &bool {
        &self.enable_depth_culling
    }
    fn global_fade_value(&self) -> &f32 {
        &self.global_fade_value
    }
    fn render_pass(&self) -> &i32 {
        &self.render_pass
    }
    fn update_order(&self) -> &i32 {
        &self.update_order
    }
    fn render_target(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.render_target
    }
    fn create_render_target_stencil(&self) -> &bool {
        &self.create_render_target_stencil
    }
    fn render_target_on_trigger(&self) -> &bool {
        &self.render_target_on_trigger
    }
    fn render_target_trigger_count(&self) -> &u32 {
        &self.render_target_trigger_count
    }
    fn clear_render_target(&self) -> &bool {
        &self.clear_render_target
    }
    fn viewport_size(&self) -> &super::core::Vec2 {
        &self.viewport_size
    }
    fn display_rect(&self) -> &super::render_base::ViewportRect {
        &self.display_rect
    }
    fn preferred_rect(&self) -> &super::render_base::ViewportRect {
        &self.preferred_rect
    }
    fn virtual_screen_size(&self) -> &super::core::Vec2 {
        &self.virtual_screen_size
    }
    fn screen_layout(&self) -> &super::core::Vec2 {
        &self.screen_layout
    }
    fn transform(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform
    }
    fn scale(&self) -> &f32 {
        &self.scale
    }
    fn center_screen(&self) -> &bool {
        &self.center_screen
    }
    fn projection_mode(&self) -> &UIScreenProjectionMode {
        &self.projection_mode
    }
    fn use_game_view_projection(&self) -> &bool {
        &self.use_game_view_projection
    }
    fn normalized_mouse_position(&self) -> &super::core::Vec2 {
        &self.normalized_mouse_position
    }
    fn texture_bindings(&self) -> &Vec<UITextureMappingAssetBinding> {
        &self.texture_bindings
    }
    fn unmapped_textures(&self) -> &Vec<super::render_base::TextureResourceHandle> {
        &self.unmapped_textures
    }
    fn screen_sampler_settings(&self) -> &UIScreenSamplerSettings {
        &self.screen_sampler_settings
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static UIIMSCREENDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImScreenDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImScreenDynamicState, enabled),
            },
            FieldInfoData {
                name: "Commands",
                flags: MemberInfoFlags::new(144),
                field_type: "UIImCommandHandle-Array",
                rust_offset: offset_of!(UIImScreenDynamicState, commands),
            },
            FieldInfoData {
                name: "EnableDepthCulling",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImScreenDynamicState, enable_depth_culling),
            },
            FieldInfoData {
                name: "GlobalFadeValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIImScreenDynamicState, global_fade_value),
            },
            FieldInfoData {
                name: "RenderPass",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIImScreenDynamicState, render_pass),
            },
            FieldInfoData {
                name: "UpdateOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIImScreenDynamicState, update_order),
            },
            FieldInfoData {
                name: "RenderTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(UIImScreenDynamicState, render_target),
            },
            FieldInfoData {
                name: "CreateRenderTargetStencil",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImScreenDynamicState, create_render_target_stencil),
            },
            FieldInfoData {
                name: "RenderTargetOnTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImScreenDynamicState, render_target_on_trigger),
            },
            FieldInfoData {
                name: "RenderTargetTriggerCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UIImScreenDynamicState, render_target_trigger_count),
            },
            FieldInfoData {
                name: "ClearRenderTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImScreenDynamicState, clear_render_target),
            },
            FieldInfoData {
                name: "ViewportSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UIImScreenDynamicState, viewport_size),
            },
            FieldInfoData {
                name: "DisplayRect",
                flags: MemberInfoFlags::new(0),
                field_type: "ViewportRect",
                rust_offset: offset_of!(UIImScreenDynamicState, display_rect),
            },
            FieldInfoData {
                name: "PreferredRect",
                flags: MemberInfoFlags::new(0),
                field_type: "ViewportRect",
                rust_offset: offset_of!(UIImScreenDynamicState, preferred_rect),
            },
            FieldInfoData {
                name: "VirtualScreenSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UIImScreenDynamicState, virtual_screen_size),
            },
            FieldInfoData {
                name: "ScreenLayout",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UIImScreenDynamicState, screen_layout),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(UIImScreenDynamicState, transform),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIImScreenDynamicState, scale),
            },
            FieldInfoData {
                name: "CenterScreen",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImScreenDynamicState, center_screen),
            },
            FieldInfoData {
                name: "ProjectionMode",
                flags: MemberInfoFlags::new(0),
                field_type: "UIScreenProjectionMode",
                rust_offset: offset_of!(UIImScreenDynamicState, projection_mode),
            },
            FieldInfoData {
                name: "UseGameViewProjection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImScreenDynamicState, use_game_view_projection),
            },
            FieldInfoData {
                name: "NormalizedMousePosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UIImScreenDynamicState, normalized_mouse_position),
            },
            FieldInfoData {
                name: "TextureBindings",
                flags: MemberInfoFlags::new(144),
                field_type: "UITextureMappingAssetBinding-Array",
                rust_offset: offset_of!(UIImScreenDynamicState, texture_bindings),
            },
            FieldInfoData {
                name: "UnmappedTextures",
                flags: MemberInfoFlags::new(144),
                field_type: "TextureResourceHandle-Array",
                rust_offset: offset_of!(UIImScreenDynamicState, unmapped_textures),
            },
            FieldInfoData {
                name: "ScreenSamplerSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "UIScreenSamplerSettings",
                rust_offset: offset_of!(UIImScreenDynamicState, screen_sampler_settings),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UIImScreenDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIIMSCREENDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIImScreenDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIIMSCREENDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIIMSCREENDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImScreenDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIScreenSamplerSettings {
    pub min_filter: super::render::TextureFilter,
    pub mag_filter: super::render::TextureFilter,
    pub mip_filter: super::render::TextureFilter,
    pub anisotropy_degree: i32,
}

pub trait UIScreenSamplerSettingsTrait: TypeObject {
    fn min_filter(&self) -> &super::render::TextureFilter;
    fn mag_filter(&self) -> &super::render::TextureFilter;
    fn mip_filter(&self) -> &super::render::TextureFilter;
    fn anisotropy_degree(&self) -> &i32;
}

impl UIScreenSamplerSettingsTrait for UIScreenSamplerSettings {
    fn min_filter(&self) -> &super::render::TextureFilter {
        &self.min_filter
    }
    fn mag_filter(&self) -> &super::render::TextureFilter {
        &self.mag_filter
    }
    fn mip_filter(&self) -> &super::render::TextureFilter {
        &self.mip_filter
    }
    fn anisotropy_degree(&self) -> &i32 {
        &self.anisotropy_degree
    }
}

pub static UISCREENSAMPLERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenSamplerSettings",
    flags: MemberInfoFlags::new(36937),
    module: "GameBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIScreenSamplerSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MinFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureFilter",
                rust_offset: offset_of!(UIScreenSamplerSettings, min_filter),
            },
            FieldInfoData {
                name: "MagFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureFilter",
                rust_offset: offset_of!(UIScreenSamplerSettings, mag_filter),
            },
            FieldInfoData {
                name: "MipFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureFilter",
                rust_offset: offset_of!(UIScreenSamplerSettings, mip_filter),
            },
            FieldInfoData {
                name: "AnisotropyDegree",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIScreenSamplerSettings, anisotropy_degree),
            },
        ],
    }),
    array_type: Some(UISCREENSAMPLERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIScreenSamplerSettings {
    fn type_info(&self) -> &'static TypeInfo {
        UISCREENSAMPLERSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UISCREENSAMPLERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenSamplerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIScreenSamplerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIImScreenStaticState {
    pub view_index: u32,
    pub compartment: u16,
    pub field_of_view: f32,
    pub z_plane: f32,
    pub field_flag_changed0: u8,
}

pub trait UIImScreenStaticStateTrait: TypeObject {
    fn view_index(&self) -> &u32;
    fn compartment(&self) -> &u16;
    fn field_of_view(&self) -> &f32;
    fn z_plane(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u8;
}

impl UIImScreenStaticStateTrait for UIImScreenStaticState {
    fn view_index(&self) -> &u32 {
        &self.view_index
    }
    fn compartment(&self) -> &u16 {
        &self.compartment
    }
    fn field_of_view(&self) -> &f32 {
        &self.field_of_view
    }
    fn z_plane(&self) -> &f32 {
        &self.z_plane
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static UIIMSCREENSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImScreenStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ViewIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UIImScreenStaticState, view_index),
            },
            FieldInfoData {
                name: "Compartment",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(UIImScreenStaticState, compartment),
            },
            FieldInfoData {
                name: "FieldOfView",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIImScreenStaticState, field_of_view),
            },
            FieldInfoData {
                name: "ZPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIImScreenStaticState, z_plane),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIImScreenStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIIMSCREENSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIImScreenStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIIMSCREENSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIIMSCREENSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImScreenStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIImReverseHandle {
}

pub trait UIImReverseHandleTrait: TypeObject {
}

impl UIImReverseHandleTrait for UIImReverseHandle {
}

pub static UIIMREVERSEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImReverseHandle",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImReverseHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIIMREVERSEHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIImReverseHandle {
    fn type_info(&self) -> &'static TypeInfo {
        UIIMREVERSEHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIIMREVERSEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImReverseHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImReverseHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIImCommandHandle {
}

pub trait UIImCommandHandleTrait: TypeObject {
}

impl UIImCommandHandleTrait for UIImCommandHandle {
}

pub static UIIMCOMMANDHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImCommandHandle",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImCommandHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIIMCOMMANDHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIImCommandHandle {
    fn type_info(&self) -> &'static TypeInfo {
        UIIMCOMMANDHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIIMCOMMANDHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImCommandHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImCommandHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIImScreenHandle {
}

pub trait UIImScreenHandleTrait: TypeObject {
}

impl UIImScreenHandleTrait for UIImScreenHandle {
}

pub static UIIMSCREENHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenHandle",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImScreenHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIIMSCREENHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIImScreenHandle {
    fn type_info(&self) -> &'static TypeInfo {
        UIIMSCREENHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIIMSCREENHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImScreenHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImScreenHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIImTextCommandConfig {
    pub effect: Option<Arc<Mutex<dyn UIElementFontEffectBaseAssetTrait>>>,
    pub style: Option<Arc<Mutex<dyn UIElementFontStyleBaseAssetTrait>>>,
    pub horizontal_alignment: UIElementAlignment,
    pub vertical_alignment: UIElementAlignment,
    pub scale: f32,
    pub clip_to_rect: bool,
    pub wordwrap: bool,
    pub password_mode: bool,
    pub scrollable: bool,
    pub offset: f32,
}

pub trait UIImTextCommandConfigTrait: TypeObject {
    fn effect(&self) -> &Option<Arc<Mutex<dyn UIElementFontEffectBaseAssetTrait>>>;
    fn style(&self) -> &Option<Arc<Mutex<dyn UIElementFontStyleBaseAssetTrait>>>;
    fn horizontal_alignment(&self) -> &UIElementAlignment;
    fn vertical_alignment(&self) -> &UIElementAlignment;
    fn scale(&self) -> &f32;
    fn clip_to_rect(&self) -> &bool;
    fn wordwrap(&self) -> &bool;
    fn password_mode(&self) -> &bool;
    fn scrollable(&self) -> &bool;
    fn offset(&self) -> &f32;
}

impl UIImTextCommandConfigTrait for UIImTextCommandConfig {
    fn effect(&self) -> &Option<Arc<Mutex<dyn UIElementFontEffectBaseAssetTrait>>> {
        &self.effect
    }
    fn style(&self) -> &Option<Arc<Mutex<dyn UIElementFontStyleBaseAssetTrait>>> {
        &self.style
    }
    fn horizontal_alignment(&self) -> &UIElementAlignment {
        &self.horizontal_alignment
    }
    fn vertical_alignment(&self) -> &UIElementAlignment {
        &self.vertical_alignment
    }
    fn scale(&self) -> &f32 {
        &self.scale
    }
    fn clip_to_rect(&self) -> &bool {
        &self.clip_to_rect
    }
    fn wordwrap(&self) -> &bool {
        &self.wordwrap
    }
    fn password_mode(&self) -> &bool {
        &self.password_mode
    }
    fn scrollable(&self) -> &bool {
        &self.scrollable
    }
    fn offset(&self) -> &f32 {
        &self.offset
    }
}

pub static UIIMTEXTCOMMANDCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImTextCommandConfig",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImTextCommandConfig as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontEffectBaseAsset",
                rust_offset: offset_of!(UIImTextCommandConfig, effect),
            },
            FieldInfoData {
                name: "Style",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontStyleBaseAsset",
                rust_offset: offset_of!(UIImTextCommandConfig, style),
            },
            FieldInfoData {
                name: "HorizontalAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementAlignment",
                rust_offset: offset_of!(UIImTextCommandConfig, horizontal_alignment),
            },
            FieldInfoData {
                name: "VerticalAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementAlignment",
                rust_offset: offset_of!(UIImTextCommandConfig, vertical_alignment),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIImTextCommandConfig, scale),
            },
            FieldInfoData {
                name: "ClipToRect",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImTextCommandConfig, clip_to_rect),
            },
            FieldInfoData {
                name: "Wordwrap",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImTextCommandConfig, wordwrap),
            },
            FieldInfoData {
                name: "PasswordMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImTextCommandConfig, password_mode),
            },
            FieldInfoData {
                name: "Scrollable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImTextCommandConfig, scrollable),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIImTextCommandConfig, offset),
            },
        ],
    }),
    array_type: Some(UIIMTEXTCOMMANDCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIImTextCommandConfig {
    fn type_info(&self) -> &'static TypeInfo {
        UIIMTEXTCOMMANDCONFIG_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIIMTEXTCOMMANDCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImTextCommandConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIImTextCommandConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UITextureMappingAssetBinding {
    pub mapping: Option<Arc<Mutex<dyn UITextureMappingBaseAssetTrait>>>,
    pub compartment: u16,
}

pub trait UITextureMappingAssetBindingTrait: TypeObject {
    fn mapping(&self) -> &Option<Arc<Mutex<dyn UITextureMappingBaseAssetTrait>>>;
    fn compartment(&self) -> &u16;
}

impl UITextureMappingAssetBindingTrait for UITextureMappingAssetBinding {
    fn mapping(&self) -> &Option<Arc<Mutex<dyn UITextureMappingBaseAssetTrait>>> {
        &self.mapping
    }
    fn compartment(&self) -> &u16 {
        &self.compartment
    }
}

pub static UITEXTUREMAPPINGASSETBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingAssetBinding",
    flags: MemberInfoFlags::new(73),
    module: "GameBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UITextureMappingAssetBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Mapping",
                flags: MemberInfoFlags::new(0),
                field_type: "UITextureMappingBaseAsset",
                rust_offset: offset_of!(UITextureMappingAssetBinding, mapping),
            },
            FieldInfoData {
                name: "Compartment",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(UITextureMappingAssetBinding, compartment),
            },
        ],
    }),
    array_type: Some(UITEXTUREMAPPINGASSETBINDING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UITextureMappingAssetBinding {
    fn type_info(&self) -> &'static TypeInfo {
        UITEXTUREMAPPINGASSETBINDING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UITEXTUREMAPPINGASSETBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingAssetBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UITextureMappingAssetBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementFontEffectBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait UIElementFontEffectBaseAssetTrait: super::core::AssetTrait {
}

impl UIElementFontEffectBaseAssetTrait for UIElementFontEffectBaseAsset {
}

impl super::core::AssetTrait for UIElementFontEffectBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UIElementFontEffectBaseAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTFONTEFFECTBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontEffectBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementFontEffectBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTFONTEFFECTBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementFontEffectBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTFONTEFFECTBASEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTFONTEFFECTBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontEffectBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIElementFontEffectBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementFontStyleBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait UIElementFontStyleBaseAssetTrait: super::core::AssetTrait {
}

impl UIElementFontStyleBaseAssetTrait for UIElementFontStyleBaseAsset {
}

impl super::core::AssetTrait for UIElementFontStyleBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UIElementFontStyleBaseAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTFONTSTYLEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontStyleBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementFontStyleBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTFONTSTYLEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementFontStyleBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTFONTSTYLEBASEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTFONTSTYLEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontStyleBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIElementFontStyleBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UITextureMappingBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait UITextureMappingBaseAssetTrait: super::core::AssetTrait {
}

impl UITextureMappingBaseAssetTrait for UITextureMappingBaseAsset {
}

impl super::core::AssetTrait for UITextureMappingBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UITextureMappingBaseAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UITEXTUREMAPPINGBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UITextureMappingBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UITEXTUREMAPPINGBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UITextureMappingBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        UITEXTUREMAPPINGBASEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UITEXTUREMAPPINGBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UITextureMappingBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIScreenProjectionMode {
    #[default]
    UIScreenProjectionMode_Default = 0,
    UIScreenProjectionMode_Billboard = 1,
    UIScreenProjectionMode_BillboardCylindrical = 2,
    UIScreenProjectionMode_BillboardFixedSize = 3,
}

pub static UISCREENPROJECTIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionMode",
    flags: MemberInfoFlags::new(49429),
    module: "GameBase",
    data: TypeInfoData::Enum,
    array_type: Some(UISCREENPROJECTIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIScreenProjectionMode {
    fn type_info(&self) -> &'static TypeInfo {
        UISCREENPROJECTIONMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UISCREENPROJECTIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIScreenProjectionMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static UIELEMENTALIGNMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementAlignment",
    flags: MemberInfoFlags::new(49429),
    module: "GameBase",
    data: TypeInfoData::Enum,
    array_type: Some(UIELEMENTALIGNMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIElementAlignment {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTALIGNMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTALIGNMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementAlignment-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameBase",
    data: TypeInfoData::Array("UIElementAlignment"),
    array_type: None,
    alignment: 8,
};


