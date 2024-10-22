use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct LinearMediaBillboardSettings {
    pub _glacier_base: super::core::DataContainer,
    pub debug_entity_logic_update_enable: bool,
    pub streaming_enable: bool,
    pub skip_l_o_d: i32,
    pub max_active_feeds: i32,
    pub support_texture2_enable: bool,
    pub support_emissive_enable: bool,
    pub l_o_d_screen_surface_scale: f32,
    pub force_lowest_mip_dynamic: bool,
}

pub trait LinearMediaBillboardSettingsTrait: super::core::DataContainerTrait {
    fn debug_entity_logic_update_enable(&self) -> &bool;
    fn debug_entity_logic_update_enable_mut(&mut self) -> &mut bool;
    fn streaming_enable(&self) -> &bool;
    fn streaming_enable_mut(&mut self) -> &mut bool;
    fn skip_l_o_d(&self) -> &i32;
    fn skip_l_o_d_mut(&mut self) -> &mut i32;
    fn max_active_feeds(&self) -> &i32;
    fn max_active_feeds_mut(&mut self) -> &mut i32;
    fn support_texture2_enable(&self) -> &bool;
    fn support_texture2_enable_mut(&mut self) -> &mut bool;
    fn support_emissive_enable(&self) -> &bool;
    fn support_emissive_enable_mut(&mut self) -> &mut bool;
    fn l_o_d_screen_surface_scale(&self) -> &f32;
    fn l_o_d_screen_surface_scale_mut(&mut self) -> &mut f32;
    fn force_lowest_mip_dynamic(&self) -> &bool;
    fn force_lowest_mip_dynamic_mut(&mut self) -> &mut bool;
}

impl LinearMediaBillboardSettingsTrait for LinearMediaBillboardSettings {
    fn debug_entity_logic_update_enable(&self) -> &bool {
        &self.debug_entity_logic_update_enable
    }
    fn debug_entity_logic_update_enable_mut(&mut self) -> &mut bool {
        &mut self.debug_entity_logic_update_enable
    }
    fn streaming_enable(&self) -> &bool {
        &self.streaming_enable
    }
    fn streaming_enable_mut(&mut self) -> &mut bool {
        &mut self.streaming_enable
    }
    fn skip_l_o_d(&self) -> &i32 {
        &self.skip_l_o_d
    }
    fn skip_l_o_d_mut(&mut self) -> &mut i32 {
        &mut self.skip_l_o_d
    }
    fn max_active_feeds(&self) -> &i32 {
        &self.max_active_feeds
    }
    fn max_active_feeds_mut(&mut self) -> &mut i32 {
        &mut self.max_active_feeds
    }
    fn support_texture2_enable(&self) -> &bool {
        &self.support_texture2_enable
    }
    fn support_texture2_enable_mut(&mut self) -> &mut bool {
        &mut self.support_texture2_enable
    }
    fn support_emissive_enable(&self) -> &bool {
        &self.support_emissive_enable
    }
    fn support_emissive_enable_mut(&mut self) -> &mut bool {
        &mut self.support_emissive_enable
    }
    fn l_o_d_screen_surface_scale(&self) -> &f32 {
        &self.l_o_d_screen_surface_scale
    }
    fn l_o_d_screen_surface_scale_mut(&mut self) -> &mut f32 {
        &mut self.l_o_d_screen_surface_scale
    }
    fn force_lowest_mip_dynamic(&self) -> &bool {
        &self.force_lowest_mip_dynamic
    }
    fn force_lowest_mip_dynamic_mut(&mut self) -> &mut bool {
        &mut self.force_lowest_mip_dynamic
    }
}

impl super::core::DataContainerTrait for LinearMediaBillboardSettings {
}

pub static LINEARMEDIABILLBOARDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardSettings",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaBillboardSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DebugEntityLogicUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardSettings, debug_entity_logic_update_enable),
            },
            FieldInfoData {
                name: "StreamingEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardSettings, streaming_enable),
            },
            FieldInfoData {
                name: "SkipLOD",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(LinearMediaBillboardSettings, skip_l_o_d),
            },
            FieldInfoData {
                name: "MaxActiveFeeds",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(LinearMediaBillboardSettings, max_active_feeds),
            },
            FieldInfoData {
                name: "SupportTexture2Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardSettings, support_texture2_enable),
            },
            FieldInfoData {
                name: "SupportEmissiveEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardSettings, support_emissive_enable),
            },
            FieldInfoData {
                name: "LODScreenSurfaceScale",
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(LinearMediaBillboardSettings, l_o_d_screen_surface_scale),
            },
            FieldInfoData {
                name: "ForceLowestMipDynamic",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardSettings, force_lowest_mip_dynamic),
            },
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaBillboardSettings {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIABILLBOARDSETTINGS_TYPE_INFO
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


pub static LINEARMEDIABILLBOARDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaBillboardClientEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub receiving_channel: i32,
    pub is_dynamic_object: bool,
    pub size_in_meter_sqr: f32,
    pub disable_l_o_d: bool,
    pub use_second_texture_stream: bool,
    pub enabled_on_startup: bool,
    pub fallback_texture: Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>,
}

