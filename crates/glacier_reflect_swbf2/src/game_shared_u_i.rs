use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct UIGeometryAsset {
    pub _glacier_base: super::core::DataContainer,
}

pub trait UIGeometryAssetTrait: super::core::DataContainerTrait {
}

impl UIGeometryAssetTrait for UIGeometryAsset {
}

impl super::core::DataContainerTrait for UIGeometryAsset {
}

pub static UIGEOMETRYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIGeometryAsset",
    name_hash: 1136441875,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(UIGeometryAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIGeometryAsset as Default>::default())),
            create_boxed: || Box::new(<UIGeometryAsset as Default>::default()),
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


pub static UIGEOMETRYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIGeometryAsset-Array",
    name_hash: 2064970919,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIGeometryAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIAutoMappedTexture {
    pub id: i32,
    pub texture_ref: glacier_reflect::builtin::ResourceRef,
}

pub trait UIAutoMappedTextureTrait: TypeObject {
    fn id(&self) -> &i32;
    fn id_mut(&mut self) -> &mut i32;
    fn texture_ref(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn texture_ref_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl UIAutoMappedTextureTrait for UIAutoMappedTexture {
    fn id(&self) -> &i32 {
        &self.id
    }
    fn id_mut(&mut self) -> &mut i32 {
        &mut self.id
    }
    fn texture_ref(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.texture_ref
    }
    fn texture_ref_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.texture_ref
    }
}

pub static UIAUTOMAPPEDTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoMappedTexture",
    name_hash: 3500657796,
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIAutoMappedTexture as Default>::default())),
            create_boxed: || Box::new(<UIAutoMappedTexture as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                name_hash: 5862152,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIAutoMappedTexture, id),
            },
            FieldInfoData {
                name: "TextureRef",
                name_hash: 4257491627,
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


pub static UIAUTOMAPPEDTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoMappedTexture-Array",
    name_hash: 3712843952,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIAutoMappedTexture"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn state_stream_enabled_mut(&mut self) -> &mut bool;
    fn state_stream_lane(&self) -> &UICppScreenStateStreamLaneType;
    fn state_stream_lane_mut(&mut self) -> &mut UICppScreenStateStreamLaneType;
    fn field_of_view(&self) -> &f32;
    fn field_of_view_mut(&mut self) -> &mut f32;
    fn scale_up_and_keep_aspect_ratio(&self) -> &bool;
    fn scale_up_and_keep_aspect_ratio_mut(&mut self) -> &mut bool;
    fn flash_compatibility_mode(&self) -> &bool;
    fn flash_compatibility_mode_mut(&mut self) -> &mut bool;
    fn screen_layout_width(&self) -> &f32;
    fn screen_layout_width_mut(&mut self) -> &mut f32;
    fn screen_layout_height(&self) -> &f32;
    fn screen_layout_height_mut(&mut self) -> &mut f32;
    fn allow_input(&self) -> &bool;
    fn allow_input_mut(&mut self) -> &mut bool;
    fn eat_all_input(&self) -> &bool;
    fn eat_all_input_mut(&mut self) -> &mut bool;
    fn layout_with_safe_zone(&self) -> &bool;
    fn layout_with_safe_zone_mut(&mut self) -> &mut bool;
    fn screen_sampler_settings(&self) -> &super::game_base::UIScreenSamplerSettings;
    fn screen_sampler_settings_mut(&mut self) -> &mut super::game_base::UIScreenSamplerSettings;
}

impl UICppScreenDataTrait for UICppScreenData {
    fn state_stream_enabled(&self) -> &bool {
        &self.state_stream_enabled
    }
    fn state_stream_enabled_mut(&mut self) -> &mut bool {
        &mut self.state_stream_enabled
    }
    fn state_stream_lane(&self) -> &UICppScreenStateStreamLaneType {
        &self.state_stream_lane
    }
    fn state_stream_lane_mut(&mut self) -> &mut UICppScreenStateStreamLaneType {
        &mut self.state_stream_lane
    }
    fn field_of_view(&self) -> &f32 {
        &self.field_of_view
    }
    fn field_of_view_mut(&mut self) -> &mut f32 {
        &mut self.field_of_view
    }
    fn scale_up_and_keep_aspect_ratio(&self) -> &bool {
        &self.scale_up_and_keep_aspect_ratio
    }
    fn scale_up_and_keep_aspect_ratio_mut(&mut self) -> &mut bool {
        &mut self.scale_up_and_keep_aspect_ratio
    }
    fn flash_compatibility_mode(&self) -> &bool {
        &self.flash_compatibility_mode
    }
    fn flash_compatibility_mode_mut(&mut self) -> &mut bool {
        &mut self.flash_compatibility_mode
    }
    fn screen_layout_width(&self) -> &f32 {
        &self.screen_layout_width
    }
    fn screen_layout_width_mut(&mut self) -> &mut f32 {
        &mut self.screen_layout_width
    }
    fn screen_layout_height(&self) -> &f32 {
        &self.screen_layout_height
    }
    fn screen_layout_height_mut(&mut self) -> &mut f32 {
        &mut self.screen_layout_height
    }
    fn allow_input(&self) -> &bool {
        &self.allow_input
    }
    fn allow_input_mut(&mut self) -> &mut bool {
        &mut self.allow_input
    }
    fn eat_all_input(&self) -> &bool {
        &self.eat_all_input
    }
    fn eat_all_input_mut(&mut self) -> &mut bool {
        &mut self.eat_all_input
    }
    fn layout_with_safe_zone(&self) -> &bool {
        &self.layout_with_safe_zone
    }
    fn layout_with_safe_zone_mut(&mut self) -> &mut bool {
        &mut self.layout_with_safe_zone
    }
    fn screen_sampler_settings(&self) -> &super::game_base::UIScreenSamplerSettings {
        &self.screen_sampler_settings
    }
    fn screen_sampler_settings_mut(&mut self) -> &mut super::game_base::UIScreenSamplerSettings {
        &mut self.screen_sampler_settings
    }
}

impl super::core::AssetTrait for UICppScreenData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for UICppScreenData {
}

pub static UICPPSCREENDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenData",
    name_hash: 1908284550,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(UICppScreenData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UICppScreenData as Default>::default())),
            create_boxed: || Box::new(<UICppScreenData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "StateStreamEnabled",
                name_hash: 3289043691,
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, state_stream_enabled),
            },
            FieldInfoData {
                name: "StateStreamLane",
                name_hash: 1758015464,
                flags: MemberInfoFlags::new(0),
                field_type: "UICppScreenStateStreamLaneType",
                rust_offset: offset_of!(UICppScreenData, state_stream_lane),
            },
            FieldInfoData {
                name: "FieldOfView",
                name_hash: 2227716035,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UICppScreenData, field_of_view),
            },
            FieldInfoData {
                name: "ScaleUpAndKeepAspectRatio",
                name_hash: 348194777,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, scale_up_and_keep_aspect_ratio),
            },
            FieldInfoData {
                name: "FlashCompatibilityMode",
                name_hash: 246112600,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, flash_compatibility_mode),
            },
            FieldInfoData {
                name: "ScreenLayoutWidth",
                name_hash: 848250837,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UICppScreenData, screen_layout_width),
            },
            FieldInfoData {
                name: "ScreenLayoutHeight",
                name_hash: 3186636684,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UICppScreenData, screen_layout_height),
            },
            FieldInfoData {
                name: "AllowInput",
                name_hash: 3351578154,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, allow_input),
            },
            FieldInfoData {
                name: "EatAllInput",
                name_hash: 1492768322,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, eat_all_input),
            },
            FieldInfoData {
                name: "LayoutWithSafeZone",
                name_hash: 3399620050,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UICppScreenData, layout_with_safe_zone),
            },
            FieldInfoData {
                name: "ScreenSamplerSettings",
                name_hash: 1329586712,
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


pub static UICPPSCREENDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenData-Array",
    name_hash: 1133250994,
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
    name_hash: 1987728323,
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


pub static UICPPSCREENSTATESTREAMLANETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreenStateStreamLaneType-Array",
    name_hash: 2024527607,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UICppScreenStateStreamLaneType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIScreenRenderTargetEntityData {
    pub _glacier_base: UIScreenRenderEntityData,
    pub render_target: Option<LockedTypeObject /* super::render::RenderTextureAsset */>,
    pub generate_render_target: bool,
    pub create_render_target_stencil: bool,
    pub clear_render_target: bool,
    pub render_on_event: bool,
}

pub trait UIScreenRenderTargetEntityDataTrait: UIScreenRenderEntityDataTrait {
    fn render_target(&self) -> &Option<LockedTypeObject /* super::render::RenderTextureAsset */>;
    fn render_target_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render::RenderTextureAsset */>;
    fn generate_render_target(&self) -> &bool;
    fn generate_render_target_mut(&mut self) -> &mut bool;
    fn create_render_target_stencil(&self) -> &bool;
    fn create_render_target_stencil_mut(&mut self) -> &mut bool;
    fn clear_render_target(&self) -> &bool;
    fn clear_render_target_mut(&mut self) -> &mut bool;
    fn render_on_event(&self) -> &bool;
    fn render_on_event_mut(&mut self) -> &mut bool;
}

impl UIScreenRenderTargetEntityDataTrait for UIScreenRenderTargetEntityData {
    fn render_target(&self) -> &Option<LockedTypeObject /* super::render::RenderTextureAsset */> {
        &self.render_target
    }
    fn render_target_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render::RenderTextureAsset */> {
        &mut self.render_target
    }
    fn generate_render_target(&self) -> &bool {
        &self.generate_render_target
    }
    fn generate_render_target_mut(&mut self) -> &mut bool {
        &mut self.generate_render_target
    }
    fn create_render_target_stencil(&self) -> &bool {
        &self.create_render_target_stencil
    }
    fn create_render_target_stencil_mut(&mut self) -> &mut bool {
        &mut self.create_render_target_stencil
    }
    fn clear_render_target(&self) -> &bool {
        &self.clear_render_target
    }
    fn clear_render_target_mut(&mut self) -> &mut bool {
        &mut self.clear_render_target
    }
    fn render_on_event(&self) -> &bool {
        &self.render_on_event
    }
    fn render_on_event_mut(&mut self) -> &mut bool {
        &mut self.render_on_event
    }
}

impl UIScreenRenderEntityDataTrait for UIScreenRenderTargetEntityData {
    fn screen_data(&self) -> &Option<LockedTypeObject /* UICppScreenData */> {
        self._glacier_base.screen_data()
    }
    fn screen_data_mut(&mut self) -> &mut Option<LockedTypeObject /* UICppScreenData */> {
        self._glacier_base.screen_data_mut()
    }
    fn use_game_view_projection(&self) -> &bool {
        self._glacier_base.use_game_view_projection()
    }
    fn use_game_view_projection_mut(&mut self) -> &mut bool {
        self._glacier_base.use_game_view_projection_mut()
    }
    fn scale(&self) -> &f32 {
        self._glacier_base.scale()
    }
    fn scale_mut(&mut self) -> &mut f32 {
        self._glacier_base.scale_mut()
    }
    fn enable_depth_culling(&self) -> &bool {
        self._glacier_base.enable_depth_culling()
    }
    fn enable_depth_culling_mut(&mut self) -> &mut bool {
        self._glacier_base.enable_depth_culling_mut()
    }
    fn projection_mode(&self) -> &super::game_base::UIScreenProjectionMode {
        self._glacier_base.projection_mode()
    }
    fn projection_mode_mut(&mut self) -> &mut super::game_base::UIScreenProjectionMode {
        self._glacier_base.projection_mode_mut()
    }
    fn render_pass(&self) -> &UIScreenRenderingPass {
        self._glacier_base.render_pass()
    }
    fn render_pass_mut(&mut self) -> &mut UIScreenRenderingPass {
        self._glacier_base.render_pass_mut()
    }
    fn update_order(&self) -> &i32 {
        self._glacier_base.update_order()
    }
    fn update_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.update_order_mut()
    }
    fn center_screen(&self) -> &bool {
        self._glacier_base.center_screen()
    }
    fn center_screen_mut(&mut self) -> &mut bool {
        self._glacier_base.center_screen_mut()
    }
    fn view_id(&self) -> &super::render_base::LocalPlayerViewId {
        self._glacier_base.view_id()
    }
    fn view_id_mut(&mut self) -> &mut super::render_base::LocalPlayerViewId {
        self._glacier_base.view_id_mut()
    }
    fn start_enabled(&self) -> &bool {
        self._glacier_base.start_enabled()
    }
    fn start_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.start_enabled_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
}

