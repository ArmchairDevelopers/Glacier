use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct UIGeometryAsset {
    pub _glacier_base: super::core::DataContainer,
}

pub trait UIGeometryAssetTrait: super::core::DataContainerTrait {
}

impl UIGeometryAssetTrait for UIGeometryAsset {
}

impl super::core::DataContainerTrait for UIGeometryAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIGEOMETRYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIGeometryAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIGeometryAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIGEOMETRYASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIGeometryAsset {
    fn type_info(&self) -> &'static TypeInfo {
        UIGEOMETRYASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIGEOMETRYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIGeometryAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIGeometryAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIAutoMappedTexture {
    pub id: i32,
    pub texture_ref: glacier_reflect::builtin::ResourceRef,
}

pub trait UIAutoMappedTextureTrait: TypeObject {
    fn id(&self) -> &i32;
    fn texture_ref(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl UIAutoMappedTextureTrait for UIAutoMappedTexture {
    fn id(&self) -> &i32 {
        &self.id
    }
    fn texture_ref(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.texture_ref
    }
}

pub static UIAUTOMAPPEDTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoMappedTexture",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIAutoMappedTexture as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIAutoMappedTexture, id),
            },
            FieldInfoData {
                name: "TextureRef",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(UIAutoMappedTexture, texture_ref),
            },
        ],
    }),
    array_type: Some(UIAUTOMAPPEDTEXTURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIAutoMappedTexture {
    fn type_info(&self) -> &'static TypeInfo {
        UIAUTOMAPPEDTEXTURE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIAUTOMAPPEDTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoMappedTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIAutoMappedTexture"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UICppScreenData {
    pub _glacier_base: super::core::Asset,
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

pub trait UICppScreenDataTrait: super::core::AssetTrait {
    fn state_stream_enabled(&self) -> &bool;
    fn state_stream_lane(&self) -> &UICppScreenStateStreamLaneType;
    fn field_of_view(&self) -> &f32;
    fn scale_up_and_keep_aspect_ratio(&self) -> &bool;
    fn flash_compatibility_mode(&self) -> &bool;
    fn screen_layout_width(&self) -> &f32;
    fn screen_layout_height(&self) -> &f32;
    fn allow_input(&self) -> &bool;
    fn eat_all_input(&self) -> &bool;
    fn layout_with_safe_zone(&self) -> &bool;
    fn screen_sampler_settings(&self) -> &super::game_base::UIScreenSamplerSettings;
}

impl UICppScreenDataTrait for UICppScreenData {
    fn state_stream_enabled(&self) -> &bool {
        &self.state_stream_enabled
    }
    fn state_stream_lane(&self) -> &UICppScreenStateStreamLaneType {
        &self.state_stream_lane
    }
    fn field_of_view(&self) -> &f32 {
        &self.field_of_view
    }
    fn scale_up_and_keep_aspect_ratio(&self) -> &bool {
        &self.scale_up_and_keep_aspect_ratio
    }
    fn flash_compatibility_mode(&self) -> &bool {
        &self.flash_compatibility_mode
    }
    fn screen_layout_width(&self) -> &f32 {
        &self.screen_layout_width
    }
    fn screen_layout_height(&self) -> &f32 {
        &self.screen_layout_height
    }
    fn allow_input(&self) -> &bool {
        &self.allow_input
    }
    fn eat_all_input(&self) -> &bool {
        &self.eat_all_input
    }
    fn layout_with_safe_zone(&self) -> &bool {
        &self.layout_with_safe_zone
    }
    fn screen_sampler_settings(&self) -> &super::game_base::UIScreenSamplerSettings {
        &self.screen_sampler_settings
    }
}

impl super::core::AssetTrait for UICppScreenData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UICppScreenData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UICPPSCREENDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UICppScreenData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StateStreamEnabled",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, state_stream_enabled),
            },
            FieldInfoData {
                name: "StateStreamLane",
                flags: MemberInfoFlags::new(0),
                field_type: "UICppScreenStateStreamLaneType",
                rust_offset: offset_of!(UICppScreenData, state_stream_lane),
            },
            FieldInfoData {
                name: "FieldOfView",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UICppScreenData, field_of_view),
            },
            FieldInfoData {
                name: "ScaleUpAndKeepAspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, scale_up_and_keep_aspect_ratio),
            },
            FieldInfoData {
                name: "FlashCompatibilityMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, flash_compatibility_mode),
            },
            FieldInfoData {
                name: "ScreenLayoutWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UICppScreenData, screen_layout_width),
            },
            FieldInfoData {
                name: "ScreenLayoutHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UICppScreenData, screen_layout_height),
            },
            FieldInfoData {
                name: "AllowInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, allow_input),
            },
            FieldInfoData {
                name: "EatAllInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, eat_all_input),
            },
            FieldInfoData {
                name: "LayoutWithSafeZone",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, layout_with_safe_zone),
            },
            FieldInfoData {
                name: "ScreenSamplerSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "UIScreenSamplerSettings",
                rust_offset: offset_of!(UICppScreenData, screen_sampler_settings),
            },
        ],
    }),
    array_type: Some(UICPPSCREENDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UICppScreenData {
    fn type_info(&self) -> &'static TypeInfo {
        UICPPSCREENDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UICPPSCREENDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UICppScreenData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UICppScreenStateStreamLaneType {
    #[default]
    UICppScreenStateStreamLaneType_Primary = 0,
    UICppScreenStateStreamLaneType_Secondary = 1,
}

pub static UICPPSCREENSTATESTREAMLANETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenStateStreamLaneType",
    flags: MemberInfoFlags::new(49429),
    module: "GameSharedUI",
    data: TypeInfoData::Enum,
    array_type: Some(UICPPSCREENSTATESTREAMLANETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UICppScreenStateStreamLaneType {
    fn type_info(&self) -> &'static TypeInfo {
        UICPPSCREENSTATESTREAMLANETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UICPPSCREENSTATESTREAMLANETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenStateStreamLaneType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UICppScreenStateStreamLaneType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIScreenRenderTargetEntityData {
    pub _glacier_base: UIScreenRenderEntityData,
    pub render_target: Option<Arc<Mutex<dyn super::render::RenderTextureAssetTrait>>>,
    pub generate_render_target: bool,
    pub create_render_target_stencil: bool,
    pub clear_render_target: bool,
    pub render_on_event: bool,
}

pub trait UIScreenRenderTargetEntityDataTrait: UIScreenRenderEntityDataTrait {
    fn render_target(&self) -> &Option<Arc<Mutex<dyn super::render::RenderTextureAssetTrait>>>;
    fn generate_render_target(&self) -> &bool;
    fn create_render_target_stencil(&self) -> &bool;
    fn clear_render_target(&self) -> &bool;
    fn render_on_event(&self) -> &bool;
}

impl UIScreenRenderTargetEntityDataTrait for UIScreenRenderTargetEntityData {
    fn render_target(&self) -> &Option<Arc<Mutex<dyn super::render::RenderTextureAssetTrait>>> {
        &self.render_target
    }
    fn generate_render_target(&self) -> &bool {
        &self.generate_render_target
    }
    fn create_render_target_stencil(&self) -> &bool {
        &self.create_render_target_stencil
    }
    fn clear_render_target(&self) -> &bool {
        &self.clear_render_target
    }
    fn render_on_event(&self) -> &bool {
        &self.render_on_event
    }
}

impl UIScreenRenderEntityDataTrait for UIScreenRenderTargetEntityData {
    fn screen_data(&self) -> &Option<Arc<Mutex<dyn UICppScreenDataTrait>>> {
        self._glacier_base.screen_data()
    }
    fn use_game_view_projection(&self) -> &bool {
        self._glacier_base.use_game_view_projection()
    }
    fn scale(&self) -> &f32 {
        self._glacier_base.scale()
    }
    fn enable_depth_culling(&self) -> &bool {
        self._glacier_base.enable_depth_culling()
    }
    fn projection_mode(&self) -> &super::game_base::UIScreenProjectionMode {
        self._glacier_base.projection_mode()
    }
    fn render_pass(&self) -> &UIScreenRenderingPass {
        self._glacier_base.render_pass()
    }
    fn update_order(&self) -> &i32 {
        self._glacier_base.update_order()
    }
    fn center_screen(&self) -> &bool {
        self._glacier_base.center_screen()
    }
    fn view_id(&self) -> &super::render_base::LocalPlayerViewId {
        self._glacier_base.view_id()
    }
    fn start_enabled(&self) -> &bool {
        self._glacier_base.start_enabled()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
}

impl super::entity::LogicReferenceObjectDataTrait for UIScreenRenderTargetEntityData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        self._glacier_base.sub_realm()
    }
}

impl super::entity::ReferenceObjectDataTrait for UIScreenRenderTargetEntityData {
    fn blueprint_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.blueprint_transform()
    }
    fn blueprint(&self) -> &Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>> {
        self._glacier_base.blueprint()
    }
    fn object_variation(&self) -> &Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>> {
        self._glacier_base.object_variation()
    }
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        self._glacier_base.stream_realm()
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override()
    }
    fn lightmap_resolution_scale(&self) -> &u32 {
        self._glacier_base.lightmap_resolution_scale()
    }
    fn lightmap_scale_with_size(&self) -> &bool {
        self._glacier_base.lightmap_scale_with_size()
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn create_indestructible_entity(&self) -> &bool {
        self._glacier_base.create_indestructible_entity()
    }
}

impl super::entity::GameObjectDataTrait for UIScreenRenderTargetEntityData {
}

impl super::core::DataBusPeerTrait for UIScreenRenderTargetEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for UIScreenRenderTargetEntityData {
}

impl super::core::DataContainerTrait for UIScreenRenderTargetEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UISCREENRENDERTARGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderTargetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UISCREENRENDERENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIScreenRenderTargetEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RenderTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderTextureAsset",
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, render_target),
            },
            FieldInfoData {
                name: "GenerateRenderTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, generate_render_target),
            },
            FieldInfoData {
                name: "CreateRenderTargetStencil",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, create_render_target_stencil),
            },
            FieldInfoData {
                name: "ClearRenderTarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, clear_render_target),
            },
            FieldInfoData {
                name: "RenderOnEvent",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, render_on_event),
            },
        ],
    }),
    array_type: Some(UISCREENRENDERTARGETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIScreenRenderTargetEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        UISCREENRENDERTARGETENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UISCREENRENDERTARGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderTargetEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIScreenRenderTargetEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIScreenRenderEntityData {
    pub _glacier_base: super::entity::LogicReferenceObjectData,
    pub screen_data: Option<Arc<Mutex<dyn UICppScreenDataTrait>>>,
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

pub trait UIScreenRenderEntityDataTrait: super::entity::LogicReferenceObjectDataTrait {
    fn screen_data(&self) -> &Option<Arc<Mutex<dyn UICppScreenDataTrait>>>;
    fn use_game_view_projection(&self) -> &bool;
    fn scale(&self) -> &f32;
    fn enable_depth_culling(&self) -> &bool;
    fn projection_mode(&self) -> &super::game_base::UIScreenProjectionMode;
    fn render_pass(&self) -> &UIScreenRenderingPass;
    fn update_order(&self) -> &i32;
    fn center_screen(&self) -> &bool;
    fn view_id(&self) -> &super::render_base::LocalPlayerViewId;
    fn start_enabled(&self) -> &bool;
    fn color(&self) -> &super::core::Vec3;
    fn alpha(&self) -> &f32;
}

impl UIScreenRenderEntityDataTrait for UIScreenRenderEntityData {
    fn screen_data(&self) -> &Option<Arc<Mutex<dyn UICppScreenDataTrait>>> {
        &self.screen_data
    }
    fn use_game_view_projection(&self) -> &bool {
        &self.use_game_view_projection
    }
    fn scale(&self) -> &f32 {
        &self.scale
    }
    fn enable_depth_culling(&self) -> &bool {
        &self.enable_depth_culling
    }
    fn projection_mode(&self) -> &super::game_base::UIScreenProjectionMode {
        &self.projection_mode
    }
    fn render_pass(&self) -> &UIScreenRenderingPass {
        &self.render_pass
    }
    fn update_order(&self) -> &i32 {
        &self.update_order
    }
    fn center_screen(&self) -> &bool {
        &self.center_screen
    }
    fn view_id(&self) -> &super::render_base::LocalPlayerViewId {
        &self.view_id
    }
    fn start_enabled(&self) -> &bool {
        &self.start_enabled
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
}

impl super::entity::LogicReferenceObjectDataTrait for UIScreenRenderEntityData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        self._glacier_base.sub_realm()
    }
}

impl super::entity::ReferenceObjectDataTrait for UIScreenRenderEntityData {
    fn blueprint_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.blueprint_transform()
    }
    fn blueprint(&self) -> &Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>> {
        self._glacier_base.blueprint()
    }
    fn object_variation(&self) -> &Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>> {
        self._glacier_base.object_variation()
    }
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        self._glacier_base.stream_realm()
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override()
    }
    fn lightmap_resolution_scale(&self) -> &u32 {
        self._glacier_base.lightmap_resolution_scale()
    }
    fn lightmap_scale_with_size(&self) -> &bool {
        self._glacier_base.lightmap_scale_with_size()
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn create_indestructible_entity(&self) -> &bool {
        self._glacier_base.create_indestructible_entity()
    }
}