pub trait LinearMediaBillboardClientEntityDataTrait: super::entity::EntityDataTrait {
    fn receiving_channel(&self) -> &i32;
    fn receiving_channel_mut(&mut self) -> &mut i32;
    fn is_dynamic_object(&self) -> &bool;
    fn is_dynamic_object_mut(&mut self) -> &mut bool;
    fn size_in_meter_sqr(&self) -> &f32;
    fn size_in_meter_sqr_mut(&mut self) -> &mut f32;
    fn disable_l_o_d(&self) -> &bool;
    fn disable_l_o_d_mut(&mut self) -> &mut bool;
    fn use_second_texture_stream(&self) -> &bool;
    fn use_second_texture_stream_mut(&mut self) -> &mut bool;
    fn enabled_on_startup(&self) -> &bool;
    fn enabled_on_startup_mut(&mut self) -> &mut bool;
    fn fallback_texture(&self) -> &Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>;
    fn fallback_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>;
}

impl LinearMediaBillboardClientEntityDataTrait for LinearMediaBillboardClientEntityData {
    fn receiving_channel(&self) -> &i32 {
        &self.receiving_channel
    }
    fn receiving_channel_mut(&mut self) -> &mut i32 {
        &mut self.receiving_channel
    }
    fn is_dynamic_object(&self) -> &bool {
        &self.is_dynamic_object
    }
    fn is_dynamic_object_mut(&mut self) -> &mut bool {
        &mut self.is_dynamic_object
    }
    fn size_in_meter_sqr(&self) -> &f32 {
        &self.size_in_meter_sqr
    }
    fn size_in_meter_sqr_mut(&mut self) -> &mut f32 {
        &mut self.size_in_meter_sqr
    }
    fn disable_l_o_d(&self) -> &bool {
        &self.disable_l_o_d
    }
    fn disable_l_o_d_mut(&mut self) -> &mut bool {
        &mut self.disable_l_o_d
    }
    fn use_second_texture_stream(&self) -> &bool {
        &self.use_second_texture_stream
    }
    fn use_second_texture_stream_mut(&mut self) -> &mut bool {
        &mut self.use_second_texture_stream
    }
    fn enabled_on_startup(&self) -> &bool {
        &self.enabled_on_startup
    }
    fn enabled_on_startup_mut(&mut self) -> &mut bool {
        &mut self.enabled_on_startup
    }
    fn fallback_texture(&self) -> &Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>> {
        &self.fallback_texture
    }
    fn fallback_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>> {
        &mut self.fallback_texture
    }
}

impl super::entity::EntityDataTrait for LinearMediaBillboardClientEntityData {
}

impl super::entity::GameObjectDataTrait for LinearMediaBillboardClientEntityData {
}

impl super::core::DataBusPeerTrait for LinearMediaBillboardClientEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LinearMediaBillboardClientEntityData {
}

impl super::core::DataContainerTrait for LinearMediaBillboardClientEntityData {
}

pub static LINEARMEDIABILLBOARDCLIENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardClientEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaBillboardClientEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ReceivingChannel",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, receiving_channel),
            },
            FieldInfoData {
                name: "isDynamicObject",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, is_dynamic_object),
            },
            FieldInfoData {
                name: "SizeInMeterSqr",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, size_in_meter_sqr),
            },
            FieldInfoData {
                name: "disableLOD",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, disable_l_o_d),
            },
            FieldInfoData {
                name: "useSecondTextureStream",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, use_second_texture_stream),
            },
            FieldInfoData {
                name: "EnabledOnStartup",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, enabled_on_startup),
            },
            FieldInfoData {
                name: "FallbackTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAsset",
                rust_offset: offset_of!(LinearMediaBillboardClientEntityData, fallback_texture),
            },
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDCLIENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaBillboardClientEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIABILLBOARDCLIENTENTITYDATA_TYPE_INFO
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


pub static LINEARMEDIABILLBOARDCLIENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardClientEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardClientEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaBillboardOverrideFeedEntityData {
    pub _glacier_base: LinearMediaBillboardFeedEntityData,
    pub channels_to_override: Vec<i32>,
}

pub trait LinearMediaBillboardOverrideFeedEntityDataTrait: LinearMediaBillboardFeedEntityDataTrait {
    fn channels_to_override(&self) -> &Vec<i32>;
    fn channels_to_override_mut(&mut self) -> &mut Vec<i32>;
}

impl LinearMediaBillboardOverrideFeedEntityDataTrait for LinearMediaBillboardOverrideFeedEntityData {
    fn channels_to_override(&self) -> &Vec<i32> {
        &self.channels_to_override
    }
    fn channels_to_override_mut(&mut self) -> &mut Vec<i32> {
        &mut self.channels_to_override
    }
}

impl LinearMediaBillboardFeedEntityDataTrait for LinearMediaBillboardOverrideFeedEntityData {
    fn emitting_channel(&self) -> &i32 {
        self._glacier_base.emitting_channel()
    }
    fn emitting_channel_mut(&mut self) -> &mut i32 {
        self._glacier_base.emitting_channel_mut()
    }
    fn texture_codes(&self) -> &Vec<LinearMediaLODCodes> {
        self._glacier_base.texture_codes()
    }
    fn texture_codes_mut(&mut self) -> &mut Vec<LinearMediaLODCodes> {
        self._glacier_base.texture_codes_mut()
    }
    fn linear_media_billboard_asset(&self) -> &Option<Arc<Mutex<dyn LMSBillboardAssetTrait>>> {
        self._glacier_base.linear_media_billboard_asset()
    }
    fn linear_media_billboard_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn LMSBillboardAssetTrait>>> {
        self._glacier_base.linear_media_billboard_asset_mut()
    }
    fn start_on_load(&self) -> &bool {
        self._glacier_base.start_on_load()
    }
    fn start_on_load_mut(&mut self) -> &mut bool {
        self._glacier_base.start_on_load_mut()
    }
    fn r#loop(&self) -> &bool {
        self._glacier_base.r#loop()
    }
    fn r#loop_mut(&mut self) -> &mut bool {
        self._glacier_base.r#loop_mut()
    }
    fn external_time(&self) -> &f32 {
        self._glacier_base.external_time()
    }
    fn external_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.external_time_mut()
    }
}

impl super::entity::EntityDataTrait for LinearMediaBillboardOverrideFeedEntityData {
}

impl super::entity::GameObjectDataTrait for LinearMediaBillboardOverrideFeedEntityData {
}

impl super::core::DataBusPeerTrait for LinearMediaBillboardOverrideFeedEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LinearMediaBillboardOverrideFeedEntityData {
}

impl super::core::DataContainerTrait for LinearMediaBillboardOverrideFeedEntityData {
}

pub static LINEARMEDIABILLBOARDOVERRIDEFEEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardOverrideFeedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINEARMEDIABILLBOARDFEEDENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaBillboardOverrideFeedEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ChannelsToOverride",
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(LinearMediaBillboardOverrideFeedEntityData, channels_to_override),
            },
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDOVERRIDEFEEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaBillboardOverrideFeedEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIABILLBOARDOVERRIDEFEEDENTITYDATA_TYPE_INFO
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


pub static LINEARMEDIABILLBOARDOVERRIDEFEEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardOverrideFeedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardOverrideFeedEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaBillboardFeedEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub emitting_channel: i32,
    pub texture_codes: Vec<LinearMediaLODCodes>,
    pub linear_media_billboard_asset: Option<Arc<Mutex<dyn LMSBillboardAssetTrait>>>,
    pub start_on_load: bool,
    pub r#loop: bool,
    pub external_time: f32,
}

pub trait LinearMediaBillboardFeedEntityDataTrait: super::entity::EntityDataTrait {
    fn emitting_channel(&self) -> &i32;
    fn emitting_channel_mut(&mut self) -> &mut i32;
    fn texture_codes(&self) -> &Vec<LinearMediaLODCodes>;
    fn texture_codes_mut(&mut self) -> &mut Vec<LinearMediaLODCodes>;
    fn linear_media_billboard_asset(&self) -> &Option<Arc<Mutex<dyn LMSBillboardAssetTrait>>>;
    fn linear_media_billboard_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn LMSBillboardAssetTrait>>>;
    fn start_on_load(&self) -> &bool;
    fn start_on_load_mut(&mut self) -> &mut bool;
    fn r#loop(&self) -> &bool;
    fn r#loop_mut(&mut self) -> &mut bool;
    fn external_time(&self) -> &f32;
    fn external_time_mut(&mut self) -> &mut f32;
}