impl super::entity::LogicReferenceObjectDataTrait for UIScreenRenderTargetEntityData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        self._glacier_base.sub_realm()
    }
    fn sub_realm_mut(&mut self) -> &mut super::entity::SubRealm {
        self._glacier_base.sub_realm_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for UIScreenRenderTargetEntityData {
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

impl super::entity::GameObjectDataTrait for UIScreenRenderTargetEntityData {
}

impl super::core::DataBusPeerTrait for UIScreenRenderTargetEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIScreenRenderTargetEntityData {
}

impl super::core::DataContainerTrait for UIScreenRenderTargetEntityData {
}

pub static UISCREENRENDERTARGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderTargetEntityData",
    name_hash: 637304357,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UISCREENRENDERENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(UIScreenRenderTargetEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIScreenRenderTargetEntityData as Default>::default())),
            create_boxed: || Box::new(<UIScreenRenderTargetEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RenderTarget",
                name_hash: 1581516062,
                flags: MemberInfoFlags::new(0),
                field_type: "RenderTextureAsset",
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, render_target),
            },
            FieldInfoData {
                name: "GenerateRenderTarget",
                name_hash: 3812951381,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, generate_render_target),
            },
            FieldInfoData {
                name: "CreateRenderTargetStencil",
                name_hash: 4291601680,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, create_render_target_stencil),
            },
            FieldInfoData {
                name: "ClearRenderTarget",
                name_hash: 2373989895,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderTargetEntityData, clear_render_target),
            },
            FieldInfoData {
                name: "RenderOnEvent",
                name_hash: 2567163394,
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


pub static UISCREENRENDERTARGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderTargetEntityData-Array",
    name_hash: 1245181585,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIScreenRenderTargetEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIScreenRenderEntityData {
    pub _glacier_base: super::entity::LogicReferenceObjectData,
    pub screen_data: Option<LockedTypeObject /* UICppScreenData */>,
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
    fn screen_data(&self) -> &Option<LockedTypeObject /* UICppScreenData */>;
    fn screen_data_mut(&mut self) -> &mut Option<LockedTypeObject /* UICppScreenData */>;
    fn use_game_view_projection(&self) -> &bool;
    fn use_game_view_projection_mut(&mut self) -> &mut bool;
    fn scale(&self) -> &f32;
    fn scale_mut(&mut self) -> &mut f32;
    fn enable_depth_culling(&self) -> &bool;
    fn enable_depth_culling_mut(&mut self) -> &mut bool;
    fn projection_mode(&self) -> &super::game_base::UIScreenProjectionMode;
    fn projection_mode_mut(&mut self) -> &mut super::game_base::UIScreenProjectionMode;
    fn render_pass(&self) -> &UIScreenRenderingPass;
    fn render_pass_mut(&mut self) -> &mut UIScreenRenderingPass;
    fn update_order(&self) -> &i32;
    fn update_order_mut(&mut self) -> &mut i32;
    fn center_screen(&self) -> &bool;
    fn center_screen_mut(&mut self) -> &mut bool;
    fn view_id(&self) -> &super::render_base::LocalPlayerViewId;
    fn view_id_mut(&mut self) -> &mut super::render_base::LocalPlayerViewId;
    fn start_enabled(&self) -> &bool;
    fn start_enabled_mut(&mut self) -> &mut bool;
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
    fn alpha(&self) -> &f32;
    fn alpha_mut(&mut self) -> &mut f32;
}

impl UIScreenRenderEntityDataTrait for UIScreenRenderEntityData {
    fn screen_data(&self) -> &Option<LockedTypeObject /* UICppScreenData */> {
        &self.screen_data
    }
    fn screen_data_mut(&mut self) -> &mut Option<LockedTypeObject /* UICppScreenData */> {
        &mut self.screen_data
    }
    fn use_game_view_projection(&self) -> &bool {
        &self.use_game_view_projection
    }
    fn use_game_view_projection_mut(&mut self) -> &mut bool {
        &mut self.use_game_view_projection
    }
    fn scale(&self) -> &f32 {
        &self.scale
    }
    fn scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }
    fn enable_depth_culling(&self) -> &bool {
        &self.enable_depth_culling
    }
    fn enable_depth_culling_mut(&mut self) -> &mut bool {
        &mut self.enable_depth_culling
    }
    fn projection_mode(&self) -> &super::game_base::UIScreenProjectionMode {
        &self.projection_mode
    }
    fn projection_mode_mut(&mut self) -> &mut super::game_base::UIScreenProjectionMode {
        &mut self.projection_mode
    }
    fn render_pass(&self) -> &UIScreenRenderingPass {
        &self.render_pass
    }
    fn render_pass_mut(&mut self) -> &mut UIScreenRenderingPass {
        &mut self.render_pass
    }
    fn update_order(&self) -> &i32 {
        &self.update_order
    }
    fn update_order_mut(&mut self) -> &mut i32 {
        &mut self.update_order
    }
    fn center_screen(&self) -> &bool {
        &self.center_screen
    }
    fn center_screen_mut(&mut self) -> &mut bool {
        &mut self.center_screen
    }
    fn view_id(&self) -> &super::render_base::LocalPlayerViewId {
        &self.view_id
    }
    fn view_id_mut(&mut self) -> &mut super::render_base::LocalPlayerViewId {
        &mut self.view_id
    }
    fn start_enabled(&self) -> &bool {
        &self.start_enabled
    }
    fn start_enabled_mut(&mut self) -> &mut bool {
        &mut self.start_enabled
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        &mut self.alpha
    }
}

impl super::entity::LogicReferenceObjectDataTrait for UIScreenRenderEntityData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        self._glacier_base.sub_realm()
    }
    fn sub_realm_mut(&mut self) -> &mut super::entity::SubRealm {
        self._glacier_base.sub_realm_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for UIScreenRenderEntityData {
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

impl super::entity::GameObjectDataTrait for UIScreenRenderEntityData {
}

impl super::core::DataBusPeerTrait for UIScreenRenderEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIScreenRenderEntityData {
}

impl super::core::DataContainerTrait for UIScreenRenderEntityData {
}

pub static UISCREENRENDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderEntityData",
    name_hash: 2055422004,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(UIScreenRenderEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIScreenRenderEntityData as Default>::default())),
            create_boxed: || Box::new(<UIScreenRenderEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ScreenData",
                name_hash: 2099895321,
                flags: MemberInfoFlags::new(0),
                field_type: "UICppScreenData",
                rust_offset: offset_of!(UIScreenRenderEntityData, screen_data),
            },
            FieldInfoData {
                name: "UseGameViewProjection",
                name_hash: 229705400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderEntityData, use_game_view_projection),
            },
            FieldInfoData {
                name: "Scale",
                name_hash: 231223453,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIScreenRenderEntityData, scale),
            },
            FieldInfoData {
                name: "EnableDepthCulling",
                name_hash: 451580447,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderEntityData, enable_depth_culling),
            },
            FieldInfoData {
                name: "ProjectionMode",
                name_hash: 2662232091,
                flags: MemberInfoFlags::new(0),
                field_type: "UIScreenProjectionMode",
                rust_offset: offset_of!(UIScreenRenderEntityData, projection_mode),
            },
            FieldInfoData {
                name: "RenderPass",
                name_hash: 605021150,
                flags: MemberInfoFlags::new(0),
                field_type: "UIScreenRenderingPass",
                rust_offset: offset_of!(UIScreenRenderEntityData, render_pass),
            },
            FieldInfoData {
                name: "UpdateOrder",
                name_hash: 1950297146,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIScreenRenderEntityData, update_order),
            },
            FieldInfoData {
                name: "CenterScreen",
                name_hash: 1445151746,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderEntityData, center_screen),
            },
            FieldInfoData {
                name: "ViewId",
                name_hash: 3151570821,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerViewId",
                rust_offset: offset_of!(UIScreenRenderEntityData, view_id),
            },
            FieldInfoData {
                name: "StartEnabled",
                name_hash: 360001056,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenRenderEntityData, start_enabled),
            },
            FieldInfoData {
                name: "Color",
                name_hash: 212387320,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIScreenRenderEntityData, color),
            },
            FieldInfoData {
                name: "Alpha",
                name_hash: 205677681,
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


pub static UISCREENRENDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderEntityData-Array",
    name_hash: 134462336,
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
    name_hash: 3176241102,
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


pub static UISCREENRENDERINGPASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderingPass-Array",
    name_hash: 1995346298,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIScreenRenderingPass"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIIMSettingsAsset {
    pub _glacier_base: super::core::Asset,
    pub auto_scroll_settings: UIAutoScrollTextSettings,
}

pub trait UIIMSettingsAssetTrait: super::core::AssetTrait {
    fn auto_scroll_settings(&self) -> &UIAutoScrollTextSettings;
    fn auto_scroll_settings_mut(&mut self) -> &mut UIAutoScrollTextSettings;
}

impl UIIMSettingsAssetTrait for UIIMSettingsAsset {
    fn auto_scroll_settings(&self) -> &UIAutoScrollTextSettings {
        &self.auto_scroll_settings
    }
    fn auto_scroll_settings_mut(&mut self) -> &mut UIAutoScrollTextSettings {
        &mut self.auto_scroll_settings
    }
}

impl super::core::AssetTrait for UIIMSettingsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for UIIMSettingsAsset {
}

pub static UIIMSETTINGSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIIMSettingsAsset",
    name_hash: 3925993608,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(UIIMSettingsAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIIMSettingsAsset as Default>::default())),
            create_boxed: || Box::new(<UIIMSettingsAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AutoScrollSettings",
                name_hash: 1124491778,
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


pub static UIIMSETTINGSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIIMSettingsAsset-Array",
    name_hash: 2450279100,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIIMSettingsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIAutoScrollTextSettings {
    pub no_scroll_wait_time: f32,
    pub fully_scrolled_wait_time: f32,
    pub max_scroll_time: f32,
    pub pixels_per_second: f32,
    pub scrollback_multiplier: f32,
}

pub trait UIAutoScrollTextSettingsTrait: TypeObject {
    fn no_scroll_wait_time(&self) -> &f32;
    fn no_scroll_wait_time_mut(&mut self) -> &mut f32;
    fn fully_scrolled_wait_time(&self) -> &f32;
    fn fully_scrolled_wait_time_mut(&mut self) -> &mut f32;
    fn max_scroll_time(&self) -> &f32;
    fn max_scroll_time_mut(&mut self) -> &mut f32;
    fn pixels_per_second(&self) -> &f32;
    fn pixels_per_second_mut(&mut self) -> &mut f32;
    fn scrollback_multiplier(&self) -> &f32;
    fn scrollback_multiplier_mut(&mut self) -> &mut f32;
}

impl UIAutoScrollTextSettingsTrait for UIAutoScrollTextSettings {
    fn no_scroll_wait_time(&self) -> &f32 {
        &self.no_scroll_wait_time
    }
    fn no_scroll_wait_time_mut(&mut self) -> &mut f32 {
        &mut self.no_scroll_wait_time
    }
    fn fully_scrolled_wait_time(&self) -> &f32 {
        &self.fully_scrolled_wait_time
    }
    fn fully_scrolled_wait_time_mut(&mut self) -> &mut f32 {
        &mut self.fully_scrolled_wait_time
    }
    fn max_scroll_time(&self) -> &f32 {
        &self.max_scroll_time
    }
    fn max_scroll_time_mut(&mut self) -> &mut f32 {
        &mut self.max_scroll_time
    }
    fn pixels_per_second(&self) -> &f32 {
        &self.pixels_per_second
    }
    fn pixels_per_second_mut(&mut self) -> &mut f32 {
        &mut self.pixels_per_second
    }
    fn scrollback_multiplier(&self) -> &f32 {
        &self.scrollback_multiplier
    }
    fn scrollback_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.scrollback_multiplier
    }
}

pub static UIAUTOSCROLLTEXTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoScrollTextSettings",
    name_hash: 1619461955,
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIAutoScrollTextSettings as Default>::default())),
            create_boxed: || Box::new(<UIAutoScrollTextSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "NoScrollWaitTime",
                name_hash: 3177388503,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIAutoScrollTextSettings, no_scroll_wait_time),
            },
            FieldInfoData {
                name: "FullyScrolledWaitTime",
                name_hash: 2227978973,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIAutoScrollTextSettings, fully_scrolled_wait_time),
            },
            FieldInfoData {
                name: "MaxScrollTime",
                name_hash: 2446976009,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIAutoScrollTextSettings, max_scroll_time),
            },
            FieldInfoData {
                name: "PixelsPerSecond",
                name_hash: 3334982569,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIAutoScrollTextSettings, pixels_per_second),
            },
            FieldInfoData {
                name: "ScrollbackMultiplier",
                name_hash: 3922113704,
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


pub static UIAUTOSCROLLTEXTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAutoScrollTextSettings-Array",
    name_hash: 827402871,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIAutoScrollTextSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn instance_name_mut(&mut self) -> &mut String;
    fn instance_name_hash(&self) -> &u32;
    fn instance_name_hash_mut(&mut self) -> &mut u32;
    fn inclusion_settings(&self) -> &UIElementInclusionSettings;
    fn inclusion_settings_mut(&mut self) -> &mut UIElementInclusionSettings;
    fn transform_pivot(&self) -> &super::core::Vec3;
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3;
    fn use_element_size(&self) -> &bool;
    fn use_element_size_mut(&mut self) -> &mut bool;
    fn size(&self) -> &super::core::Vec2;
    fn size_mut(&mut self) -> &mut super::core::Vec2;
    fn layout_mode(&self) -> &UILayoutMode;
    fn layout_mode_mut(&mut self) -> &mut UILayoutMode;
    fn offset(&self) -> &UIElementOffset;
    fn offset_mut(&mut self) -> &mut UIElementOffset;
    fn anchor(&self) -> &UIElementAnchor;
    fn anchor_mut(&mut self) -> &mut UIElementAnchor;
    fn position(&self) -> &UIElementOffset;
    fn position_mut(&mut self) -> &mut UIElementOffset;
    fn expansion(&self) -> &UIElementRectExpansion;
    fn expansion_mut(&mut self) -> &mut UIElementRectExpansion;
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
    fn alpha(&self) -> &f32;
    fn alpha_mut(&mut self) -> &mut f32;
    fn code_access_identifier(&self) -> &String;
    fn code_access_identifier_mut(&mut self) -> &mut String;
}

impl UIElementWidgetReferenceEntityDataTrait for UIElementWidgetReferenceEntityData {
    fn instance_name(&self) -> &String {
        &self.instance_name
    }
    fn instance_name_mut(&mut self) -> &mut String {
        &mut self.instance_name
    }
    fn instance_name_hash(&self) -> &u32 {
        &self.instance_name_hash
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.instance_name_hash
    }
    fn inclusion_settings(&self) -> &UIElementInclusionSettings {
        &self.inclusion_settings
    }
    fn inclusion_settings_mut(&mut self) -> &mut UIElementInclusionSettings {
        &mut self.inclusion_settings
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        &self.transform_pivot
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.transform_pivot
    }
    fn use_element_size(&self) -> &bool {
        &self.use_element_size
    }
    fn use_element_size_mut(&mut self) -> &mut bool {
        &mut self.use_element_size
    }
    fn size(&self) -> &super::core::Vec2 {
        &self.size
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.size
    }
    fn layout_mode(&self) -> &UILayoutMode {
        &self.layout_mode
    }
    fn layout_mode_mut(&mut self) -> &mut UILayoutMode {
        &mut self.layout_mode
    }
    fn offset(&self) -> &UIElementOffset {
        &self.offset
    }
    fn offset_mut(&mut self) -> &mut UIElementOffset {
        &mut self.offset
    }
    fn anchor(&self) -> &UIElementAnchor {
        &self.anchor
    }
    fn anchor_mut(&mut self) -> &mut UIElementAnchor {
        &mut self.anchor
    }
    fn position(&self) -> &UIElementOffset {
        &self.position
    }
    fn position_mut(&mut self) -> &mut UIElementOffset {
        &mut self.position
    }
    fn expansion(&self) -> &UIElementRectExpansion {
        &self.expansion
    }
    fn expansion_mut(&mut self) -> &mut UIElementRectExpansion {
        &mut self.expansion
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        &mut self.alpha
    }
    fn code_access_identifier(&self) -> &String {
        &self.code_access_identifier
    }
    fn code_access_identifier_mut(&mut self) -> &mut String {
        &mut self.code_access_identifier
    }
}

impl super::entity::LogicReferenceObjectDataTrait for UIElementWidgetReferenceEntityData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
    fn sub_realm(&self) -> &super::entity::SubRealm {
        self._glacier_base.sub_realm()
    }
    fn sub_realm_mut(&mut self) -> &mut super::entity::SubRealm {
        self._glacier_base.sub_realm_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for UIElementWidgetReferenceEntityData {
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

impl super::entity::GameObjectDataTrait for UIElementWidgetReferenceEntityData {
}

impl super::core::DataBusPeerTrait for UIElementWidgetReferenceEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIElementWidgetReferenceEntityData {
}

impl super::core::DataContainerTrait for UIElementWidgetReferenceEntityData {
}

pub static UIELEMENTWIDGETREFERENCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementWidgetReferenceEntityData",
    name_hash: 3895090827,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(UIElementWidgetReferenceEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementWidgetReferenceEntityData as Default>::default())),
            create_boxed: || Box::new(<UIElementWidgetReferenceEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InstanceName",
                name_hash: 1186954283,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, instance_name),
            },
            FieldInfoData {
                name: "InstanceNameHash",
                name_hash: 1430708601,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, instance_name_hash),
            },
            FieldInfoData {
                name: "InclusionSettings",
                name_hash: 1787638950,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementInclusionSettings",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, inclusion_settings),
            },
            FieldInfoData {
                name: "TransformPivot",
                name_hash: 3594781853,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, transform_pivot),
            },
            FieldInfoData {
                name: "UseElementSize",
                name_hash: 354467197,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, use_element_size),
            },
            FieldInfoData {
                name: "Size",
                name_hash: 2089429248,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, size),
            },
            FieldInfoData {
                name: "LayoutMode",
                name_hash: 1932103164,
                flags: MemberInfoFlags::new(0),
                field_type: "UILayoutMode",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, layout_mode),
            },
            FieldInfoData {
                name: "Offset",
                name_hash: 2871410728,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementOffset",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, offset),
            },
            FieldInfoData {
                name: "Anchor",
                name_hash: 2489622940,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementAnchor",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, anchor),
            },
            FieldInfoData {
                name: "Position",
                name_hash: 3402582524,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementOffset",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, position),
            },
            FieldInfoData {
                name: "Expansion",
                name_hash: 2333524892,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementRectExpansion",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, expansion),
            },
            FieldInfoData {
                name: "Color",
                name_hash: 212387320,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, color),
            },
            FieldInfoData {
                name: "Alpha",
                name_hash: 205677681,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementWidgetReferenceEntityData, alpha),
            },
            FieldInfoData {
                name: "CodeAccessIdentifier",
                name_hash: 3690889807,
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


pub static UIELEMENTWIDGETREFERENCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementWidgetReferenceEntityData-Array",
    name_hash: 1010339135,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementWidgetReferenceEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementBitmapDistanceFieldParams {
    pub alpha_threshold: f32,
    pub distance_scale: f32,
    pub outline_inner: f32,
    pub outline_outer: f32,
    pub outline_color: UIElementColor,
}

pub trait UIElementBitmapDistanceFieldParamsTrait: TypeObject {
    fn alpha_threshold(&self) -> &f32;
    fn alpha_threshold_mut(&mut self) -> &mut f32;
    fn distance_scale(&self) -> &f32;
    fn distance_scale_mut(&mut self) -> &mut f32;
    fn outline_inner(&self) -> &f32;
    fn outline_inner_mut(&mut self) -> &mut f32;
    fn outline_outer(&self) -> &f32;
    fn outline_outer_mut(&mut self) -> &mut f32;
    fn outline_color(&self) -> &UIElementColor;
    fn outline_color_mut(&mut self) -> &mut UIElementColor;
}

impl UIElementBitmapDistanceFieldParamsTrait for UIElementBitmapDistanceFieldParams {
    fn alpha_threshold(&self) -> &f32 {
        &self.alpha_threshold
    }
    fn alpha_threshold_mut(&mut self) -> &mut f32 {
        &mut self.alpha_threshold
    }
    fn distance_scale(&self) -> &f32 {
        &self.distance_scale
    }
    fn distance_scale_mut(&mut self) -> &mut f32 {
        &mut self.distance_scale
    }
    fn outline_inner(&self) -> &f32 {
        &self.outline_inner
    }
    fn outline_inner_mut(&mut self) -> &mut f32 {
        &mut self.outline_inner
    }
    fn outline_outer(&self) -> &f32 {
        &self.outline_outer
    }
    fn outline_outer_mut(&mut self) -> &mut f32 {
        &mut self.outline_outer
    }
    fn outline_color(&self) -> &UIElementColor {
        &self.outline_color
    }
    fn outline_color_mut(&mut self) -> &mut UIElementColor {
        &mut self.outline_color
    }
}

pub static UIELEMENTBITMAPDISTANCEFIELDPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementBitmapDistanceFieldParams",
    name_hash: 300361145,
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementBitmapDistanceFieldParams as Default>::default())),
            create_boxed: || Box::new(<UIElementBitmapDistanceFieldParams as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AlphaThreshold",
                name_hash: 2686813414,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, alpha_threshold),
            },
            FieldInfoData {
                name: "DistanceScale",
                name_hash: 773140030,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, distance_scale),
            },
            FieldInfoData {
                name: "OutlineInner",
                name_hash: 3946538011,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, outline_inner),
            },
            FieldInfoData {
                name: "OutlineOuter",
                name_hash: 3949917564,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementBitmapDistanceFieldParams, outline_outer),
            },
            FieldInfoData {
                name: "OutlineColor",
                name_hash: 3944405112,
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


pub static UIELEMENTBITMAPDISTANCEFIELDPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementBitmapDistanceFieldParams-Array",
    name_hash: 2838803469,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementBitmapDistanceFieldParams"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIMaskingContainerEntityData {
    pub _glacier_base: UIContainerEntityData,
    pub masks: Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>,
    pub mask_threshold: f32,
}

pub trait UIMaskingContainerEntityDataTrait: UIContainerEntityDataTrait {
    fn masks(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn masks_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn mask_threshold(&self) -> &f32;
    fn mask_threshold_mut(&mut self) -> &mut f32;
}

impl UIMaskingContainerEntityDataTrait for UIMaskingContainerEntityData {
    fn masks(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &self.masks
    }
    fn masks_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &mut self.masks
    }
    fn mask_threshold(&self) -> &f32 {
        &self.mask_threshold
    }
    fn mask_threshold_mut(&mut self) -> &mut f32 {
        &mut self.mask_threshold
    }
}

impl UIContainerEntityDataTrait for UIMaskingContainerEntityData {
    fn elements(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.elements()
    }
    fn elements_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.elements_mut()
    }
}

impl UIElementEntityDataTrait for UIMaskingContainerEntityData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIMaskingContainerEntityData {
}

impl super::core::DataContainerTrait for UIMaskingContainerEntityData {
}

pub static UIMASKINGCONTAINERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingContainerEntityData",
    name_hash: 188418849,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UICONTAINERENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(UIMaskingContainerEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMaskingContainerEntityData as Default>::default())),
            create_boxed: || Box::new(<UIMaskingContainerEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Masks",
                name_hash: 210251234,
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(UIMaskingContainerEntityData, masks),
            },
            FieldInfoData {
                name: "MaskThreshold",
                name_hash: 2938748902,
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


pub static UIMASKINGCONTAINERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingContainerEntityData-Array",
    name_hash: 1479316885,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIMaskingContainerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIContainerEntityData {
    pub _glacier_base: UIElementEntityData,
    pub elements: Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>,
}

pub trait UIContainerEntityDataTrait: UIElementEntityDataTrait {
    fn elements(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn elements_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
}

impl UIContainerEntityDataTrait for UIContainerEntityData {
    fn elements(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &self.elements
    }
    fn elements_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &mut self.elements
    }
}

impl UIElementEntityDataTrait for UIContainerEntityData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIContainerEntityData {
}

impl super::core::DataContainerTrait for UIContainerEntityData {
}

pub static UICONTAINERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIContainerEntityData",
    name_hash: 498346133,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(UIContainerEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIContainerEntityData as Default>::default())),
            create_boxed: || Box::new(<UIContainerEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Elements",
                name_hash: 2347524808,
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


pub static UICONTAINERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIContainerEntityData-Array",
    name_hash: 1015838753,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIContainerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn instance_name_mut(&mut self) -> &mut String;
    fn instance_name_hash(&self) -> &u32;
    fn instance_name_hash_mut(&mut self) -> &mut u32;
    fn transform_pivot(&self) -> &super::core::Vec3;
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3;
    fn size(&self) -> &super::core::Vec2;
    fn size_mut(&mut self) -> &mut super::core::Vec2;
    fn layout_mode(&self) -> &UILayoutMode;
    fn layout_mode_mut(&mut self) -> &mut UILayoutMode;
    fn offset(&self) -> &UIElementOffset;
    fn offset_mut(&mut self) -> &mut UIElementOffset;
    fn anchor(&self) -> &UIElementAnchor;
    fn anchor_mut(&mut self) -> &mut UIElementAnchor;
    fn position(&self) -> &UIElementOffset;
    fn position_mut(&mut self) -> &mut UIElementOffset;
    fn expansion(&self) -> &UIElementRectExpansion;
    fn expansion_mut(&mut self) -> &mut UIElementRectExpansion;
    fn visible(&self) -> &bool;
    fn visible_mut(&mut self) -> &mut bool;
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
    fn alpha(&self) -> &f32;
    fn alpha_mut(&mut self) -> &mut f32;
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl UIElementEntityDataTrait for UIElementEntityData {
    fn instance_name(&self) -> &String {
        &self.instance_name
    }
    fn instance_name_mut(&mut self) -> &mut String {
        &mut self.instance_name
    }
    fn instance_name_hash(&self) -> &u32 {
        &self.instance_name_hash
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.instance_name_hash
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        &self.transform_pivot
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.transform_pivot
    }
    fn size(&self) -> &super::core::Vec2 {
        &self.size
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.size
    }
    fn layout_mode(&self) -> &UILayoutMode {
        &self.layout_mode
    }
    fn layout_mode_mut(&mut self) -> &mut UILayoutMode {
        &mut self.layout_mode
    }
    fn offset(&self) -> &UIElementOffset {
        &self.offset
    }
    fn offset_mut(&mut self) -> &mut UIElementOffset {
        &mut self.offset
    }
    fn anchor(&self) -> &UIElementAnchor {
        &self.anchor
    }
    fn anchor_mut(&mut self) -> &mut UIElementAnchor {
        &mut self.anchor
    }
    fn position(&self) -> &UIElementOffset {
        &self.position
    }
    fn position_mut(&mut self) -> &mut UIElementOffset {
        &mut self.position
    }
    fn expansion(&self) -> &UIElementRectExpansion {
        &self.expansion
    }
    fn expansion_mut(&mut self) -> &mut UIElementRectExpansion {
        &mut self.expansion
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        &mut self.alpha
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIElementEntityData {
}

impl super::core::DataContainerTrait for UIElementEntityData {
}

pub static UIELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementEntityData",
    name_hash: 2880311948,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(UIElementEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementEntityData as Default>::default())),
            create_boxed: || Box::new(<UIElementEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InstanceName",
                name_hash: 1186954283,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIElementEntityData, instance_name),
            },
            FieldInfoData {
                name: "InstanceNameHash",
                name_hash: 1430708601,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UIElementEntityData, instance_name_hash),
            },
            FieldInfoData {
                name: "TransformPivot",
                name_hash: 3594781853,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIElementEntityData, transform_pivot),
            },
            FieldInfoData {
                name: "Size",
                name_hash: 2089429248,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UIElementEntityData, size),
            },
            FieldInfoData {
                name: "LayoutMode",
                name_hash: 1932103164,
                flags: MemberInfoFlags::new(0),
                field_type: "UILayoutMode",
                rust_offset: offset_of!(UIElementEntityData, layout_mode),
            },
            FieldInfoData {
                name: "Offset",
                name_hash: 2871410728,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementOffset",
                rust_offset: offset_of!(UIElementEntityData, offset),
            },
            FieldInfoData {
                name: "Anchor",
                name_hash: 2489622940,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementAnchor",
                rust_offset: offset_of!(UIElementEntityData, anchor),
            },
            FieldInfoData {
                name: "Position",
                name_hash: 3402582524,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementOffset",
                rust_offset: offset_of!(UIElementEntityData, position),
            },
            FieldInfoData {
                name: "Expansion",
                name_hash: 2333524892,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementRectExpansion",
                rust_offset: offset_of!(UIElementEntityData, expansion),
            },
            FieldInfoData {
                name: "Visible",
                name_hash: 901540267,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementEntityData, visible),
            },
            FieldInfoData {
                name: "Color",
                name_hash: 212387320,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIElementEntityData, color),
            },
            FieldInfoData {
                name: "Alpha",
                name_hash: 205677681,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementEntityData, alpha),
            },
            FieldInfoData {
                name: "Transform",
                name_hash: 2270319721,
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


pub static UIELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementEntityData-Array",
    name_hash: 334471864,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIWidgetEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub size: UIElementSize,
    pub layers: Vec<Option<LockedTypeObject /* UIElementLayerEntityData */>>,
    pub texture_mappings: Vec<Option<LockedTypeObject /* UITextureMappingAsset */>>,
    pub components: Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>,
    pub visible: bool,
}

pub trait UIWidgetEntityDataTrait: super::entity::EntityDataTrait {
    fn size(&self) -> &UIElementSize;
    fn size_mut(&mut self) -> &mut UIElementSize;
    fn layers(&self) -> &Vec<Option<LockedTypeObject /* UIElementLayerEntityData */>>;
    fn layers_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* UIElementLayerEntityData */>>;
    fn texture_mappings(&self) -> &Vec<Option<LockedTypeObject /* UITextureMappingAsset */>>;
    fn texture_mappings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* UITextureMappingAsset */>>;
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn visible(&self) -> &bool;
    fn visible_mut(&mut self) -> &mut bool;
}

impl UIWidgetEntityDataTrait for UIWidgetEntityData {
    fn size(&self) -> &UIElementSize {
        &self.size
    }
    fn size_mut(&mut self) -> &mut UIElementSize {
        &mut self.size
    }
    fn layers(&self) -> &Vec<Option<LockedTypeObject /* UIElementLayerEntityData */>> {
        &self.layers
    }
    fn layers_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* UIElementLayerEntityData */>> {
        &mut self.layers
    }
    fn texture_mappings(&self) -> &Vec<Option<LockedTypeObject /* UITextureMappingAsset */>> {
        &self.texture_mappings
    }
    fn texture_mappings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* UITextureMappingAsset */>> {
        &mut self.texture_mappings
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &self.components
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &mut self.components
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIWidgetEntityData {
}

impl super::core::DataContainerTrait for UIWidgetEntityData {
}

pub static UIWIDGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetEntityData",
    name_hash: 2972319166,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(UIWidgetEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWidgetEntityData as Default>::default())),
            create_boxed: || Box::new(<UIWidgetEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Size",
                name_hash: 2089429248,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementSize",
                rust_offset: offset_of!(UIWidgetEntityData, size),
            },
            FieldInfoData {
                name: "Layers",
                name_hash: 2902835477,
                flags: MemberInfoFlags::new(144),
                field_type: "UIElementLayerEntityData-Array",
                rust_offset: offset_of!(UIWidgetEntityData, layers),
            },
            FieldInfoData {
                name: "TextureMappings",
                name_hash: 367478405,
                flags: MemberInfoFlags::new(144),
                field_type: "UITextureMappingAsset-Array",
                rust_offset: offset_of!(UIWidgetEntityData, texture_mappings),
            },
            FieldInfoData {
                name: "Components",
                name_hash: 3391050425,
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(UIWidgetEntityData, components),
            },
            FieldInfoData {
                name: "Visible",
                name_hash: 901540267,
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


pub static UIWIDGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetEntityData-Array",
    name_hash: 1004170762,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIWidgetEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementLayerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub elements: Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>,
    pub inclusion_settings: UIElementInclusionSettings,
    pub visible: bool,
    pub internal_layer_name: String,
}

pub trait UIElementLayerEntityDataTrait: super::entity::EntityDataTrait {
    fn elements(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn elements_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn inclusion_settings(&self) -> &UIElementInclusionSettings;
    fn inclusion_settings_mut(&mut self) -> &mut UIElementInclusionSettings;
    fn visible(&self) -> &bool;
    fn visible_mut(&mut self) -> &mut bool;
    fn internal_layer_name(&self) -> &String;
    fn internal_layer_name_mut(&mut self) -> &mut String;
}

impl UIElementLayerEntityDataTrait for UIElementLayerEntityData {
    fn elements(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &self.elements
    }
    fn elements_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &mut self.elements
    }
    fn inclusion_settings(&self) -> &UIElementInclusionSettings {
        &self.inclusion_settings
    }
    fn inclusion_settings_mut(&mut self) -> &mut UIElementInclusionSettings {
        &mut self.inclusion_settings
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
    fn internal_layer_name(&self) -> &String {
        &self.internal_layer_name
    }
    fn internal_layer_name_mut(&mut self) -> &mut String {
        &mut self.internal_layer_name
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIElementLayerEntityData {
}

impl super::core::DataContainerTrait for UIElementLayerEntityData {
}

pub static UIELEMENTLAYERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementLayerEntityData",
    name_hash: 834086319,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(UIElementLayerEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementLayerEntityData as Default>::default())),
            create_boxed: || Box::new(<UIElementLayerEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Elements",
                name_hash: 2347524808,
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(UIElementLayerEntityData, elements),
            },
            FieldInfoData {
                name: "InclusionSettings",
                name_hash: 1787638950,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementInclusionSettings",
                rust_offset: offset_of!(UIElementLayerEntityData, inclusion_settings),
            },
            FieldInfoData {
                name: "Visible",
                name_hash: 901540267,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementLayerEntityData, visible),
            },
            FieldInfoData {
                name: "InternalLayerName",
                name_hash: 409549158,
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


pub static UIELEMENTLAYERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementLayerEntityData-Array",
    name_hash: 1357517595,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementLayerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementInclusionSettings {
    pub is_singleplayer_layer: bool,
    pub is_multiplayer_layer: bool,
    pub custom_inclusion_critera: Vec<String>,
    pub is_s_d_layer: bool,
    pub is_h_d_layer: bool,
}

pub trait UIElementInclusionSettingsTrait: TypeObject {
    fn is_singleplayer_layer(&self) -> &bool;
    fn is_singleplayer_layer_mut(&mut self) -> &mut bool;
    fn is_multiplayer_layer(&self) -> &bool;
    fn is_multiplayer_layer_mut(&mut self) -> &mut bool;
    fn custom_inclusion_critera(&self) -> &Vec<String>;
    fn custom_inclusion_critera_mut(&mut self) -> &mut Vec<String>;
    fn is_s_d_layer(&self) -> &bool;
    fn is_s_d_layer_mut(&mut self) -> &mut bool;
    fn is_h_d_layer(&self) -> &bool;
    fn is_h_d_layer_mut(&mut self) -> &mut bool;
}

impl UIElementInclusionSettingsTrait for UIElementInclusionSettings {
    fn is_singleplayer_layer(&self) -> &bool {
        &self.is_singleplayer_layer
    }
    fn is_singleplayer_layer_mut(&mut self) -> &mut bool {
        &mut self.is_singleplayer_layer
    }
    fn is_multiplayer_layer(&self) -> &bool {
        &self.is_multiplayer_layer
    }
    fn is_multiplayer_layer_mut(&mut self) -> &mut bool {
        &mut self.is_multiplayer_layer
    }
    fn custom_inclusion_critera(&self) -> &Vec<String> {
        &self.custom_inclusion_critera
    }
    fn custom_inclusion_critera_mut(&mut self) -> &mut Vec<String> {
        &mut self.custom_inclusion_critera
    }
    fn is_s_d_layer(&self) -> &bool {
        &self.is_s_d_layer
    }
    fn is_s_d_layer_mut(&mut self) -> &mut bool {
        &mut self.is_s_d_layer
    }
    fn is_h_d_layer(&self) -> &bool {
        &self.is_h_d_layer
    }
    fn is_h_d_layer_mut(&mut self) -> &mut bool {
        &mut self.is_h_d_layer
    }
}

pub static UIELEMENTINCLUSIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementInclusionSettings",
    name_hash: 2067889284,
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementInclusionSettings as Default>::default())),
            create_boxed: || Box::new(<UIElementInclusionSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsSingleplayerLayer",
                name_hash: 546430741,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementInclusionSettings, is_singleplayer_layer),
            },
            FieldInfoData {
                name: "IsMultiplayerLayer",
                name_hash: 470702054,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementInclusionSettings, is_multiplayer_layer),
            },
            FieldInfoData {
                name: "CustomInclusionCritera",
                name_hash: 1278590090,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(UIElementInclusionSettings, custom_inclusion_critera),
            },
            FieldInfoData {
                name: "IsSDLayer",
                name_hash: 3270056299,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementInclusionSettings, is_s_d_layer),
            },
            FieldInfoData {
                name: "IsHDLayer",
                name_hash: 464678960,
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


pub static UIELEMENTINCLUSIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementInclusionSettings-Array",
    name_hash: 1617637552,
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
    name_hash: 255742683,
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


pub static UIBLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIBlendMode-Array",
    name_hash: 2890217967,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIBlendMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementGradient {
    pub top_left_color: UIElementColor,
    pub top_right_color: UIElementColor,
    pub bottom_left_color: UIElementColor,
    pub bottom_right_color: UIElementColor,
}

pub trait UIElementGradientTrait: TypeObject {
    fn top_left_color(&self) -> &UIElementColor;
    fn top_left_color_mut(&mut self) -> &mut UIElementColor;
    fn top_right_color(&self) -> &UIElementColor;
    fn top_right_color_mut(&mut self) -> &mut UIElementColor;
    fn bottom_left_color(&self) -> &UIElementColor;
    fn bottom_left_color_mut(&mut self) -> &mut UIElementColor;
    fn bottom_right_color(&self) -> &UIElementColor;
    fn bottom_right_color_mut(&mut self) -> &mut UIElementColor;
}

impl UIElementGradientTrait for UIElementGradient {
    fn top_left_color(&self) -> &UIElementColor {
        &self.top_left_color
    }
    fn top_left_color_mut(&mut self) -> &mut UIElementColor {
        &mut self.top_left_color
    }
    fn top_right_color(&self) -> &UIElementColor {
        &self.top_right_color
    }
    fn top_right_color_mut(&mut self) -> &mut UIElementColor {
        &mut self.top_right_color
    }
    fn bottom_left_color(&self) -> &UIElementColor {
        &self.bottom_left_color
    }
    fn bottom_left_color_mut(&mut self) -> &mut UIElementColor {
        &mut self.bottom_left_color
    }
    fn bottom_right_color(&self) -> &UIElementColor {
        &self.bottom_right_color
    }
    fn bottom_right_color_mut(&mut self) -> &mut UIElementColor {
        &mut self.bottom_right_color
    }
}

pub static UIELEMENTGRADIENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementGradient",
    name_hash: 1884052129,
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementGradient as Default>::default())),
            create_boxed: || Box::new(<UIElementGradient as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TopLeftColor",
                name_hash: 1000158472,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementGradient, top_left_color),
            },
            FieldInfoData {
                name: "TopRightColor",
                name_hash: 2022051251,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementGradient, top_right_color),
            },
            FieldInfoData {
                name: "BottomLeftColor",
                name_hash: 1324188044,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementGradient, bottom_left_color),
            },
            FieldInfoData {
                name: "BottomRightColor",
                name_hash: 3457108407,
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


pub static UIELEMENTGRADIENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementGradient-Array",
    name_hash: 1004242197,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementGradient"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementAnchor {
    pub x: f32,
    pub y: f32,
}