impl super::entity::GameObjectDataTrait for UIScreenRenderEntityData {
}

impl super::core::DataBusPeerTrait for UIScreenRenderEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for UIScreenRenderEntityData {
}

impl super::core::DataContainerTrait for UIScreenRenderEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UISCREENRENDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIScreenRenderEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ScreenData",
                flags: MemberInfoFlags::new(0),
                field_type: "UICppScreenData",
                rust_offset: offset_of!(UIScreenRenderEntityData, screen_data),
            },
            FieldInfoData {
                name: "UseGameViewProjection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderEntityData, use_game_view_projection),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIScreenRenderEntityData, scale),
            },
            FieldInfoData {
                name: "EnableDepthCulling",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderEntityData, enable_depth_culling),
            },
            FieldInfoData {
                name: "ProjectionMode",
                flags: MemberInfoFlags::new(0),
                field_type: "UIScreenProjectionMode",
                rust_offset: offset_of!(UIScreenRenderEntityData, projection_mode),
            },
            FieldInfoData {
                name: "RenderPass",
                flags: MemberInfoFlags::new(0),
                field_type: "UIScreenRenderingPass",
                rust_offset: offset_of!(UIScreenRenderEntityData, render_pass),
            },
            FieldInfoData {
                name: "UpdateOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIScreenRenderEntityData, update_order),
            },
            FieldInfoData {
                name: "CenterScreen",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderEntityData, center_screen),
            },
            FieldInfoData {
                name: "ViewId",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerViewId",
                rust_offset: offset_of!(UIScreenRenderEntityData, view_id),
            },
            FieldInfoData {
                name: "StartEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderEntityData, start_enabled),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIScreenRenderEntityData, color),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIScreenRenderEntityData, alpha),
            },
        ],
    }),
    array_type: Some(UISCREENRENDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIScreenRenderEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        UISCREENRENDERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UISCREENRENDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIScreenRenderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIScreenRenderingPass {
    #[default]
    UIScreenRenderingPass_RenderTarget = 0,
    UIScreenRenderingPass_AboveFlash = 1,
    UIScreenRenderingPass_BelowFlash = 2,
    UIScreenRenderingPass_Dialog = 3,
    UIScreenRenderingPass_Count = 4,
}

pub static UISCREENRENDERINGPASS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderingPass",
    flags: MemberInfoFlags::new(49429),
    module: "GameSharedUI",
    data: TypeInfoData::Enum,
    array_type: Some(UISCREENRENDERINGPASS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIScreenRenderingPass {
    fn type_info(&self) -> &'static TypeInfo {
        UISCREENRENDERINGPASS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UISCREENRENDERINGPASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderingPass-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIScreenRenderingPass"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIIMSettingsAsset {
    pub _glacier_base: super::core::Asset,
    pub auto_scroll_settings: UIAutoScrollTextSettings,
}

pub trait UIIMSettingsAssetTrait: super::core::AssetTrait {
    fn auto_scroll_settings(&self) -> &UIAutoScrollTextSettings;
}

impl UIIMSettingsAssetTrait for UIIMSettingsAsset {
    fn auto_scroll_settings(&self) -> &UIAutoScrollTextSettings {
        &self.auto_scroll_settings
    }
}

impl super::core::AssetTrait for UIIMSettingsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UIIMSettingsAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIIMSETTINGSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIIMSettingsAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIIMSettingsAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AutoScrollSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "UIAutoScrollTextSettings",
                rust_offset: offset_of!(UIIMSettingsAsset, auto_scroll_settings),
            },
        ],
    }),
    array_type: Some(UIIMSETTINGSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIIMSettingsAsset {
    fn type_info(&self) -> &'static TypeInfo {
        UIIMSETTINGSASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIIMSETTINGSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIIMSettingsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIIMSettingsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIAutoScrollTextSettings {
    pub no_scroll_wait_time: f32,
    pub fully_scrolled_wait_time: f32,
    pub max_scroll_time: f32,
    pub pixels_per_second: f32,
    pub scrollback_multiplier: f32,
}

pub trait UIAutoScrollTextSettingsTrait: TypeObject {
    fn no_scroll_wait_time(&self) -> &f32;
    fn fully_scrolled_wait_time(&self) -> &f32;
    fn max_scroll_time(&self) -> &f32;
    fn pixels_per_second(&self) -> &f32;
    fn scrollback_multiplier(&self) -> &f32;
}

impl UIAutoScrollTextSettingsTrait for UIAutoScrollTextSettings {
    fn no_scroll_wait_time(&self) -> &f32 {
        &self.no_scroll_wait_time
    }
    fn fully_scrolled_wait_time(&self) -> &f32 {
        &self.fully_scrolled_wait_time
    }
    fn max_scroll_time(&self) -> &f32 {
        &self.max_scroll_time
    }
    fn pixels_per_second(&self) -> &f32 {
        &self.pixels_per_second
    }
    fn scrollback_multiplier(&self) -> &f32 {
        &self.scrollback_multiplier
    }
}

pub static UIAUTOSCROLLTEXTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoScrollTextSettings",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIAutoScrollTextSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "NoScrollWaitTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIAutoScrollTextSettings, no_scroll_wait_time),
            },
            FieldInfoData {
                name: "FullyScrolledWaitTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIAutoScrollTextSettings, fully_scrolled_wait_time),
            },
            FieldInfoData {
                name: "MaxScrollTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIAutoScrollTextSettings, max_scroll_time),
            },
            FieldInfoData {
                name: "PixelsPerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIAutoScrollTextSettings, pixels_per_second),
            },
            FieldInfoData {
                name: "ScrollbackMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIAutoScrollTextSettings, scrollback_multiplier),
            },
        ],
    }),
    array_type: Some(UIAUTOSCROLLTEXTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIAutoScrollTextSettings {
    fn type_info(&self) -> &'static TypeInfo {
        UIAUTOSCROLLTEXTSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIAUTOSCROLLTEXTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoScrollTextSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIAutoScrollTextSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementWidgetReferenceEntityData {
    pub _glacier_base: super::entity::LogicReferenceObjectData,
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

pub trait UIElementWidgetReferenceEntityDataTrait: super::entity::LogicReferenceObjectDataTrait {
    fn instance_name(&self) -> &String;
    fn instance_name_hash(&self) -> &u32;
    fn inclusion_settings(&self) -> &UIElementInclusionSettings;
    fn transform_pivot(&self) -> &super::core::Vec3;
    fn use_element_size(&self) -> &bool;
    fn size(&self) -> &super::core::Vec2;
    fn layout_mode(&self) -> &UILayoutMode;
    fn offset(&self) -> &UIElementOffset;
    fn anchor(&self) -> &UIElementAnchor;
    fn position(&self) -> &UIElementOffset;
    fn expansion(&self) -> &UIElementRectExpansion;
    fn color(&self) -> &super::core::Vec3;
    fn alpha(&self) -> &f32;
    fn code_access_identifier(&self) -> &String;
}

impl UIElementWidgetReferenceEntityDataTrait for UIElementWidgetReferenceEntityData {
    fn instance_name(&self) -> &String {
        &self.instance_name
    }
    fn instance_name_hash(&self) -> &u32 {
        &self.instance_name_hash
    }
    fn inclusion_settings(&self) -> &UIElementInclusionSettings {
        &self.inclusion_settings
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        &self.transform_pivot
    }
    fn use_element_size(&self) -> &bool {
        &self.use_element_size
    }
    fn size(&self) -> &super::core::Vec2 {
        &self.size
    }
    fn layout_mode(&self) -> &UILayoutMode {
        &self.layout_mode
    }
    fn offset(&self) -> &UIElementOffset {
        &self.offset
    }
    fn anchor(&self) -> &UIElementAnchor {
        &self.anchor
    }
    fn position(&self) -> &UIElementOffset {
        &self.position
    }
    fn expansion(&self) -> &UIElementRectExpansion {
        &self.expansion
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
    fn code_access_identifier(&self) -> &String {
        &self.code_access_identifier
    }
}

impl super::entity::LogicReferenceObjectDataTrait for UIElementWidgetReferenceEntityData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        self._glacier_base.sub_realm()
    }
}

impl super::entity::ReferenceObjectDataTrait for UIElementWidgetReferenceEntityData {
    fn blueprint_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.blueprint_transform()
    }
    fn blueprint(&self) -> &Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>> {
        self._glacier_base.blueprint()
    }
    fn object_variation(&self) -> &Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>> {
        self._glacier_base.object_variation()
    }
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        self._glacier_base.stream_realm()
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override()
    }
    fn lightmap_resolution_scale(&self) -> &u32 {
        self._glacier_base.lightmap_resolution_scale()
    }
    fn lightmap_scale_with_size(&self) -> &bool {
        self._glacier_base.lightmap_scale_with_size()
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn create_indestructible_entity(&self) -> &bool {
        self._glacier_base.create_indestructible_entity()
    }
}

impl super::entity::GameObjectDataTrait for UIElementWidgetReferenceEntityData {
}

impl super::core::DataBusPeerTrait for UIElementWidgetReferenceEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for UIElementWidgetReferenceEntityData {
}

impl super::core::DataContainerTrait for UIElementWidgetReferenceEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTWIDGETREFERENCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementWidgetReferenceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementWidgetReferenceEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InstanceName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, instance_name),
            },
            FieldInfoData {
                name: "InstanceNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, instance_name_hash),
            },
            FieldInfoData {
                name: "InclusionSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementInclusionSettings",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, inclusion_settings),
            },
            FieldInfoData {
                name: "TransformPivot",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, transform_pivot),
            },
            FieldInfoData {
                name: "UseElementSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, use_element_size),
            },
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, size),
            },
            FieldInfoData {
                name: "LayoutMode",
                flags: MemberInfoFlags::new(0),
                field_type: "UILayoutMode",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, layout_mode),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementOffset",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, offset),
            },
            FieldInfoData {
                name: "Anchor",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementAnchor",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, anchor),
            },
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementOffset",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, position),
            },
            FieldInfoData {
                name: "Expansion",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementRectExpansion",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, expansion),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, color),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, alpha),
            },
            FieldInfoData {
                name: "CodeAccessIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, code_access_identifier),
            },
        ],
    }),
    array_type: Some(UIELEMENTWIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementWidgetReferenceEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTWIDGETREFERENCEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTWIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementWidgetReferenceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementWidgetReferenceEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementBitmapDistanceFieldParams {
    pub alpha_threshold: f32,
    pub distance_scale: f32,
    pub outline_inner: f32,
    pub outline_outer: f32,
    pub outline_color: UIElementColor,
}

pub trait UIElementBitmapDistanceFieldParamsTrait: TypeObject {
    fn alpha_threshold(&self) -> &f32;
    fn distance_scale(&self) -> &f32;
    fn outline_inner(&self) -> &f32;
    fn outline_outer(&self) -> &f32;
    fn outline_color(&self) -> &UIElementColor;
}

impl UIElementBitmapDistanceFieldParamsTrait for UIElementBitmapDistanceFieldParams {
    fn alpha_threshold(&self) -> &f32 {
        &self.alpha_threshold
    }
    fn distance_scale(&self) -> &f32 {
        &self.distance_scale
    }
    fn outline_inner(&self) -> &f32 {
        &self.outline_inner
    }
    fn outline_outer(&self) -> &f32 {
        &self.outline_outer
    }
    fn outline_color(&self) -> &UIElementColor {
        &self.outline_color
    }
}

pub static UIELEMENTBITMAPDISTANCEFIELDPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementBitmapDistanceFieldParams",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementBitmapDistanceFieldParams as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AlphaThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, alpha_threshold),
            },
            FieldInfoData {
                name: "DistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, distance_scale),
            },
            FieldInfoData {
                name: "OutlineInner",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, outline_inner),
            },
            FieldInfoData {
                name: "OutlineOuter",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, outline_outer),
            },
            FieldInfoData {
                name: "OutlineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, outline_color),
            },
        ],
    }),
    array_type: Some(UIELEMENTBITMAPDISTANCEFIELDPARAMS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementBitmapDistanceFieldParams {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTBITMAPDISTANCEFIELDPARAMS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTBITMAPDISTANCEFIELDPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementBitmapDistanceFieldParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementBitmapDistanceFieldParams"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIMaskingContainerEntityData {
    pub _glacier_base: UIContainerEntityData,
    pub masks: Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>,
    pub mask_threshold: f32,
}

pub trait UIMaskingContainerEntityDataTrait: UIContainerEntityDataTrait {
    fn masks(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>;
    fn mask_threshold(&self) -> &f32;
}

impl UIMaskingContainerEntityDataTrait for UIMaskingContainerEntityData {
    fn masks(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        &self.masks
    }
    fn mask_threshold(&self) -> &f32 {
        &self.mask_threshold
    }
}

impl UIContainerEntityDataTrait for UIMaskingContainerEntityData {
    fn elements(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.elements()
    }
}

impl UIElementEntityDataTrait for UIMaskingContainerEntityData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn layout_mode(&self) -> &UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn offset(&self) -> &UIElementOffset {
        self._glacier_base.offset()
    }
    fn anchor(&self) -> &UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn position(&self) -> &UIElementOffset {
        self._glacier_base.position()
    }
    fn expansion(&self) -> &UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for UIMaskingContainerEntityData {
}

impl super::entity::GameObjectDataTrait for UIMaskingContainerEntityData {
}

impl super::core::DataBusPeerTrait for UIMaskingContainerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for UIMaskingContainerEntityData {
}

impl super::core::DataContainerTrait for UIMaskingContainerEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIMASKINGCONTAINERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingContainerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UICONTAINERENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMaskingContainerEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Masks",
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(UIMaskingContainerEntityData, masks),
            },
            FieldInfoData {
                name: "MaskThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIMaskingContainerEntityData, mask_threshold),
            },
        ],
    }),
    array_type: Some(UIMASKINGCONTAINERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIMaskingContainerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        UIMASKINGCONTAINERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIMASKINGCONTAINERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingContainerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIMaskingContainerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIContainerEntityData {
    pub _glacier_base: UIElementEntityData,
    pub elements: Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>,
}

pub trait UIContainerEntityDataTrait: UIElementEntityDataTrait {
    fn elements(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>;
}

impl UIContainerEntityDataTrait for UIContainerEntityData {
    fn elements(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        &self.elements
    }
}

impl UIElementEntityDataTrait for UIContainerEntityData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn layout_mode(&self) -> &UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn offset(&self) -> &UIElementOffset {
        self._glacier_base.offset()
    }
    fn anchor(&self) -> &UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn position(&self) -> &UIElementOffset {
        self._glacier_base.position()
    }
    fn expansion(&self) -> &UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for UIContainerEntityData {
}

impl super::entity::GameObjectDataTrait for UIContainerEntityData {
}

impl super::core::DataBusPeerTrait for UIContainerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for UIContainerEntityData {
}

impl super::core::DataContainerTrait for UIContainerEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UICONTAINERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIContainerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIContainerEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Elements",
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(UIContainerEntityData, elements),
            },
        ],
    }),
    array_type: Some(UICONTAINERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIContainerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        UICONTAINERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UICONTAINERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIContainerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIContainerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementEntityData {
    pub _glacier_base: super::entity::EntityData,
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

pub trait UIElementEntityDataTrait: super::entity::EntityDataTrait {
    fn instance_name(&self) -> &String;
    fn instance_name_hash(&self) -> &u32;
    fn transform_pivot(&self) -> &super::core::Vec3;
    fn size(&self) -> &super::core::Vec2;
    fn layout_mode(&self) -> &UILayoutMode;
    fn offset(&self) -> &UIElementOffset;
    fn anchor(&self) -> &UIElementAnchor;
    fn position(&self) -> &UIElementOffset;
    fn expansion(&self) -> &UIElementRectExpansion;
    fn visible(&self) -> &bool;
    fn color(&self) -> &super::core::Vec3;
    fn alpha(&self) -> &f32;
    fn transform(&self) -> &super::core::LinearTransform;
}

impl UIElementEntityDataTrait for UIElementEntityData {
    fn instance_name(&self) -> &String {
        &self.instance_name
    }
    fn instance_name_hash(&self) -> &u32 {
        &self.instance_name_hash
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        &self.transform_pivot
    }
    fn size(&self) -> &super::core::Vec2 {
        &self.size
    }
    fn layout_mode(&self) -> &UILayoutMode {
        &self.layout_mode
    }
    fn offset(&self) -> &UIElementOffset {
        &self.offset
    }
    fn anchor(&self) -> &UIElementAnchor {
        &self.anchor
    }
    fn position(&self) -> &UIElementOffset {
        &self.position
    }
    fn expansion(&self) -> &UIElementRectExpansion {
        &self.expansion
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
}

impl super::entity::EntityDataTrait for UIElementEntityData {
}

impl super::entity::GameObjectDataTrait for UIElementEntityData {
}

impl super::core::DataBusPeerTrait for UIElementEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for UIElementEntityData {
}

impl super::core::DataContainerTrait for UIElementEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InstanceName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIElementEntityData, instance_name),
            },
            FieldInfoData {
                name: "InstanceNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UIElementEntityData, instance_name_hash),
            },
            FieldInfoData {
                name: "TransformPivot",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIElementEntityData, transform_pivot),
            },
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UIElementEntityData, size),
            },
            FieldInfoData {
                name: "LayoutMode",
                flags: MemberInfoFlags::new(0),
                field_type: "UILayoutMode",
                rust_offset: offset_of!(UIElementEntityData, layout_mode),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementOffset",
                rust_offset: offset_of!(UIElementEntityData, offset),
            },
            FieldInfoData {
                name: "Anchor",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementAnchor",
                rust_offset: offset_of!(UIElementEntityData, anchor),
            },
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementOffset",
                rust_offset: offset_of!(UIElementEntityData, position),
            },
            FieldInfoData {
                name: "Expansion",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementRectExpansion",
                rust_offset: offset_of!(UIElementEntityData, expansion),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementEntityData, visible),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIElementEntityData, color),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementEntityData, alpha),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(UIElementEntityData, transform),
            },
        ],
    }),
    array_type: Some(UIELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIElementEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIWidgetEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub size: UIElementSize,
    pub layers: Vec<Option<Arc<Mutex<dyn UIElementLayerEntityDataTrait>>>>,
    pub texture_mappings: Vec<Option<Arc<Mutex<dyn UITextureMappingAssetTrait>>>>,
    pub components: Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>,
    pub visible: bool,
}