impl LinearMediaBillboardFeedEntityDataTrait for LinearMediaBillboardFeedEntityData {
    fn emitting_channel(&self) -> &i32 {
        &self.emitting_channel
    }
    fn emitting_channel_mut(&mut self) -> &mut i32 {
        &mut self.emitting_channel
    }
    fn texture_codes(&self) -> &Vec<LinearMediaLODCodes> {
        &self.texture_codes
    }
    fn texture_codes_mut(&mut self) -> &mut Vec<LinearMediaLODCodes> {
        &mut self.texture_codes
    }
    fn linear_media_billboard_asset(&self) -> &Option<Arc<Mutex<dyn LMSBillboardAssetTrait>>> {
        &self.linear_media_billboard_asset
    }
    fn linear_media_billboard_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn LMSBillboardAssetTrait>>> {
        &mut self.linear_media_billboard_asset
    }
    fn start_on_load(&self) -> &bool {
        &self.start_on_load
    }
    fn start_on_load_mut(&mut self) -> &mut bool {
        &mut self.start_on_load
    }
    fn r#loop(&self) -> &bool {
        &self.r#loop
    }
    fn r#loop_mut(&mut self) -> &mut bool {
        &mut self.r#loop
    }
    fn external_time(&self) -> &f32 {
        &self.external_time
    }
    fn external_time_mut(&mut self) -> &mut f32 {
        &mut self.external_time
    }
}

impl super::entity::EntityDataTrait for LinearMediaBillboardFeedEntityData {
}

impl super::entity::GameObjectDataTrait for LinearMediaBillboardFeedEntityData {
}

impl super::core::DataBusPeerTrait for LinearMediaBillboardFeedEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LinearMediaBillboardFeedEntityData {
}

impl super::core::DataContainerTrait for LinearMediaBillboardFeedEntityData {
}

pub static LINEARMEDIABILLBOARDFEEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardFeedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaBillboardFeedEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EmittingChannel",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, emitting_channel),
            },
            FieldInfoData {
                name: "TextureCodes",
                flags: MemberInfoFlags::new(144),
                field_type: "LinearMediaLODCodes-Array",
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, texture_codes),
            },
            FieldInfoData {
                name: "LinearMediaBillboardAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "LMSBillboardAsset",
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, linear_media_billboard_asset),
            },
            FieldInfoData {
                name: "StartOnLoad",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, start_on_load),
            },
            FieldInfoData {
                name: "Loop",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, r#loop),
            },
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LinearMediaBillboardFeedEntityData, external_time),
            },
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDFEEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaBillboardFeedEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIABILLBOARDFEEDENTITYDATA_TYPE_INFO
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


pub static LINEARMEDIABILLBOARDFEEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardFeedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardFeedEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaLODCodes {
    pub texture_out_code: u64,
    pub texture2_out_code: u64,
    pub emissive_texture_out_code: u64,
}

pub trait LinearMediaLODCodesTrait: TypeObject {
    fn texture_out_code(&self) -> &u64;
    fn texture_out_code_mut(&mut self) -> &mut u64;
    fn texture2_out_code(&self) -> &u64;
    fn texture2_out_code_mut(&mut self) -> &mut u64;
    fn emissive_texture_out_code(&self) -> &u64;
    fn emissive_texture_out_code_mut(&mut self) -> &mut u64;
}

impl LinearMediaLODCodesTrait for LinearMediaLODCodes {
    fn texture_out_code(&self) -> &u64 {
        &self.texture_out_code
    }
    fn texture_out_code_mut(&mut self) -> &mut u64 {
        &mut self.texture_out_code
    }
    fn texture2_out_code(&self) -> &u64 {
        &self.texture2_out_code
    }
    fn texture2_out_code_mut(&mut self) -> &mut u64 {
        &mut self.texture2_out_code
    }
    fn emissive_texture_out_code(&self) -> &u64 {
        &self.emissive_texture_out_code
    }
    fn emissive_texture_out_code_mut(&mut self) -> &mut u64 {
        &mut self.emissive_texture_out_code
    }
}