pub trait UIElementAnchorTrait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
}

impl UIElementAnchorTrait for UIElementAnchor {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
}

pub static UIELEMENTANCHOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementAnchor",
    name_hash: 2625455550,
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementAnchor as Default>::default())),
            create_boxed: || Box::new(<UIElementAnchor as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementAnchor, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
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


pub static UIELEMENTANCHOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementAnchor-Array",
    name_hash: 2055452170,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementAnchor"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementSize {
    pub x: f32,
    pub y: f32,
}

pub trait UIElementSizeTrait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
}

impl UIElementSizeTrait for UIElementSize {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
}

pub static UIELEMENTSIZE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementSize",
    name_hash: 539440162,
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementSize as Default>::default())),
            create_boxed: || Box::new(<UIElementSize as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementSize, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
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


pub static UIELEMENTSIZE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementSize-Array",
    name_hash: 1194635926,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementSize"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementOffset {
    pub x: f32,
    pub y: f32,
}

pub trait UIElementOffsetTrait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
}

impl UIElementOffsetTrait for UIElementOffset {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
}

pub static UIELEMENTOFFSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementOffset",
    name_hash: 2859906122,
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementOffset as Default>::default())),
            create_boxed: || Box::new(<UIElementOffset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementOffset, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
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


pub static UIELEMENTOFFSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementOffset-Array",
    name_hash: 2790744062,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementOffset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementRectExpansion {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub trait UIElementRectExpansionTrait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn width(&self) -> &f32;
    fn width_mut(&mut self) -> &mut f32;
    fn height(&self) -> &f32;
    fn height_mut(&mut self) -> &mut f32;
}

impl UIElementRectExpansionTrait for UIElementRectExpansion {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn width(&self) -> &f32 {
        &self.width
    }
    fn width_mut(&mut self) -> &mut f32 {
        &mut self.width
    }
    fn height(&self) -> &f32 {
        &self.height
    }
    fn height_mut(&mut self) -> &mut f32 {
        &mut self.height
    }
}

pub static UIELEMENTRECTEXPANSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementRectExpansion",
    name_hash: 3658146878,
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementRectExpansion as Default>::default())),
            create_boxed: || Box::new(<UIElementRectExpansion as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementRectExpansion, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementRectExpansion, y),
            },
            FieldInfoData {
                name: "Width",
                name_hash: 226981187,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementRectExpansion, width),
            },
            FieldInfoData {
                name: "Height",
                name_hash: 3054065626,
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


pub static UIELEMENTRECTEXPANSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementRectExpansion-Array",
    name_hash: 3698209930,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementRectExpansion"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementFontStyle {
    pub _glacier_base: super::game_base::UIElementFontStyleBaseAsset,
    pub hd: Option<LockedTypeObject /* UIElementFontDefinition */>,
    pub sd: Option<LockedTypeObject /* UIElementFontDefinition */>,
    pub color: UIElementColor,
    pub language_overrides: Vec<BoxedTypeObject /* UIElementFontDefinitionOverride */>,
}

pub trait UIElementFontStyleTrait: super::game_base::UIElementFontStyleBaseAssetTrait {
    fn hd(&self) -> &Option<LockedTypeObject /* UIElementFontDefinition */>;
    fn hd_mut(&mut self) -> &mut Option<LockedTypeObject /* UIElementFontDefinition */>;
    fn sd(&self) -> &Option<LockedTypeObject /* UIElementFontDefinition */>;
    fn sd_mut(&mut self) -> &mut Option<LockedTypeObject /* UIElementFontDefinition */>;
    fn color(&self) -> &UIElementColor;
    fn color_mut(&mut self) -> &mut UIElementColor;
    fn language_overrides(&self) -> &Vec<BoxedTypeObject /* UIElementFontDefinitionOverride */>;
    fn language_overrides_mut(&mut self) -> &mut Vec<BoxedTypeObject /* UIElementFontDefinitionOverride */>;
}

impl UIElementFontStyleTrait for UIElementFontStyle {
    fn hd(&self) -> &Option<LockedTypeObject /* UIElementFontDefinition */> {
        &self.hd
    }
    fn hd_mut(&mut self) -> &mut Option<LockedTypeObject /* UIElementFontDefinition */> {
        &mut self.hd
    }
    fn sd(&self) -> &Option<LockedTypeObject /* UIElementFontDefinition */> {
        &self.sd
    }
    fn sd_mut(&mut self) -> &mut Option<LockedTypeObject /* UIElementFontDefinition */> {
        &mut self.sd
    }
    fn color(&self) -> &UIElementColor {
        &self.color
    }
    fn color_mut(&mut self) -> &mut UIElementColor {
        &mut self.color
    }
    fn language_overrides(&self) -> &Vec<BoxedTypeObject /* UIElementFontDefinitionOverride */> {
        &self.language_overrides
    }
    fn language_overrides_mut(&mut self) -> &mut Vec<BoxedTypeObject /* UIElementFontDefinitionOverride */> {
        &mut self.language_overrides
    }
}

impl super::game_base::UIElementFontStyleBaseAssetTrait for UIElementFontStyle {
}