pub trait UIWidgetEntityDataTrait: super::entity::EntityDataTrait {
    fn size(&self) -> &UIElementSize;
    fn layers(&self) -> &Vec<Option<Arc<Mutex<dyn UIElementLayerEntityDataTrait>>>>;
    fn texture_mappings(&self) -> &Vec<Option<Arc<Mutex<dyn UITextureMappingAssetTrait>>>>;
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>;
    fn visible(&self) -> &bool;
}

impl UIWidgetEntityDataTrait for UIWidgetEntityData {
    fn size(&self) -> &UIElementSize {
        &self.size
    }
    fn layers(&self) -> &Vec<Option<Arc<Mutex<dyn UIElementLayerEntityDataTrait>>>> {
        &self.layers
    }
    fn texture_mappings(&self) -> &Vec<Option<Arc<Mutex<dyn UITextureMappingAssetTrait>>>> {
        &self.texture_mappings
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        &self.components
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
}

impl super::entity::EntityDataTrait for UIWidgetEntityData {
}

impl super::entity::GameObjectDataTrait for UIWidgetEntityData {
}

impl super::core::DataBusPeerTrait for UIWidgetEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for UIWidgetEntityData {
}

impl super::core::DataContainerTrait for UIWidgetEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIWIDGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWidgetEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementSize",
                rust_offset: offset_of!(UIWidgetEntityData, size),
            },
            FieldInfoData {
                name: "Layers",
                flags: MemberInfoFlags::new(144),
                field_type: "UIElementLayerEntityData-Array",
                rust_offset: offset_of!(UIWidgetEntityData, layers),
            },
            FieldInfoData {
                name: "TextureMappings",
                flags: MemberInfoFlags::new(144),
                field_type: "UITextureMappingAsset-Array",
                rust_offset: offset_of!(UIWidgetEntityData, texture_mappings),
            },
            FieldInfoData {
                name: "Components",
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(UIWidgetEntityData, components),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIWidgetEntityData, visible),
            },
        ],
    }),
    array_type: Some(UIWIDGETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIWidgetEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        UIWIDGETENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIWIDGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIWidgetEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementLayerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub elements: Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>,
    pub inclusion_settings: UIElementInclusionSettings,
    pub visible: bool,
    pub internal_layer_name: String,
}

