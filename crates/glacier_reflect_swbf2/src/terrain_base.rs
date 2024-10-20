use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_terrain_base_types(registry: &mut TypeRegistry) {
    registry.register_type(VISUALTERRAINDYNAMICSTATE_TYPE_INFO);
    registry.register_type(VISUALTERRAINDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VISUALTERRAINSTATICSTATE_TYPE_INFO);
    registry.register_type(VISUALTERRAINSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VISUALTERRAINHANDLE_TYPE_INFO);
    registry.register_type(VISUALTERRAINHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(VISUALTERRAINBASESETTINGS_TYPE_INFO);
    registry.register_type(VISUALTERRAINBASESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINSTREAMINGSETTINGS_TYPE_INFO);
    registry.register_type(TERRAINSTREAMINGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINQUADDECALDATA_TYPE_INFO);
    registry.register_type(TERRAINQUADDECALDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINQUADDECALATLASTILETEMPLATEDATA_TYPE_INFO);
    registry.register_type(TERRAINQUADDECALATLASTILETEMPLATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINQUADDECALATLASTILE_TYPE_INFO);
    registry.register_type(TERRAINQUADDECALATLASTILE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINFILLDECALDATA_TYPE_INFO);
    registry.register_type(TERRAINFILLDECALDATA_ARRAY_TYPE_INFO);
    registry.register_type(LAKEDATA_TYPE_INFO);
    registry.register_type(LAKEDATA_ARRAY_TYPE_INFO);
    registry.register_type(RIVERDATA_TYPE_INFO);
    registry.register_type(RIVERDATA_ARRAY_TYPE_INFO);
    registry.register_type(ROADDATA_TYPE_INFO);
    registry.register_type(ROADDATA_ARRAY_TYPE_INFO);
    registry.register_type(RIBBONDATA_TYPE_INFO);
    registry.register_type(RIBBONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RIBBONPOINTDATA_TYPE_INFO);
    registry.register_type(RIBBONPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(VISUALVECTORSHAPEDATA_TYPE_INFO);
    registry.register_type(VISUALVECTORSHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO);
    registry.register_type(TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINSHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO);
    registry.register_type(TERRAINSHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINSETTINGS_TYPE_INFO);
    registry.register_type(TERRAINSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINMODIFICATIONDYNAMICSTATE_TYPE_INFO);
    registry.register_type(TERRAINMODIFICATIONDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINMODIFICATIONTYPE_TYPE_INFO);
    registry.register_type(TERRAINMODIFICATIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINEDITINGDYNAMICSTATE_TYPE_INFO);
    registry.register_type(TERRAINEDITINGDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINEDITINGSTATICSTATE_TYPE_INFO);
    registry.register_type(TERRAINEDITINGSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINEDITINGEVENT_TYPE_INFO);
    registry.register_type(TERRAINEDITINGEVENT_ARRAY_TYPE_INFO);
    registry.register_type(PLAYABLEPIXELSPERMETER_TYPE_INFO);
    registry.register_type(PLAYABLEPIXELSPERMETER_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINANCHOR_TYPE_INFO);
    registry.register_type(TERRAINANCHOR_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINSIZE_TYPE_INFO);
    registry.register_type(TERRAINSIZE_ARRAY_TYPE_INFO);
    registry.register_type(HIGHRESTERRAINSIZE_TYPE_INFO);
    registry.register_type(HIGHRESTERRAINSIZE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINLAYERCOMBINATIONSDATA_TYPE_INFO);
    registry.register_type(TERRAINLAYERCOMBINATIONSDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINLAYERSHADERDATA_TYPE_INFO);
    registry.register_type(TERRAINLAYERSHADERDATA_ARRAY_TYPE_INFO);
    registry.register_type(MESHSCATTERINGSPAWNDATA_TYPE_INFO);
    registry.register_type(MESHSCATTERINGSPAWNDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINSTREAMINGTREEASSET_TYPE_INFO);
    registry.register_type(TERRAINSTREAMINGTREEASSET_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINLAYERMASKDATA_TYPE_INFO);
    registry.register_type(TERRAINLAYERMASKDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGMASKRASTERDATA_TYPE_INFO);
    registry.register_type(PATHFINDINGMASKRASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINDATA_TYPE_INFO);
    registry.register_type(TERRAINDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENMESHFILTERTYPE_TYPE_INFO);
    registry.register_type(ENLIGHTENMESHFILTERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RASTERTREEBUILDMODE_TYPE_INFO);
    registry.register_type(RASTERTREEBUILDMODE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINLAYERCOMBINATIONDRAWDATA_TYPE_INFO);
    registry.register_type(TERRAINLAYERCOMBINATIONDRAWDATA_ARRAY_TYPE_INFO);
    registry.register_type(SURFACE3DDRAWMETHODDATA_TYPE_INFO);
    registry.register_type(SURFACE3DDRAWMETHODDATA_ARRAY_TYPE_INFO);
    registry.register_type(SINGLELAYERMASKDRAWMETHODDATA_TYPE_INFO);
    registry.register_type(SINGLELAYERMASKDRAWMETHODDATA_ARRAY_TYPE_INFO);
    registry.register_type(SINGLELAYERMASKDRAWPASSDATA_TYPE_INFO);
    registry.register_type(SINGLELAYERMASKDRAWPASSDATA_ARRAY_TYPE_INFO);
    registry.register_type(MESHSCATTERINGMASKSCALEDRAWMETHODDATA_TYPE_INFO);
    registry.register_type(MESHSCATTERINGMASKSCALEDRAWMETHODDATA_ARRAY_TYPE_INFO);
    registry.register_type(DISPLACEMENT2DDRAWMETHODDATA_TYPE_INFO);
    registry.register_type(DISPLACEMENT2DDRAWMETHODDATA_ARRAY_TYPE_INFO);
    registry.register_type(DISPLACEMENT2DDRAWPASSDATA_TYPE_INFO);
    registry.register_type(DISPLACEMENT2DDRAWPASSDATA_ARRAY_TYPE_INFO);
    registry.register_type(SURFACE2DDRAWMETHODDATA_TYPE_INFO);
    registry.register_type(SURFACE2DDRAWMETHODDATA_ARRAY_TYPE_INFO);
    registry.register_type(SURFACE2DDRAWPASSDATA_TYPE_INFO);
    registry.register_type(SURFACE2DDRAWPASSDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINLAYERCOMBINATIONDRAWPASSDATA_TYPE_INFO);
    registry.register_type(TERRAINLAYERCOMBINATIONDRAWPASSDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINDRAWPASSTYPE_TYPE_INFO);
    registry.register_type(TERRAINDRAWPASSTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINBRUSHDETAILOPERATION_TYPE_INFO);
    registry.register_type(TERRAINBRUSHDETAILOPERATION_ARRAY_TYPE_INFO);
    registry.register_type(SINGLETERRAINLAYERDATA_TYPE_INFO);
    registry.register_type(SINGLETERRAINLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINLAYERDATA_TYPE_INFO);
    registry.register_type(TERRAINLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(COLORIMPORTSETTINGS_TYPE_INFO);
    registry.register_type(COLORIMPORTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINLAYERTYPE_TYPE_INFO);
    registry.register_type(TERRAINLAYERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINLAYERPROCEDURALMASK_TYPE_INFO);
    registry.register_type(TERRAINLAYERPROCEDURALMASK_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINGEOTEXTURE_TYPE_INFO);
    registry.register_type(TERRAINGEOTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINMESHSCATTERINGTYPE_TYPE_INFO);
    registry.register_type(TERRAINMESHSCATTERINGTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MESHSCATTERINGINSTANCEDATAMODE_TYPE_INFO);
    registry.register_type(MESHSCATTERINGINSTANCEDATAMODE_ARRAY_TYPE_INFO);
    registry.register_type(UNDERGROWTHORIENTATIONMODE_TYPE_INFO);
    registry.register_type(UNDERGROWTHORIENTATIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(MESHSCATTERINGBILLBOARDMODE_TYPE_INFO);
    registry.register_type(MESHSCATTERINGBILLBOARDMODE_ARRAY_TYPE_INFO);
    registry.register_type(MESHSCATTERINGORIENTATIONMODE_TYPE_INFO);
    registry.register_type(MESHSCATTERINGORIENTATIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(UNDERGROWTHROTATIONMODE_TYPE_INFO);
    registry.register_type(UNDERGROWTHROTATIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(MESHSCATTERINGROTATIONMODE_TYPE_INFO);
    registry.register_type(MESHSCATTERINGROTATIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(MESHSCATTERINGELEVATIONMODE_TYPE_INFO);
    registry.register_type(MESHSCATTERINGELEVATIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINHEIGHTFIELDDATA_TYPE_INFO);
    registry.register_type(TERRAINHEIGHTFIELDDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINBRUSHTYPE_TYPE_INFO);
    registry.register_type(TERRAINBRUSHTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINDYNAMICDECALTEMPLATEDATA_TYPE_INFO);
    registry.register_type(TERRAINDYNAMICDECALTEMPLATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(HEIGHTFIELDDECALASSET_TYPE_INFO);
    registry.register_type(HEIGHTFIELDDECALASSET_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINBASEASSET_TYPE_INFO);
    registry.register_type(TERRAINBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGRASTERDATA_TYPE_INFO);
    registry.register_type(PATHFINDINGRASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(RASTERCOVERAGEDATA_TYPE_INFO);
    registry.register_type(RASTERCOVERAGEDATA_ARRAY_TYPE_INFO);
    registry.register_type(DENSITYMAPRASTERDATA_TYPE_INFO);
    registry.register_type(DENSITYMAPRASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(BIOMERASTERDATA_TYPE_INFO);
    registry.register_type(BIOMERASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(INDEXEDRASTERDATA_TYPE_INFO);
    registry.register_type(INDEXEDRASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(TILEBIOMELIST_TYPE_INFO);
    registry.register_type(TILEBIOMELIST_ARRAY_TYPE_INFO);
    registry.register_type(BIOMESPEC_TYPE_INFO);
    registry.register_type(BIOMESPEC_ARRAY_TYPE_INFO);
    registry.register_type(FLOWMAPRASTERDATA_TYPE_INFO);
    registry.register_type(FLOWMAPRASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONDEPTHRASTERDATA_TYPE_INFO);
    registry.register_type(DESTRUCTIONDEPTHRASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONDEPTHGENERATEOPTIONS_TYPE_INFO);
    registry.register_type(DESTRUCTIONDEPTHGENERATEOPTIONS_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONDEPTHGENERATESOURCE_TYPE_INFO);
    registry.register_type(DESTRUCTIONDEPTHGENERATESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSMATERIALSRASTERDATA_TYPE_INFO);
    registry.register_type(PHYSICSMATERIALSRASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(BYTERASTERDATA_TYPE_INFO);
    registry.register_type(BYTERASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(RGBRASTERDATA_TYPE_INFO);
    registry.register_type(RGBRASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(RGBARASTERDATA_TYPE_INFO);
    registry.register_type(RGBARASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(HEIGHTFIELDRASTERDATA_TYPE_INFO);
    registry.register_type(HEIGHTFIELDRASTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(DENSITYMAP_FILTERTYPE_TYPE_INFO);
    registry.register_type(DENSITYMAP_FILTERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(VIRTUALRASTERQUADTREEDATA_TYPE_INFO);
    registry.register_type(VIRTUALRASTERQUADTREEDATA_ARRAY_TYPE_INFO);
    registry.register_type(RASTERQUADTREEDATA_TYPE_INFO);
    registry.register_type(RASTERQUADTREEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SAMPLECENTER_TYPE_INFO);
    registry.register_type(SAMPLECENTER_ARRAY_TYPE_INFO);
    registry.register_type(RASTERQUADTREENODEDATA_TYPE_INFO);
    registry.register_type(RASTERQUADTREENODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(STYLETRANSFERTEXTURE_TYPE_INFO);
    registry.register_type(STYLETRANSFERTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(OVERLAYTYPE_TYPE_INFO);
    registry.register_type(OVERLAYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTOVERLAYTYPE_TYPE_INFO);
    registry.register_type(EFFECTOVERLAYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RASTERNODEUSAGE_TYPE_INFO);
    registry.register_type(RASTERNODEUSAGE_ARRAY_TYPE_INFO);
    registry.register_type(RECTANGULARCOVERAGEDATA_TYPE_INFO);
    registry.register_type(RECTANGULARCOVERAGEDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUT_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUT_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUTOVERRIDE_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUTOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUTS_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUTS_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPAINTCONFIGS_TYPE_INFO);
    registry.register_type(AUTOPAINTCONFIGS_ARRAY_TYPE_INFO);
    registry.register_type(RASTERTYPETORASTERFORMAT_TYPE_INFO);
    registry.register_type(RASTERTYPETORASTERFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(CLASSTYPEAUTOPAINTOUTPUTSMAP_TYPE_INFO);
    registry.register_type(CLASSTYPEAUTOPAINTOUTPUTSMAP_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPAINTMESHDATA_TYPE_INFO);
    registry.register_type(AUTOPAINTMESHDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPAINTROADDATA_TYPE_INFO);
    registry.register_type(AUTOPAINTROADDATA_ARRAY_TYPE_INFO);
    registry.register_type(OUTPUTTYPE_TYPE_INFO);
    registry.register_type(OUTPUTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TPABLENDMODE_TYPE_INFO);
    registry.register_type(TPABLENDMODE_ARRAY_TYPE_INFO);
    registry.register_type(DEPTHBUFFER_TYPE_INFO);
    registry.register_type(DEPTHBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(FACECULLING_TYPE_INFO);
    registry.register_type(FACECULLING_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINSTREAMINGTREE_TYPE_INFO);
    registry.register_type(TERRAINSTREAMINGTREE_ARRAY_TYPE_INFO);
    registry.register_type(TERRAIN_TYPE_INFO);
    registry.register_type(TERRAIN_ARRAY_TYPE_INFO);
    registry.register_type(ITERRAIN_TYPE_INFO);
    registry.register_type(ITERRAIN_ARRAY_TYPE_INFO);
    registry.register_type(HEIGHTFIELDDECAL_TYPE_INFO);
    registry.register_type(HEIGHTFIELDDECAL_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct VisualTerrainDynamicState {
    pub visible: bool,
    pub draw_enable: bool,
    pub override_draw_enable: bool,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait VisualTerrainDynamicStateTrait: TypeObject {
    fn visible(&self) -> &bool;
    fn draw_enable(&self) -> &bool;
    fn override_draw_enable(&self) -> &bool;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl VisualTerrainDynamicStateTrait for VisualTerrainDynamicState {
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn override_draw_enable(&self) -> &bool {
        &self.override_draw_enable
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static VISUALTERRAINDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualTerrainDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainDynamicState, visible),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainDynamicState, draw_enable),
            },
            FieldInfoData {
                name: "OverrideDrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainDynamicState, override_draw_enable),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VisualTerrainDynamicState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VisualTerrainDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISUALTERRAINDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VisualTerrainDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALTERRAINDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALTERRAINDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualTerrainStaticState {
    pub terrain: Option<Arc<Mutex<dyn TerrainBaseAssetTrait>>>,
    pub decals_resource_handle: super::world_base::ResourceRefHandle,
    pub settings: Option<Arc<Mutex<dyn VisualTerrainBaseSettingsTrait>>>,
    pub field_flag_changed0: u8,
}

pub trait VisualTerrainStaticStateTrait: TypeObject {
    fn terrain(&self) -> &Option<Arc<Mutex<dyn TerrainBaseAssetTrait>>>;
    fn decals_resource_handle(&self) -> &super::world_base::ResourceRefHandle;
    fn settings(&self) -> &Option<Arc<Mutex<dyn VisualTerrainBaseSettingsTrait>>>;
    fn field_flag_changed0(&self) -> &u8;
}

impl VisualTerrainStaticStateTrait for VisualTerrainStaticState {
    fn terrain(&self) -> &Option<Arc<Mutex<dyn TerrainBaseAssetTrait>>> {
        &self.terrain
    }
    fn decals_resource_handle(&self) -> &super::world_base::ResourceRefHandle {
        &self.decals_resource_handle
    }
    fn settings(&self) -> &Option<Arc<Mutex<dyn VisualTerrainBaseSettingsTrait>>> {
        &self.settings
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static VISUALTERRAINSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainStaticState",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualTerrainStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Terrain",
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainBaseAsset",
                rust_offset: offset_of!(VisualTerrainStaticState, terrain),
            },
            FieldInfoData {
                name: "DecalsResourceHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRefHandle",
                rust_offset: offset_of!(VisualTerrainStaticState, decals_resource_handle),
            },
            FieldInfoData {
                name: "Settings",
                flags: MemberInfoFlags::new(0),
                field_type: "VisualTerrainBaseSettings",
                rust_offset: offset_of!(VisualTerrainStaticState, settings),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VisualTerrainStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISUALTERRAINSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualTerrainStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALTERRAINSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALTERRAINSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualTerrainHandle {
}

pub trait VisualTerrainHandleTrait: TypeObject {
}

impl VisualTerrainHandleTrait for VisualTerrainHandle {
}

pub static VISUALTERRAINHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainHandle",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualTerrainHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VISUALTERRAINHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for VisualTerrainHandle {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALTERRAINHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALTERRAINHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualTerrainBaseSettings {
    pub _glacier_base: super::core::DataContainer,
}

pub trait VisualTerrainBaseSettingsTrait: super::core::DataContainerTrait {
}

impl VisualTerrainBaseSettingsTrait for VisualTerrainBaseSettings {
}

impl super::core::DataContainerTrait for VisualTerrainBaseSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VISUALTERRAINBASESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainBaseSettings",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualTerrainBaseSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VISUALTERRAINBASESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualTerrainBaseSettings {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALTERRAINBASESETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALTERRAINBASESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainBaseSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainBaseSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainStreamingSettings {
    pub _glacier_base: super::core::DataContainer,
    pub data_load_job_count: u32,
    pub active_free_streaming_data_load_job_count: u32,
    pub load_occluder_data_enable: bool,
    pub additional_blurriness: u32,
    pub invisible_detail_reduction_factor: f32,
    pub occluded_detail_reduction_factor: f32,
    pub keep_pool_full_enable: bool,
    pub heightfield_atlas_sample_count_x_factor: u32,
    pub heightfield_atlas_sample_count_y_factor: u32,
    pub mask_atlas_sample_count_x_factor: u32,
    pub mask_atlas_sample_count_y_factor: u32,
    pub mask_additional_blurriness: u32,
    pub color_atlas_sample_count_x_factor: u32,
    pub color_atlas_sample_count_y_factor: u32,
    pub color_additional_blurriness: u32,
}

pub trait TerrainStreamingSettingsTrait: super::core::DataContainerTrait {
    fn data_load_job_count(&self) -> &u32;
    fn active_free_streaming_data_load_job_count(&self) -> &u32;
    fn load_occluder_data_enable(&self) -> &bool;
    fn additional_blurriness(&self) -> &u32;
    fn invisible_detail_reduction_factor(&self) -> &f32;
    fn occluded_detail_reduction_factor(&self) -> &f32;
    fn keep_pool_full_enable(&self) -> &bool;
    fn heightfield_atlas_sample_count_x_factor(&self) -> &u32;
    fn heightfield_atlas_sample_count_y_factor(&self) -> &u32;
    fn mask_atlas_sample_count_x_factor(&self) -> &u32;
    fn mask_atlas_sample_count_y_factor(&self) -> &u32;
    fn mask_additional_blurriness(&self) -> &u32;
    fn color_atlas_sample_count_x_factor(&self) -> &u32;
    fn color_atlas_sample_count_y_factor(&self) -> &u32;
    fn color_additional_blurriness(&self) -> &u32;
}

impl TerrainStreamingSettingsTrait for TerrainStreamingSettings {
    fn data_load_job_count(&self) -> &u32 {
        &self.data_load_job_count
    }
    fn active_free_streaming_data_load_job_count(&self) -> &u32 {
        &self.active_free_streaming_data_load_job_count
    }
    fn load_occluder_data_enable(&self) -> &bool {
        &self.load_occluder_data_enable
    }
    fn additional_blurriness(&self) -> &u32 {
        &self.additional_blurriness
    }
    fn invisible_detail_reduction_factor(&self) -> &f32 {
        &self.invisible_detail_reduction_factor
    }
    fn occluded_detail_reduction_factor(&self) -> &f32 {
        &self.occluded_detail_reduction_factor
    }
    fn keep_pool_full_enable(&self) -> &bool {
        &self.keep_pool_full_enable
    }
    fn heightfield_atlas_sample_count_x_factor(&self) -> &u32 {
        &self.heightfield_atlas_sample_count_x_factor
    }
    fn heightfield_atlas_sample_count_y_factor(&self) -> &u32 {
        &self.heightfield_atlas_sample_count_y_factor
    }
    fn mask_atlas_sample_count_x_factor(&self) -> &u32 {
        &self.mask_atlas_sample_count_x_factor
    }
    fn mask_atlas_sample_count_y_factor(&self) -> &u32 {
        &self.mask_atlas_sample_count_y_factor
    }
    fn mask_additional_blurriness(&self) -> &u32 {
        &self.mask_additional_blurriness
    }
    fn color_atlas_sample_count_x_factor(&self) -> &u32 {
        &self.color_atlas_sample_count_x_factor
    }
    fn color_atlas_sample_count_y_factor(&self) -> &u32 {
        &self.color_atlas_sample_count_y_factor
    }
    fn color_additional_blurriness(&self) -> &u32 {
        &self.color_additional_blurriness
    }
}

impl super::core::DataContainerTrait for TerrainStreamingSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINSTREAMINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingSettings",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainStreamingSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DataLoadJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, data_load_job_count),
            },
            FieldInfoData {
                name: "ActiveFreeStreamingDataLoadJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, active_free_streaming_data_load_job_count),
            },
            FieldInfoData {
                name: "LoadOccluderDataEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainStreamingSettings, load_occluder_data_enable),
            },
            FieldInfoData {
                name: "AdditionalBlurriness",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, additional_blurriness),
            },
            FieldInfoData {
                name: "InvisibleDetailReductionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainStreamingSettings, invisible_detail_reduction_factor),
            },
            FieldInfoData {
                name: "OccludedDetailReductionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainStreamingSettings, occluded_detail_reduction_factor),
            },
            FieldInfoData {
                name: "KeepPoolFullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainStreamingSettings, keep_pool_full_enable),
            },
            FieldInfoData {
                name: "HeightfieldAtlasSampleCountXFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, heightfield_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "HeightfieldAtlasSampleCountYFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, heightfield_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "MaskAtlasSampleCountXFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, mask_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "MaskAtlasSampleCountYFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, mask_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "MaskAdditionalBlurriness",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, mask_additional_blurriness),
            },
            FieldInfoData {
                name: "ColorAtlasSampleCountXFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, color_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "ColorAtlasSampleCountYFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, color_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "ColorAdditionalBlurriness",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, color_additional_blurriness),
            },
        ],
    }),
    array_type: Some(TERRAINSTREAMINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainStreamingSettings {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINSTREAMINGSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINSTREAMINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainStreamingSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainQuadDecalData {
    pub _glacier_base: VisualVectorShapeData,
    pub shader2d: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader2d_mesh_scattering_mask: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader2d_single_layer_mask: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader3d_z_only: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader2d_displacement: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub stick_to_terrain: bool,
    pub user_masks: super::core::Vec4,
    pub tangent_space_enable: bool,
    pub atlas_tile_template: Option<Arc<Mutex<dyn TerrainQuadDecalAtlasTileTemplateDataTrait>>>,
    pub atlas_tile: TerrainQuadDecalAtlasTile,
}

pub trait TerrainQuadDecalDataTrait: VisualVectorShapeDataTrait {
    fn shader2d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader2d_mesh_scattering_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader2d_single_layer_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader3d_z_only(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader2d_displacement(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn stick_to_terrain(&self) -> &bool;
    fn user_masks(&self) -> &super::core::Vec4;
    fn tangent_space_enable(&self) -> &bool;
    fn atlas_tile_template(&self) -> &Option<Arc<Mutex<dyn TerrainQuadDecalAtlasTileTemplateDataTrait>>>;
    fn atlas_tile(&self) -> &TerrainQuadDecalAtlasTile;
}

impl TerrainQuadDecalDataTrait for TerrainQuadDecalData {
    fn shader2d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d
    }
    fn shader2d_mesh_scattering_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d_mesh_scattering_mask
    }
    fn shader2d_single_layer_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d_single_layer_mask
    }
    fn shader3d_z_only(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader3d_z_only
    }
    fn shader2d_displacement(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d_displacement
    }
    fn stick_to_terrain(&self) -> &bool {
        &self.stick_to_terrain
    }
    fn user_masks(&self) -> &super::core::Vec4 {
        &self.user_masks
    }
    fn tangent_space_enable(&self) -> &bool {
        &self.tangent_space_enable
    }
    fn atlas_tile_template(&self) -> &Option<Arc<Mutex<dyn TerrainQuadDecalAtlasTileTemplateDataTrait>>> {
        &self.atlas_tile_template
    }
    fn atlas_tile(&self) -> &TerrainQuadDecalAtlasTile {
        &self.atlas_tile
    }
}

impl VisualVectorShapeDataTrait for TerrainQuadDecalData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn shader3d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        self._glacier_base.shader3d()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
}

impl super::entity::VectorShapeDataTrait for TerrainQuadDecalData {
    fn points(&self) -> &Vec<super::core::Vec3> {
        self._glacier_base.points()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
}

impl super::entity::BaseShapeDataTrait for TerrainQuadDecalData {
}

impl super::entity::BaseShapeDataBaseTrait for TerrainQuadDecalData {
}

impl super::entity::GameObjectDataTrait for TerrainQuadDecalData {
}

impl super::core::DataBusPeerTrait for TerrainQuadDecalData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for TerrainQuadDecalData {
}

impl super::core::DataContainerTrait for TerrainQuadDecalData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINQUADDECALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainQuadDecalData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shader2d",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d),
            },
            FieldInfoData {
                name: "Shader2dMeshScatteringMask",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d_mesh_scattering_mask),
            },
            FieldInfoData {
                name: "Shader2dSingleLayerMask",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d_single_layer_mask),
            },
            FieldInfoData {
                name: "Shader3dZOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainQuadDecalData, shader3d_z_only),
            },
            FieldInfoData {
                name: "Shader2dDisplacement",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d_displacement),
            },
            FieldInfoData {
                name: "StickToTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainQuadDecalData, stick_to_terrain),
            },
            FieldInfoData {
                name: "UserMasks",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(TerrainQuadDecalData, user_masks),
            },
            FieldInfoData {
                name: "TangentSpaceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainQuadDecalData, tangent_space_enable),
            },
            FieldInfoData {
                name: "AtlasTileTemplate",
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainQuadDecalAtlasTileTemplateData",
                rust_offset: offset_of!(TerrainQuadDecalData, atlas_tile_template),
            },
            FieldInfoData {
                name: "AtlasTile",
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainQuadDecalAtlasTile",
                rust_offset: offset_of!(TerrainQuadDecalData, atlas_tile),
            },
        ],
    }),
    array_type: Some(TERRAINQUADDECALDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TerrainQuadDecalData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINQUADDECALDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINQUADDECALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainQuadDecalData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainQuadDecalAtlasTileTemplateData {
    pub _glacier_base: super::core::Asset,
    pub atlas_tile: TerrainQuadDecalAtlasTile,
}

pub trait TerrainQuadDecalAtlasTileTemplateDataTrait: super::core::AssetTrait {
    fn atlas_tile(&self) -> &TerrainQuadDecalAtlasTile;
}

impl TerrainQuadDecalAtlasTileTemplateDataTrait for TerrainQuadDecalAtlasTileTemplateData {
    fn atlas_tile(&self) -> &TerrainQuadDecalAtlasTile {
        &self.atlas_tile
    }
}

impl super::core::AssetTrait for TerrainQuadDecalAtlasTileTemplateData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for TerrainQuadDecalAtlasTileTemplateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINQUADDECALATLASTILETEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTileTemplateData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainQuadDecalAtlasTileTemplateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AtlasTile",
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainQuadDecalAtlasTile",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTileTemplateData, atlas_tile),
            },
        ],
    }),
    array_type: Some(TERRAINQUADDECALATLASTILETEMPLATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainQuadDecalAtlasTileTemplateData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINQUADDECALATLASTILETEMPLATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINQUADDECALATLASTILETEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTileTemplateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainQuadDecalAtlasTileTemplateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainQuadDecalAtlasTile {
    pub tile_index_x: u32,
    pub tile_index_y: u32,
    pub tile_count_x: u32,
    pub tile_count_y: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}

pub trait TerrainQuadDecalAtlasTileTrait: TypeObject {
    fn tile_index_x(&self) -> &u32;
    fn tile_index_y(&self) -> &u32;
    fn tile_count_x(&self) -> &u32;
    fn tile_count_y(&self) -> &u32;
    fn flip_x(&self) -> &bool;
    fn flip_y(&self) -> &bool;
}

impl TerrainQuadDecalAtlasTileTrait for TerrainQuadDecalAtlasTile {
    fn tile_index_x(&self) -> &u32 {
        &self.tile_index_x
    }
    fn tile_index_y(&self) -> &u32 {
        &self.tile_index_y
    }
    fn tile_count_x(&self) -> &u32 {
        &self.tile_count_x
    }
    fn tile_count_y(&self) -> &u32 {
        &self.tile_count_y
    }
    fn flip_x(&self) -> &bool {
        &self.flip_x
    }
    fn flip_y(&self) -> &bool {
        &self.flip_y
    }
}

pub static TERRAINQUADDECALATLASTILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTile",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainQuadDecalAtlasTile as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TileIndexX",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_index_x),
            },
            FieldInfoData {
                name: "TileIndexY",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_index_y),
            },
            FieldInfoData {
                name: "TileCountX",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_count_x),
            },
            FieldInfoData {
                name: "TileCountY",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_count_y),
            },
            FieldInfoData {
                name: "FlipX",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, flip_x),
            },
            FieldInfoData {
                name: "FlipY",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, flip_y),
            },
        ],
    }),
    array_type: Some(TERRAINQUADDECALATLASTILE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TerrainQuadDecalAtlasTile {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINQUADDECALATLASTILE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINQUADDECALATLASTILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTile-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainQuadDecalAtlasTile"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainFillDecalData {
    pub _glacier_base: VisualVectorShapeData,
    pub shader2d: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader2d_mesh_scattering_mask: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader2d_single_layer_mask: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader3d_z_only: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
}

pub trait TerrainFillDecalDataTrait: VisualVectorShapeDataTrait {
    fn shader2d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader2d_mesh_scattering_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader2d_single_layer_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader3d_z_only(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
}

impl TerrainFillDecalDataTrait for TerrainFillDecalData {
    fn shader2d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d
    }
    fn shader2d_mesh_scattering_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d_mesh_scattering_mask
    }
    fn shader2d_single_layer_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d_single_layer_mask
    }
    fn shader3d_z_only(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader3d_z_only
    }
}