pub static LINEARMEDIALODCODES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaLODCodes",
    flags: MemberInfoFlags::new(36937),
    module: "LMSBillboard",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaLODCodes as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TextureOutCode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(LinearMediaLODCodes, texture_out_code),
            },
            FieldInfoData {
                name: "Texture2OutCode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(LinearMediaLODCodes, texture2_out_code),
            },
            FieldInfoData {
                name: "EmissiveTextureOutCode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(LinearMediaLODCodes, emissive_texture_out_code),
            },
        ],
    }),
    array_type: Some(LINEARMEDIALODCODES_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaLODCodes {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIALODCODES_TYPE_INFO
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


pub static LINEARMEDIALODCODES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaLODCodes-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaLODCodes"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaBillboardProviderEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub fallbacktexture: Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>,
    pub is_active: bool,
}

pub trait LinearMediaBillboardProviderEntityDataTrait: super::entity::EntityDataTrait {
    fn fallbacktexture(&self) -> &Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>;
    fn fallbacktexture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>;
    fn is_active(&self) -> &bool;
    fn is_active_mut(&mut self) -> &mut bool;
}

impl LinearMediaBillboardProviderEntityDataTrait for LinearMediaBillboardProviderEntityData {
    fn fallbacktexture(&self) -> &Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>> {
        &self.fallbacktexture
    }
    fn fallbacktexture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>> {
        &mut self.fallbacktexture
    }
    fn is_active(&self) -> &bool {
        &self.is_active
    }
    fn is_active_mut(&mut self) -> &mut bool {
        &mut self.is_active
    }
}

impl super::entity::EntityDataTrait for LinearMediaBillboardProviderEntityData {
}

impl super::entity::GameObjectDataTrait for LinearMediaBillboardProviderEntityData {
}

impl super::core::DataBusPeerTrait for LinearMediaBillboardProviderEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LinearMediaBillboardProviderEntityData {
}

impl super::core::DataContainerTrait for LinearMediaBillboardProviderEntityData {
}

pub static LINEARMEDIABILLBOARDPROVIDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardProviderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaBillboardProviderEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Fallbacktexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAsset",
                rust_offset: offset_of!(LinearMediaBillboardProviderEntityData, fallbacktexture),
            },
            FieldInfoData {
                name: "IsActive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaBillboardProviderEntityData, is_active),
            },
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDPROVIDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaBillboardProviderEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIABILLBOARDPROVIDERENTITYDATA_TYPE_INFO
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


pub static LINEARMEDIABILLBOARDPROVIDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardProviderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardProviderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LMSBillboardAsset {
    pub _glacier_base: super::linear_media::LinearMediaAssetDesc,
    pub base_texture: Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>,
}

pub trait LMSBillboardAssetTrait: super::linear_media::LinearMediaAssetDescTrait {
    fn base_texture(&self) -> &Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>;
    fn base_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>;
}

impl LMSBillboardAssetTrait for LMSBillboardAsset {
    fn base_texture(&self) -> &Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>> {
        &self.base_texture
    }
    fn base_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>> {
        &mut self.base_texture
    }
}

impl super::linear_media::LinearMediaAssetDescTrait for LMSBillboardAsset {
    fn resources(&self) -> &Vec<super::linear_media::LinearMediaRuntimeResource> {
        self._glacier_base.resources()
    }
    fn resources_mut(&mut self) -> &mut Vec<super::linear_media::LinearMediaRuntimeResource> {
        self._glacier_base.resources_mut()
    }
}

impl super::core::AssetTrait for LMSBillboardAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for LMSBillboardAsset {
}