impl super::core::AssetTrait for UIElementFontStyle {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for UIElementFontStyle {
}

pub static UIELEMENTFONTSTYLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontStyle",
    name_hash: 4160288579,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_base::UIELEMENTFONTSTYLEBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(UIElementFontStyle, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementFontStyle as Default>::default())),
            create_boxed: || Box::new(<UIElementFontStyle as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Hd",
                name_hash: 5862377,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontDefinition",
                rust_offset: offset_of!(UIElementFontStyle, hd),
            },
            FieldInfoData {
                name: "Sd",
                name_hash: 5862610,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontDefinition",
                rust_offset: offset_of!(UIElementFontStyle, sd),
            },
            FieldInfoData {
                name: "Color",
                name_hash: 212387320,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementFontStyle, color),
            },
            FieldInfoData {
                name: "LanguageOverrides",
                name_hash: 2228980432,
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


pub static UIELEMENTFONTSTYLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontStyle-Array",
    name_hash: 1920408183,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontStyle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementFontEffect {
    pub _glacier_base: super::game_base::UIElementFontEffectBaseAsset,
    pub effect_script: String,
}

pub trait UIElementFontEffectTrait: super::game_base::UIElementFontEffectBaseAssetTrait {
    fn effect_script(&self) -> &String;
    fn effect_script_mut(&mut self) -> &mut String;
}

impl UIElementFontEffectTrait for UIElementFontEffect {
    fn effect_script(&self) -> &String {
        &self.effect_script
    }
    fn effect_script_mut(&mut self) -> &mut String {
        &mut self.effect_script
    }
}

impl super::game_base::UIElementFontEffectBaseAssetTrait for UIElementFontEffect {
}

impl super::core::AssetTrait for UIElementFontEffect {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for UIElementFontEffect {
}

pub static UIELEMENTFONTEFFECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontEffect",
    name_hash: 78454115,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_base::UIELEMENTFONTEFFECTBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(UIElementFontEffect, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementFontEffect as Default>::default())),
            create_boxed: || Box::new(<UIElementFontEffect as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EffectScript",
                name_hash: 2336463997,
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


pub static UIELEMENTFONTEFFECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontEffect-Array",
    name_hash: 2565853527,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontEffect"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementFontDefinition {
    pub _glacier_base: super::core::DataContainer,
    pub font_lookup: Vec<BoxedTypeObject /* UIImmediateModeFontLookup */>,
    pub point_size: f32,
    pub letter_spacing: f32,
    pub row_spacing: i32,
}

pub trait UIElementFontDefinitionTrait: super::core::DataContainerTrait {
    fn font_lookup(&self) -> &Vec<BoxedTypeObject /* UIImmediateModeFontLookup */>;
    fn font_lookup_mut(&mut self) -> &mut Vec<BoxedTypeObject /* UIImmediateModeFontLookup */>;
    fn point_size(&self) -> &f32;
    fn point_size_mut(&mut self) -> &mut f32;
    fn letter_spacing(&self) -> &f32;
    fn letter_spacing_mut(&mut self) -> &mut f32;
    fn row_spacing(&self) -> &i32;
    fn row_spacing_mut(&mut self) -> &mut i32;
}

impl UIElementFontDefinitionTrait for UIElementFontDefinition {
    fn font_lookup(&self) -> &Vec<BoxedTypeObject /* UIImmediateModeFontLookup */> {
        &self.font_lookup
    }
    fn font_lookup_mut(&mut self) -> &mut Vec<BoxedTypeObject /* UIImmediateModeFontLookup */> {
        &mut self.font_lookup
    }
    fn point_size(&self) -> &f32 {
        &self.point_size
    }
    fn point_size_mut(&mut self) -> &mut f32 {
        &mut self.point_size
    }
    fn letter_spacing(&self) -> &f32 {
        &self.letter_spacing
    }
    fn letter_spacing_mut(&mut self) -> &mut f32 {
        &mut self.letter_spacing
    }
    fn row_spacing(&self) -> &i32 {
        &self.row_spacing
    }
    fn row_spacing_mut(&mut self) -> &mut i32 {
        &mut self.row_spacing
    }
}

impl super::core::DataContainerTrait for UIElementFontDefinition {
}

pub static UIELEMENTFONTDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinition",
    name_hash: 4205582529,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(UIElementFontDefinition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementFontDefinition as Default>::default())),
            create_boxed: || Box::new(<UIElementFontDefinition as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FontLookup",
                name_hash: 2289006292,
                flags: MemberInfoFlags::new(144),
                field_type: "UIImmediateModeFontLookup-Array",
                rust_offset: offset_of!(UIElementFontDefinition, font_lookup),
            },
            FieldInfoData {
                name: "PointSize",
                name_hash: 1834692108,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementFontDefinition, point_size),
            },
            FieldInfoData {
                name: "LetterSpacing",
                name_hash: 4002407066,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementFontDefinition, letter_spacing),
            },
            FieldInfoData {
                name: "RowSpacing",
                name_hash: 3371971726,
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


pub static UIELEMENTFONTDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinition-Array",
    name_hash: 2494636789,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontDefinition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementFontDefinitionOverride {
    pub language: super::core::LanguageFormat,
    pub hd: Option<LockedTypeObject /* UIElementFontDefinition */>,
    pub sd: Option<LockedTypeObject /* UIElementFontDefinition */>,
}

pub trait UIElementFontDefinitionOverrideTrait: TypeObject {
    fn language(&self) -> &super::core::LanguageFormat;
    fn language_mut(&mut self) -> &mut super::core::LanguageFormat;
    fn hd(&self) -> &Option<LockedTypeObject /* UIElementFontDefinition */>;
    fn hd_mut(&mut self) -> &mut Option<LockedTypeObject /* UIElementFontDefinition */>;
    fn sd(&self) -> &Option<LockedTypeObject /* UIElementFontDefinition */>;
    fn sd_mut(&mut self) -> &mut Option<LockedTypeObject /* UIElementFontDefinition */>;
}

impl UIElementFontDefinitionOverrideTrait for UIElementFontDefinitionOverride {
    fn language(&self) -> &super::core::LanguageFormat {
        &self.language
    }
    fn language_mut(&mut self) -> &mut super::core::LanguageFormat {
        &mut self.language
    }
    fn hd(&self) -> &Option<LockedTypeObject /* UIElementFontDefinition */> {
        &self.hd
    }
    fn hd_mut(&mut self) -> &mut Option<LockedTypeObject /* UIElementFontDefinition */> {
        &mut self.hd
    }
    fn sd(&self) -> &Option<LockedTypeObject /* UIElementFontDefinition */> {
        &self.sd
    }
    fn sd_mut(&mut self) -> &mut Option<LockedTypeObject /* UIElementFontDefinition */> {
        &mut self.sd
    }
}

pub static UIELEMENTFONTDEFINITIONOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinitionOverride",
    name_hash: 1732366133,
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementFontDefinitionOverride as Default>::default())),
            create_boxed: || Box::new(<UIElementFontDefinitionOverride as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Language",
                name_hash: 3872303031,
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(UIElementFontDefinitionOverride, language),
            },
            FieldInfoData {
                name: "Hd",
                name_hash: 5862377,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontDefinition",
                rust_offset: offset_of!(UIElementFontDefinitionOverride, hd),
            },
            FieldInfoData {
                name: "Sd",
                name_hash: 5862610,
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


pub static UIELEMENTFONTDEFINITIONOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementFontDefinitionOverride-Array",
    name_hash: 2819149697,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementFontDefinitionOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn knock_out_mut(&mut self) -> &mut bool;
    fn hide_object(&self) -> &bool;
    fn hide_object_mut(&mut self) -> &mut bool;
    fn fine_blur(&self) -> &bool;
    fn fine_blur_mut(&mut self) -> &mut bool;
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn strength(&self) -> &f32;
    fn strength_mut(&mut self) -> &mut f32;
    fn color(&self) -> &UIElementColor;
    fn color_mut(&mut self) -> &mut UIElementColor;
}

impl UIElementTextFilterGlowTrait for UIElementTextFilterGlow {
    fn knock_out(&self) -> &bool {
        &self.knock_out
    }
    fn knock_out_mut(&mut self) -> &mut bool {
        &mut self.knock_out
    }
    fn hide_object(&self) -> &bool {
        &self.hide_object
    }
    fn hide_object_mut(&mut self) -> &mut bool {
        &mut self.hide_object
    }
    fn fine_blur(&self) -> &bool {
        &self.fine_blur
    }
    fn fine_blur_mut(&mut self) -> &mut bool {
        &mut self.fine_blur
    }
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn strength_mut(&mut self) -> &mut f32 {
        &mut self.strength
    }
    fn color(&self) -> &UIElementColor {
        &self.color
    }
    fn color_mut(&mut self) -> &mut UIElementColor {
        &mut self.color
    }
}

impl UIElementTextFilterTrait for UIElementTextFilterGlow {
}

impl super::core::DataContainerTrait for UIElementTextFilterGlow {
}

pub static UIELEMENTTEXTFILTERGLOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterGlow",
    name_hash: 1074389673,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTTEXTFILTER_TYPE_INFO),
        super_class_offset: offset_of!(UIElementTextFilterGlow, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementTextFilterGlow as Default>::default())),
            create_boxed: || Box::new(<UIElementTextFilterGlow as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "KnockOut",
                name_hash: 583274505,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterGlow, knock_out),
            },
            FieldInfoData {
                name: "HideObject",
                name_hash: 2874443024,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterGlow, hide_object),
            },
            FieldInfoData {
                name: "FineBlur",
                name_hash: 1207865960,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterGlow, fine_blur),
            },
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterGlow, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterGlow, y),
            },
            FieldInfoData {
                name: "Strength",
                name_hash: 3531643328,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterGlow, strength),
            },
            FieldInfoData {
                name: "Color",
                name_hash: 212387320,
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


pub static UIELEMENTTEXTFILTERGLOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterGlow-Array",
    name_hash: 1478387997,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilterGlow"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn knock_out_mut(&mut self) -> &mut bool;
    fn hide_object(&self) -> &bool;
    fn hide_object_mut(&mut self) -> &mut bool;
    fn fine_blur(&self) -> &bool;
    fn fine_blur_mut(&mut self) -> &mut bool;
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn strength(&self) -> &f32;
    fn strength_mut(&mut self) -> &mut f32;
    fn color(&self) -> &UIElementColor;
    fn color_mut(&mut self) -> &mut UIElementColor;
    fn angle(&self) -> &f32;
    fn angle_mut(&mut self) -> &mut f32;
    fn distance(&self) -> &f32;
    fn distance_mut(&mut self) -> &mut f32;
}

impl UIElementTextFilterDropShadowTrait for UIElementTextFilterDropShadow {
    fn knock_out(&self) -> &bool {
        &self.knock_out
    }
    fn knock_out_mut(&mut self) -> &mut bool {
        &mut self.knock_out
    }
    fn hide_object(&self) -> &bool {
        &self.hide_object
    }
    fn hide_object_mut(&mut self) -> &mut bool {
        &mut self.hide_object
    }
    fn fine_blur(&self) -> &bool {
        &self.fine_blur
    }
    fn fine_blur_mut(&mut self) -> &mut bool {
        &mut self.fine_blur
    }
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn strength_mut(&mut self) -> &mut f32 {
        &mut self.strength
    }
    fn color(&self) -> &UIElementColor {
        &self.color
    }
    fn color_mut(&mut self) -> &mut UIElementColor {
        &mut self.color
    }
    fn angle(&self) -> &f32 {
        &self.angle
    }
    fn angle_mut(&mut self) -> &mut f32 {
        &mut self.angle
    }
    fn distance(&self) -> &f32 {
        &self.distance
    }
    fn distance_mut(&mut self) -> &mut f32 {
        &mut self.distance
    }
}

impl UIElementTextFilterTrait for UIElementTextFilterDropShadow {
}

impl super::core::DataContainerTrait for UIElementTextFilterDropShadow {
}

pub static UIELEMENTTEXTFILTERDROPSHADOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterDropShadow",
    name_hash: 3700679381,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTTEXTFILTER_TYPE_INFO),
        super_class_offset: offset_of!(UIElementTextFilterDropShadow, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementTextFilterDropShadow as Default>::default())),
            create_boxed: || Box::new(<UIElementTextFilterDropShadow as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "KnockOut",
                name_hash: 583274505,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, knock_out),
            },
            FieldInfoData {
                name: "HideObject",
                name_hash: 2874443024,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, hide_object),
            },
            FieldInfoData {
                name: "FineBlur",
                name_hash: 1207865960,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, fine_blur),
            },
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, y),
            },
            FieldInfoData {
                name: "Strength",
                name_hash: 3531643328,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, strength),
            },
            FieldInfoData {
                name: "Color",
                name_hash: 212387320,
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementColor",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, color),
            },
            FieldInfoData {
                name: "Angle",
                name_hash: 205597860,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterDropShadow, angle),
            },
            FieldInfoData {
                name: "Distance",
                name_hash: 408560070,
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


pub static UIELEMENTTEXTFILTERDROPSHADOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterDropShadow-Array",
    name_hash: 1822089953,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilterDropShadow"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementTextFilterBlur {
    pub _glacier_base: UIElementTextFilter,
    pub x: f32,
    pub y: f32,
    pub strength: f32,
}

pub trait UIElementTextFilterBlurTrait: UIElementTextFilterTrait {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn strength(&self) -> &f32;
    fn strength_mut(&mut self) -> &mut f32;
}

impl UIElementTextFilterBlurTrait for UIElementTextFilterBlur {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn strength_mut(&mut self) -> &mut f32 {
        &mut self.strength
    }
}

impl UIElementTextFilterTrait for UIElementTextFilterBlur {
}

impl super::core::DataContainerTrait for UIElementTextFilterBlur {
}

pub static UIELEMENTTEXTFILTERBLUR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterBlur",
    name_hash: 1074287443,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTTEXTFILTER_TYPE_INFO),
        super_class_offset: offset_of!(UIElementTextFilterBlur, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementTextFilterBlur as Default>::default())),
            create_boxed: || Box::new(<UIElementTextFilterBlur as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterBlur, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIElementTextFilterBlur, y),
            },
            FieldInfoData {
                name: "Strength",
                name_hash: 3531643328,
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


pub static UIELEMENTTEXTFILTERBLUR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilterBlur-Array",
    name_hash: 120471143,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilterBlur"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementTextFilter {
    pub _glacier_base: super::core::DataContainer,
}

pub trait UIElementTextFilterTrait: super::core::DataContainerTrait {
}

impl UIElementTextFilterTrait for UIElementTextFilter {
}

impl super::core::DataContainerTrait for UIElementTextFilter {
}

pub static UIELEMENTTEXTFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilter",
    name_hash: 1901153946,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(UIElementTextFilter, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementTextFilter as Default>::default())),
            create_boxed: || Box::new(<UIElementTextFilter as Default>::default()),
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


pub static UIELEMENTTEXTFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementTextFilter-Array",
    name_hash: 906965422,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementTextFilter"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIElementColor {
    pub rgb: super::core::Vec3,
    pub alpha: f32,
}

pub trait UIElementColorTrait: TypeObject {
    fn rgb(&self) -> &super::core::Vec3;
    fn rgb_mut(&mut self) -> &mut super::core::Vec3;
    fn alpha(&self) -> &f32;
    fn alpha_mut(&mut self) -> &mut f32;
}

impl UIElementColorTrait for UIElementColor {
    fn rgb(&self) -> &super::core::Vec3 {
        &self.rgb
    }
    fn rgb_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.rgb
    }
    fn alpha(&self) -> &f32 {
        &self.alpha
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        &mut self.alpha
    }
}