impl VisualVectorShapeDataTrait for TerrainFillDecalData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn shader3d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        self._glacier_base.shader3d()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
}

impl super::entity::VectorShapeDataTrait for TerrainFillDecalData {
    fn points(&self) -> &Vec<super::core::Vec3> {
        self._glacier_base.points()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
}

impl super::entity::BaseShapeDataTrait for TerrainFillDecalData {
}

impl super::entity::BaseShapeDataBaseTrait for TerrainFillDecalData {
}

impl super::entity::GameObjectDataTrait for TerrainFillDecalData {
}

impl super::core::DataBusPeerTrait for TerrainFillDecalData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for TerrainFillDecalData {
}

impl super::core::DataContainerTrait for TerrainFillDecalData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINFILLDECALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainFillDecalData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainFillDecalData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shader2d",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainFillDecalData, shader2d),
            },
            FieldInfoData {
                name: "Shader2dMeshScatteringMask",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainFillDecalData, shader2d_mesh_scattering_mask),
            },
            FieldInfoData {
                name: "Shader2dSingleLayerMask",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainFillDecalData, shader2d_single_layer_mask),
            },
            FieldInfoData {
                name: "Shader3dZOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainFillDecalData, shader3d_z_only),
            },
        ],
    }),
    array_type: Some(TERRAINFILLDECALDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainFillDecalData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINFILLDECALDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINFILLDECALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainFillDecalData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainFillDecalData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LakeData {
    pub _glacier_base: VisualVectorShapeData,
}

pub trait LakeDataTrait: VisualVectorShapeDataTrait {
}

impl LakeDataTrait for LakeData {
}

impl VisualVectorShapeDataTrait for LakeData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn shader3d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        self._glacier_base.shader3d()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
}