pub trait UIElementLayerEntityDataTrait: super::entity::EntityDataTrait {
    fn elements(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>>;
    fn inclusion_settings(&self) -> &UIElementInclusionSettings;
    fn visible(&self) -> &bool;
    fn internal_layer_name(&self) -> &String;
}

impl UIElementLayerEntityDataTrait for UIElementLayerEntityData {
    fn elements(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        &self.elements
    }
    fn inclusion_settings(&self) -> &UIElementInclusionSettings {
        &self.inclusion_settings
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn internal_layer_name(&self) -> &String {
        &self.internal_layer_name
    }
}

impl super::entity::EntityDataTrait for UIElementLayerEntityData {
}

impl super::entity::GameObjectDataTrait for UIElementLayerEntityData {
}

impl super::core::DataBusPeerTrait for UIElementLayerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for UIElementLayerEntityData {
}

impl super::core::DataContainerTrait for UIElementLayerEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTLAYERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementLayerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementLayerEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Elements",
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(UIElementLayerEntityData, elements),
            },
            FieldInfoData {
                name: "InclusionSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementInclusionSettings",
                rust_offset: offset_of!(UIElementLayerEntityData, inclusion_settings),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementLayerEntityData, visible),
            },
            FieldInfoData {
                name: "InternalLayerName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIElementLayerEntityData, internal_layer_name),
            },
        ],
    }),
    array_type: Some(UIELEMENTLAYERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementLayerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTLAYERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTLAYERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementLayerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementLayerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementInclusionSettings {
    pub is_singleplayer_layer: bool,
    pub is_multiplayer_layer: bool,
    pub custom_inclusion_critera: Vec<String>,
    pub is_s_d_layer: bool,
    pub is_h_d_layer: bool,
}

pub trait UIElementInclusionSettingsTrait: TypeObject {
    fn is_singleplayer_layer(&self) -> &bool;
    fn is_multiplayer_layer(&self) -> &bool;
    fn custom_inclusion_critera(&self) -> &Vec<String>;
    fn is_s_d_layer(&self) -> &bool;
    fn is_h_d_layer(&self) -> &bool;
}

impl UIElementInclusionSettingsTrait for UIElementInclusionSettings {
    fn is_singleplayer_layer(&self) -> &bool {
        &self.is_singleplayer_layer
    }
    fn is_multiplayer_layer(&self) -> &bool {
        &self.is_multiplayer_layer
    }
    fn custom_inclusion_critera(&self) -> &Vec<String> {
        &self.custom_inclusion_critera
    }
    fn is_s_d_layer(&self) -> &bool {
        &self.is_s_d_layer
    }
    fn is_h_d_layer(&self) -> &bool {
        &self.is_h_d_layer
    }
}

pub static UIELEMENTINCLUSIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementInclusionSettings",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementInclusionSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IsSingleplayerLayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementInclusionSettings, is_singleplayer_layer),
            },
            FieldInfoData {
                name: "IsMultiplayerLayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementInclusionSettings, is_multiplayer_layer),
            },
            FieldInfoData {
                name: "CustomInclusionCritera",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(UIElementInclusionSettings, custom_inclusion_critera),
            },
            FieldInfoData {
                name: "IsSDLayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementInclusionSettings, is_s_d_layer),
            },
            FieldInfoData {
                name: "IsHDLayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementInclusionSettings, is_h_d_layer),
            },
        ],
    }),
    array_type: Some(UIELEMENTINCLUSIONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementInclusionSettings {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTINCLUSIONSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTINCLUSIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementInclusionSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementInclusionSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static UIBLENDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIBlendMode",
    flags: MemberInfoFlags::new(49429),
    module: "GameSharedUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIBLENDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIBlendMode {
    fn type_info(&self) -> &'static TypeInfo {
        UIBLENDMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIBLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIBlendMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIBlendMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementGradient {
    pub top_left_color: UIElementColor,
    pub top_right_color: UIElementColor,
    pub bottom_left_color: UIElementColor,
    pub bottom_right_color: UIElementColor,
}

pub trait UIElementGradientTrait: TypeObject {
    fn top_left_color(&self) -> &UIElementColor;
    fn top_right_color(&self) -> &UIElementColor;
    fn bottom_left_color(&self) -> &UIElementColor;
    fn bottom_right_color(&self) -> &UIElementColor;
}

impl UIElementGradientTrait for UIElementGradient {
    fn top_left_color(&self) -> &UIElementColor {
        &self.top_left_color
    }
    fn top_right_color(&self) -> &UIElementColor {
        &self.top_right_color
    }
    fn bottom_left_color(&self) -> &UIElementColor {
        &self.bottom_left_color
    }
    fn bottom_right_color(&self) -> &UIElementColor {
        &self.bottom_right_color
    }
}

pub static UIELEMENTGRADIENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementGradient",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementGradient as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TopLeftColor",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementGradient, top_left_color),
            },
            FieldInfoData {
                name: "TopRightColor",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementGradient, top_right_color),
            },
            FieldInfoData {
                name: "BottomLeftColor",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementGradient, bottom_left_color),
            },
            FieldInfoData {
                name: "BottomRightColor",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementGradient, bottom_right_color),
            },
        ],
    }),
    array_type: Some(UIELEMENTGRADIENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementGradient {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTGRADIENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTGRADIENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementGradient-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementGradient"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementAnchor {
    pub x: f32,
    pub y: f32,
}

pub trait UIElementAnchorTrait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
}