pub static UIELEMENTCOLOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementColor",
    name_hash: 602494106,
    flags: MemberInfoFlags::new(36937),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementColor as Default>::default())),
            create_boxed: || Box::new(<UIElementColor as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Rgb",
                name_hash: 193465042,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UIElementColor, rgb),
            },
            FieldInfoData {
                name: "Alpha",
                name_hash: 205677681,
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


pub static UIELEMENTCOLOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementColor-Array",
    name_hash: 1477687214,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIElementColor"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UITextureMappingAsset {
    pub _glacier_base: super::game_base::UITextureMappingBaseAsset,
    pub compartment: UITextureMappingCompartment,
    pub output: Vec<BoxedTypeObject /* UITextureMappingOutputEntry */>,
    pub disable_atlas: bool,
    pub force_atlas: bool,
}

pub trait UITextureMappingAssetTrait: super::game_base::UITextureMappingBaseAssetTrait {
    fn compartment(&self) -> &UITextureMappingCompartment;
    fn compartment_mut(&mut self) -> &mut UITextureMappingCompartment;
    fn output(&self) -> &Vec<BoxedTypeObject /* UITextureMappingOutputEntry */>;
    fn output_mut(&mut self) -> &mut Vec<BoxedTypeObject /* UITextureMappingOutputEntry */>;
    fn disable_atlas(&self) -> &bool;
    fn disable_atlas_mut(&mut self) -> &mut bool;
    fn force_atlas(&self) -> &bool;
    fn force_atlas_mut(&mut self) -> &mut bool;
}

impl UITextureMappingAssetTrait for UITextureMappingAsset {
    fn compartment(&self) -> &UITextureMappingCompartment {
        &self.compartment
    }
    fn compartment_mut(&mut self) -> &mut UITextureMappingCompartment {
        &mut self.compartment
    }
    fn output(&self) -> &Vec<BoxedTypeObject /* UITextureMappingOutputEntry */> {
        &self.output
    }
    fn output_mut(&mut self) -> &mut Vec<BoxedTypeObject /* UITextureMappingOutputEntry */> {
        &mut self.output
    }
    fn disable_atlas(&self) -> &bool {
        &self.disable_atlas
    }
    fn disable_atlas_mut(&mut self) -> &mut bool {
        &mut self.disable_atlas
    }
    fn force_atlas(&self) -> &bool {
        &self.force_atlas
    }
    fn force_atlas_mut(&mut self) -> &mut bool {
        &mut self.force_atlas
    }
}

impl super::game_base::UITextureMappingBaseAssetTrait for UITextureMappingAsset {
}

impl super::core::AssetTrait for UITextureMappingAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for UITextureMappingAsset {
}

pub static UITEXTUREMAPPINGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingAsset",
    name_hash: 1053575386,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_base::UITEXTUREMAPPINGBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(UITextureMappingAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UITextureMappingAsset as Default>::default())),
            create_boxed: || Box::new(<UITextureMappingAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Compartment",
                name_hash: 1500500641,
                flags: MemberInfoFlags::new(0),
                field_type: "UITextureMappingCompartment",
                rust_offset: offset_of!(UITextureMappingAsset, compartment),
            },
            FieldInfoData {
                name: "Output",
                name_hash: 2895736442,
                flags: MemberInfoFlags::new(144),
                field_type: "UITextureMappingOutputEntry-Array",
                rust_offset: offset_of!(UITextureMappingAsset, output),
            },
            FieldInfoData {
                name: "DisableAtlas",
                name_hash: 3560262138,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UITextureMappingAsset, disable_atlas),
            },
            FieldInfoData {
                name: "ForceAtlas",
                name_hash: 1048545107,
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


pub static UITEXTUREMAPPINGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingAsset-Array",
    name_hash: 1581063278,
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
    name_hash: 1844629888,
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


pub static UILAYOUTMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UILayoutMode-Array",
    name_hash: 3368589236,
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
    name_hash: 3425655310,
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


pub static UITEXTUREMAPPINGCOMPARTMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingCompartment-Array",
    name_hash: 1322566458,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITextureMappingCompartment"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UITextureMappingOutputEntry {
    pub id: i32,
    pub texture_ref: glacier_reflect::builtin::ResourceRef,
    pub uv_rect: super::core::Vec4,
}

pub trait UITextureMappingOutputEntryTrait: TypeObject {
    fn id(&self) -> &i32;
    fn id_mut(&mut self) -> &mut i32;
    fn texture_ref(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn texture_ref_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
    fn uv_rect(&self) -> &super::core::Vec4;
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4;
}

impl UITextureMappingOutputEntryTrait for UITextureMappingOutputEntry {
    fn id(&self) -> &i32 {
        &self.id
    }
    fn id_mut(&mut self) -> &mut i32 {
        &mut self.id
    }
    fn texture_ref(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.texture_ref
    }
    fn texture_ref_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.texture_ref
    }
    fn uv_rect(&self) -> &super::core::Vec4 {
        &self.uv_rect
    }
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.uv_rect
    }
}

pub static UITEXTUREMAPPINGOUTPUTENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingOutputEntry",
    name_hash: 1746222913,
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UITextureMappingOutputEntry as Default>::default())),
            create_boxed: || Box::new(<UITextureMappingOutputEntry as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                name_hash: 5862152,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UITextureMappingOutputEntry, id),
            },
            FieldInfoData {
                name: "TextureRef",
                name_hash: 4257491627,
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(UITextureMappingOutputEntry, texture_ref),
            },
            FieldInfoData {
                name: "UvRect",
                name_hash: 2939810758,
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


pub static UITEXTUREMAPPINGOUTPUTENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextureMappingOutputEntry-Array",
    name_hash: 1325214069,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITextureMappingOutputEntry"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIWidgetBlueprint {
    pub _glacier_base: super::entity::ObjectBlueprint,
}

pub trait UIWidgetBlueprintTrait: super::entity::ObjectBlueprintTrait {
}

impl UIWidgetBlueprintTrait for UIWidgetBlueprint {
}

impl super::entity::ObjectBlueprintTrait for UIWidgetBlueprint {
    fn object(&self) -> &Option<LockedTypeObject /* super::entity::EntityData */> {
        self._glacier_base.object()
    }
    fn object_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::EntityData */> {
        self._glacier_base.object_mut()
    }
}

impl super::entity::BlueprintTrait for UIWidgetBlueprint {
    fn objects(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects()
    }
    fn objects_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects_mut()
    }
    fn schematics(&self) -> &Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics()
    }
    fn schematics_mut(&mut self) -> &mut Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics_mut()
    }
}

impl super::entity::EntityBusDataTrait for UIWidgetBlueprint {
    fn event_connections(&self) -> &Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections()
    }
    fn event_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections_mut()
    }
}

impl super::core::DataBusDataTrait for UIWidgetBlueprint {
    fn flags(&self) -> &u16 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.flags_mut()
    }
    fn property_connections(&self) -> &Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections()
    }
    fn property_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections_mut()
    }
    fn link_connections(&self) -> &Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections()
    }
    fn link_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections_mut()
    }
    fn interface(&self) -> &Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface()
    }
    fn interface_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface_mut()
    }
}

impl super::core::AssetTrait for UIWidgetBlueprint {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for UIWidgetBlueprint {
}

pub static UIWIDGETBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetBlueprint",
    name_hash: 4010790234,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBJECTBLUEPRINT_TYPE_INFO),
        super_class_offset: offset_of!(UIWidgetBlueprint, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWidgetBlueprint as Default>::default())),
            create_boxed: || Box::new(<UIWidgetBlueprint as Default>::default()),
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


pub static UIWIDGETBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetBlueprint-Array",
    name_hash: 2033320174,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIWidgetBlueprint"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIImmediateModeFontConfigurationAsset {
    pub _glacier_base: super::gameplay_sim::UIFontConfigurationAssetBase,
    pub font_bundles: Vec<BoxedTypeObject /* UIImmediateModeFontBundle */>,
    pub font_dpi: i32,
    pub glyph_cache_size: i32,
    pub glyph_cache_size_low_end: i32,
    pub glyph_cache_padding: i32,
    pub auto_hinting: bool,
}

pub trait UIImmediateModeFontConfigurationAssetTrait: super::gameplay_sim::UIFontConfigurationAssetBaseTrait {
    fn font_bundles(&self) -> &Vec<BoxedTypeObject /* UIImmediateModeFontBundle */>;
    fn font_bundles_mut(&mut self) -> &mut Vec<BoxedTypeObject /* UIImmediateModeFontBundle */>;
    fn font_dpi(&self) -> &i32;
    fn font_dpi_mut(&mut self) -> &mut i32;
    fn glyph_cache_size(&self) -> &i32;
    fn glyph_cache_size_mut(&mut self) -> &mut i32;
    fn glyph_cache_size_low_end(&self) -> &i32;
    fn glyph_cache_size_low_end_mut(&mut self) -> &mut i32;
    fn glyph_cache_padding(&self) -> &i32;
    fn glyph_cache_padding_mut(&mut self) -> &mut i32;
    fn auto_hinting(&self) -> &bool;
    fn auto_hinting_mut(&mut self) -> &mut bool;
}

impl UIImmediateModeFontConfigurationAssetTrait for UIImmediateModeFontConfigurationAsset {
    fn font_bundles(&self) -> &Vec<BoxedTypeObject /* UIImmediateModeFontBundle */> {
        &self.font_bundles
    }
    fn font_bundles_mut(&mut self) -> &mut Vec<BoxedTypeObject /* UIImmediateModeFontBundle */> {
        &mut self.font_bundles
    }
    fn font_dpi(&self) -> &i32 {
        &self.font_dpi
    }
    fn font_dpi_mut(&mut self) -> &mut i32 {
        &mut self.font_dpi
    }
    fn glyph_cache_size(&self) -> &i32 {
        &self.glyph_cache_size
    }
    fn glyph_cache_size_mut(&mut self) -> &mut i32 {
        &mut self.glyph_cache_size
    }
    fn glyph_cache_size_low_end(&self) -> &i32 {
        &self.glyph_cache_size_low_end
    }
    fn glyph_cache_size_low_end_mut(&mut self) -> &mut i32 {
        &mut self.glyph_cache_size_low_end
    }
    fn glyph_cache_padding(&self) -> &i32 {
        &self.glyph_cache_padding
    }
    fn glyph_cache_padding_mut(&mut self) -> &mut i32 {
        &mut self.glyph_cache_padding
    }
    fn auto_hinting(&self) -> &bool {
        &self.auto_hinting
    }
    fn auto_hinting_mut(&mut self) -> &mut bool {
        &mut self.auto_hinting
    }
}

impl super::gameplay_sim::UIFontConfigurationAssetBaseTrait for UIImmediateModeFontConfigurationAsset {
}

impl super::core::AssetTrait for UIImmediateModeFontConfigurationAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for UIImmediateModeFontConfigurationAsset {
}

pub static UIIMMEDIATEMODEFONTCONFIGURATIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontConfigurationAsset",
    name_hash: 1353454584,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::UIFONTCONFIGURATIONASSETBASE_TYPE_INFO),
        super_class_offset: offset_of!(UIImmediateModeFontConfigurationAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImmediateModeFontConfigurationAsset as Default>::default())),
            create_boxed: || Box::new(<UIImmediateModeFontConfigurationAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FontBundles",
                name_hash: 2266661617,
                flags: MemberInfoFlags::new(144),
                field_type: "UIImmediateModeFontBundle-Array",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, font_bundles),
            },
            FieldInfoData {
                name: "FontDpi",
                name_hash: 1954702955,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, font_dpi),
            },
            FieldInfoData {
                name: "GlyphCacheSize",
                name_hash: 4150869798,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, glyph_cache_size),
            },
            FieldInfoData {
                name: "GlyphCacheSizeLowEnd",
                name_hash: 2437032349,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, glyph_cache_size_low_end),
            },
            FieldInfoData {
                name: "GlyphCachePadding",
                name_hash: 1439257714,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIImmediateModeFontConfigurationAsset, glyph_cache_padding),
            },
            FieldInfoData {
                name: "AutoHinting",
                name_hash: 3065069617,
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


pub static UIIMMEDIATEMODEFONTCONFIGURATIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontConfigurationAsset-Array",
    name_hash: 4033351116,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIImmediateModeFontConfigurationAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIImmediateModeFontLookup {
    pub language: super::core::LanguageFormat,
    pub font_asset_path: String,
}

pub trait UIImmediateModeFontLookupTrait: TypeObject {
    fn language(&self) -> &super::core::LanguageFormat;
    fn language_mut(&mut self) -> &mut super::core::LanguageFormat;
    fn font_asset_path(&self) -> &String;
    fn font_asset_path_mut(&mut self) -> &mut String;
}

impl UIImmediateModeFontLookupTrait for UIImmediateModeFontLookup {
    fn language(&self) -> &super::core::LanguageFormat {
        &self.language
    }
    fn language_mut(&mut self) -> &mut super::core::LanguageFormat {
        &mut self.language
    }
    fn font_asset_path(&self) -> &String {
        &self.font_asset_path
    }
    fn font_asset_path_mut(&mut self) -> &mut String {
        &mut self.font_asset_path
    }
}

pub static UIIMMEDIATEMODEFONTLOOKUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontLookup",
    name_hash: 1811017082,
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImmediateModeFontLookup as Default>::default())),
            create_boxed: || Box::new(<UIImmediateModeFontLookup as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Language",
                name_hash: 3872303031,
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(UIImmediateModeFontLookup, language),
            },
            FieldInfoData {
                name: "FontAssetPath",
                name_hash: 2823862219,
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


pub static UIIMMEDIATEMODEFONTLOOKUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontLookup-Array",
    name_hash: 3753328718,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIImmediateModeFontLookup"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIImmediateModeFontBundle {
    pub language: super::core::LanguageFormat,
    pub bundle_path: String,
    pub ttf_assets: Vec<Option<LockedTypeObject /* UITtfAsset */>>,
}