impl super::entity::VectorShapeDataTrait for LakeData {
    fn points(&self) -> &Vec<super::core::Vec3> {
        self._glacier_base.points()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
}

impl super::entity::BaseShapeDataTrait for LakeData {
}

impl super::entity::BaseShapeDataBaseTrait for LakeData {
}

impl super::entity::GameObjectDataTrait for LakeData {
}

impl super::core::DataBusPeerTrait for LakeData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for LakeData {
}

impl super::core::DataContainerTrait for LakeData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LAKEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LakeData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LakeData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LAKEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LakeData {
    fn type_info(&self) -> &'static TypeInfo {
        LAKEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LAKEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LakeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("LakeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RiverData {
    pub _glacier_base: RibbonData,
}

pub trait RiverDataTrait: RibbonDataTrait {
}

impl RiverDataTrait for RiverData {
}

impl RibbonDataTrait for RiverData {
    fn ribbon_points(&self) -> &Vec<RibbonPointData> {
        self._glacier_base.ribbon_points()
    }
}

impl VisualVectorShapeDataTrait for RiverData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn shader3d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        self._glacier_base.shader3d()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
}

impl super::entity::VectorShapeDataTrait for RiverData {
    fn points(&self) -> &Vec<super::core::Vec3> {
        self._glacier_base.points()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
}

impl super::entity::BaseShapeDataTrait for RiverData {
}

impl super::entity::BaseShapeDataBaseTrait for RiverData {
}

impl super::entity::GameObjectDataTrait for RiverData {
}

impl super::core::DataBusPeerTrait for RiverData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for RiverData {
}

impl super::core::DataContainerTrait for RiverData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RIVERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RiverData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RIBBONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RiverData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RIVERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RiverData {
    fn type_info(&self) -> &'static TypeInfo {
        RIVERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RIVERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RiverData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RiverData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RoadData {
    pub _glacier_base: RibbonData,
    pub shader2d: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader2d_mesh_scattering_mask: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader2d_single_layer_mask: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader3d_z_only: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub shader2d_displacement: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub stick_to_terrain: bool,
    pub uv_tile_factor: f32,
    pub tangent_space_enable: bool,
}

pub trait RoadDataTrait: RibbonDataTrait {
    fn shader2d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader2d_mesh_scattering_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader2d_single_layer_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader3d_z_only(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn shader2d_displacement(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn stick_to_terrain(&self) -> &bool;
    fn uv_tile_factor(&self) -> &f32;
    fn tangent_space_enable(&self) -> &bool;
}

impl RoadDataTrait for RoadData {
    fn shader2d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d
    }
    fn shader2d_mesh_scattering_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d_mesh_scattering_mask
    }
    fn shader2d_single_layer_mask(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d_single_layer_mask
    }
    fn shader3d_z_only(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader3d_z_only
    }
    fn shader2d_displacement(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader2d_displacement
    }
    fn stick_to_terrain(&self) -> &bool {
        &self.stick_to_terrain
    }
    fn uv_tile_factor(&self) -> &f32 {
        &self.uv_tile_factor
    }
    fn tangent_space_enable(&self) -> &bool {
        &self.tangent_space_enable
    }
}

impl RibbonDataTrait for RoadData {
    fn ribbon_points(&self) -> &Vec<RibbonPointData> {
        self._glacier_base.ribbon_points()
    }
}

impl VisualVectorShapeDataTrait for RoadData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn shader3d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        self._glacier_base.shader3d()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
}

impl super::entity::VectorShapeDataTrait for RoadData {
    fn points(&self) -> &Vec<super::core::Vec3> {
        self._glacier_base.points()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
}

impl super::entity::BaseShapeDataTrait for RoadData {
}

impl super::entity::BaseShapeDataBaseTrait for RoadData {
}

impl super::entity::GameObjectDataTrait for RoadData {
}

impl super::core::DataBusPeerTrait for RoadData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for RoadData {
}

impl super::core::DataContainerTrait for RoadData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ROADDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RoadData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RIBBONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RoadData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shader2d",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(RoadData, shader2d),
            },
            FieldInfoData {
                name: "Shader2dMeshScatteringMask",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(RoadData, shader2d_mesh_scattering_mask),
            },
            FieldInfoData {
                name: "Shader2dSingleLayerMask",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(RoadData, shader2d_single_layer_mask),
            },
            FieldInfoData {
                name: "Shader3dZOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(RoadData, shader3d_z_only),
            },
            FieldInfoData {
                name: "Shader2dDisplacement",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(RoadData, shader2d_displacement),
            },
            FieldInfoData {
                name: "StickToTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RoadData, stick_to_terrain),
            },
            FieldInfoData {
                name: "UvTileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RoadData, uv_tile_factor),
            },
            FieldInfoData {
                name: "TangentSpaceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RoadData, tangent_space_enable),
            },
        ],
    }),
    array_type: Some(ROADDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RoadData {
    fn type_info(&self) -> &'static TypeInfo {
        ROADDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ROADDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RoadData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RoadData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RibbonData {
    pub _glacier_base: VisualVectorShapeData,
    pub ribbon_points: Vec<RibbonPointData>,
}

pub trait RibbonDataTrait: VisualVectorShapeDataTrait {
    fn ribbon_points(&self) -> &Vec<RibbonPointData>;
}

impl RibbonDataTrait for RibbonData {
    fn ribbon_points(&self) -> &Vec<RibbonPointData> {
        &self.ribbon_points
    }
}

impl VisualVectorShapeDataTrait for RibbonData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn shader3d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        self._glacier_base.shader3d()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
}

impl super::entity::VectorShapeDataTrait for RibbonData {
    fn points(&self) -> &Vec<super::core::Vec3> {
        self._glacier_base.points()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
}

impl super::entity::BaseShapeDataTrait for RibbonData {
}

impl super::entity::BaseShapeDataBaseTrait for RibbonData {
}

impl super::entity::GameObjectDataTrait for RibbonData {
}

impl super::core::DataBusPeerTrait for RibbonData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for RibbonData {
}

impl super::core::DataContainerTrait for RibbonData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RIBBONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RibbonData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RibbonPoints",
                flags: MemberInfoFlags::new(144),
                field_type: "RibbonPointData-Array",
                rust_offset: offset_of!(RibbonData, ribbon_points),
            },
        ],
    }),
    array_type: Some(RIBBONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RibbonData {
    fn type_info(&self) -> &'static TypeInfo {
        RIBBONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RIBBONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RibbonData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RibbonPointData {
    pub left: f32,
    pub right: f32,
    pub user_mask_left: super::core::Vec4,
    pub user_mask_right: super::core::Vec4,
}

pub trait RibbonPointDataTrait: TypeObject {
    fn left(&self) -> &f32;
    fn right(&self) -> &f32;
    fn user_mask_left(&self) -> &super::core::Vec4;
    fn user_mask_right(&self) -> &super::core::Vec4;
}

impl RibbonPointDataTrait for RibbonPointData {
    fn left(&self) -> &f32 {
        &self.left
    }
    fn right(&self) -> &f32 {
        &self.right
    }
    fn user_mask_left(&self) -> &super::core::Vec4 {
        &self.user_mask_left
    }
    fn user_mask_right(&self) -> &super::core::Vec4 {
        &self.user_mask_right
    }
}

pub static RIBBONPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonPointData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RibbonPointData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Left",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RibbonPointData, left),
            },
            FieldInfoData {
                name: "Right",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RibbonPointData, right),
            },
            FieldInfoData {
                name: "UserMaskLeft",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(RibbonPointData, user_mask_left),
            },
            FieldInfoData {
                name: "UserMaskRight",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(RibbonPointData, user_mask_right),
            },
        ],
    }),
    array_type: Some(RIBBONPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RibbonPointData {
    fn type_info(&self) -> &'static TypeInfo {
        RIBBONPOINTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RIBBONPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonPointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RibbonPointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualVectorShapeData {
    pub _glacier_base: super::entity::VectorShapeData,
    pub error_tolerance: f32,
    pub shader3d: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub draw_order_index: u32,
    pub tessellation_triangle_size: f32,
    pub split_to_match_heightfield: bool,
}

pub trait VisualVectorShapeDataTrait: super::entity::VectorShapeDataTrait {
    fn error_tolerance(&self) -> &f32;
    fn shader3d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn draw_order_index(&self) -> &u32;
    fn tessellation_triangle_size(&self) -> &f32;
    fn split_to_match_heightfield(&self) -> &bool;
}

impl VisualVectorShapeDataTrait for VisualVectorShapeData {
    fn error_tolerance(&self) -> &f32 {
        &self.error_tolerance
    }
    fn shader3d(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader3d
    }
    fn draw_order_index(&self) -> &u32 {
        &self.draw_order_index
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        &self.tessellation_triangle_size
    }
    fn split_to_match_heightfield(&self) -> &bool {
        &self.split_to_match_heightfield
    }
}

impl super::entity::VectorShapeDataTrait for VisualVectorShapeData {
    fn points(&self) -> &Vec<super::core::Vec3> {
        self._glacier_base.points()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
}

impl super::entity::BaseShapeDataTrait for VisualVectorShapeData {
}

impl super::entity::BaseShapeDataBaseTrait for VisualVectorShapeData {
}

impl super::entity::GameObjectDataTrait for VisualVectorShapeData {
}

impl super::core::DataBusPeerTrait for VisualVectorShapeData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for VisualVectorShapeData {
}

impl super::core::DataContainerTrait for VisualVectorShapeData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VISUALVECTORSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualVectorShapeData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::VECTORSHAPEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualVectorShapeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ErrorTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualVectorShapeData, error_tolerance),
            },
            FieldInfoData {
                name: "Shader3d",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(VisualVectorShapeData, shader3d),
            },
            FieldInfoData {
                name: "DrawOrderIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualVectorShapeData, draw_order_index),
            },
            FieldInfoData {
                name: "TessellationTriangleSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualVectorShapeData, tessellation_triangle_size),
            },
            FieldInfoData {
                name: "SplitToMatchHeightfield",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualVectorShapeData, split_to_match_heightfield),
            },
        ],
    }),
    array_type: Some(VISUALVECTORSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualVectorShapeData {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALVECTORSHAPEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALVECTORSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualVectorShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualVectorShapeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainShaderParameterBlockDynamicState {
}

pub trait TerrainShaderParameterBlockDynamicStateTrait: TypeObject {
}

impl TerrainShaderParameterBlockDynamicStateTrait for TerrainShaderParameterBlockDynamicState {
}

pub static TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainShaderParameterBlockDynamicState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainShaderParameterBlockDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainShaderParameterBlockDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainShaderParameterBlockStaticState {
    pub shader_block_handle: super::render_base::ShaderParameterBlockHandle,
    pub field_flag_changed0: u8,
}

pub trait TerrainShaderParameterBlockStaticStateTrait: TypeObject {
    fn shader_block_handle(&self) -> &super::render_base::ShaderParameterBlockHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl TerrainShaderParameterBlockStaticStateTrait for TerrainShaderParameterBlockStaticState {
    fn shader_block_handle(&self) -> &super::render_base::ShaderParameterBlockHandle {
        &self.shader_block_handle
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static TERRAINSHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockStaticState",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainShaderParameterBlockStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ShaderBlockHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderParameterBlockHandle",
                rust_offset: offset_of!(TerrainShaderParameterBlockStaticState, shader_block_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TerrainShaderParameterBlockStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TERRAINSHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for TerrainShaderParameterBlockStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINSHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINSHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainShaderParameterBlockStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainSettings {
    pub _glacier_base: super::core::DataContainer,
    pub height_query_cache_size: u32,
    pub modifiers_enable: bool,
    pub modifiers_capacity: u32,
    pub intersecting_modifiers_max: u32,
    pub modifier_slope_max: f32,
    pub modifier_depth_factor: f32,
    pub modifiers_applied_per_frame_max: u32,
    pub prioritization_on_several_frames: bool,
    pub refining_during_prioritization: bool,
    pub refining_during_prioritization_min_priority: f32,
}

pub trait TerrainSettingsTrait: super::core::DataContainerTrait {
    fn height_query_cache_size(&self) -> &u32;
    fn modifiers_enable(&self) -> &bool;
    fn modifiers_capacity(&self) -> &u32;
    fn intersecting_modifiers_max(&self) -> &u32;
    fn modifier_slope_max(&self) -> &f32;
    fn modifier_depth_factor(&self) -> &f32;
    fn modifiers_applied_per_frame_max(&self) -> &u32;
    fn prioritization_on_several_frames(&self) -> &bool;
    fn refining_during_prioritization(&self) -> &bool;
    fn refining_during_prioritization_min_priority(&self) -> &f32;
}

impl TerrainSettingsTrait for TerrainSettings {
    fn height_query_cache_size(&self) -> &u32 {
        &self.height_query_cache_size
    }
    fn modifiers_enable(&self) -> &bool {
        &self.modifiers_enable
    }
    fn modifiers_capacity(&self) -> &u32 {
        &self.modifiers_capacity
    }
    fn intersecting_modifiers_max(&self) -> &u32 {
        &self.intersecting_modifiers_max
    }
    fn modifier_slope_max(&self) -> &f32 {
        &self.modifier_slope_max
    }
    fn modifier_depth_factor(&self) -> &f32 {
        &self.modifier_depth_factor
    }
    fn modifiers_applied_per_frame_max(&self) -> &u32 {
        &self.modifiers_applied_per_frame_max
    }
    fn prioritization_on_several_frames(&self) -> &bool {
        &self.prioritization_on_several_frames
    }
    fn refining_during_prioritization(&self) -> &bool {
        &self.refining_during_prioritization
    }
    fn refining_during_prioritization_min_priority(&self) -> &f32 {
        &self.refining_during_prioritization_min_priority
    }
}

impl super::core::DataContainerTrait for TerrainSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSettings",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HeightQueryCacheSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainSettings, height_query_cache_size),
            },
            FieldInfoData {
                name: "ModifiersEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainSettings, modifiers_enable),
            },
            FieldInfoData {
                name: "ModifiersCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainSettings, modifiers_capacity),
            },
            FieldInfoData {
                name: "IntersectingModifiersMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainSettings, intersecting_modifiers_max),
            },
            FieldInfoData {
                name: "ModifierSlopeMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainSettings, modifier_slope_max),
            },
            FieldInfoData {
                name: "ModifierDepthFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainSettings, modifier_depth_factor),
            },
            FieldInfoData {
                name: "ModifiersAppliedPerFrameMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainSettings, modifiers_applied_per_frame_max),
            },
            FieldInfoData {
                name: "PrioritizationOnSeveralFrames",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainSettings, prioritization_on_several_frames),
            },
            FieldInfoData {
                name: "RefiningDuringPrioritization",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainSettings, refining_during_prioritization),
            },
            FieldInfoData {
                name: "RefiningDuringPrioritizationMinPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainSettings, refining_during_prioritization_min_priority),
            },
        ],
    }),
    array_type: Some(TERRAINSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainSettings {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainModificationDynamicState {
    pub burn_map: Vec<u8>,
    pub field_flag_changed0: u8,
}

pub trait TerrainModificationDynamicStateTrait: TypeObject {
    fn burn_map(&self) -> &Vec<u8>;
    fn field_flag_changed0(&self) -> &u8;
}

impl TerrainModificationDynamicStateTrait for TerrainModificationDynamicState {
    fn burn_map(&self) -> &Vec<u8> {
        &self.burn_map
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static TERRAINMODIFICATIONDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainModificationDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BurnMap",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint8-Array",
                rust_offset: offset_of!(TerrainModificationDynamicState, burn_map),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TerrainModificationDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TERRAINMODIFICATIONDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainModificationDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINMODIFICATIONDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINMODIFICATIONDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainModificationDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TerrainModificationType {
    #[default]
    TerrainModificationType_DynamicDetail = 0,
    TerrainModificationType_DynamicExternalRaster = 1,
}

pub static TERRAINMODIFICATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINMODIFICATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainModificationType {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINMODIFICATIONTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINMODIFICATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainModificationType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainEditingDynamicState {
}

pub trait TerrainEditingDynamicStateTrait: TypeObject {
}

impl TerrainEditingDynamicStateTrait for TerrainEditingDynamicState {
}

pub static TERRAINEDITINGDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainEditingDynamicState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINEDITINGDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainEditingDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINEDITINGDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINEDITINGDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainEditingDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainEditingStaticState {
    pub event_type: TerrainEditingEvent,
    pub mesh_scattering_type: Option<Arc<Mutex<dyn TerrainMeshScatteringTypeTrait>>>,
    pub mesh_scattering_field_number: u32,
    pub decals: Vec<Option<Arc<Mutex<dyn VisualVectorShapeDataTrait>>>>,
    pub field_flag_changed0: u8,
}

pub trait TerrainEditingStaticStateTrait: TypeObject {
    fn event_type(&self) -> &TerrainEditingEvent;
    fn mesh_scattering_type(&self) -> &Option<Arc<Mutex<dyn TerrainMeshScatteringTypeTrait>>>;
    fn mesh_scattering_field_number(&self) -> &u32;
    fn decals(&self) -> &Vec<Option<Arc<Mutex<dyn VisualVectorShapeDataTrait>>>>;
    fn field_flag_changed0(&self) -> &u8;
}

impl TerrainEditingStaticStateTrait for TerrainEditingStaticState {
    fn event_type(&self) -> &TerrainEditingEvent {
        &self.event_type
    }
    fn mesh_scattering_type(&self) -> &Option<Arc<Mutex<dyn TerrainMeshScatteringTypeTrait>>> {
        &self.mesh_scattering_type
    }
    fn mesh_scattering_field_number(&self) -> &u32 {
        &self.mesh_scattering_field_number
    }
    fn decals(&self) -> &Vec<Option<Arc<Mutex<dyn VisualVectorShapeDataTrait>>>> {
        &self.decals
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static TERRAINEDITINGSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingStaticState",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainEditingStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EventType",
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainEditingEvent",
                rust_offset: offset_of!(TerrainEditingStaticState, event_type),
            },
            FieldInfoData {
                name: "MeshScatteringType",
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainMeshScatteringType",
                rust_offset: offset_of!(TerrainEditingStaticState, mesh_scattering_type),
            },
            FieldInfoData {
                name: "MeshScatteringFieldNumber",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainEditingStaticState, mesh_scattering_field_number),
            },
            FieldInfoData {
                name: "Decals",
                flags: MemberInfoFlags::new(144),
                field_type: "VisualVectorShapeData-Array",
                rust_offset: offset_of!(TerrainEditingStaticState, decals),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TerrainEditingStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TERRAINEDITINGSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainEditingStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINEDITINGSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINEDITINGSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainEditingStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TerrainEditingEvent {
    #[default]
    TerrainEditingEvent_MeshScattering = 0,
    TerrainEditingEvent_RemoveDecals = 1,
    TerrainEditingEvent_EditDecals = 2,
    TerrainEditingEvent_EditDecalsReducedDetail = 3,
}

pub static TERRAINEDITINGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingEvent",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINEDITINGEVENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainEditingEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINEDITINGEVENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINEDITINGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainEditingEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PlayablePixelsPerMeter {
    #[default]
    ppm0125 = 125,
    ppmQuarter = 250,
    ppmHalf = 500,
    ppmOne = 1000,
    ppmTwo = 2000,
    ppmFour = 4000,
    ppmEight = 8000,
}

pub static PLAYABLEPIXELSPERMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayablePixelsPerMeter",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(PLAYABLEPIXELSPERMETER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PlayablePixelsPerMeter {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYABLEPIXELSPERMETER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLAYABLEPIXELSPERMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayablePixelsPerMeter-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PlayablePixelsPerMeter"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TerrainAnchor {
    #[default]
    topLeftCorner = 0,
    top = 1,
    topRightCorner = 2,
    middleLeft = 3,
    middle = 4,
    middleRight = 5,
    bottomLeftCorner = 6,
    bottom = 7,
    bottomRightCorner = 8,
}

pub static TERRAINANCHOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainAnchor",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINANCHOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainAnchor {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINANCHOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINANCHOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainAnchor-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainAnchor"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TerrainSize {
    #[default]
    sws1024 = 1024,
    sws2048 = 2048,
    sws4096 = 4096,
    sws8192 = 8192,
    sws16384 = 16384,
    sws32768 = 32768,
    sws65536 = 65536,
    sws131072 = 131072,
}

pub static TERRAINSIZE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSize",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINSIZE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainSize {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINSIZE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINSIZE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSize-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainSize"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum HighResTerrainSize {
    #[default]
    pws512 = 512,
    pws1024 = 1024,
    pws2048 = 2048,
    pws4096 = 4096,
    pws8192 = 8192,
}

pub static HIGHRESTERRAINSIZE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HighResTerrainSize",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(HIGHRESTERRAINSIZE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for HighResTerrainSize {
    fn type_info(&self) -> &'static TypeInfo {
        HIGHRESTERRAINSIZE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HIGHRESTERRAINSIZE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HighResTerrainSize-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HighResTerrainSize"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainLayerCombinationsData {
    pub _glacier_base: super::core::Asset,
}

pub trait TerrainLayerCombinationsDataTrait: super::core::AssetTrait {
}

impl TerrainLayerCombinationsDataTrait for TerrainLayerCombinationsData {
}

impl super::core::AssetTrait for TerrainLayerCombinationsData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for TerrainLayerCombinationsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINLAYERCOMBINATIONSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationsData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerCombinationsData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERCOMBINATIONSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainLayerCombinationsData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINLAYERCOMBINATIONSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINLAYERCOMBINATIONSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerCombinationsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainLayerShaderData {
}

pub trait TerrainLayerShaderDataTrait: TypeObject {
}

impl TerrainLayerShaderDataTrait for TerrainLayerShaderData {
}

pub static TERRAINLAYERSHADERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerShaderData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerShaderData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERSHADERDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainLayerShaderData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINLAYERSHADERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINLAYERSHADERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerShaderData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerShaderData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshScatteringSpawnData {
}

pub trait MeshScatteringSpawnDataTrait: TypeObject {
}

impl MeshScatteringSpawnDataTrait for MeshScatteringSpawnData {
}

pub static MESHSCATTERINGSPAWNDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringSpawnData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshScatteringSpawnData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHSCATTERINGSPAWNDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringSpawnData {
    fn type_info(&self) -> &'static TypeInfo {
        MESHSCATTERINGSPAWNDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHSCATTERINGSPAWNDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringSpawnData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringSpawnData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainStreamingTreeAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait TerrainStreamingTreeAssetTrait: super::core::AssetTrait {
}

impl TerrainStreamingTreeAssetTrait for TerrainStreamingTreeAsset {
}

impl super::core::AssetTrait for TerrainStreamingTreeAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for TerrainStreamingTreeAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINSTREAMINGTREEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTreeAsset",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainStreamingTreeAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINSTREAMINGTREEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainStreamingTreeAsset {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINSTREAMINGTREEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINSTREAMINGTREEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTreeAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainStreamingTreeAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainLayerMaskData {
}

pub trait TerrainLayerMaskDataTrait: TypeObject {
}

impl TerrainLayerMaskDataTrait for TerrainLayerMaskData {
}

pub static TERRAINLAYERMASKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerMaskData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerMaskData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERMASKDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainLayerMaskData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINLAYERMASKDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINLAYERMASKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerMaskData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerMaskData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PathfindingMaskRasterData {
    pub _glacier_base: ByteRasterData,
}

pub trait PathfindingMaskRasterDataTrait: ByteRasterDataTrait {
}

impl PathfindingMaskRasterDataTrait for PathfindingMaskRasterData {
}

impl ByteRasterDataTrait for PathfindingMaskRasterData {
}

impl RasterQuadtreeDataTrait for PathfindingMaskRasterData {
}

impl super::core::AssetTrait for PathfindingMaskRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PathfindingMaskRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PATHFINDINGMASKRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingMaskRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingMaskRasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PATHFINDINGMASKRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingMaskRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGMASKRASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PATHFINDINGMASKRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingMaskRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PathfindingMaskRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainData {
    pub _glacier_base: TerrainBaseAsset,
    pub terrain_layers: Vec<Option<Arc<Mutex<dyn TerrainLayerDataTrait>>>>,
    pub dynamic_mask_enable: bool,
    pub detail_displacement_max_level_diff: u32,
    pub detail_displacement_indirection_texture_tile_x: u32,
    pub override_occluder_settings: bool,
    pub occluder_enable: bool,
    pub occluder_patch_faces_per_side: u32,
    pub occluder_lod_scale: f32,
    pub terrain_streaming_tree_resource: glacier_reflect::builtin::ResourceRef,
    pub visual_resource: glacier_reflect::builtin::ResourceRef,
    pub terrain_layer_combinations_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait TerrainDataTrait: TerrainBaseAssetTrait {
    fn terrain_layers(&self) -> &Vec<Option<Arc<Mutex<dyn TerrainLayerDataTrait>>>>;
    fn dynamic_mask_enable(&self) -> &bool;
    fn detail_displacement_max_level_diff(&self) -> &u32;
    fn detail_displacement_indirection_texture_tile_x(&self) -> &u32;
    fn override_occluder_settings(&self) -> &bool;
    fn occluder_enable(&self) -> &bool;
    fn occluder_patch_faces_per_side(&self) -> &u32;
    fn occluder_lod_scale(&self) -> &f32;
    fn terrain_streaming_tree_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn visual_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn terrain_layer_combinations_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl TerrainDataTrait for TerrainData {
    fn terrain_layers(&self) -> &Vec<Option<Arc<Mutex<dyn TerrainLayerDataTrait>>>> {
        &self.terrain_layers
    }
    fn dynamic_mask_enable(&self) -> &bool {
        &self.dynamic_mask_enable
    }
    fn detail_displacement_max_level_diff(&self) -> &u32 {
        &self.detail_displacement_max_level_diff
    }
    fn detail_displacement_indirection_texture_tile_x(&self) -> &u32 {
        &self.detail_displacement_indirection_texture_tile_x
    }
    fn override_occluder_settings(&self) -> &bool {
        &self.override_occluder_settings
    }
    fn occluder_enable(&self) -> &bool {
        &self.occluder_enable
    }
    fn occluder_patch_faces_per_side(&self) -> &u32 {
        &self.occluder_patch_faces_per_side
    }
    fn occluder_lod_scale(&self) -> &f32 {
        &self.occluder_lod_scale
    }
    fn terrain_streaming_tree_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.terrain_streaming_tree_resource
    }
    fn visual_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.visual_resource
    }
    fn terrain_layer_combinations_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.terrain_layer_combinations_resource
    }
}

impl TerrainBaseAssetTrait for TerrainData {
}

impl super::core::AssetTrait for TerrainData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for TerrainData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TERRAINBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TerrainLayers",
                flags: MemberInfoFlags::new(144),
                field_type: "TerrainLayerData-Array",
                rust_offset: offset_of!(TerrainData, terrain_layers),
            },
            FieldInfoData {
                name: "DynamicMaskEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainData, dynamic_mask_enable),
            },
            FieldInfoData {
                name: "DetailDisplacementMaxLevelDiff",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainData, detail_displacement_max_level_diff),
            },
            FieldInfoData {
                name: "DetailDisplacementIndirectionTextureTileX",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainData, detail_displacement_indirection_texture_tile_x),
            },
            FieldInfoData {
                name: "OverrideOccluderSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainData, override_occluder_settings),
            },
            FieldInfoData {
                name: "OccluderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainData, occluder_enable),
            },
            FieldInfoData {
                name: "OccluderPatchFacesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainData, occluder_patch_faces_per_side),
            },
            FieldInfoData {
                name: "OccluderLodScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainData, occluder_lod_scale),
            },
            FieldInfoData {
                name: "TerrainStreamingTreeResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(TerrainData, terrain_streaming_tree_resource),
            },
            FieldInfoData {
                name: "VisualResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(TerrainData, visual_resource),
            },
            FieldInfoData {
                name: "TerrainLayerCombinationsResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(TerrainData, terrain_layer_combinations_resource),
            },
        ],
    }),
    array_type: Some(TERRAINDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EnlightenMeshFilterType {
    #[default]
    EnlightenMeshFilterType_Nearest = 0,
    EnlightenMeshFilterType_Bilinear = 1,
}

pub static ENLIGHTENMESHFILTERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenMeshFilterType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(ENLIGHTENMESHFILTERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EnlightenMeshFilterType {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENMESHFILTERTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENLIGHTENMESHFILTERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenMeshFilterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("EnlightenMeshFilterType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RasterTreeBuildMode {
    #[default]
    RasterTreeBuildMode_InlinePersistentStreamRest = 0,
    RasterTreeBuildMode_InlinePersistentRemoveRest = 1,
    RasterTreeBuildMode_InlineAll = 2,
}

pub static RASTERTREEBUILDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTreeBuildMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(RASTERTREEBUILDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RasterTreeBuildMode {
    fn type_info(&self) -> &'static TypeInfo {
        RASTERTREEBUILDMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RASTERTREEBUILDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTreeBuildMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterTreeBuildMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainLayerCombinationDrawData {
}

pub trait TerrainLayerCombinationDrawDataTrait: TypeObject {
}

impl TerrainLayerCombinationDrawDataTrait for TerrainLayerCombinationDrawData {
}

pub static TERRAINLAYERCOMBINATIONDRAWDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerCombinationDrawData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERCOMBINATIONDRAWDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainLayerCombinationDrawData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINLAYERCOMBINATIONDRAWDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINLAYERCOMBINATIONDRAWDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerCombinationDrawData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Surface3dDrawMethodData {
}

pub trait Surface3dDrawMethodDataTrait: TypeObject {
}

impl Surface3dDrawMethodDataTrait for Surface3dDrawMethodData {
}

pub static SURFACE3DDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface3dDrawMethodData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Surface3dDrawMethodData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SURFACE3DDRAWMETHODDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Surface3dDrawMethodData {
    fn type_info(&self) -> &'static TypeInfo {
        SURFACE3DDRAWMETHODDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SURFACE3DDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface3dDrawMethodData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Surface3dDrawMethodData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SingleLayerMaskDrawMethodData {
}

pub trait SingleLayerMaskDrawMethodDataTrait: TypeObject {
}

impl SingleLayerMaskDrawMethodDataTrait for SingleLayerMaskDrawMethodData {
}

pub static SINGLELAYERMASKDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawMethodData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SingleLayerMaskDrawMethodData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SINGLELAYERMASKDRAWMETHODDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SingleLayerMaskDrawMethodData {
    fn type_info(&self) -> &'static TypeInfo {
        SINGLELAYERMASKDRAWMETHODDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SINGLELAYERMASKDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawMethodData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SingleLayerMaskDrawMethodData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SingleLayerMaskDrawPassData {
}

pub trait SingleLayerMaskDrawPassDataTrait: TypeObject {
}

impl SingleLayerMaskDrawPassDataTrait for SingleLayerMaskDrawPassData {
}

pub static SINGLELAYERMASKDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawPassData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SingleLayerMaskDrawPassData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SINGLELAYERMASKDRAWPASSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SingleLayerMaskDrawPassData {
    fn type_info(&self) -> &'static TypeInfo {
        SINGLELAYERMASKDRAWPASSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SINGLELAYERMASKDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawPassData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SingleLayerMaskDrawPassData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshScatteringMaskScaleDrawMethodData {
}

pub trait MeshScatteringMaskScaleDrawMethodDataTrait: TypeObject {
}

impl MeshScatteringMaskScaleDrawMethodDataTrait for MeshScatteringMaskScaleDrawMethodData {
}

pub static MESHSCATTERINGMASKSCALEDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringMaskScaleDrawMethodData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshScatteringMaskScaleDrawMethodData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHSCATTERINGMASKSCALEDRAWMETHODDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringMaskScaleDrawMethodData {
    fn type_info(&self) -> &'static TypeInfo {
        MESHSCATTERINGMASKSCALEDRAWMETHODDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHSCATTERINGMASKSCALEDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringMaskScaleDrawMethodData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringMaskScaleDrawMethodData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Displacement2dDrawMethodData {
}

pub trait Displacement2dDrawMethodDataTrait: TypeObject {
}

impl Displacement2dDrawMethodDataTrait for Displacement2dDrawMethodData {
}

pub static DISPLACEMENT2DDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawMethodData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Displacement2dDrawMethodData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DISPLACEMENT2DDRAWMETHODDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Displacement2dDrawMethodData {
    fn type_info(&self) -> &'static TypeInfo {
        DISPLACEMENT2DDRAWMETHODDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISPLACEMENT2DDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawMethodData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Displacement2dDrawMethodData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Displacement2dDrawPassData {
}

pub trait Displacement2dDrawPassDataTrait: TypeObject {
}

impl Displacement2dDrawPassDataTrait for Displacement2dDrawPassData {
}

pub static DISPLACEMENT2DDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawPassData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Displacement2dDrawPassData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DISPLACEMENT2DDRAWPASSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Displacement2dDrawPassData {
    fn type_info(&self) -> &'static TypeInfo {
        DISPLACEMENT2DDRAWPASSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISPLACEMENT2DDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawPassData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Displacement2dDrawPassData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Surface2dDrawMethodData {
}

pub trait Surface2dDrawMethodDataTrait: TypeObject {
}

impl Surface2dDrawMethodDataTrait for Surface2dDrawMethodData {
}

pub static SURFACE2DDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawMethodData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Surface2dDrawMethodData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SURFACE2DDRAWMETHODDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Surface2dDrawMethodData {
    fn type_info(&self) -> &'static TypeInfo {
        SURFACE2DDRAWMETHODDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SURFACE2DDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawMethodData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Surface2dDrawMethodData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Surface2dDrawPassData {
}

pub trait Surface2dDrawPassDataTrait: TypeObject {
}

impl Surface2dDrawPassDataTrait for Surface2dDrawPassData {
}

pub static SURFACE2DDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawPassData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Surface2dDrawPassData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SURFACE2DDRAWPASSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Surface2dDrawPassData {
    fn type_info(&self) -> &'static TypeInfo {
        SURFACE2DDRAWPASSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SURFACE2DDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawPassData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Surface2dDrawPassData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainLayerCombinationDrawPassData {
}

pub trait TerrainLayerCombinationDrawPassDataTrait: TypeObject {
}

impl TerrainLayerCombinationDrawPassDataTrait for TerrainLayerCombinationDrawPassData {
}

pub static TERRAINLAYERCOMBINATIONDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawPassData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerCombinationDrawPassData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERCOMBINATIONDRAWPASSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainLayerCombinationDrawPassData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINLAYERCOMBINATIONDRAWPASSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINLAYERCOMBINATIONDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawPassData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerCombinationDrawPassData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TerrainDrawPassType {
    #[default]
    TerrainDrawPassType_SinglePass = 0,
    TerrainDrawPassType_MultipassFirst = 1,
    TerrainDrawPassType_MultipassConsecutive = 2,
}

pub static TERRAINDRAWPASSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDrawPassType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINDRAWPASSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainDrawPassType {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINDRAWPASSTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINDRAWPASSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDrawPassType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainDrawPassType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TerrainBrushDetailOperation {
    #[default]
    Lerp = 0,
    Add = 1,
    Multiply = 2,
}

pub static TERRAINBRUSHDETAILOPERATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBrushDetailOperation",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINBRUSHDETAILOPERATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainBrushDetailOperation {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINBRUSHDETAILOPERATION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINBRUSHDETAILOPERATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBrushDetailOperation-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainBrushDetailOperation"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SingleTerrainLayerData {
    pub _glacier_base: TerrainLayerData,
    pub mesh_scattering_types: Vec<Option<Arc<Mutex<dyn TerrainMeshScatteringTypeTrait>>>>,
}

pub trait SingleTerrainLayerDataTrait: TerrainLayerDataTrait {
    fn mesh_scattering_types(&self) -> &Vec<Option<Arc<Mutex<dyn TerrainMeshScatteringTypeTrait>>>>;
}

impl SingleTerrainLayerDataTrait for SingleTerrainLayerData {
    fn mesh_scattering_types(&self) -> &Vec<Option<Arc<Mutex<dyn TerrainMeshScatteringTypeTrait>>>> {
        &self.mesh_scattering_types
    }
}

impl TerrainLayerDataTrait for SingleTerrainLayerData {
}

impl super::core::DataContainerTrait for SingleTerrainLayerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SINGLETERRAINLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleTerrainLayerData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TERRAINLAYERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SingleTerrainLayerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MeshScatteringTypes",
                flags: MemberInfoFlags::new(144),
                field_type: "TerrainMeshScatteringType-Array",
                rust_offset: offset_of!(SingleTerrainLayerData, mesh_scattering_types),
            },
        ],
    }),
    array_type: Some(SINGLETERRAINLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SingleTerrainLayerData {
    fn type_info(&self) -> &'static TypeInfo {
        SINGLETERRAINLAYERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SINGLETERRAINLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleTerrainLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SingleTerrainLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainLayerData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait TerrainLayerDataTrait: super::core::DataContainerTrait {
}

impl TerrainLayerDataTrait for TerrainLayerData {
}

impl super::core::DataContainerTrait for TerrainLayerData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainLayerData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINLAYERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ColorImportSettings {
}

pub trait ColorImportSettingsTrait: TypeObject {
}

impl ColorImportSettingsTrait for ColorImportSettings {
}

pub static COLORIMPORTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorImportSettings",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ColorImportSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(COLORIMPORTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ColorImportSettings {
    fn type_info(&self) -> &'static TypeInfo {
        COLORIMPORTSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COLORIMPORTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorImportSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ColorImportSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TerrainLayerType {
    #[default]
    TerrainLayerType_IgnoreMask = 0,
    TerrainLayerType_Masked = 1,
    TerrainLayerType_BinaryMasked = 2,
}

pub static TERRAINLAYERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINLAYERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainLayerType {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINLAYERTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINLAYERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainLayerProceduralMask {
    pub altitude_min: f32,
}

pub trait TerrainLayerProceduralMaskTrait: TypeObject {
    fn altitude_min(&self) -> &f32;
}

impl TerrainLayerProceduralMaskTrait for TerrainLayerProceduralMask {
    fn altitude_min(&self) -> &f32 {
        &self.altitude_min
    }
}

pub static TERRAINLAYERPROCEDURALMASK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerProceduralMask",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerProceduralMask as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AltitudeMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainLayerProceduralMask, altitude_min),
            },
        ],
    }),
    array_type: Some(TERRAINLAYERPROCEDURALMASK_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TerrainLayerProceduralMask {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINLAYERPROCEDURALMASK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINLAYERPROCEDURALMASK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerProceduralMask-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerProceduralMask"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainGeoTexture {
}

pub trait TerrainGeoTextureTrait: TypeObject {
}

impl TerrainGeoTextureTrait for TerrainGeoTexture {
}

pub static TERRAINGEOTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainGeoTexture",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainGeoTexture as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINGEOTEXTURE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainGeoTexture {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINGEOTEXTURE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINGEOTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainGeoTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainGeoTexture"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainMeshScatteringType {
    pub _glacier_base: super::core::DataContainer,
    pub identifier: u32,
    pub min_scale: super::core::Vec2,
    pub max_scale: super::core::Vec2,
    pub scale_randomness: f32,
    pub lod0_dissolve_out_distance_factor: f32,
    pub lod1_dissolve_in_distance_factor: f32,
    pub lod1_dissolve_out_distance_factor: f32,
    pub lod2_dissolve_in_distance_factor: f32,
    pub lod2_dissolve_out_distance_factor: f32,
    pub lod3_dissolve_in_distance_factor: f32,
    pub density: super::core::QualityScalableFloat,
    pub first_spawn_level: u32,
    pub wind_scale: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub wind_wiggle: f32,
    pub use_vertex_color_weights: bool,
    pub dissolve_range_ratio: f32,
}

pub trait TerrainMeshScatteringTypeTrait: super::core::DataContainerTrait {
    fn identifier(&self) -> &u32;
    fn min_scale(&self) -> &super::core::Vec2;
    fn max_scale(&self) -> &super::core::Vec2;
    fn scale_randomness(&self) -> &f32;
    fn lod0_dissolve_out_distance_factor(&self) -> &f32;
    fn lod1_dissolve_in_distance_factor(&self) -> &f32;
    fn lod1_dissolve_out_distance_factor(&self) -> &f32;
    fn lod2_dissolve_in_distance_factor(&self) -> &f32;
    fn lod2_dissolve_out_distance_factor(&self) -> &f32;
    fn lod3_dissolve_in_distance_factor(&self) -> &f32;
    fn density(&self) -> &super::core::QualityScalableFloat;
    fn first_spawn_level(&self) -> &u32;
    fn wind_scale(&self) -> &f32;
    fn stiffness(&self) -> &f32;
    fn damping(&self) -> &f32;
    fn mass(&self) -> &f32;
    fn wind_wiggle(&self) -> &f32;
    fn use_vertex_color_weights(&self) -> &bool;
    fn dissolve_range_ratio(&self) -> &f32;
}

impl TerrainMeshScatteringTypeTrait for TerrainMeshScatteringType {
    fn identifier(&self) -> &u32 {
        &self.identifier
    }
    fn min_scale(&self) -> &super::core::Vec2 {
        &self.min_scale
    }
    fn max_scale(&self) -> &super::core::Vec2 {
        &self.max_scale
    }
    fn scale_randomness(&self) -> &f32 {
        &self.scale_randomness
    }
    fn lod0_dissolve_out_distance_factor(&self) -> &f32 {
        &self.lod0_dissolve_out_distance_factor
    }
    fn lod1_dissolve_in_distance_factor(&self) -> &f32 {
        &self.lod1_dissolve_in_distance_factor
    }
    fn lod1_dissolve_out_distance_factor(&self) -> &f32 {
        &self.lod1_dissolve_out_distance_factor
    }
    fn lod2_dissolve_in_distance_factor(&self) -> &f32 {
        &self.lod2_dissolve_in_distance_factor
    }
    fn lod2_dissolve_out_distance_factor(&self) -> &f32 {
        &self.lod2_dissolve_out_distance_factor
    }
    fn lod3_dissolve_in_distance_factor(&self) -> &f32 {
        &self.lod3_dissolve_in_distance_factor
    }
    fn density(&self) -> &super::core::QualityScalableFloat {
        &self.density
    }
    fn first_spawn_level(&self) -> &u32 {
        &self.first_spawn_level
    }
    fn wind_scale(&self) -> &f32 {
        &self.wind_scale
    }
    fn stiffness(&self) -> &f32 {
        &self.stiffness
    }
    fn damping(&self) -> &f32 {
        &self.damping
    }
    fn mass(&self) -> &f32 {
        &self.mass
    }
    fn wind_wiggle(&self) -> &f32 {
        &self.wind_wiggle
    }
    fn use_vertex_color_weights(&self) -> &bool {
        &self.use_vertex_color_weights
    }
    fn dissolve_range_ratio(&self) -> &f32 {
        &self.dissolve_range_ratio
    }
}

impl super::core::DataContainerTrait for TerrainMeshScatteringType {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINMESHSCATTERINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainMeshScatteringType",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainMeshScatteringType as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Identifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainMeshScatteringType, identifier),
            },
            FieldInfoData {
                name: "MinScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(TerrainMeshScatteringType, min_scale),
            },
            FieldInfoData {
                name: "MaxScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(TerrainMeshScatteringType, max_scale),
            },
            FieldInfoData {
                name: "ScaleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, scale_randomness),
            },
            FieldInfoData {
                name: "Lod0DissolveOutDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod0_dissolve_out_distance_factor),
            },
            FieldInfoData {
                name: "Lod1DissolveInDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod1_dissolve_in_distance_factor),
            },
            FieldInfoData {
                name: "Lod1DissolveOutDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod1_dissolve_out_distance_factor),
            },
            FieldInfoData {
                name: "Lod2DissolveInDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod2_dissolve_in_distance_factor),
            },
            FieldInfoData {
                name: "Lod2DissolveOutDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod2_dissolve_out_distance_factor),
            },
            FieldInfoData {
                name: "Lod3DissolveInDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod3_dissolve_in_distance_factor),
            },
            FieldInfoData {
                name: "Density",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(TerrainMeshScatteringType, density),
            },
            FieldInfoData {
                name: "FirstSpawnLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainMeshScatteringType, first_spawn_level),
            },
            FieldInfoData {
                name: "WindScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, wind_scale),
            },
            FieldInfoData {
                name: "Stiffness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, stiffness),
            },
            FieldInfoData {
                name: "Damping",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, damping),
            },
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, mass),
            },
            FieldInfoData {
                name: "WindWiggle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, wind_wiggle),
            },
            FieldInfoData {
                name: "UseVertexColorWeights",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainMeshScatteringType, use_vertex_color_weights),
            },
            FieldInfoData {
                name: "DissolveRangeRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, dissolve_range_ratio),
            },
        ],
    }),
    array_type: Some(TERRAINMESHSCATTERINGTYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainMeshScatteringType {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINMESHSCATTERINGTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINMESHSCATTERINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainMeshScatteringType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainMeshScatteringType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MeshScatteringInstanceDataMode {
    #[default]
    MeshScatteringInstanceDataMode_None = 0,
    MeshScatteringInstanceDataMode_Normal = 1,
    MeshScatteringInstanceDataMode_NormalAndAtlasIndex = 2,
    MeshScatteringInstanceDataMode_NormalAndColor = 3,
    MeshScatteringInstanceDataMode_WindAndAtlasIndex = 4,
    MeshScatteringInstanceDataMode_Wind = 5,
}

pub static MESHSCATTERINGINSTANCEDATAMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringInstanceDataMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSCATTERINGINSTANCEDATAMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringInstanceDataMode {
    fn type_info(&self) -> &'static TypeInfo {
        MESHSCATTERINGINSTANCEDATAMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHSCATTERINGINSTANCEDATAMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringInstanceDataMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringInstanceDataMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UndergrowthOrientationMode {
    #[default]
    UndergrowthOrientationMode_Horizontal = 0,
    UndergrowthOrientationMode_LeanToTerrain = 1,
    UndergrowthOrientationMode_SkewToTerrain = 2,
}

pub static UNDERGROWTHORIENTATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UndergrowthOrientationMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(UNDERGROWTHORIENTATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UndergrowthOrientationMode {
    fn type_info(&self) -> &'static TypeInfo {
        UNDERGROWTHORIENTATIONMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UNDERGROWTHORIENTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UndergrowthOrientationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("UndergrowthOrientationMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MeshScatteringBillboardMode {
    #[default]
    MeshScatteringBillboardMode_VerticalAxis = 0,
    MeshScatteringBillboardMode_VerticalAxisBending = 1,
    MeshScatteringBillboardMode_NormalAxisBending = 2,
    MeshScatteringBillboardMode_Count = 3,
}

pub static MESHSCATTERINGBILLBOARDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringBillboardMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSCATTERINGBILLBOARDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringBillboardMode {
    fn type_info(&self) -> &'static TypeInfo {
        MESHSCATTERINGBILLBOARDMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHSCATTERINGBILLBOARDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringBillboardMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringBillboardMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MeshScatteringOrientationMode {
    #[default]
    MeshScatteringOrientationMode_Horizontal = 0,
    MeshScatteringOrientationMode_LeanToTerrain = 1,
    MeshScatteringOrientationMode_SkewToTerrain = 2,
}

pub static MESHSCATTERINGORIENTATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringOrientationMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSCATTERINGORIENTATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringOrientationMode {
    fn type_info(&self) -> &'static TypeInfo {
        MESHSCATTERINGORIENTATIONMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHSCATTERINGORIENTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringOrientationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringOrientationMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UndergrowthRotationMode {
    #[default]
    UndergrowthRotationMode_Random = 0,
    UndergrowthRotationMode_TowardsSlope = 1,
    UndergrowthRotationMode_Fixed = 2,
}

pub static UNDERGROWTHROTATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UndergrowthRotationMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(UNDERGROWTHROTATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UndergrowthRotationMode {
    fn type_info(&self) -> &'static TypeInfo {
        UNDERGROWTHROTATIONMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UNDERGROWTHROTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UndergrowthRotationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("UndergrowthRotationMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MeshScatteringRotationMode {
    #[default]
    MeshScatteringRotationMode_Random = 0,
    MeshScatteringRotationMode_TowardsSlope = 1,
    MeshScatteringRotationMode_Fixed = 2,
}

pub static MESHSCATTERINGROTATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringRotationMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSCATTERINGROTATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringRotationMode {
    fn type_info(&self) -> &'static TypeInfo {
        MESHSCATTERINGROTATIONMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHSCATTERINGROTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringRotationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringRotationMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MeshScatteringElevationMode {
    #[default]
    MeshScatteringElevationMode_SnapBoundingBox = 0,
    MeshScatteringElevationMode_SnapPivotPoint = 1,
}

pub static MESHSCATTERINGELEVATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringElevationMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSCATTERINGELEVATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringElevationMode {
    fn type_info(&self) -> &'static TypeInfo {
        MESHSCATTERINGELEVATIONMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHSCATTERINGELEVATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringElevationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringElevationMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainHeightfieldData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait TerrainHeightfieldDataTrait: super::core::DataContainerTrait {
}

impl TerrainHeightfieldDataTrait for TerrainHeightfieldData {
}

impl super::core::DataContainerTrait for TerrainHeightfieldData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINHEIGHTFIELDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainHeightfieldData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainHeightfieldData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINHEIGHTFIELDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainHeightfieldData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINHEIGHTFIELDDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINHEIGHTFIELDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainHeightfieldData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainHeightfieldData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TerrainBrushType {
    #[default]
    BrushType_Absolute = 0,
    BrushType_Max = 1,
    BrushType_InvertedMax = 2,
    BrushType_Min = 3,
    BrushType_InvertedMin = 4,
    BrushType_Add = 5,
    BrushType_Subtract = 6,
}

pub static TERRAINBRUSHTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBrushType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINBRUSHTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainBrushType {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINBRUSHTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINBRUSHTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBrushType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainBrushType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainDynamicDecalTemplateData {
    pub _glacier_base: super::core::Asset,
    pub width: f32,
    pub relative_width_deviation: f32,
    pub depth: f32,
    pub relative_depth_deviation: f32,
    pub rotation_random_amount: f32,
    pub slope_max: f32,
    pub depth_mask: Option<Arc<Mutex<dyn HeightfieldDecalAssetTrait>>>,
    pub mask_shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub displacement_shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub dynamic_mask_decal_width_scale: f32,
    pub tangent_space_enable: bool,
    pub scale_with_destruction_depth: bool,
    pub force_up_scale: bool,
    pub slope_min_threshold: f32,
    pub slope_scalar_max: f32,
    pub slope_multiplier: f32,
    pub max_opposing_slopes: f32,
    pub min_weight_threshold: f32,
}

pub trait TerrainDynamicDecalTemplateDataTrait: super::core::AssetTrait {
    fn width(&self) -> &f32;
    fn relative_width_deviation(&self) -> &f32;
    fn depth(&self) -> &f32;
    fn relative_depth_deviation(&self) -> &f32;
    fn rotation_random_amount(&self) -> &f32;
    fn slope_max(&self) -> &f32;
    fn depth_mask(&self) -> &Option<Arc<Mutex<dyn HeightfieldDecalAssetTrait>>>;
    fn mask_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn displacement_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn dynamic_mask_decal_width_scale(&self) -> &f32;
    fn tangent_space_enable(&self) -> &bool;
    fn scale_with_destruction_depth(&self) -> &bool;
    fn force_up_scale(&self) -> &bool;
    fn slope_min_threshold(&self) -> &f32;
    fn slope_scalar_max(&self) -> &f32;
    fn slope_multiplier(&self) -> &f32;
    fn max_opposing_slopes(&self) -> &f32;
    fn min_weight_threshold(&self) -> &f32;
}

impl TerrainDynamicDecalTemplateDataTrait for TerrainDynamicDecalTemplateData {
    fn width(&self) -> &f32 {
        &self.width
    }
    fn relative_width_deviation(&self) -> &f32 {
        &self.relative_width_deviation
    }
    fn depth(&self) -> &f32 {
        &self.depth
    }
    fn relative_depth_deviation(&self) -> &f32 {
        &self.relative_depth_deviation
    }
    fn rotation_random_amount(&self) -> &f32 {
        &self.rotation_random_amount
    }
    fn slope_max(&self) -> &f32 {
        &self.slope_max
    }
    fn depth_mask(&self) -> &Option<Arc<Mutex<dyn HeightfieldDecalAssetTrait>>> {
        &self.depth_mask
    }
    fn mask_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.mask_shader
    }
    fn displacement_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.displacement_shader
    }
    fn dynamic_mask_decal_width_scale(&self) -> &f32 {
        &self.dynamic_mask_decal_width_scale
    }
    fn tangent_space_enable(&self) -> &bool {
        &self.tangent_space_enable
    }
    fn scale_with_destruction_depth(&self) -> &bool {
        &self.scale_with_destruction_depth
    }
    fn force_up_scale(&self) -> &bool {
        &self.force_up_scale
    }
    fn slope_min_threshold(&self) -> &f32 {
        &self.slope_min_threshold
    }
    fn slope_scalar_max(&self) -> &f32 {
        &self.slope_scalar_max
    }
    fn slope_multiplier(&self) -> &f32 {
        &self.slope_multiplier
    }
    fn max_opposing_slopes(&self) -> &f32 {
        &self.max_opposing_slopes
    }
    fn min_weight_threshold(&self) -> &f32 {
        &self.min_weight_threshold
    }
}

impl super::core::AssetTrait for TerrainDynamicDecalTemplateData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for TerrainDynamicDecalTemplateData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINDYNAMICDECALTEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDynamicDecalTemplateData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainDynamicDecalTemplateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, width),
            },
            FieldInfoData {
                name: "RelativeWidthDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, relative_width_deviation),
            },
            FieldInfoData {
                name: "Depth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, depth),
            },
            FieldInfoData {
                name: "RelativeDepthDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, relative_depth_deviation),
            },
            FieldInfoData {
                name: "RotationRandomAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, rotation_random_amount),
            },
            FieldInfoData {
                name: "SlopeMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_max),
            },
            FieldInfoData {
                name: "DepthMask",
                flags: MemberInfoFlags::new(0),
                field_type: "HeightfieldDecalAsset",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, depth_mask),
            },
            FieldInfoData {
                name: "MaskShader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, mask_shader),
            },
            FieldInfoData {
                name: "DisplacementShader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, displacement_shader),
            },
            FieldInfoData {
                name: "DynamicMaskDecalWidthScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, dynamic_mask_decal_width_scale),
            },
            FieldInfoData {
                name: "TangentSpaceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, tangent_space_enable),
            },
            FieldInfoData {
                name: "ScaleWithDestructionDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, scale_with_destruction_depth),
            },
            FieldInfoData {
                name: "ForceUpScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, force_up_scale),
            },
            FieldInfoData {
                name: "SlopeMinThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_min_threshold),
            },
            FieldInfoData {
                name: "SlopeScalarMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_scalar_max),
            },
            FieldInfoData {
                name: "SlopeMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_multiplier),
            },
            FieldInfoData {
                name: "MaxOpposingSlopes",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, max_opposing_slopes),
            },
            FieldInfoData {
                name: "MinWeightThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, min_weight_threshold),
            },
        ],
    }),
    array_type: Some(TERRAINDYNAMICDECALTEMPLATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainDynamicDecalTemplateData {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINDYNAMICDECALTEMPLATEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINDYNAMICDECALTEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDynamicDecalTemplateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainDynamicDecalTemplateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct HeightfieldDecalAsset {
    pub _glacier_base: super::core::Asset,
    pub resource: glacier_reflect::builtin::ResourceRef,
    pub mid_point128: bool,
}

pub trait HeightfieldDecalAssetTrait: super::core::AssetTrait {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn mid_point128(&self) -> &bool;
}

impl HeightfieldDecalAssetTrait for HeightfieldDecalAsset {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
    fn mid_point128(&self) -> &bool {
        &self.mid_point128
    }
}

impl super::core::AssetTrait for HeightfieldDecalAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for HeightfieldDecalAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static HEIGHTFIELDDECALASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecalAsset",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HeightfieldDecalAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(HeightfieldDecalAsset, resource),
            },
            FieldInfoData {
                name: "MidPoint128",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HeightfieldDecalAsset, mid_point128),
            },
        ],
    }),
    array_type: Some(HEIGHTFIELDDECALASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HeightfieldDecalAsset {
    fn type_info(&self) -> &'static TypeInfo {
        HEIGHTFIELDDECALASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HEIGHTFIELDDECALASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecalAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HeightfieldDecalAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait TerrainBaseAssetTrait: super::core::AssetTrait {
}

impl TerrainBaseAssetTrait for TerrainBaseAsset {
}

impl super::core::AssetTrait for TerrainBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for TerrainBaseAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TERRAINBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINBASEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PathfindingRasterData {
    pub _glacier_base: RGBRasterData,
}

pub trait PathfindingRasterDataTrait: RGBRasterDataTrait {
}

impl PathfindingRasterDataTrait for PathfindingRasterData {
}

impl RGBRasterDataTrait for PathfindingRasterData {
}

impl RasterQuadtreeDataTrait for PathfindingRasterData {
}

impl super::core::AssetTrait for PathfindingRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PathfindingRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PATHFINDINGRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RGBRASTERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingRasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PATHFINDINGRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGRASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PATHFINDINGRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PathfindingRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RasterCoverageData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait RasterCoverageDataTrait: super::core::DataContainerTrait {
}

impl RasterCoverageDataTrait for RasterCoverageData {
}

impl super::core::DataContainerTrait for RasterCoverageData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RASTERCOVERAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterCoverageData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RasterCoverageData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RASTERCOVERAGEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RasterCoverageData {
    fn type_info(&self) -> &'static TypeInfo {
        RASTERCOVERAGEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RASTERCOVERAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterCoverageData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterCoverageData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DensityMapRasterData {
    pub _glacier_base: ByteRasterData,
}

pub trait DensityMapRasterDataTrait: ByteRasterDataTrait {
}

impl DensityMapRasterDataTrait for DensityMapRasterData {
}

impl ByteRasterDataTrait for DensityMapRasterData {
}

impl RasterQuadtreeDataTrait for DensityMapRasterData {
}

impl super::core::AssetTrait for DensityMapRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for DensityMapRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DENSITYMAPRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMapRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DensityMapRasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DENSITYMAPRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DensityMapRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        DENSITYMAPRASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DENSITYMAPRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMapRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DensityMapRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BiomeRasterData {
    pub _glacier_base: IndexedRasterData,
    pub biomes: Vec<BiomeSpec>,
}

pub trait BiomeRasterDataTrait: IndexedRasterDataTrait {
    fn biomes(&self) -> &Vec<BiomeSpec>;
}

impl BiomeRasterDataTrait for BiomeRasterData {
    fn biomes(&self) -> &Vec<BiomeSpec> {
        &self.biomes
    }
}

impl IndexedRasterDataTrait for BiomeRasterData {
}

impl ByteRasterDataTrait for BiomeRasterData {
}

impl RasterQuadtreeDataTrait for BiomeRasterData {
}

impl super::core::AssetTrait for BiomeRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for BiomeRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static BIOMERASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(INDEXEDRASTERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BiomeRasterData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Biomes",
                flags: MemberInfoFlags::new(144),
                field_type: "BiomeSpec-Array",
                rust_offset: offset_of!(BiomeRasterData, biomes),
            },
        ],
    }),
    array_type: Some(BIOMERASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BiomeRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        BIOMERASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BIOMERASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("BiomeRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IndexedRasterData {
    pub _glacier_base: ByteRasterData,
}

pub trait IndexedRasterDataTrait: ByteRasterDataTrait {
}

impl IndexedRasterDataTrait for IndexedRasterData {
}

impl ByteRasterDataTrait for IndexedRasterData {
}

impl RasterQuadtreeDataTrait for IndexedRasterData {
}

impl super::core::AssetTrait for IndexedRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for IndexedRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static INDEXEDRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexedRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IndexedRasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INDEXEDRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IndexedRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        INDEXEDRASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INDEXEDRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexedRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("IndexedRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TileBiomeList {
    pub hash: i32,
    pub biomes: Vec<u8>,
}

pub trait TileBiomeListTrait: TypeObject {
    fn hash(&self) -> &i32;
    fn biomes(&self) -> &Vec<u8>;
}

impl TileBiomeListTrait for TileBiomeList {
    fn hash(&self) -> &i32 {
        &self.hash
    }
    fn biomes(&self) -> &Vec<u8> {
        &self.biomes
    }
}

pub static TILEBIOMELIST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TileBiomeList",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TileBiomeList as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Hash",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TileBiomeList, hash),
            },
            FieldInfoData {
                name: "Biomes",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint8-Array",
                rust_offset: offset_of!(TileBiomeList, biomes),
            },
        ],
    }),
    array_type: Some(TILEBIOMELIST_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TileBiomeList {
    fn type_info(&self) -> &'static TypeInfo {
        TILEBIOMELIST_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TILEBIOMELIST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TileBiomeList-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TileBiomeList"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BiomeSpec {
    pub name: String,
    pub value: u32,
}

pub trait BiomeSpecTrait: TypeObject {
    fn name(&self) -> &String;
    fn value(&self) -> &u32;
}

impl BiomeSpecTrait for BiomeSpec {
    fn name(&self) -> &String {
        &self.name
    }
    fn value(&self) -> &u32 {
        &self.value
    }
}

pub static BIOMESPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeSpec",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BiomeSpec as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BiomeSpec, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(BiomeSpec, value),
            },
        ],
    }),
    array_type: Some(BIOMESPEC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BiomeSpec {
    fn type_info(&self) -> &'static TypeInfo {
        BIOMESPEC_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BIOMESPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("BiomeSpec"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FlowMapRasterData {
    pub _glacier_base: ByteRasterData,
}

pub trait FlowMapRasterDataTrait: ByteRasterDataTrait {
}

impl FlowMapRasterDataTrait for FlowMapRasterData {
}

impl ByteRasterDataTrait for FlowMapRasterData {
}

impl RasterQuadtreeDataTrait for FlowMapRasterData {
}

impl super::core::AssetTrait for FlowMapRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for FlowMapRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FLOWMAPRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlowMapRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FlowMapRasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FLOWMAPRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FlowMapRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        FLOWMAPRASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOWMAPRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlowMapRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("FlowMapRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionDepthRasterData {
    pub _glacier_base: ByteRasterData,
}

pub trait DestructionDepthRasterDataTrait: ByteRasterDataTrait {
}

impl DestructionDepthRasterDataTrait for DestructionDepthRasterData {
}

impl ByteRasterDataTrait for DestructionDepthRasterData {
}

impl RasterQuadtreeDataTrait for DestructionDepthRasterData {
}

impl super::core::AssetTrait for DestructionDepthRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for DestructionDepthRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DESTRUCTIONDEPTHRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionDepthRasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONDEPTHRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionDepthRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONDEPTHRASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONDEPTHRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DestructionDepthRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionDepthGenerateOptions {
    pub _glacier_base: super::core::DataContainer,
}

pub trait DestructionDepthGenerateOptionsTrait: super::core::DataContainerTrait {
}

impl DestructionDepthGenerateOptionsTrait for DestructionDepthGenerateOptions {
}

impl super::core::DataContainerTrait for DestructionDepthGenerateOptions {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DESTRUCTIONDEPTHGENERATEOPTIONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateOptions",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionDepthGenerateOptions as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONDEPTHGENERATEOPTIONS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionDepthGenerateOptions {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONDEPTHGENERATEOPTIONS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONDEPTHGENERATEOPTIONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateOptions-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DestructionDepthGenerateOptions"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DestructionDepthGenerateSource {
    #[default]
    DestructionDepthGenerateSource_None = 0,
    DestructionDepthGenerateSource_Collisions = 1,
    DestructionDepthGenerateSource_PhysicsMaterials = 2,
    DestructionDepthGenerateSource_Oceans = 3,
    DestructionDepthGenerateSource_DestructionMasks = 4,
}

pub static DESTRUCTIONDEPTHGENERATESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateSource",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(DESTRUCTIONDEPTHGENERATESOURCE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DestructionDepthGenerateSource {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONDEPTHGENERATESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONDEPTHGENERATESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateSource-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DestructionDepthGenerateSource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsMaterialsRasterData {
    pub _glacier_base: RasterQuadtreeData,
}

pub trait PhysicsMaterialsRasterDataTrait: RasterQuadtreeDataTrait {
}

impl PhysicsMaterialsRasterDataTrait for PhysicsMaterialsRasterData {
}

impl RasterQuadtreeDataTrait for PhysicsMaterialsRasterData {
}

impl super::core::AssetTrait for PhysicsMaterialsRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PhysicsMaterialsRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PHYSICSMATERIALSRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsMaterialsRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsMaterialsRasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSMATERIALSRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsMaterialsRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSMATERIALSRASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSMATERIALSRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsMaterialsRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PhysicsMaterialsRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ByteRasterData {
    pub _glacier_base: RasterQuadtreeData,
}

pub trait ByteRasterDataTrait: RasterQuadtreeDataTrait {
}

impl ByteRasterDataTrait for ByteRasterData {
}

impl RasterQuadtreeDataTrait for ByteRasterData {
}

impl super::core::AssetTrait for ByteRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for ByteRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static BYTERASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ByteRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ByteRasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BYTERASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ByteRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        BYTERASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BYTERASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ByteRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ByteRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RGBRasterData {
    pub _glacier_base: RasterQuadtreeData,
}

pub trait RGBRasterDataTrait: RasterQuadtreeDataTrait {
}

impl RGBRasterDataTrait for RGBRasterData {
}

impl RasterQuadtreeDataTrait for RGBRasterData {
}

impl super::core::AssetTrait for RGBRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for RGBRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RGBRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RGBRasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RGBRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RGBRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        RGBRASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RGBRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RGBRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RGBARasterData {
    pub _glacier_base: RasterQuadtreeData,
}

pub trait RGBARasterDataTrait: RasterQuadtreeDataTrait {
}

impl RGBARasterDataTrait for RGBARasterData {
}

impl RasterQuadtreeDataTrait for RGBARasterData {
}

impl super::core::AssetTrait for RGBARasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for RGBARasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RGBARASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBARasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RGBARasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RGBARASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RGBARasterData {
    fn type_info(&self) -> &'static TypeInfo {
        RGBARASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RGBARASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBARasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RGBARasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct HeightfieldRasterData {
    pub _glacier_base: RasterQuadtreeData,
}

pub trait HeightfieldRasterDataTrait: RasterQuadtreeDataTrait {
}

impl HeightfieldRasterDataTrait for HeightfieldRasterData {
}

impl RasterQuadtreeDataTrait for HeightfieldRasterData {
}

impl super::core::AssetTrait for HeightfieldRasterData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for HeightfieldRasterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static HEIGHTFIELDRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HeightfieldRasterData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(HEIGHTFIELDRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HeightfieldRasterData {
    fn type_info(&self) -> &'static TypeInfo {
        HEIGHTFIELDRASTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HEIGHTFIELDRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HeightfieldRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DensityMap_FilterType {
    #[default]
    DensityMapFilter_SecondOrderDifference = 0,
    DensityMapFilter_GaussianCurvature = 1,
    DensityMapFilter_MeanCurvature = 2,
    DensityMapFilter_LaplaceBeltrami = 3,
    DensityMapFilter_LaplaceBeltramiNoVoronoi = 4,
}

pub static DENSITYMAP_FILTERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMap_FilterType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(DENSITYMAP_FILTERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DensityMap_FilterType {
    fn type_info(&self) -> &'static TypeInfo {
        DENSITYMAP_FILTERTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DENSITYMAP_FILTERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMap_FilterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DensityMap_FilterType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VirtualRasterQuadtreeData {
    pub _glacier_base: RasterQuadtreeData,
}

pub trait VirtualRasterQuadtreeDataTrait: RasterQuadtreeDataTrait {
}

impl VirtualRasterQuadtreeDataTrait for VirtualRasterQuadtreeData {
}

impl RasterQuadtreeDataTrait for VirtualRasterQuadtreeData {
}

impl super::core::AssetTrait for VirtualRasterQuadtreeData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for VirtualRasterQuadtreeData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VIRTUALRASTERQUADTREEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VirtualRasterQuadtreeData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VirtualRasterQuadtreeData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VIRTUALRASTERQUADTREEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VirtualRasterQuadtreeData {
    fn type_info(&self) -> &'static TypeInfo {
        VIRTUALRASTERQUADTREEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VIRTUALRASTERQUADTREEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VirtualRasterQuadtreeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VirtualRasterQuadtreeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RasterQuadtreeData {
    pub _glacier_base: super::core::Asset,
}

pub trait RasterQuadtreeDataTrait: super::core::AssetTrait {
}

impl RasterQuadtreeDataTrait for RasterQuadtreeData {
}

impl super::core::AssetTrait for RasterQuadtreeData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for RasterQuadtreeData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RASTERQUADTREEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RasterQuadtreeData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RASTERQUADTREEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RasterQuadtreeData {
    fn type_info(&self) -> &'static TypeInfo {
        RASTERQUADTREEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RASTERQUADTREEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterQuadtreeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SampleCenter {
    #[default]
    SampleCenter_Center = 0,
    SampleCenter_TopLeft = 1,
}

pub static SAMPLECENTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SampleCenter",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(SAMPLECENTER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SampleCenter {
    fn type_info(&self) -> &'static TypeInfo {
        SAMPLECENTER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SAMPLECENTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SampleCenter-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SampleCenter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RasterQuadtreeNodeData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait RasterQuadtreeNodeDataTrait: super::core::DataContainerTrait {
}

impl RasterQuadtreeNodeDataTrait for RasterQuadtreeNodeData {
}

impl super::core::DataContainerTrait for RasterQuadtreeNodeData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RASTERQUADTREENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeNodeData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RasterQuadtreeNodeData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RASTERQUADTREENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RasterQuadtreeNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        RASTERQUADTREENODEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RASTERQUADTREENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterQuadtreeNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum StyleTransferTexture {
    #[default]
    StyleTransferTexture_CedarMountain = 0,
    StyleTransferTexture_Canyon = 1,
    StyleTransferTexture_Glacier = 2,
    StyleTransferTexture_Custom = 3,
}

pub static STYLETRANSFERTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StyleTransferTexture",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(STYLETRANSFERTEXTURE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StyleTransferTexture {
    fn type_info(&self) -> &'static TypeInfo {
        STYLETRANSFERTEXTURE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STYLETRANSFERTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StyleTransferTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("StyleTransferTexture"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum OverlayType {
    #[default]
    OverlayType_Base = 0,
    OverlayType_Effect = 1,
    OverlayType_Autopaint = 2,
    OverlayType_Paint = 3,
    OverlayType_Folder = 4,
    OverlayType_Synthetic = 5,
}

pub static OVERLAYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OverlayType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(OVERLAYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OverlayType {
    fn type_info(&self) -> &'static TypeInfo {
        OVERLAYTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OVERLAYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OverlayType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("OverlayType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EffectOverlayType {
    #[default]
    EffectOverlayType_Filter = 0,
    EffectOverlayType_Generator = 1,
}

pub static EFFECTOVERLAYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectOverlayType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(EFFECTOVERLAYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EffectOverlayType {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTOVERLAYTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTOVERLAYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectOverlayType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("EffectOverlayType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RasterNodeUsage {
    #[default]
    RasterNodeUsage_Default = 0,
    RasterNodeUsage_Disabled = 1,
    RasterNodeUsage_Persistent = 2,
    RasterNodeUsage_PersistentDedicatedServer = 3,
    RasterNodeUsage_Skipped = 4,
}

pub static RASTERNODEUSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterNodeUsage",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(RASTERNODEUSAGE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RasterNodeUsage {
    fn type_info(&self) -> &'static TypeInfo {
        RASTERNODEUSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RASTERNODEUSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterNodeUsage-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterNodeUsage"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RectangularCoverageData {
}

pub trait RectangularCoverageDataTrait: TypeObject {
}

impl RectangularCoverageDataTrait for RectangularCoverageData {
}

pub static RECTANGULARCOVERAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RectangularCoverageData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RectangularCoverageData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RECTANGULARCOVERAGEDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RectangularCoverageData {
    fn type_info(&self) -> &'static TypeInfo {
        RECTANGULARCOVERAGEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RECTANGULARCOVERAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RectangularCoverageData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RectangularCoverageData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutopaintOutput {
    pub _glacier_base: super::entity::AutopaintOutputBase,
}

pub trait AutopaintOutputTrait: super::entity::AutopaintOutputBaseTrait {
}

impl AutopaintOutputTrait for AutopaintOutput {
}

impl super::entity::AutopaintOutputBaseTrait for AutopaintOutput {
}

impl super::core::AssetTrait for AutopaintOutput {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for AutopaintOutput {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static AUTOPAINTOUTPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutput",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::AUTOPAINTOUTPUTBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutopaintOutput as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTOUTPUT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutopaintOutput {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPAINTOUTPUT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUTOPAINTOUTPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutput-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintOutput"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutopaintOutputOverride {
    pub _glacier_base: super::entity::AutopaintOutputOverrideBase,
}

pub trait AutopaintOutputOverrideTrait: super::entity::AutopaintOutputOverrideBaseTrait {
}

impl AutopaintOutputOverrideTrait for AutopaintOutputOverride {
}

impl super::entity::AutopaintOutputOverrideBaseTrait for AutopaintOutputOverride {
}

impl super::core::DataContainerTrait for AutopaintOutputOverride {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static AUTOPAINTOUTPUTOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputOverride",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::AUTOPAINTOUTPUTOVERRIDEBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutopaintOutputOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTOUTPUTOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutopaintOutputOverride {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPAINTOUTPUTOVERRIDE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUTOPAINTOUTPUTOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintOutputOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutopaintOutputs {
    pub _glacier_base: super::entity::AutopaintOutputsBase,
}

pub trait AutopaintOutputsTrait: super::entity::AutopaintOutputsBaseTrait {
}

impl AutopaintOutputsTrait for AutopaintOutputs {
}

impl super::entity::AutopaintOutputsBaseTrait for AutopaintOutputs {
    fn outputs(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::AutopaintOutputBaseTrait>>>> {
        self._glacier_base.outputs()
    }
}

impl super::core::AssetTrait for AutopaintOutputs {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for AutopaintOutputs {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static AUTOPAINTOUTPUTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputs",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::AUTOPAINTOUTPUTSBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutopaintOutputs as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTOUTPUTS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutopaintOutputs {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPAINTOUTPUTS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUTOPAINTOUTPUTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputs-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintOutputs"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutopaintConfigs {
    pub _glacier_base: super::core::Asset,
}

pub trait AutopaintConfigsTrait: super::core::AssetTrait {
}

impl AutopaintConfigsTrait for AutopaintConfigs {
}

impl super::core::AssetTrait for AutopaintConfigs {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for AutopaintConfigs {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static AUTOPAINTCONFIGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintConfigs",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutopaintConfigs as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTCONFIGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutopaintConfigs {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPAINTCONFIGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUTOPAINTCONFIGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintConfigs-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintConfigs"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RasterTypeToRasterFormat {
    pub raster_type: super::entity::RasterType,
    pub raster_format: super::entity::RasterFormat,
}

pub trait RasterTypeToRasterFormatTrait: TypeObject {
    fn raster_type(&self) -> &super::entity::RasterType;
    fn raster_format(&self) -> &super::entity::RasterFormat;
}

impl RasterTypeToRasterFormatTrait for RasterTypeToRasterFormat {
    fn raster_type(&self) -> &super::entity::RasterType {
        &self.raster_type
    }
    fn raster_format(&self) -> &super::entity::RasterFormat {
        &self.raster_format
    }
}

pub static RASTERTYPETORASTERFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTypeToRasterFormat",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RasterTypeToRasterFormat as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RasterType",
                flags: MemberInfoFlags::new(0),
                field_type: "RasterType",
                rust_offset: offset_of!(RasterTypeToRasterFormat, raster_type),
            },
            FieldInfoData {
                name: "RasterFormat",
                flags: MemberInfoFlags::new(0),
                field_type: "RasterFormat",
                rust_offset: offset_of!(RasterTypeToRasterFormat, raster_format),
            },
        ],
    }),
    array_type: Some(RASTERTYPETORASTERFORMAT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RasterTypeToRasterFormat {
    fn type_info(&self) -> &'static TypeInfo {
        RASTERTYPETORASTERFORMAT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RASTERTYPETORASTERFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTypeToRasterFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterTypeToRasterFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClassTypeAutopaintOutputsMap {
    pub class_type: String,
    pub autopaint_outputs: Option<Arc<Mutex<dyn AutopaintOutputsTrait>>>,
}

pub trait ClassTypeAutopaintOutputsMapTrait: TypeObject {
    fn class_type(&self) -> &String;
    fn autopaint_outputs(&self) -> &Option<Arc<Mutex<dyn AutopaintOutputsTrait>>>;
}

impl ClassTypeAutopaintOutputsMapTrait for ClassTypeAutopaintOutputsMap {
    fn class_type(&self) -> &String {
        &self.class_type
    }
    fn autopaint_outputs(&self) -> &Option<Arc<Mutex<dyn AutopaintOutputsTrait>>> {
        &self.autopaint_outputs
    }
}

pub static CLASSTYPEAUTOPAINTOUTPUTSMAP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassTypeAutopaintOutputsMap",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClassTypeAutopaintOutputsMap as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ClassType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ClassTypeAutopaintOutputsMap, class_type),
            },
            FieldInfoData {
                name: "AutopaintOutputs",
                flags: MemberInfoFlags::new(0),
                field_type: "AutopaintOutputs",
                rust_offset: offset_of!(ClassTypeAutopaintOutputsMap, autopaint_outputs),
            },
        ],
    }),
    array_type: Some(CLASSTYPEAUTOPAINTOUTPUTSMAP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClassTypeAutopaintOutputsMap {
    fn type_info(&self) -> &'static TypeInfo {
        CLASSTYPEAUTOPAINTOUTPUTSMAP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLASSTYPEAUTOPAINTOUTPUTSMAP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassTypeAutopaintOutputsMap-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ClassTypeAutopaintOutputsMap"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPaintMeshData {
}

pub trait AutoPaintMeshDataTrait: TypeObject {
}

impl AutoPaintMeshDataTrait for AutoPaintMeshData {
}

pub static AUTOPAINTMESHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintMeshData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPaintMeshData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTMESHDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AutoPaintMeshData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPAINTMESHDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUTOPAINTMESHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintMeshData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutoPaintMeshData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPaintRoadData {
}

pub trait AutoPaintRoadDataTrait: TypeObject {
}

impl AutoPaintRoadDataTrait for AutoPaintRoadData {
}

pub static AUTOPAINTROADDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintRoadData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPaintRoadData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTROADDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AutoPaintRoadData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPAINTROADDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUTOPAINTROADDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintRoadData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutoPaintRoadData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum OutputType {
    #[default]
    OutputType_Mesh = 0,
    OutputType_Quad = 1,
    OutputType_Count = 2,
}

pub static OUTPUTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutputType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(OUTPUTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OutputType {
    fn type_info(&self) -> &'static TypeInfo {
        OUTPUTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OUTPUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutputType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("OutputType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TPABlendMode {
    #[default]
    BlendMode_Normal = 0,
    BlendMode_Multiply = 1,
    BlendMode_Add = 2,
    BlendMode_Sub = 3,
    BlendMode_Screen = 4,
    BlendMode_Overlay = 5,
    BlendMode_Darken = 6,
    BlendMode_Lighten = 7,
    BlendMode_ColorDodge = 8,
    BlendMode_ColorBurn = 9,
    BlendMode_HardLight = 10,
    BlendMode_SoftLight = 11,
    BlendMode_Exclusion = 12,
    BlendMode_SmoothMin = 13,
    BlendMode_SmoothMax = 14,
    BlendMode_Overwrite = 15,
    BlendMode_Count = 16,
}

pub static TPABLENDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TPABlendMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TPABLENDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TPABlendMode {
    fn type_info(&self) -> &'static TypeInfo {
        TPABLENDMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TPABLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TPABlendMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TPABlendMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DepthBuffer {
    #[default]
    DepthBuffer_Disabled = 0,
    DepthBuffer_Increasing = 1,
    DepthBuffer_Decreasing = 2,
}

pub static DEPTHBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DepthBuffer",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(DEPTHBUFFER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DepthBuffer {
    fn type_info(&self) -> &'static TypeInfo {
        DEPTHBUFFER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEPTHBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DepthBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DepthBuffer"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum FaceCulling {
    #[default]
    FaceCulling_Back = 0,
    FaceCulling_Front = 1,
    FaceCulling_None = 2,
    FaceCulling_Count = 3,
}

pub static FACECULLING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceCulling",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(FACECULLING_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FaceCulling {
    fn type_info(&self) -> &'static TypeInfo {
        FACECULLING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FACECULLING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceCulling-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("FaceCulling"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainStreamingTree {
}

pub trait TerrainStreamingTreeTrait: TypeObject {
}

impl TerrainStreamingTreeTrait for TerrainStreamingTree {
}

pub static TERRAINSTREAMINGTREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTree",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainStreamingTree as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINSTREAMINGTREE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TerrainStreamingTree {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINSTREAMINGTREE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINSTREAMINGTREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTree-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainStreamingTree"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Terrain {
    pub _glacier_base: ITerrain,
}

pub trait TerrainTrait: ITerrainTrait {
}

impl TerrainTrait for Terrain {
}

impl ITerrainTrait for Terrain {
}

pub static TERRAIN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Terrain",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ITERRAIN_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Terrain as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAIN_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Terrain {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAIN_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAIN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Terrain-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Terrain"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ITerrain {
}

pub trait ITerrainTrait: TypeObject {
}

impl ITerrainTrait for ITerrain {
}

pub static ITERRAIN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITerrain",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ITerrain as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ITERRAIN_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ITerrain {
    fn type_info(&self) -> &'static TypeInfo {
        ITERRAIN_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ITERRAIN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITerrain-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ITerrain"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct HeightfieldDecal {
}

pub trait HeightfieldDecalTrait: TypeObject {
}

impl HeightfieldDecalTrait for HeightfieldDecal {
}

pub static HEIGHTFIELDDECAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecal",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HeightfieldDecal as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(HEIGHTFIELDDECAL_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HeightfieldDecal {
    fn type_info(&self) -> &'static TypeInfo {
        HEIGHTFIELDDECAL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HEIGHTFIELDDECAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecal-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HeightfieldDecal"),
    array_type: None,
    alignment: 8,
};