pub static LMSBILLBOARDASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSBillboardAsset",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::linear_media::LINEARMEDIAASSETDESC_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LMSBillboardAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BaseTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAsset",
                rust_offset: offset_of!(LMSBillboardAsset, base_texture),
            },
        ],
    }),
    array_type: Some(LMSBILLBOARDASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LMSBillboardAsset {
    fn type_info(&self) -> &'static TypeInfo {
        LMSBILLBOARDASSET_TYPE_INFO
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


pub static LMSBILLBOARDASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LMSBillboardAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LMSBillboardAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LODDimension {
    pub width: i32,
    pub height: i32,
}

pub trait LODDimensionTrait: TypeObject {
    fn width(&self) -> &i32;
    fn width_mut(&mut self) -> &mut i32;
    fn height(&self) -> &i32;
    fn height_mut(&mut self) -> &mut i32;
}

impl LODDimensionTrait for LODDimension {
    fn width(&self) -> &i32 {
        &self.width
    }
    fn width_mut(&mut self) -> &mut i32 {
        &mut self.width
    }
    fn height(&self) -> &i32 {
        &self.height
    }
    fn height_mut(&mut self) -> &mut i32 {
        &mut self.height
    }
}

pub static LODDIMENSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LODDimension",
    flags: MemberInfoFlags::new(36937),
    module: "LMSBillboard",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LODDimension as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LODDimension, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LODDimension, height),
            },
        ],
    }),
    array_type: Some(LODDIMENSION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LODDimension {
    fn type_info(&self) -> &'static TypeInfo {
        LODDIMENSION_TYPE_INFO
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


pub static LODDIMENSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LODDimension-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LODDimension"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LinearMediaBillboardDefs {
    #[default]
    LMBBD_MaxSpeakersPerBillboardFeed = 4,
    LMBBD_MaxSupportedBillboardFeeds = 32,
}

pub static LINEARMEDIABILLBOARDDEFS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardDefs",
    flags: MemberInfoFlags::new(49429),
    module: "LMSBillboard",
    data: TypeInfoData::Enum,
    array_type: Some(LINEARMEDIABILLBOARDDEFS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LinearMediaBillboardDefs {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIABILLBOARDDEFS_TYPE_INFO
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


pub static LINEARMEDIABILLBOARDDEFS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardDefs-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardDefs"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaBillboardProviderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LinearMediaBillboardProviderEntityTrait: super::entity::EntityTrait {
}

impl LinearMediaBillboardProviderEntityTrait for LinearMediaBillboardProviderEntity {
}

impl super::entity::EntityTrait for LinearMediaBillboardProviderEntity {
}

impl super::entity::EntityBusPeerTrait for LinearMediaBillboardProviderEntity {
}

pub static LINEARMEDIABILLBOARDPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaBillboardProviderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaBillboardProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIABILLBOARDPROVIDERENTITY_TYPE_INFO
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


pub static LINEARMEDIABILLBOARDPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaBillboardOverrideFeedEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LinearMediaBillboardOverrideFeedEntityTrait: super::entity::EntityTrait {
}

impl LinearMediaBillboardOverrideFeedEntityTrait for LinearMediaBillboardOverrideFeedEntity {
}

impl super::entity::EntityTrait for LinearMediaBillboardOverrideFeedEntity {
}

impl super::entity::EntityBusPeerTrait for LinearMediaBillboardOverrideFeedEntity {
}

pub static LINEARMEDIABILLBOARDOVERRIDEFEEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardOverrideFeedEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaBillboardOverrideFeedEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDOVERRIDEFEEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaBillboardOverrideFeedEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIABILLBOARDOVERRIDEFEEDENTITY_TYPE_INFO
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


pub static LINEARMEDIABILLBOARDOVERRIDEFEEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardOverrideFeedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardOverrideFeedEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaBillboardFeedEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LinearMediaBillboardFeedEntityTrait: super::entity::EntityTrait {
}

impl LinearMediaBillboardFeedEntityTrait for LinearMediaBillboardFeedEntity {
}

impl super::entity::EntityTrait for LinearMediaBillboardFeedEntity {
}

impl super::entity::EntityBusPeerTrait for LinearMediaBillboardFeedEntity {
}

pub static LINEARMEDIABILLBOARDFEEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardFeedEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaBillboardFeedEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDFEEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaBillboardFeedEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIABILLBOARDFEEDENTITY_TYPE_INFO
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


pub static LINEARMEDIABILLBOARDFEEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardFeedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardFeedEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaBillboardClientEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LinearMediaBillboardClientEntityTrait: super::entity::EntityTrait {
}

impl LinearMediaBillboardClientEntityTrait for LinearMediaBillboardClientEntity {
}

impl super::entity::EntityTrait for LinearMediaBillboardClientEntity {
}

impl super::entity::EntityBusPeerTrait for LinearMediaBillboardClientEntity {
}

pub static LINEARMEDIABILLBOARDCLIENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardClientEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSBillboard",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaBillboardClientEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIABILLBOARDCLIENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaBillboardClientEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIABILLBOARDCLIENTENTITY_TYPE_INFO
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


pub static LINEARMEDIABILLBOARDCLIENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaBillboardClientEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSBillboard",
    data: TypeInfoData::Array("LinearMediaBillboardClientEntity"),
    array_type: None,
    alignment: 8,
};