pub trait UIImmediateModeFontBundleTrait: TypeObject {
    fn language(&self) -> &super::core::LanguageFormat;
    fn language_mut(&mut self) -> &mut super::core::LanguageFormat;
    fn bundle_path(&self) -> &String;
    fn bundle_path_mut(&mut self) -> &mut String;
    fn ttf_assets(&self) -> &Vec<Option<LockedTypeObject /* UITtfAsset */>>;
    fn ttf_assets_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* UITtfAsset */>>;
}

impl UIImmediateModeFontBundleTrait for UIImmediateModeFontBundle {
    fn language(&self) -> &super::core::LanguageFormat {
        &self.language
    }
    fn language_mut(&mut self) -> &mut super::core::LanguageFormat {
        &mut self.language
    }
    fn bundle_path(&self) -> &String {
        &self.bundle_path
    }
    fn bundle_path_mut(&mut self) -> &mut String {
        &mut self.bundle_path
    }
    fn ttf_assets(&self) -> &Vec<Option<LockedTypeObject /* UITtfAsset */>> {
        &self.ttf_assets
    }
    fn ttf_assets_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* UITtfAsset */>> {
        &mut self.ttf_assets
    }
}

pub static UIIMMEDIATEMODEFONTBUNDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontBundle",
    name_hash: 1877098348,
    flags: MemberInfoFlags::new(73),
    module: "GameSharedUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIImmediateModeFontBundle as Default>::default())),
            create_boxed: || Box::new(<UIImmediateModeFontBundle as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Language",
                name_hash: 3872303031,
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(UIImmediateModeFontBundle, language),
            },
            FieldInfoData {
                name: "BundlePath",
                name_hash: 459940508,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIImmediateModeFontBundle, bundle_path),
            },
            FieldInfoData {
                name: "TtfAssets",
                name_hash: 3080452608,
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


pub static UIIMMEDIATEMODEFONTBUNDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIImmediateModeFontBundle-Array",
    name_hash: 4145921880,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIImmediateModeFontBundle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UITtfAsset {
    pub _glacier_base: super::core::Asset,
    pub font_family_name: String,
    pub italic: bool,
    pub bold: bool,
    pub font_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait UITtfAssetTrait: super::core::AssetTrait {
    fn font_family_name(&self) -> &String;
    fn font_family_name_mut(&mut self) -> &mut String;
    fn italic(&self) -> &bool;
    fn italic_mut(&mut self) -> &mut bool;
    fn bold(&self) -> &bool;
    fn bold_mut(&mut self) -> &mut bool;
    fn font_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn font_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl UITtfAssetTrait for UITtfAsset {
    fn font_family_name(&self) -> &String {
        &self.font_family_name
    }
    fn font_family_name_mut(&mut self) -> &mut String {
        &mut self.font_family_name
    }
    fn italic(&self) -> &bool {
        &self.italic
    }
    fn italic_mut(&mut self) -> &mut bool {
        &mut self.italic
    }
    fn bold(&self) -> &bool {
        &self.bold
    }
    fn bold_mut(&mut self) -> &mut bool {
        &mut self.bold
    }
    fn font_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.font_resource
    }
    fn font_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.font_resource
    }
}

impl super::core::AssetTrait for UITtfAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for UITtfAsset {
}

pub static UITTFASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITtfAsset",
    name_hash: 2663104303,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(UITtfAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UITtfAsset as Default>::default())),
            create_boxed: || Box::new(<UITtfAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FontFamilyName",
                name_hash: 1196432615,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UITtfAsset, font_family_name),
            },
            FieldInfoData {
                name: "Italic",
                name_hash: 2814641183,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UITtfAsset, italic),
            },
            FieldInfoData {
                name: "Bold",
                name_hash: 2088812576,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UITtfAsset, bold),
            },
            FieldInfoData {
                name: "FontResource",
                name_hash: 2191416636,
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


pub static UITTFASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITtfAsset-Array",
    name_hash: 2086390939,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UITtfAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIInputEntityData {
}

impl super::core::DataContainerTrait for UIInputEntityData {
}

pub static UIINPUTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputEntityData",
    name_hash: 1607254212,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(UIInputEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIInputEntityData as Default>::default())),
            create_boxed: || Box::new(<UIInputEntityData as Default>::default()),
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


pub static UIINPUTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputEntityData-Array",
    name_hash: 11314544,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("UIInputEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MovieTrackData {
    pub _glacier_base: super::timeline::GuideTrackData,
    pub keyframes: Vec<Option<LockedTypeObject /* MovieTrackKeyframe */>>,
    pub volume: f32,
    pub expose_on_movie_started: bool,
    pub disable_world_renderer: bool,
}

pub trait MovieTrackDataTrait: super::timeline::GuideTrackDataTrait {
    fn keyframes(&self) -> &Vec<Option<LockedTypeObject /* MovieTrackKeyframe */>>;
    fn keyframes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* MovieTrackKeyframe */>>;
    fn volume(&self) -> &f32;
    fn volume_mut(&mut self) -> &mut f32;
    fn expose_on_movie_started(&self) -> &bool;
    fn expose_on_movie_started_mut(&mut self) -> &mut bool;
    fn disable_world_renderer(&self) -> &bool;
    fn disable_world_renderer_mut(&mut self) -> &mut bool;
}

impl MovieTrackDataTrait for MovieTrackData {
    fn keyframes(&self) -> &Vec<Option<LockedTypeObject /* MovieTrackKeyframe */>> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* MovieTrackKeyframe */>> {
        &mut self.keyframes
    }
    fn volume(&self) -> &f32 {
        &self.volume
    }
    fn volume_mut(&mut self) -> &mut f32 {
        &mut self.volume
    }
    fn expose_on_movie_started(&self) -> &bool {
        &self.expose_on_movie_started
    }
    fn expose_on_movie_started_mut(&mut self) -> &mut bool {
        &mut self.expose_on_movie_started
    }
    fn disable_world_renderer(&self) -> &bool {
        &self.disable_world_renderer
    }
    fn disable_world_renderer_mut(&mut self) -> &mut bool {
        &mut self.disable_world_renderer
    }
}

impl super::timeline::GuideTrackDataTrait for MovieTrackData {
    fn guide_track_priority(&self) -> &i32 {
        self._glacier_base.guide_track_priority()
    }
    fn guide_track_priority_mut(&mut self) -> &mut i32 {
        self._glacier_base.guide_track_priority_mut()
    }
}

impl super::timeline::TimelineTrackDataTrait for MovieTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<LockedTypeObject /* super::timeline::TimelineTrackDataConditionsBase */>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::timeline::TimelineTrackDataConditionsBase */>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for MovieTrackData {
}

impl super::core::DataBusPeerTrait for MovieTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MovieTrackData {
}

impl super::core::DataContainerTrait for MovieTrackData {
}

pub static MOVIETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackData",
    name_hash: 265400386,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::GUIDETRACKDATA_TYPE_INFO),
        super_class_offset: offset_of!(MovieTrackData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieTrackData as Default>::default())),
            create_boxed: || Box::new(<MovieTrackData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                name_hash: 2213598044,
                flags: MemberInfoFlags::new(144),
                field_type: "MovieTrackKeyframe-Array",
                rust_offset: offset_of!(MovieTrackData, keyframes),
            },
            FieldInfoData {
                name: "Volume",
                name_hash: 3158011725,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieTrackData, volume),
            },
            FieldInfoData {
                name: "ExposeOnMovieStarted",
                name_hash: 1787478857,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTrackData, expose_on_movie_started),
            },
            FieldInfoData {
                name: "DisableWorldRenderer",
                name_hash: 4043778894,
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


pub static MOVIETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackData-Array",
    name_hash: 947912694,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("MovieTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MovieTrackKeyframe {
    pub _glacier_base: super::core::DataContainer,
    pub time: f32,
    pub length: f32,
    pub movie: Option<LockedTypeObject /* super::movie_base::MovieTextureBaseAsset */>,
    pub pause_on_ending: bool,
    pub renderable_count: u32,
    pub thread_count: u32,
}

pub trait MovieTrackKeyframeTrait: super::core::DataContainerTrait {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn length(&self) -> &f32;
    fn length_mut(&mut self) -> &mut f32;
    fn movie(&self) -> &Option<LockedTypeObject /* super::movie_base::MovieTextureBaseAsset */>;
    fn movie_mut(&mut self) -> &mut Option<LockedTypeObject /* super::movie_base::MovieTextureBaseAsset */>;
    fn pause_on_ending(&self) -> &bool;
    fn pause_on_ending_mut(&mut self) -> &mut bool;
    fn renderable_count(&self) -> &u32;
    fn renderable_count_mut(&mut self) -> &mut u32;
    fn thread_count(&self) -> &u32;
    fn thread_count_mut(&mut self) -> &mut u32;
}

impl MovieTrackKeyframeTrait for MovieTrackKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn length_mut(&mut self) -> &mut f32 {
        &mut self.length
    }
    fn movie(&self) -> &Option<LockedTypeObject /* super::movie_base::MovieTextureBaseAsset */> {
        &self.movie
    }
    fn movie_mut(&mut self) -> &mut Option<LockedTypeObject /* super::movie_base::MovieTextureBaseAsset */> {
        &mut self.movie
    }
    fn pause_on_ending(&self) -> &bool {
        &self.pause_on_ending
    }
    fn pause_on_ending_mut(&mut self) -> &mut bool {
        &mut self.pause_on_ending
    }
    fn renderable_count(&self) -> &u32 {
        &self.renderable_count
    }
    fn renderable_count_mut(&mut self) -> &mut u32 {
        &mut self.renderable_count
    }
    fn thread_count(&self) -> &u32 {
        &self.thread_count
    }
    fn thread_count_mut(&mut self) -> &mut u32 {
        &mut self.thread_count
    }
}

impl super::core::DataContainerTrait for MovieTrackKeyframe {
}

pub static MOVIETRACKKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackKeyframe",
    name_hash: 1905193464,
    flags: MemberInfoFlags::new(101),
    module: "GameSharedUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(MovieTrackKeyframe, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieTrackKeyframe as Default>::default())),
            create_boxed: || Box::new(<MovieTrackKeyframe as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                name_hash: 2089313744,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieTrackKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                name_hash: 2906827577,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieTrackKeyframe, length),
            },
            FieldInfoData {
                name: "Movie",
                name_hash: 210030653,
                flags: MemberInfoFlags::new(0),
                field_type: "MovieTextureBaseAsset",
                rust_offset: offset_of!(MovieTrackKeyframe, movie),
            },
            FieldInfoData {
                name: "PauseOnEnding",
                name_hash: 3821927929,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTrackKeyframe, pause_on_ending),
            },
            FieldInfoData {
                name: "RenderableCount",
                name_hash: 1982187302,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTrackKeyframe, renderable_count),
            },
            FieldInfoData {
                name: "ThreadCount",
                name_hash: 886477064,
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


pub static MOVIETRACKKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTrackKeyframe-Array",
    name_hash: 3011079116,
    flags: MemberInfoFlags::new(145),
    module: "GameSharedUI",
    data: TypeInfoData::Array("MovieTrackKeyframe"),
    array_type: None,
    alignment: 8,
};