impl UIElementAnchorTrait for UIElementAnchor {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
}

pub static UIELEMENTANCHOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementAnchor",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementAnchor as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementAnchor, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementAnchor, y),
            },
        ],
    }),
    array_type: Some(UIELEMENTANCHOR_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIElementAnchor {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTANCHOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTANCHOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementAnchor-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementAnchor"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementSize {
    pub x: f32,
    pub y: f32,
}

pub trait UIElementSizeTrait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
}

impl UIElementSizeTrait for UIElementSize {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
}

pub static UIELEMENTSIZE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementSize",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementSize as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementSize, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementSize, y),
            },
        ],
    }),
    array_type: Some(UIELEMENTSIZE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIElementSize {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTSIZE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTSIZE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementSize-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementSize"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementOffset {
    pub x: f32,
    pub y: f32,
}

pub trait UIElementOffsetTrait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
}

impl UIElementOffsetTrait for UIElementOffset {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
}

pub static UIELEMENTOFFSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementOffset",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementOffset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementOffset, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementOffset, y),
            },
        ],
    }),
    array_type: Some(UIELEMENTOFFSET_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIElementOffset {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTOFFSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTOFFSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementOffset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementOffset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementRectExpansion {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub trait UIElementRectExpansionTrait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn width(&self) -> &f32;
    fn height(&self) -> &f32;
}

impl UIElementRectExpansionTrait for UIElementRectExpansion {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn width(&self) -> &f32 {
        &self.width
    }
    fn height(&self) -> &f32 {
        &self.height
    }
}

pub static UIELEMENTRECTEXPANSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementRectExpansion",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementRectExpansion as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementRectExpansion, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementRectExpansion, y),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementRectExpansion, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementRectExpansion, height),
            },
        ],
    }),
    array_type: Some(UIELEMENTRECTEXPANSION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIElementRectExpansion {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTRECTEXPANSION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTRECTEXPANSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementRectExpansion-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementRectExpansion"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementFontStyle {
    pub _glacier_base: super::game_base::UIElementFontStyleBaseAsset,
    pub hd: Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>>,
    pub sd: Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>>,
    pub color: UIElementColor,
    pub language_overrides: Vec<UIElementFontDefinitionOverride>,
}

pub trait UIElementFontStyleTrait: super::game_base::UIElementFontStyleBaseAssetTrait {
    fn hd(&self) -> &Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>>;
    fn sd(&self) -> &Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>>;
    fn color(&self) -> &UIElementColor;
    fn language_overrides(&self) -> &Vec<UIElementFontDefinitionOverride>;
}

impl UIElementFontStyleTrait for UIElementFontStyle {
    fn hd(&self) -> &Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>> {
        &self.hd
    }
    fn sd(&self) -> &Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>> {
        &self.sd
    }
    fn color(&self) -> &UIElementColor {
        &self.color
    }
    fn language_overrides(&self) -> &Vec<UIElementFontDefinitionOverride> {
        &self.language_overrides
    }
}

impl super::game_base::UIElementFontStyleBaseAssetTrait for UIElementFontStyle {
}

impl super::core::AssetTrait for UIElementFontStyle {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UIElementFontStyle {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTFONTSTYLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontStyle",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_base::UIELEMENTFONTSTYLEBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementFontStyle as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Hd",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontDefinition",
                rust_offset: offset_of!(UIElementFontStyle, hd),
            },
            FieldInfoData {
                name: "Sd",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontDefinition",
                rust_offset: offset_of!(UIElementFontStyle, sd),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementFontStyle, color),
            },
            FieldInfoData {
                name: "LanguageOverrides",
                flags: MemberInfoFlags::new(144),
                field_type: "UIElementFontDefinitionOverride-Array",
                rust_offset: offset_of!(UIElementFontStyle, language_overrides),
            },
        ],
    }),
    array_type: Some(UIELEMENTFONTSTYLE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementFontStyle {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTFONTSTYLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTFONTSTYLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontStyle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontStyle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementFontEffect {
    pub _glacier_base: super::game_base::UIElementFontEffectBaseAsset,
    pub effect_script: String,
}

pub trait UIElementFontEffectTrait: super::game_base::UIElementFontEffectBaseAssetTrait {
    fn effect_script(&self) -> &String;
}

impl UIElementFontEffectTrait for UIElementFontEffect {
    fn effect_script(&self) -> &String {
        &self.effect_script
    }
}

impl super::game_base::UIElementFontEffectBaseAssetTrait for UIElementFontEffect {
}

impl super::core::AssetTrait for UIElementFontEffect {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UIElementFontEffect {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTFONTEFFECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontEffect",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_base::UIELEMENTFONTEFFECTBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementFontEffect as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EffectScript",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIElementFontEffect, effect_script),
            },
        ],
    }),
    array_type: Some(UIELEMENTFONTEFFECT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementFontEffect {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTFONTEFFECT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTFONTEFFECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontEffect-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontEffect"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementFontDefinition {
    pub _glacier_base: super::core::DataContainer,
    pub font_lookup: Vec<UIImmediateModeFontLookup>,
    pub point_size: f32,
    pub letter_spacing: f32,
    pub row_spacing: i32,
}

pub trait UIElementFontDefinitionTrait: super::core::DataContainerTrait {
    fn font_lookup(&self) -> &Vec<UIImmediateModeFontLookup>;
    fn point_size(&self) -> &f32;
    fn letter_spacing(&self) -> &f32;
    fn row_spacing(&self) -> &i32;
}

impl UIElementFontDefinitionTrait for UIElementFontDefinition {
    fn font_lookup(&self) -> &Vec<UIImmediateModeFontLookup> {
        &self.font_lookup
    }
    fn point_size(&self) -> &f32 {
        &self.point_size
    }
    fn letter_spacing(&self) -> &f32 {
        &self.letter_spacing
    }
    fn row_spacing(&self) -> &i32 {
        &self.row_spacing
    }
}

impl super::core::DataContainerTrait for UIElementFontDefinition {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTFONTDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinition",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementFontDefinition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FontLookup",
                flags: MemberInfoFlags::new(144),
                field_type: "UIImmediateModeFontLookup-Array",
                rust_offset: offset_of!(UIElementFontDefinition, font_lookup),
            },
            FieldInfoData {
                name: "PointSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementFontDefinition, point_size),
            },
            FieldInfoData {
                name: "LetterSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementFontDefinition, letter_spacing),
            },
            FieldInfoData {
                name: "RowSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIElementFontDefinition, row_spacing),
            },
        ],
    }),
    array_type: Some(UIELEMENTFONTDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementFontDefinition {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTFONTDEFINITION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTFONTDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontDefinition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementFontDefinitionOverride {
    pub language: super::core::LanguageFormat,
    pub hd: Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>>,
    pub sd: Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>>,
}

pub trait UIElementFontDefinitionOverrideTrait: TypeObject {
    fn language(&self) -> &super::core::LanguageFormat;
    fn hd(&self) -> &Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>>;
    fn sd(&self) -> &Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>>;
}

impl UIElementFontDefinitionOverrideTrait for UIElementFontDefinitionOverride {
    fn language(&self) -> &super::core::LanguageFormat {
        &self.language
    }
    fn hd(&self) -> &Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>> {
        &self.hd
    }
    fn sd(&self) -> &Option<Arc<Mutex<dyn UIElementFontDefinitionTrait>>> {
        &self.sd
    }
}

pub static UIELEMENTFONTDEFINITIONOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinitionOverride",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementFontDefinitionOverride as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(UIElementFontDefinitionOverride, language),
            },
            FieldInfoData {
                name: "Hd",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontDefinition",
                rust_offset: offset_of!(UIElementFontDefinitionOverride, hd),
            },
            FieldInfoData {
                name: "Sd",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontDefinition",
                rust_offset: offset_of!(UIElementFontDefinitionOverride, sd),
            },
        ],
    }),
    array_type: Some(UIELEMENTFONTDEFINITIONOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementFontDefinitionOverride {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTFONTDEFINITIONOVERRIDE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTFONTDEFINITIONOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinitionOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontDefinitionOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementTextFilterGlow {
    pub _glacier_base: UIElementTextFilter,
    pub knock_out: bool,
    pub hide_object: bool,
    pub fine_blur: bool,
    pub x: f32,
    pub y: f32,
    pub strength: f32,
    pub color: UIElementColor,
}

pub trait UIElementTextFilterGlowTrait: UIElementTextFilterTrait {
    fn knock_out(&self) -> &bool;
    fn hide_object(&self) -> &bool;
    fn fine_blur(&self) -> &bool;
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn strength(&self) -> &f32;
    fn color(&self) -> &UIElementColor;
}

impl UIElementTextFilterGlowTrait for UIElementTextFilterGlow {
    fn knock_out(&self) -> &bool {
        &self.knock_out
    }
    fn hide_object(&self) -> &bool {
        &self.hide_object
    }
    fn fine_blur(&self) -> &bool {
        &self.fine_blur
    }
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn color(&self) -> &UIElementColor {
        &self.color
    }
}

impl UIElementTextFilterTrait for UIElementTextFilterGlow {
}

impl super::core::DataContainerTrait for UIElementTextFilterGlow {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTTEXTFILTERGLOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterGlow",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTTEXTFILTER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementTextFilterGlow as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "KnockOut",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterGlow, knock_out),
            },
            FieldInfoData {
                name: "HideObject",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterGlow, hide_object),
            },
            FieldInfoData {
                name: "FineBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterGlow, fine_blur),
            },
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterGlow, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterGlow, y),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterGlow, strength),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementTextFilterGlow, color),
            },
        ],
    }),
    array_type: Some(UIELEMENTTEXTFILTERGLOW_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementTextFilterGlow {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTTEXTFILTERGLOW_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTTEXTFILTERGLOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterGlow-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilterGlow"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementTextFilterDropShadow {
    pub _glacier_base: UIElementTextFilter,
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

pub trait UIElementTextFilterDropShadowTrait: UIElementTextFilterTrait {
    fn knock_out(&self) -> &bool;
    fn hide_object(&self) -> &bool;
    fn fine_blur(&self) -> &bool;
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn strength(&self) -> &f32;
    fn color(&self) -> &UIElementColor;
    fn angle(&self) -> &f32;
    fn distance(&self) -> &f32;
}

impl UIElementTextFilterDropShadowTrait for UIElementTextFilterDropShadow {
    fn knock_out(&self) -> &bool {
        &self.knock_out
    }
    fn hide_object(&self) -> &bool {
        &self.hide_object
    }
    fn fine_blur(&self) -> &bool {
        &self.fine_blur
    }
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn color(&self) -> &UIElementColor {
        &self.color
    }
    fn angle(&self) -> &f32 {
        &self.angle
    }
    fn distance(&self) -> &f32 {
        &self.distance
    }
}

impl UIElementTextFilterTrait for UIElementTextFilterDropShadow {
}

impl super::core::DataContainerTrait for UIElementTextFilterDropShadow {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTTEXTFILTERDROPSHADOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterDropShadow",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTTEXTFILTER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementTextFilterDropShadow as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "KnockOut",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, knock_out),
            },
            FieldInfoData {
                name: "HideObject",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, hide_object),
            },
            FieldInfoData {
                name: "FineBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, fine_blur),
            },
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, y),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, strength),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, color),
            },
            FieldInfoData {
                name: "Angle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, angle),
            },
            FieldInfoData {
                name: "Distance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, distance),
            },
        ],
    }),
    array_type: Some(UIELEMENTTEXTFILTERDROPSHADOW_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementTextFilterDropShadow {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTTEXTFILTERDROPSHADOW_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTTEXTFILTERDROPSHADOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterDropShadow-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilterDropShadow"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementTextFilterBlur {
    pub _glacier_base: UIElementTextFilter,
    pub x: f32,
    pub y: f32,
    pub strength: f32,
}

pub trait UIElementTextFilterBlurTrait: UIElementTextFilterTrait {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn strength(&self) -> &f32;
}

impl UIElementTextFilterBlurTrait for UIElementTextFilterBlur {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
}

impl UIElementTextFilterTrait for UIElementTextFilterBlur {
}

impl super::core::DataContainerTrait for UIElementTextFilterBlur {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTTEXTFILTERBLUR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterBlur",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTTEXTFILTER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementTextFilterBlur as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterBlur, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterBlur, y),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterBlur, strength),
            },
        ],
    }),
    array_type: Some(UIELEMENTTEXTFILTERBLUR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementTextFilterBlur {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTTEXTFILTERBLUR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTTEXTFILTERBLUR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterBlur-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilterBlur"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementTextFilter {
    pub _glacier_base: super::core::DataContainer,
}

pub trait UIElementTextFilterTrait: super::core::DataContainerTrait {
}

impl UIElementTextFilterTrait for UIElementTextFilter {
}

impl super::core::DataContainerTrait for UIElementTextFilter {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIELEMENTTEXTFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilter",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementTextFilter as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTTEXTFILTER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIElementTextFilter {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTTEXTFILTER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTTEXTFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementColor {
    pub rgb: super::core::Vec3,
    pub alpha: f32,
}

pub trait UIElementColorTrait: TypeObject {
    fn rgb(&self) -> &super::core::Vec3;
    fn alpha(&self) -> &f32;
}

impl UIElementColorTrait for UIElementColor {
    fn rgb(&self) -> &super::core::Vec3 {
        &self.rgb
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
}

pub static UIELEMENTCOLOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementColor",
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementColor as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Rgb",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIElementColor, rgb),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementColor, alpha),
            },
        ],
    }),
    array_type: Some(UIELEMENTCOLOR_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIElementColor {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTCOLOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIELEMENTCOLOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementColor-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementColor"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UITextureMappingAsset {
    pub _glacier_base: super::game_base::UITextureMappingBaseAsset,
    pub compartment: UITextureMappingCompartment,
    pub output: Vec<UITextureMappingOutputEntry>,
    pub disable_atlas: bool,
    pub force_atlas: bool,
}

pub trait UITextureMappingAssetTrait: super::game_base::UITextureMappingBaseAssetTrait {
    fn compartment(&self) -> &UITextureMappingCompartment;
    fn output(&self) -> &Vec<UITextureMappingOutputEntry>;
    fn disable_atlas(&self) -> &bool;
    fn force_atlas(&self) -> &bool;
}

impl UITextureMappingAssetTrait for UITextureMappingAsset {
    fn compartment(&self) -> &UITextureMappingCompartment {
        &self.compartment
    }
    fn output(&self) -> &Vec<UITextureMappingOutputEntry> {
        &self.output
    }
    fn disable_atlas(&self) -> &bool {
        &self.disable_atlas
    }
    fn force_atlas(&self) -> &bool {
        &self.force_atlas
    }
}

impl super::game_base::UITextureMappingBaseAssetTrait for UITextureMappingAsset {
}

impl super::core::AssetTrait for UITextureMappingAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UITextureMappingAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UITEXTUREMAPPINGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_base::UITEXTUREMAPPINGBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UITextureMappingAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Compartment",
                flags: MemberInfoFlags::new(0),
                field_type: "UITextureMappingCompartment",
                rust_offset: offset_of!(UITextureMappingAsset, compartment),
            },
            FieldInfoData {
                name: "Output",
                flags: MemberInfoFlags::new(144),
                field_type: "UITextureMappingOutputEntry-Array",
                rust_offset: offset_of!(UITextureMappingAsset, output),
            },
            FieldInfoData {
                name: "DisableAtlas",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UITextureMappingAsset, disable_atlas),
            },
            FieldInfoData {
                name: "ForceAtlas",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UITextureMappingAsset, force_atlas),
            },
        ],
    }),
    array_type: Some(UITEXTUREMAPPINGASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UITextureMappingAsset {
    fn type_info(&self) -> &'static TypeInfo {
        UITEXTUREMAPPINGASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UITEXTUREMAPPINGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITextureMappingAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UILayoutMode {
    #[default]
    UILayoutMode_AnchorOffset = 0,
    UILayoutMode_PositionExpansion = 1,
}

pub static UILAYOUTMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UILayoutMode",
    flags: MemberInfoFlags::new(49429),
    module: "GameSharedUI",
    data: TypeInfoData::Enum,
    array_type: Some(UILAYOUTMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UILayoutMode {
    fn type_info(&self) -> &'static TypeInfo {
        UILAYOUTMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UILAYOUTMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UILayoutMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UILayoutMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UITextureMappingCompartment {
    #[default]
    UITextureMappingCompartment_Default = 0,
    UITextureMappingCompartment_Static = 1,
}

pub static UITEXTUREMAPPINGCOMPARTMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingCompartment",
    flags: MemberInfoFlags::new(49429),
    module: "GameSharedUI",
    data: TypeInfoData::Enum,
    array_type: Some(UITEXTUREMAPPINGCOMPARTMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UITextureMappingCompartment {
    fn type_info(&self) -> &'static TypeInfo {
        UITEXTUREMAPPINGCOMPARTMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UITEXTUREMAPPINGCOMPARTMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingCompartment-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITextureMappingCompartment"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UITextureMappingOutputEntry {
    pub id: i32,
    pub texture_ref: glacier_reflect::builtin::ResourceRef,
    pub uv_rect: super::core::Vec4,
}

pub trait UITextureMappingOutputEntryTrait: TypeObject {
    fn id(&self) -> &i32;
    fn texture_ref(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn uv_rect(&self) -> &super::core::Vec4;
}

impl UITextureMappingOutputEntryTrait for UITextureMappingOutputEntry {
    fn id(&self) -> &i32 {
        &self.id
    }
    fn texture_ref(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.texture_ref
    }
    fn uv_rect(&self) -> &super::core::Vec4 {
        &self.uv_rect
    }
}

pub static UITEXTUREMAPPINGOUTPUTENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingOutputEntry",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UITextureMappingOutputEntry as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UITextureMappingOutputEntry, id),
            },
            FieldInfoData {
                name: "TextureRef",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(UITextureMappingOutputEntry, texture_ref),
            },
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UITextureMappingOutputEntry, uv_rect),
            },
        ],
    }),
    array_type: Some(UITEXTUREMAPPINGOUTPUTENTRY_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UITextureMappingOutputEntry {
    fn type_info(&self) -> &'static TypeInfo {
        UITEXTUREMAPPINGOUTPUTENTRY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UITEXTUREMAPPINGOUTPUTENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingOutputEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITextureMappingOutputEntry"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIWidgetBlueprint {
    pub _glacier_base: super::entity::ObjectBlueprint,
}

pub trait UIWidgetBlueprintTrait: super::entity::ObjectBlueprintTrait {
}

impl UIWidgetBlueprintTrait for UIWidgetBlueprint {
}

impl super::entity::ObjectBlueprintTrait for UIWidgetBlueprint {
    fn object(&self) -> &Option<Arc<Mutex<dyn super::entity::EntityDataTrait>>> {
        self._glacier_base.object()
    }
}

impl super::entity::BlueprintTrait for UIWidgetBlueprint {
    fn objects(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.objects()
    }
    fn schematics(&self) -> &Option<Arc<Mutex<dyn super::schematics::SchematicsBaseAssetTrait>>> {
        self._glacier_base.schematics()
    }
}

impl super::entity::EntityBusDataTrait for UIWidgetBlueprint {
    fn event_connections(&self) -> &Vec<super::entity::EventConnection> {
        self._glacier_base.event_connections()
    }
}

impl super::core::DataBusDataTrait for UIWidgetBlueprint {
    fn flags(&self) -> &u16 {
        self._glacier_base.flags()
    }
    fn property_connections(&self) -> &Vec<super::core::PropertyConnection> {
        self._glacier_base.property_connections()
    }
    fn link_connections(&self) -> &Vec<super::core::LinkConnection> {
        self._glacier_base.link_connections()
    }
    fn interface(&self) -> &Option<Arc<Mutex<dyn super::core::DynamicDataContainerTrait>>> {
        self._glacier_base.interface()
    }
}

impl super::core::AssetTrait for UIWidgetBlueprint {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UIWidgetBlueprint {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIWIDGETBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBJECTBLUEPRINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWidgetBlueprint as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIWIDGETBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIWidgetBlueprint {
    fn type_info(&self) -> &'static TypeInfo {
        UIWIDGETBLUEPRINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIWIDGETBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIWidgetBlueprint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIImmediateModeFontConfigurationAsset {
    pub _glacier_base: super::gameplay_sim::UIFontConfigurationAssetBase,
    pub font_bundles: Vec<UIImmediateModeFontBundle>,
    pub font_dpi: i32,
    pub glyph_cache_size: i32,
    pub glyph_cache_size_low_end: i32,
    pub glyph_cache_padding: i32,
    pub auto_hinting: bool,
}

pub trait UIImmediateModeFontConfigurationAssetTrait: super::gameplay_sim::UIFontConfigurationAssetBaseTrait {
    fn font_bundles(&self) -> &Vec<UIImmediateModeFontBundle>;
    fn font_dpi(&self) -> &i32;
    fn glyph_cache_size(&self) -> &i32;
    fn glyph_cache_size_low_end(&self) -> &i32;
    fn glyph_cache_padding(&self) -> &i32;
    fn auto_hinting(&self) -> &bool;
}

impl UIImmediateModeFontConfigurationAssetTrait for UIImmediateModeFontConfigurationAsset {
    fn font_bundles(&self) -> &Vec<UIImmediateModeFontBundle> {
        &self.font_bundles
    }
    fn font_dpi(&self) -> &i32 {
        &self.font_dpi
    }
    fn glyph_cache_size(&self) -> &i32 {
        &self.glyph_cache_size
    }
    fn glyph_cache_size_low_end(&self) -> &i32 {
        &self.glyph_cache_size_low_end
    }
    fn glyph_cache_padding(&self) -> &i32 {
        &self.glyph_cache_padding
    }
    fn auto_hinting(&self) -> &bool {
        &self.auto_hinting
    }
}

impl super::gameplay_sim::UIFontConfigurationAssetBaseTrait for UIImmediateModeFontConfigurationAsset {
}

impl super::core::AssetTrait for UIImmediateModeFontConfigurationAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UIImmediateModeFontConfigurationAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIIMMEDIATEMODEFONTCONFIGURATIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontConfigurationAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::UIFONTCONFIGURATIONASSETBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImmediateModeFontConfigurationAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FontBundles",
                flags: MemberInfoFlags::new(144),
                field_type: "UIImmediateModeFontBundle-Array",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, font_bundles),
            },
            FieldInfoData {
                name: "FontDpi",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, font_dpi),
            },
            FieldInfoData {
                name: "GlyphCacheSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, glyph_cache_size),
            },
            FieldInfoData {
                name: "GlyphCacheSizeLowEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, glyph_cache_size_low_end),
            },
            FieldInfoData {
                name: "GlyphCachePadding",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, glyph_cache_padding),
            },
            FieldInfoData {
                name: "AutoHinting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, auto_hinting),
            },
        ],
    }),
    array_type: Some(UIIMMEDIATEMODEFONTCONFIGURATIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIImmediateModeFontConfigurationAsset {
    fn type_info(&self) -> &'static TypeInfo {
        UIIMMEDIATEMODEFONTCONFIGURATIONASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIIMMEDIATEMODEFONTCONFIGURATIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontConfigurationAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIImmediateModeFontConfigurationAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIImmediateModeFontLookup {
    pub language: super::core::LanguageFormat,
    pub font_asset_path: String,
}

pub trait UIImmediateModeFontLookupTrait: TypeObject {
    fn language(&self) -> &super::core::LanguageFormat;
    fn font_asset_path(&self) -> &String;
}

impl UIImmediateModeFontLookupTrait for UIImmediateModeFontLookup {
    fn language(&self) -> &super::core::LanguageFormat {
        &self.language
    }
    fn font_asset_path(&self) -> &String {
        &self.font_asset_path
    }
}

pub static UIIMMEDIATEMODEFONTLOOKUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontLookup",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImmediateModeFontLookup as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(UIImmediateModeFontLookup, language),
            },
            FieldInfoData {
                name: "FontAssetPath",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIImmediateModeFontLookup, font_asset_path),
            },
        ],
    }),
    array_type: Some(UIIMMEDIATEMODEFONTLOOKUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIImmediateModeFontLookup {
    fn type_info(&self) -> &'static TypeInfo {
        UIIMMEDIATEMODEFONTLOOKUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIIMMEDIATEMODEFONTLOOKUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontLookup-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIImmediateModeFontLookup"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIImmediateModeFontBundle {
    pub language: super::core::LanguageFormat,
    pub bundle_path: String,
    pub ttf_assets: Vec<Option<Arc<Mutex<dyn UITtfAssetTrait>>>>,
}

pub trait UIImmediateModeFontBundleTrait: TypeObject {
    fn language(&self) -> &super::core::LanguageFormat;
    fn bundle_path(&self) -> &String;
    fn ttf_assets(&self) -> &Vec<Option<Arc<Mutex<dyn UITtfAssetTrait>>>>;
}

impl UIImmediateModeFontBundleTrait for UIImmediateModeFontBundle {
    fn language(&self) -> &super::core::LanguageFormat {
        &self.language
    }
    fn bundle_path(&self) -> &String {
        &self.bundle_path
    }
    fn ttf_assets(&self) -> &Vec<Option<Arc<Mutex<dyn UITtfAssetTrait>>>> {
        &self.ttf_assets
    }
}

pub static UIIMMEDIATEMODEFONTBUNDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontBundle",
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImmediateModeFontBundle as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(UIImmediateModeFontBundle, language),
            },
            FieldInfoData {
                name: "BundlePath",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIImmediateModeFontBundle, bundle_path),
            },
            FieldInfoData {
                name: "TtfAssets",
                flags: MemberInfoFlags::new(144),
                field_type: "UITtfAsset-Array",
                rust_offset: offset_of!(UIImmediateModeFontBundle, ttf_assets),
            },
        ],
    }),
    array_type: Some(UIIMMEDIATEMODEFONTBUNDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIImmediateModeFontBundle {
    fn type_info(&self) -> &'static TypeInfo {
        UIIMMEDIATEMODEFONTBUNDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIIMMEDIATEMODEFONTBUNDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontBundle-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIImmediateModeFontBundle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UITtfAsset {
    pub _glacier_base: super::core::Asset,
    pub font_family_name: String,
    pub italic: bool,
    pub bold: bool,
    pub font_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait UITtfAssetTrait: super::core::AssetTrait {
    fn font_family_name(&self) -> &String;
    fn italic(&self) -> &bool;
    fn bold(&self) -> &bool;
    fn font_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl UITtfAssetTrait for UITtfAsset {
    fn font_family_name(&self) -> &String {
        &self.font_family_name
    }
    fn italic(&self) -> &bool {
        &self.italic
    }
    fn bold(&self) -> &bool {
        &self.bold
    }
    fn font_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.font_resource
    }
}

impl super::core::AssetTrait for UITtfAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UITtfAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UITTFASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITtfAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UITtfAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FontFamilyName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UITtfAsset, font_family_name),
            },
            FieldInfoData {
                name: "Italic",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UITtfAsset, italic),
            },
            FieldInfoData {
                name: "Bold",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UITtfAsset, bold),
            },
            FieldInfoData {
                name: "FontResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(UITtfAsset, font_resource),
            },
        ],
    }),
    array_type: Some(UITTFASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UITtfAsset {
    fn type_info(&self) -> &'static TypeInfo {
        UITTFASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UITTFASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITtfAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITtfAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIInputEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait UIInputEntityDataTrait: super::entity::EntityDataTrait {
}

impl UIInputEntityDataTrait for UIInputEntityData {
}

impl super::entity::EntityDataTrait for UIInputEntityData {
}

impl super::entity::GameObjectDataTrait for UIInputEntityData {
}

impl super::core::DataBusPeerTrait for UIInputEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for UIInputEntityData {
}

impl super::core::DataContainerTrait for UIInputEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIINPUTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIInputEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIINPUTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIInputEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        UIINPUTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIINPUTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIInputEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MovieTrackData {
    pub _glacier_base: super::timeline::GuideTrackData,
    pub keyframes: Vec<Option<Arc<Mutex<dyn MovieTrackKeyframeTrait>>>>,
    pub volume: f32,
    pub expose_on_movie_started: bool,
    pub disable_world_renderer: bool,
}

pub trait MovieTrackDataTrait: super::timeline::GuideTrackDataTrait {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn MovieTrackKeyframeTrait>>>>;
    fn volume(&self) -> &f32;
    fn expose_on_movie_started(&self) -> &bool;
    fn disable_world_renderer(&self) -> &bool;
}

impl MovieTrackDataTrait for MovieTrackData {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn MovieTrackKeyframeTrait>>>> {
        &self.keyframes
    }
    fn volume(&self) -> &f32 {
        &self.volume
    }
    fn expose_on_movie_started(&self) -> &bool {
        &self.expose_on_movie_started
    }
    fn disable_world_renderer(&self) -> &bool {
        &self.disable_world_renderer
    }
}

impl super::timeline::GuideTrackDataTrait for MovieTrackData {
    fn guide_track_priority(&self) -> &i32 {
        self._glacier_base.guide_track_priority()
    }
}

impl super::timeline::TimelineTrackDataTrait for MovieTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
}

impl super::entity::GameObjectDataTrait for MovieTrackData {
}

impl super::core::DataBusPeerTrait for MovieTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for MovieTrackData {
}

impl super::core::DataContainerTrait for MovieTrackData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MOVIETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::GUIDETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "MovieTrackKeyframe-Array",
                rust_offset: offset_of!(MovieTrackData, keyframes),
            },
            FieldInfoData {
                name: "Volume",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieTrackData, volume),
            },
            FieldInfoData {
                name: "ExposeOnMovieStarted",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTrackData, expose_on_movie_started),
            },
            FieldInfoData {
                name: "DisableWorldRenderer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTrackData, disable_world_renderer),
            },
        ],
    }),
    array_type: Some(MOVIETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIETRACKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVIETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("MovieTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MovieTrackKeyframe {
    pub _glacier_base: super::core::DataContainer,
    pub time: f32,
    pub length: f32,
    pub movie: Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>>,
    pub pause_on_ending: bool,
    pub renderable_count: u32,
    pub thread_count: u32,
}

pub trait MovieTrackKeyframeTrait: super::core::DataContainerTrait {
    fn time(&self) -> &f32;
    fn length(&self) -> &f32;
    fn movie(&self) -> &Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>>;
    fn pause_on_ending(&self) -> &bool;
    fn renderable_count(&self) -> &u32;
    fn thread_count(&self) -> &u32;
}

impl MovieTrackKeyframeTrait for MovieTrackKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn movie(&self) -> &Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>> {
        &self.movie
    }
    fn pause_on_ending(&self) -> &bool {
        &self.pause_on_ending
    }
    fn renderable_count(&self) -> &u32 {
        &self.renderable_count
    }
    fn thread_count(&self) -> &u32 {
        &self.thread_count
    }
}

impl super::core::DataContainerTrait for MovieTrackKeyframe {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MOVIETRACKKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieTrackKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieTrackKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieTrackKeyframe, length),
            },
            FieldInfoData {
                name: "Movie",
                flags: MemberInfoFlags::new(0),
                field_type: "MovieTextureBaseAsset",
                rust_offset: offset_of!(MovieTrackKeyframe, movie),
            },
            FieldInfoData {
                name: "PauseOnEnding",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTrackKeyframe, pause_on_ending),
            },
            FieldInfoData {
                name: "RenderableCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTrackKeyframe, renderable_count),
            },
            FieldInfoData {
                name: "ThreadCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTrackKeyframe, thread_count),
            },
        ],
    }),
    array_type: Some(MOVIETRACKKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieTrackKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIETRACKKEYFRAME_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVIETRACKKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("MovieTrackKeyframe"),
    array_type: None,
    alignment: 8,
};


