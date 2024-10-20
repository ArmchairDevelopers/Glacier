use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VisualTerrainDynamicState {
    pub visible: bool,
    pub draw_enable: bool,
    pub override_draw_enable: bool,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const VISUALTERRAINDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainDynamicState, visible),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainDynamicState, draw_enable),
            },
            FieldInfoData {
                name: "OverrideDrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainDynamicState, override_draw_enable),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainDynamicState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISUALTERRAINDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VisualTerrainDynamicState {
    fn type_info() -> &'static TypeInfo {
        VISUALTERRAINDYNAMICSTATE_TYPE_INFO
    }
}


pub const VISUALTERRAINDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VisualTerrainStaticState {
    pub terrain: TerrainBaseAsset,
    pub decals_resource_handle: super::world_base::ResourceRefHandle,
    pub settings: VisualTerrainBaseSettings,
    pub field_flag_changed0: u8,
}

pub const VISUALTERRAINSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainStaticState",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Terrain",
                flags: MemberInfoFlags::new(0),
                field_type: TERRAINBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainStaticState, terrain),
            },
            FieldInfoData {
                name: "DecalsResourceHandle",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREFHANDLE_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainStaticState, decals_resource_handle),
            },
            FieldInfoData {
                name: "Settings",
                flags: MemberInfoFlags::new(0),
                field_type: VISUALTERRAINBASESETTINGS_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainStaticState, settings),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISUALTERRAINSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualTerrainStaticState {
    fn type_info() -> &'static TypeInfo {
        VISUALTERRAINSTATICSTATE_TYPE_INFO
    }
}


pub const VISUALTERRAINSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VisualTerrainHandle {
}

pub const VISUALTERRAINHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainHandle",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(VISUALTERRAINHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for VisualTerrainHandle {
    fn type_info() -> &'static TypeInfo {
        VISUALTERRAINHANDLE_TYPE_INFO
    }
}


pub const VISUALTERRAINHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VisualTerrainBaseSettings {
}

pub const VISUALTERRAINBASESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainBaseSettings",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VISUALTERRAINBASESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualTerrainBaseSettings {
    fn type_info() -> &'static TypeInfo {
        VISUALTERRAINBASESETTINGS_TYPE_INFO
    }
}


pub const VISUALTERRAINBASESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainBaseSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainBaseSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainStreamingSettings {
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

pub const TERRAINSTREAMINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingSettings",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DataLoadJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, data_load_job_count),
            },
            FieldInfoData {
                name: "ActiveFreeStreamingDataLoadJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, active_free_streaming_data_load_job_count),
            },
            FieldInfoData {
                name: "LoadOccluderDataEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, load_occluder_data_enable),
            },
            FieldInfoData {
                name: "AdditionalBlurriness",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, additional_blurriness),
            },
            FieldInfoData {
                name: "InvisibleDetailReductionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, invisible_detail_reduction_factor),
            },
            FieldInfoData {
                name: "OccludedDetailReductionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, occluded_detail_reduction_factor),
            },
            FieldInfoData {
                name: "KeepPoolFullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, keep_pool_full_enable),
            },
            FieldInfoData {
                name: "HeightfieldAtlasSampleCountXFactor",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, heightfield_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "HeightfieldAtlasSampleCountYFactor",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, heightfield_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "MaskAtlasSampleCountXFactor",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, mask_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "MaskAtlasSampleCountYFactor",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, mask_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "MaskAdditionalBlurriness",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, mask_additional_blurriness),
            },
            FieldInfoData {
                name: "ColorAtlasSampleCountXFactor",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, color_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "ColorAtlasSampleCountYFactor",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, color_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "ColorAdditionalBlurriness",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainStreamingSettings, color_additional_blurriness),
            },
        ],
    }),
    array_type: Some(TERRAINSTREAMINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainStreamingSettings {
    fn type_info() -> &'static TypeInfo {
        TERRAINSTREAMINGSETTINGS_TYPE_INFO
    }
}


pub const TERRAINSTREAMINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainStreamingSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainQuadDecalData {
    pub shader2d: super::render_base::SurfaceShaderBaseAsset,
    pub shader2d_mesh_scattering_mask: super::render_base::SurfaceShaderBaseAsset,
    pub shader2d_single_layer_mask: super::render_base::SurfaceShaderBaseAsset,
    pub shader3d_z_only: super::render_base::SurfaceShaderBaseAsset,
    pub shader2d_displacement: super::render_base::SurfaceShaderBaseAsset,
    pub stick_to_terrain: bool,
    pub user_masks: super::core::Vec4,
    pub tangent_space_enable: bool,
    pub atlas_tile_template: TerrainQuadDecalAtlasTileTemplateData,
    pub atlas_tile: TerrainQuadDecalAtlasTile,
}

pub const TERRAINQUADDECALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shader2d",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d),
            },
            FieldInfoData {
                name: "Shader2dMeshScatteringMask",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d_mesh_scattering_mask),
            },
            FieldInfoData {
                name: "Shader2dSingleLayerMask",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d_single_layer_mask),
            },
            FieldInfoData {
                name: "Shader3dZOnly",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalData, shader3d_z_only),
            },
            FieldInfoData {
                name: "Shader2dDisplacement",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d_displacement),
            },
            FieldInfoData {
                name: "StickToTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalData, stick_to_terrain),
            },
            FieldInfoData {
                name: "UserMasks",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalData, user_masks),
            },
            FieldInfoData {
                name: "TangentSpaceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalData, tangent_space_enable),
            },
            FieldInfoData {
                name: "AtlasTileTemplate",
                flags: MemberInfoFlags::new(0),
                field_type: TERRAINQUADDECALATLASTILETEMPLATEDATA_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalData, atlas_tile_template),
            },
            FieldInfoData {
                name: "AtlasTile",
                flags: MemberInfoFlags::new(0),
                field_type: TERRAINQUADDECALATLASTILE_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalData, atlas_tile),
            },
        ],
    }),
    array_type: Some(TERRAINQUADDECALDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TerrainQuadDecalData {
    fn type_info() -> &'static TypeInfo {
        TERRAINQUADDECALDATA_TYPE_INFO
    }
}


pub const TERRAINQUADDECALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainQuadDecalData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainQuadDecalAtlasTileTemplateData {
    pub atlas_tile: TerrainQuadDecalAtlasTile,
}

pub const TERRAINQUADDECALATLASTILETEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTileTemplateData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AtlasTile",
                flags: MemberInfoFlags::new(0),
                field_type: TERRAINQUADDECALATLASTILE_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalAtlasTileTemplateData, atlas_tile),
            },
        ],
    }),
    array_type: Some(TERRAINQUADDECALATLASTILETEMPLATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainQuadDecalAtlasTileTemplateData {
    fn type_info() -> &'static TypeInfo {
        TERRAINQUADDECALATLASTILETEMPLATEDATA_TYPE_INFO
    }
}


pub const TERRAINQUADDECALATLASTILETEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTileTemplateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainQuadDecalAtlasTileTemplateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainQuadDecalAtlasTile {
    pub tile_index_x: u32,
    pub tile_index_y: u32,
    pub tile_count_x: u32,
    pub tile_count_y: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}

pub const TERRAINQUADDECALATLASTILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTile",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TileIndexX",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_index_x),
            },
            FieldInfoData {
                name: "TileIndexY",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_index_y),
            },
            FieldInfoData {
                name: "TileCountX",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_count_x),
            },
            FieldInfoData {
                name: "TileCountY",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_count_y),
            },
            FieldInfoData {
                name: "FlipX",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, flip_x),
            },
            FieldInfoData {
                name: "FlipY",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, flip_y),
            },
        ],
    }),
    array_type: Some(TERRAINQUADDECALATLASTILE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TerrainQuadDecalAtlasTile {
    fn type_info() -> &'static TypeInfo {
        TERRAINQUADDECALATLASTILE_TYPE_INFO
    }
}


pub const TERRAINQUADDECALATLASTILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTile-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainQuadDecalAtlasTile-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainFillDecalData {
    pub shader2d: super::render_base::SurfaceShaderBaseAsset,
    pub shader2d_mesh_scattering_mask: super::render_base::SurfaceShaderBaseAsset,
    pub shader2d_single_layer_mask: super::render_base::SurfaceShaderBaseAsset,
    pub shader3d_z_only: super::render_base::SurfaceShaderBaseAsset,
}

pub const TERRAINFILLDECALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainFillDecalData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shader2d",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainFillDecalData, shader2d),
            },
            FieldInfoData {
                name: "Shader2dMeshScatteringMask",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainFillDecalData, shader2d_mesh_scattering_mask),
            },
            FieldInfoData {
                name: "Shader2dSingleLayerMask",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainFillDecalData, shader2d_single_layer_mask),
            },
            FieldInfoData {
                name: "Shader3dZOnly",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainFillDecalData, shader3d_z_only),
            },
        ],
    }),
    array_type: Some(TERRAINFILLDECALDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainFillDecalData {
    fn type_info() -> &'static TypeInfo {
        TERRAINFILLDECALDATA_TYPE_INFO
    }
}


pub const TERRAINFILLDECALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainFillDecalData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainFillDecalData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LakeData {
}

pub const LAKEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LakeData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LAKEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LakeData {
    fn type_info() -> &'static TypeInfo {
        LAKEDATA_TYPE_INFO
    }
}


pub const LAKEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LakeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("LakeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RiverData {
}

pub const RIVERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RiverData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RIBBONDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RIVERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RiverData {
    fn type_info() -> &'static TypeInfo {
        RIVERDATA_TYPE_INFO
    }
}


pub const RIVERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RiverData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RiverData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RoadData {
    pub shader2d: super::render_base::SurfaceShaderBaseAsset,
    pub shader2d_mesh_scattering_mask: super::render_base::SurfaceShaderBaseAsset,
    pub shader2d_single_layer_mask: super::render_base::SurfaceShaderBaseAsset,
    pub shader3d_z_only: super::render_base::SurfaceShaderBaseAsset,
    pub shader2d_displacement: super::render_base::SurfaceShaderBaseAsset,
    pub stick_to_terrain: bool,
    pub uv_tile_factor: f32,
    pub tangent_space_enable: bool,
}

pub const ROADDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RoadData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RIBBONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shader2d",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(RoadData, shader2d),
            },
            FieldInfoData {
                name: "Shader2dMeshScatteringMask",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(RoadData, shader2d_mesh_scattering_mask),
            },
            FieldInfoData {
                name: "Shader2dSingleLayerMask",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(RoadData, shader2d_single_layer_mask),
            },
            FieldInfoData {
                name: "Shader3dZOnly",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(RoadData, shader3d_z_only),
            },
            FieldInfoData {
                name: "Shader2dDisplacement",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(RoadData, shader2d_displacement),
            },
            FieldInfoData {
                name: "StickToTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RoadData, stick_to_terrain),
            },
            FieldInfoData {
                name: "UvTileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RoadData, uv_tile_factor),
            },
            FieldInfoData {
                name: "TangentSpaceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RoadData, tangent_space_enable),
            },
        ],
    }),
    array_type: Some(ROADDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RoadData {
    fn type_info() -> &'static TypeInfo {
        ROADDATA_TYPE_INFO
    }
}


pub const ROADDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RoadData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RoadData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RibbonData {
    pub ribbon_points: Vec<RibbonPointData>,
}

pub const RIBBONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RibbonPoints",
                flags: MemberInfoFlags::new(144),
                field_type: RIBBONPOINTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RibbonData, ribbon_points),
            },
        ],
    }),
    array_type: Some(RIBBONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RibbonData {
    fn type_info() -> &'static TypeInfo {
        RIBBONDATA_TYPE_INFO
    }
}


pub const RIBBONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RibbonData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RibbonPointData {
    pub left: f32,
    pub right: f32,
    pub user_mask_left: super::core::Vec4,
    pub user_mask_right: super::core::Vec4,
}

pub const RIBBONPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonPointData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Left",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RibbonPointData, left),
            },
            FieldInfoData {
                name: "Right",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RibbonPointData, right),
            },
            FieldInfoData {
                name: "UserMaskLeft",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(RibbonPointData, user_mask_left),
            },
            FieldInfoData {
                name: "UserMaskRight",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(RibbonPointData, user_mask_right),
            },
        ],
    }),
    array_type: Some(RIBBONPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RibbonPointData {
    fn type_info() -> &'static TypeInfo {
        RIBBONPOINTDATA_TYPE_INFO
    }
}


pub const RIBBONPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonPointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RibbonPointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VisualVectorShapeData {
    pub error_tolerance: f32,
    pub shader3d: super::render_base::SurfaceShaderBaseAsset,
    pub draw_order_index: u32,
    pub tessellation_triangle_size: f32,
    pub split_to_match_heightfield: bool,
}

pub const VISUALVECTORSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualVectorShapeData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VECTORSHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ErrorTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualVectorShapeData, error_tolerance),
            },
            FieldInfoData {
                name: "Shader3d",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(VisualVectorShapeData, shader3d),
            },
            FieldInfoData {
                name: "DrawOrderIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualVectorShapeData, draw_order_index),
            },
            FieldInfoData {
                name: "TessellationTriangleSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualVectorShapeData, tessellation_triangle_size),
            },
            FieldInfoData {
                name: "SplitToMatchHeightfield",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualVectorShapeData, split_to_match_heightfield),
            },
        ],
    }),
    array_type: Some(VISUALVECTORSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualVectorShapeData {
    fn type_info() -> &'static TypeInfo {
        VISUALVECTORSHAPEDATA_TYPE_INFO
    }
}


pub const VISUALVECTORSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualVectorShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualVectorShapeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainShaderParameterBlockDynamicState {
}

pub const TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainShaderParameterBlockDynamicState {
    fn type_info() -> &'static TypeInfo {
        TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO
    }
}


pub const TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainShaderParameterBlockDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainShaderParameterBlockStaticState {
    pub shader_block_handle: super::render_base::ShaderParameterBlockHandle,
    pub field_flag_changed0: u8,
}

pub const TERRAINSHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockStaticState",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ShaderBlockHandle",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERPARAMETERBLOCKHANDLE_TYPE_INFO,
                rust_offset: offset_of!(TerrainShaderParameterBlockStaticState, shader_block_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TerrainShaderParameterBlockStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TERRAINSHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for TerrainShaderParameterBlockStaticState {
    fn type_info() -> &'static TypeInfo {
        TERRAINSHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO
    }
}


pub const TERRAINSHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainShaderParameterBlockStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainSettings {
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

pub const TERRAINSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSettings",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HeightQueryCacheSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainSettings, height_query_cache_size),
            },
            FieldInfoData {
                name: "ModifiersEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainSettings, modifiers_enable),
            },
            FieldInfoData {
                name: "ModifiersCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainSettings, modifiers_capacity),
            },
            FieldInfoData {
                name: "IntersectingModifiersMax",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainSettings, intersecting_modifiers_max),
            },
            FieldInfoData {
                name: "ModifierSlopeMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainSettings, modifier_slope_max),
            },
            FieldInfoData {
                name: "ModifierDepthFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainSettings, modifier_depth_factor),
            },
            FieldInfoData {
                name: "ModifiersAppliedPerFrameMax",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainSettings, modifiers_applied_per_frame_max),
            },
            FieldInfoData {
                name: "PrioritizationOnSeveralFrames",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainSettings, prioritization_on_several_frames),
            },
            FieldInfoData {
                name: "RefiningDuringPrioritization",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainSettings, refining_during_prioritization),
            },
            FieldInfoData {
                name: "RefiningDuringPrioritizationMinPriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainSettings, refining_during_prioritization_min_priority),
            },
        ],
    }),
    array_type: Some(TERRAINSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainSettings {
    fn type_info() -> &'static TypeInfo {
        TERRAINSETTINGS_TYPE_INFO
    }
}


pub const TERRAINSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainModificationDynamicState {
    pub burn_map: Vec<u8>,
    pub field_flag_changed0: u8,
}

pub const TERRAINMODIFICATIONDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BurnMap",
                flags: MemberInfoFlags::new(144),
                field_type: UINT8_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TerrainModificationDynamicState, burn_map),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TerrainModificationDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TERRAINMODIFICATIONDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainModificationDynamicState {
    fn type_info() -> &'static TypeInfo {
        TERRAINMODIFICATIONDYNAMICSTATE_TYPE_INFO
    }
}


pub const TERRAINMODIFICATIONDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainModificationDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TerrainModificationType {
    #[default]
    TerrainModificationType_DynamicDetail = 0,
    TerrainModificationType_DynamicExternalRaster = 1,
}

pub const TERRAINMODIFICATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINMODIFICATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainModificationType {
    fn type_info() -> &'static TypeInfo {
        TERRAINMODIFICATIONTYPE_TYPE_INFO
    }
}


pub const TERRAINMODIFICATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainModificationType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainEditingDynamicState {
}

pub const TERRAINEDITINGDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TERRAINEDITINGDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainEditingDynamicState {
    fn type_info() -> &'static TypeInfo {
        TERRAINEDITINGDYNAMICSTATE_TYPE_INFO
    }
}


pub const TERRAINEDITINGDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainEditingDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainEditingStaticState {
    pub event_type: TerrainEditingEvent,
    pub mesh_scattering_type: TerrainMeshScatteringType,
    pub mesh_scattering_field_number: u32,
    pub decals: Vec<VisualVectorShapeData>,
    pub field_flag_changed0: u8,
}

pub const TERRAINEDITINGSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingStaticState",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EventType",
                flags: MemberInfoFlags::new(0),
                field_type: TERRAINEDITINGEVENT_TYPE_INFO,
                rust_offset: offset_of!(TerrainEditingStaticState, event_type),
            },
            FieldInfoData {
                name: "MeshScatteringType",
                flags: MemberInfoFlags::new(0),
                field_type: TERRAINMESHSCATTERINGTYPE_TYPE_INFO,
                rust_offset: offset_of!(TerrainEditingStaticState, mesh_scattering_type),
            },
            FieldInfoData {
                name: "MeshScatteringFieldNumber",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainEditingStaticState, mesh_scattering_field_number),
            },
            FieldInfoData {
                name: "Decals",
                flags: MemberInfoFlags::new(144),
                field_type: VISUALVECTORSHAPEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TerrainEditingStaticState, decals),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TerrainEditingStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TERRAINEDITINGSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainEditingStaticState {
    fn type_info() -> &'static TypeInfo {
        TERRAINEDITINGSTATICSTATE_TYPE_INFO
    }
}


pub const TERRAINEDITINGSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainEditingStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TerrainEditingEvent {
    #[default]
    TerrainEditingEvent_MeshScattering = 0,
    TerrainEditingEvent_RemoveDecals = 1,
    TerrainEditingEvent_EditDecals = 2,
    TerrainEditingEvent_EditDecalsReducedDetail = 3,
}

pub const TERRAINEDITINGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingEvent",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINEDITINGEVENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainEditingEvent {
    fn type_info() -> &'static TypeInfo {
        TERRAINEDITINGEVENT_TYPE_INFO
    }
}


pub const TERRAINEDITINGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainEditingEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const PLAYABLEPIXELSPERMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayablePixelsPerMeter",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(PLAYABLEPIXELSPERMETER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PlayablePixelsPerMeter {
    fn type_info() -> &'static TypeInfo {
        PLAYABLEPIXELSPERMETER_TYPE_INFO
    }
}


pub const PLAYABLEPIXELSPERMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayablePixelsPerMeter-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PlayablePixelsPerMeter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const TERRAINANCHOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainAnchor",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINANCHOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainAnchor {
    fn type_info() -> &'static TypeInfo {
        TERRAINANCHOR_TYPE_INFO
    }
}


pub const TERRAINANCHOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainAnchor-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainAnchor-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const TERRAINSIZE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSize",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINSIZE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainSize {
    fn type_info() -> &'static TypeInfo {
        TERRAINSIZE_TYPE_INFO
    }
}


pub const TERRAINSIZE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSize-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainSize-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum HighResTerrainSize {
    #[default]
    pws512 = 512,
    pws1024 = 1024,
    pws2048 = 2048,
    pws4096 = 4096,
    pws8192 = 8192,
}

pub const HIGHRESTERRAINSIZE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HighResTerrainSize",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(HIGHRESTERRAINSIZE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for HighResTerrainSize {
    fn type_info() -> &'static TypeInfo {
        HIGHRESTERRAINSIZE_TYPE_INFO
    }
}


pub const HIGHRESTERRAINSIZE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HighResTerrainSize-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HighResTerrainSize-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainLayerCombinationsData {
}

pub const TERRAINLAYERCOMBINATIONSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationsData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERCOMBINATIONSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainLayerCombinationsData {
    fn type_info() -> &'static TypeInfo {
        TERRAINLAYERCOMBINATIONSDATA_TYPE_INFO
    }
}


pub const TERRAINLAYERCOMBINATIONSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerCombinationsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainLayerShaderData {
}

pub const TERRAINLAYERSHADERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerShaderData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERSHADERDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainLayerShaderData {
    fn type_info() -> &'static TypeInfo {
        TERRAINLAYERSHADERDATA_TYPE_INFO
    }
}


pub const TERRAINLAYERSHADERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerShaderData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerShaderData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshScatteringSpawnData {
}

pub const MESHSCATTERINGSPAWNDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringSpawnData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(MESHSCATTERINGSPAWNDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringSpawnData {
    fn type_info() -> &'static TypeInfo {
        MESHSCATTERINGSPAWNDATA_TYPE_INFO
    }
}


pub const MESHSCATTERINGSPAWNDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringSpawnData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringSpawnData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainStreamingTreeAsset {
}

pub const TERRAINSTREAMINGTREEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTreeAsset",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TERRAINSTREAMINGTREEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainStreamingTreeAsset {
    fn type_info() -> &'static TypeInfo {
        TERRAINSTREAMINGTREEASSET_TYPE_INFO
    }
}


pub const TERRAINSTREAMINGTREEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTreeAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainStreamingTreeAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainLayerMaskData {
}

pub const TERRAINLAYERMASKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerMaskData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERMASKDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainLayerMaskData {
    fn type_info() -> &'static TypeInfo {
        TERRAINLAYERMASKDATA_TYPE_INFO
    }
}


pub const TERRAINLAYERMASKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerMaskData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerMaskData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingMaskRasterData {
}

pub const PATHFINDINGMASKRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingMaskRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PATHFINDINGMASKRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingMaskRasterData {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGMASKRASTERDATA_TYPE_INFO
    }
}


pub const PATHFINDINGMASKRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingMaskRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PathfindingMaskRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainData {
    pub terrain_layers: Vec<TerrainLayerData>,
    pub dynamic_mask_enable: bool,
    pub detail_displacement_max_level_diff: u32,
    pub detail_displacement_indirection_texture_tile_x: u32,
    pub override_occluder_settings: bool,
    pub occluder_enable: bool,
    pub occluder_patch_faces_per_side: u32,
    pub occluder_lod_scale: f32,
    pub terrain_streaming_tree_resource: super::core::ResourceRef,
    pub visual_resource: super::core::ResourceRef,
    pub terrain_layer_combinations_resource: super::core::ResourceRef,
}

pub const TERRAINDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TERRAINBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TerrainLayers",
                flags: MemberInfoFlags::new(144),
                field_type: TERRAINLAYERDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, terrain_layers),
            },
            FieldInfoData {
                name: "DynamicMaskEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, dynamic_mask_enable),
            },
            FieldInfoData {
                name: "DetailDisplacementMaxLevelDiff",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, detail_displacement_max_level_diff),
            },
            FieldInfoData {
                name: "DetailDisplacementIndirectionTextureTileX",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, detail_displacement_indirection_texture_tile_x),
            },
            FieldInfoData {
                name: "OverrideOccluderSettings",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, override_occluder_settings),
            },
            FieldInfoData {
                name: "OccluderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, occluder_enable),
            },
            FieldInfoData {
                name: "OccluderPatchFacesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, occluder_patch_faces_per_side),
            },
            FieldInfoData {
                name: "OccluderLodScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, occluder_lod_scale),
            },
            FieldInfoData {
                name: "TerrainStreamingTreeResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, terrain_streaming_tree_resource),
            },
            FieldInfoData {
                name: "VisualResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, visual_resource),
            },
            FieldInfoData {
                name: "TerrainLayerCombinationsResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(TerrainData, terrain_layer_combinations_resource),
            },
        ],
    }),
    array_type: Some(TERRAINDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainData {
    fn type_info() -> &'static TypeInfo {
        TERRAINDATA_TYPE_INFO
    }
}


pub const TERRAINDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EnlightenMeshFilterType {
    #[default]
    EnlightenMeshFilterType_Nearest = 0,
    EnlightenMeshFilterType_Bilinear = 1,
}

pub const ENLIGHTENMESHFILTERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenMeshFilterType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(ENLIGHTENMESHFILTERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EnlightenMeshFilterType {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENMESHFILTERTYPE_TYPE_INFO
    }
}


pub const ENLIGHTENMESHFILTERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenMeshFilterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("EnlightenMeshFilterType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RasterTreeBuildMode {
    #[default]
    RasterTreeBuildMode_InlinePersistentStreamRest = 0,
    RasterTreeBuildMode_InlinePersistentRemoveRest = 1,
    RasterTreeBuildMode_InlineAll = 2,
}

pub const RASTERTREEBUILDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTreeBuildMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(RASTERTREEBUILDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RasterTreeBuildMode {
    fn type_info() -> &'static TypeInfo {
        RASTERTREEBUILDMODE_TYPE_INFO
    }
}


pub const RASTERTREEBUILDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTreeBuildMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterTreeBuildMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainLayerCombinationDrawData {
}

pub const TERRAINLAYERCOMBINATIONDRAWDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERCOMBINATIONDRAWDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainLayerCombinationDrawData {
    fn type_info() -> &'static TypeInfo {
        TERRAINLAYERCOMBINATIONDRAWDATA_TYPE_INFO
    }
}


pub const TERRAINLAYERCOMBINATIONDRAWDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerCombinationDrawData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Surface3dDrawMethodData {
}

pub const SURFACE3DDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface3dDrawMethodData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SURFACE3DDRAWMETHODDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Surface3dDrawMethodData {
    fn type_info() -> &'static TypeInfo {
        SURFACE3DDRAWMETHODDATA_TYPE_INFO
    }
}


pub const SURFACE3DDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface3dDrawMethodData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Surface3dDrawMethodData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SingleLayerMaskDrawMethodData {
}

pub const SINGLELAYERMASKDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawMethodData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SINGLELAYERMASKDRAWMETHODDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SingleLayerMaskDrawMethodData {
    fn type_info() -> &'static TypeInfo {
        SINGLELAYERMASKDRAWMETHODDATA_TYPE_INFO
    }
}


pub const SINGLELAYERMASKDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawMethodData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SingleLayerMaskDrawMethodData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SingleLayerMaskDrawPassData {
}

pub const SINGLELAYERMASKDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawPassData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SINGLELAYERMASKDRAWPASSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SingleLayerMaskDrawPassData {
    fn type_info() -> &'static TypeInfo {
        SINGLELAYERMASKDRAWPASSDATA_TYPE_INFO
    }
}


pub const SINGLELAYERMASKDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawPassData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SingleLayerMaskDrawPassData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshScatteringMaskScaleDrawMethodData {
}

pub const MESHSCATTERINGMASKSCALEDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringMaskScaleDrawMethodData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(MESHSCATTERINGMASKSCALEDRAWMETHODDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringMaskScaleDrawMethodData {
    fn type_info() -> &'static TypeInfo {
        MESHSCATTERINGMASKSCALEDRAWMETHODDATA_TYPE_INFO
    }
}


pub const MESHSCATTERINGMASKSCALEDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringMaskScaleDrawMethodData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringMaskScaleDrawMethodData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Displacement2dDrawMethodData {
}

pub const DISPLACEMENT2DDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawMethodData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DISPLACEMENT2DDRAWMETHODDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Displacement2dDrawMethodData {
    fn type_info() -> &'static TypeInfo {
        DISPLACEMENT2DDRAWMETHODDATA_TYPE_INFO
    }
}


pub const DISPLACEMENT2DDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawMethodData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Displacement2dDrawMethodData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Displacement2dDrawPassData {
}

pub const DISPLACEMENT2DDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawPassData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(DISPLACEMENT2DDRAWPASSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Displacement2dDrawPassData {
    fn type_info() -> &'static TypeInfo {
        DISPLACEMENT2DDRAWPASSDATA_TYPE_INFO
    }
}


pub const DISPLACEMENT2DDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawPassData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Displacement2dDrawPassData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Surface2dDrawMethodData {
}

pub const SURFACE2DDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawMethodData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SURFACE2DDRAWMETHODDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Surface2dDrawMethodData {
    fn type_info() -> &'static TypeInfo {
        SURFACE2DDRAWMETHODDATA_TYPE_INFO
    }
}


pub const SURFACE2DDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawMethodData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Surface2dDrawMethodData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Surface2dDrawPassData {
}

pub const SURFACE2DDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawPassData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SURFACE2DDRAWPASSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Surface2dDrawPassData {
    fn type_info() -> &'static TypeInfo {
        SURFACE2DDRAWPASSDATA_TYPE_INFO
    }
}


pub const SURFACE2DDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawPassData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Surface2dDrawPassData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainLayerCombinationDrawPassData {
}

pub const TERRAINLAYERCOMBINATIONDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawPassData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERCOMBINATIONDRAWPASSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainLayerCombinationDrawPassData {
    fn type_info() -> &'static TypeInfo {
        TERRAINLAYERCOMBINATIONDRAWPASSDATA_TYPE_INFO
    }
}


pub const TERRAINLAYERCOMBINATIONDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawPassData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerCombinationDrawPassData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TerrainDrawPassType {
    #[default]
    TerrainDrawPassType_SinglePass = 0,
    TerrainDrawPassType_MultipassFirst = 1,
    TerrainDrawPassType_MultipassConsecutive = 2,
}

pub const TERRAINDRAWPASSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDrawPassType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINDRAWPASSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainDrawPassType {
    fn type_info() -> &'static TypeInfo {
        TERRAINDRAWPASSTYPE_TYPE_INFO
    }
}


pub const TERRAINDRAWPASSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDrawPassType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainDrawPassType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TerrainBrushDetailOperation {
    #[default]
    Lerp = 0,
    Add = 1,
    Multiply = 2,
}

pub const TERRAINBRUSHDETAILOPERATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBrushDetailOperation",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINBRUSHDETAILOPERATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainBrushDetailOperation {
    fn type_info() -> &'static TypeInfo {
        TERRAINBRUSHDETAILOPERATION_TYPE_INFO
    }
}


pub const TERRAINBRUSHDETAILOPERATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBrushDetailOperation-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainBrushDetailOperation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SingleTerrainLayerData {
    pub mesh_scattering_types: Vec<TerrainMeshScatteringType>,
}

pub const SINGLETERRAINLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleTerrainLayerData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TERRAINLAYERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MeshScatteringTypes",
                flags: MemberInfoFlags::new(144),
                field_type: TERRAINMESHSCATTERINGTYPE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SingleTerrainLayerData, mesh_scattering_types),
            },
        ],
    }),
    array_type: Some(SINGLETERRAINLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SingleTerrainLayerData {
    fn type_info() -> &'static TypeInfo {
        SINGLETERRAINLAYERDATA_TYPE_INFO
    }
}


pub const SINGLETERRAINLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleTerrainLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SingleTerrainLayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainLayerData {
}

pub const TERRAINLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainLayerData {
    fn type_info() -> &'static TypeInfo {
        TERRAINLAYERDATA_TYPE_INFO
    }
}


pub const TERRAINLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColorImportSettings {
}

pub const COLORIMPORTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorImportSettings",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(COLORIMPORTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ColorImportSettings {
    fn type_info() -> &'static TypeInfo {
        COLORIMPORTSETTINGS_TYPE_INFO
    }
}


pub const COLORIMPORTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorImportSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ColorImportSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TerrainLayerType {
    #[default]
    TerrainLayerType_IgnoreMask = 0,
    TerrainLayerType_Masked = 1,
    TerrainLayerType_BinaryMasked = 2,
}

pub const TERRAINLAYERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINLAYERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainLayerType {
    fn type_info() -> &'static TypeInfo {
        TERRAINLAYERTYPE_TYPE_INFO
    }
}


pub const TERRAINLAYERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainLayerProceduralMask {
    pub altitude_min: f32,
}

pub const TERRAINLAYERPROCEDURALMASK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerProceduralMask",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AltitudeMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainLayerProceduralMask, altitude_min),
            },
        ],
    }),
    array_type: Some(TERRAINLAYERPROCEDURALMASK_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TerrainLayerProceduralMask {
    fn type_info() -> &'static TypeInfo {
        TERRAINLAYERPROCEDURALMASK_TYPE_INFO
    }
}


pub const TERRAINLAYERPROCEDURALMASK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerProceduralMask-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerProceduralMask-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainGeoTexture {
}

pub const TERRAINGEOTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainGeoTexture",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TERRAINGEOTEXTURE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainGeoTexture {
    fn type_info() -> &'static TypeInfo {
        TERRAINGEOTEXTURE_TYPE_INFO
    }
}


pub const TERRAINGEOTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainGeoTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainGeoTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainMeshScatteringType {
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

pub const TERRAINMESHSCATTERINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainMeshScatteringType",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Identifier",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, identifier),
            },
            FieldInfoData {
                name: "MinScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, min_scale),
            },
            FieldInfoData {
                name: "MaxScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, max_scale),
            },
            FieldInfoData {
                name: "ScaleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, scale_randomness),
            },
            FieldInfoData {
                name: "Lod0DissolveOutDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, lod0_dissolve_out_distance_factor),
            },
            FieldInfoData {
                name: "Lod1DissolveInDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, lod1_dissolve_in_distance_factor),
            },
            FieldInfoData {
                name: "Lod1DissolveOutDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, lod1_dissolve_out_distance_factor),
            },
            FieldInfoData {
                name: "Lod2DissolveInDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, lod2_dissolve_in_distance_factor),
            },
            FieldInfoData {
                name: "Lod2DissolveOutDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, lod2_dissolve_out_distance_factor),
            },
            FieldInfoData {
                name: "Lod3DissolveInDistanceFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, lod3_dissolve_in_distance_factor),
            },
            FieldInfoData {
                name: "Density",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, density),
            },
            FieldInfoData {
                name: "FirstSpawnLevel",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, first_spawn_level),
            },
            FieldInfoData {
                name: "WindScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, wind_scale),
            },
            FieldInfoData {
                name: "Stiffness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, stiffness),
            },
            FieldInfoData {
                name: "Damping",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, damping),
            },
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, mass),
            },
            FieldInfoData {
                name: "WindWiggle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, wind_wiggle),
            },
            FieldInfoData {
                name: "UseVertexColorWeights",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, use_vertex_color_weights),
            },
            FieldInfoData {
                name: "DissolveRangeRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainMeshScatteringType, dissolve_range_ratio),
            },
        ],
    }),
    array_type: Some(TERRAINMESHSCATTERINGTYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainMeshScatteringType {
    fn type_info() -> &'static TypeInfo {
        TERRAINMESHSCATTERINGTYPE_TYPE_INFO
    }
}


pub const TERRAINMESHSCATTERINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainMeshScatteringType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainMeshScatteringType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MeshScatteringInstanceDataMode {
    #[default]
    MeshScatteringInstanceDataMode_None = 0,
    MeshScatteringInstanceDataMode_Normal = 1,
    MeshScatteringInstanceDataMode_NormalAndAtlasIndex = 2,
    MeshScatteringInstanceDataMode_NormalAndColor = 3,
    MeshScatteringInstanceDataMode_WindAndAtlasIndex = 4,
    MeshScatteringInstanceDataMode_Wind = 5,
}

pub const MESHSCATTERINGINSTANCEDATAMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringInstanceDataMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSCATTERINGINSTANCEDATAMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringInstanceDataMode {
    fn type_info() -> &'static TypeInfo {
        MESHSCATTERINGINSTANCEDATAMODE_TYPE_INFO
    }
}


pub const MESHSCATTERINGINSTANCEDATAMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringInstanceDataMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringInstanceDataMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UndergrowthOrientationMode {
    #[default]
    UndergrowthOrientationMode_Horizontal = 0,
    UndergrowthOrientationMode_LeanToTerrain = 1,
    UndergrowthOrientationMode_SkewToTerrain = 2,
}

pub const UNDERGROWTHORIENTATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UndergrowthOrientationMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(UNDERGROWTHORIENTATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UndergrowthOrientationMode {
    fn type_info() -> &'static TypeInfo {
        UNDERGROWTHORIENTATIONMODE_TYPE_INFO
    }
}


pub const UNDERGROWTHORIENTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UndergrowthOrientationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("UndergrowthOrientationMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MeshScatteringBillboardMode {
    #[default]
    MeshScatteringBillboardMode_VerticalAxis = 0,
    MeshScatteringBillboardMode_VerticalAxisBending = 1,
    MeshScatteringBillboardMode_NormalAxisBending = 2,
    MeshScatteringBillboardMode_Count = 3,
}

pub const MESHSCATTERINGBILLBOARDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringBillboardMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSCATTERINGBILLBOARDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringBillboardMode {
    fn type_info() -> &'static TypeInfo {
        MESHSCATTERINGBILLBOARDMODE_TYPE_INFO
    }
}


pub const MESHSCATTERINGBILLBOARDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringBillboardMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringBillboardMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MeshScatteringOrientationMode {
    #[default]
    MeshScatteringOrientationMode_Horizontal = 0,
    MeshScatteringOrientationMode_LeanToTerrain = 1,
    MeshScatteringOrientationMode_SkewToTerrain = 2,
}

pub const MESHSCATTERINGORIENTATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringOrientationMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSCATTERINGORIENTATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringOrientationMode {
    fn type_info() -> &'static TypeInfo {
        MESHSCATTERINGORIENTATIONMODE_TYPE_INFO
    }
}


pub const MESHSCATTERINGORIENTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringOrientationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringOrientationMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UndergrowthRotationMode {
    #[default]
    UndergrowthRotationMode_Random = 0,
    UndergrowthRotationMode_TowardsSlope = 1,
    UndergrowthRotationMode_Fixed = 2,
}

pub const UNDERGROWTHROTATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UndergrowthRotationMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(UNDERGROWTHROTATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UndergrowthRotationMode {
    fn type_info() -> &'static TypeInfo {
        UNDERGROWTHROTATIONMODE_TYPE_INFO
    }
}


pub const UNDERGROWTHROTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UndergrowthRotationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("UndergrowthRotationMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MeshScatteringRotationMode {
    #[default]
    MeshScatteringRotationMode_Random = 0,
    MeshScatteringRotationMode_TowardsSlope = 1,
    MeshScatteringRotationMode_Fixed = 2,
}

pub const MESHSCATTERINGROTATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringRotationMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSCATTERINGROTATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringRotationMode {
    fn type_info() -> &'static TypeInfo {
        MESHSCATTERINGROTATIONMODE_TYPE_INFO
    }
}


pub const MESHSCATTERINGROTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringRotationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringRotationMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MeshScatteringElevationMode {
    #[default]
    MeshScatteringElevationMode_SnapBoundingBox = 0,
    MeshScatteringElevationMode_SnapPivotPoint = 1,
}

pub const MESHSCATTERINGELEVATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringElevationMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSCATTERINGELEVATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshScatteringElevationMode {
    fn type_info() -> &'static TypeInfo {
        MESHSCATTERINGELEVATIONMODE_TYPE_INFO
    }
}


pub const MESHSCATTERINGELEVATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringElevationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringElevationMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainHeightfieldData {
}

pub const TERRAINHEIGHTFIELDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainHeightfieldData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TERRAINHEIGHTFIELDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainHeightfieldData {
    fn type_info() -> &'static TypeInfo {
        TERRAINHEIGHTFIELDDATA_TYPE_INFO
    }
}


pub const TERRAINHEIGHTFIELDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainHeightfieldData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainHeightfieldData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const TERRAINBRUSHTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBrushType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINBRUSHTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainBrushType {
    fn type_info() -> &'static TypeInfo {
        TERRAINBRUSHTYPE_TYPE_INFO
    }
}


pub const TERRAINBRUSHTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBrushType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainBrushType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TerrainDynamicDecalTemplateData {
    pub width: f32,
    pub relative_width_deviation: f32,
    pub depth: f32,
    pub relative_depth_deviation: f32,
    pub rotation_random_amount: f32,
    pub slope_max: f32,
    pub depth_mask: HeightfieldDecalAsset,
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

pub const TERRAINDYNAMICDECALTEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDynamicDecalTemplateData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, width),
            },
            FieldInfoData {
                name: "RelativeWidthDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, relative_width_deviation),
            },
            FieldInfoData {
                name: "Depth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, depth),
            },
            FieldInfoData {
                name: "RelativeDepthDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, relative_depth_deviation),
            },
            FieldInfoData {
                name: "RotationRandomAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, rotation_random_amount),
            },
            FieldInfoData {
                name: "SlopeMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_max),
            },
            FieldInfoData {
                name: "DepthMask",
                flags: MemberInfoFlags::new(0),
                field_type: HEIGHTFIELDDECALASSET_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, depth_mask),
            },
            FieldInfoData {
                name: "MaskShader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, mask_shader),
            },
            FieldInfoData {
                name: "DisplacementShader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, displacement_shader),
            },
            FieldInfoData {
                name: "DynamicMaskDecalWidthScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, dynamic_mask_decal_width_scale),
            },
            FieldInfoData {
                name: "TangentSpaceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, tangent_space_enable),
            },
            FieldInfoData {
                name: "ScaleWithDestructionDepth",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, scale_with_destruction_depth),
            },
            FieldInfoData {
                name: "ForceUpScale",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, force_up_scale),
            },
            FieldInfoData {
                name: "SlopeMinThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_min_threshold),
            },
            FieldInfoData {
                name: "SlopeScalarMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_scalar_max),
            },
            FieldInfoData {
                name: "SlopeMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_multiplier),
            },
            FieldInfoData {
                name: "MaxOpposingSlopes",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, max_opposing_slopes),
            },
            FieldInfoData {
                name: "MinWeightThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, min_weight_threshold),
            },
        ],
    }),
    array_type: Some(TERRAINDYNAMICDECALTEMPLATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainDynamicDecalTemplateData {
    fn type_info() -> &'static TypeInfo {
        TERRAINDYNAMICDECALTEMPLATEDATA_TYPE_INFO
    }
}


pub const TERRAINDYNAMICDECALTEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDynamicDecalTemplateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainDynamicDecalTemplateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HeightfieldDecalAsset {
    pub resource: super::core::ResourceRef,
    pub mid_point128: bool,
}

pub const HEIGHTFIELDDECALASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecalAsset",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(HeightfieldDecalAsset, resource),
            },
            FieldInfoData {
                name: "MidPoint128",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HeightfieldDecalAsset, mid_point128),
            },
        ],
    }),
    array_type: Some(HEIGHTFIELDDECALASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HeightfieldDecalAsset {
    fn type_info() -> &'static TypeInfo {
        HEIGHTFIELDDECALASSET_TYPE_INFO
    }
}


pub const HEIGHTFIELDDECALASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecalAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HeightfieldDecalAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainBaseAsset {
}

pub const TERRAINBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TERRAINBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TerrainBaseAsset {
    fn type_info() -> &'static TypeInfo {
        TERRAINBASEASSET_TYPE_INFO
    }
}


pub const TERRAINBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingRasterData {
}

pub const PATHFINDINGRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RGBRASTERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PATHFINDINGRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingRasterData {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGRASTERDATA_TYPE_INFO
    }
}


pub const PATHFINDINGRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PathfindingRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RasterCoverageData {
}

pub const RASTERCOVERAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterCoverageData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RASTERCOVERAGEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RasterCoverageData {
    fn type_info() -> &'static TypeInfo {
        RASTERCOVERAGEDATA_TYPE_INFO
    }
}


pub const RASTERCOVERAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterCoverageData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterCoverageData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DensityMapRasterData {
}

pub const DENSITYMAPRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMapRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DENSITYMAPRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DensityMapRasterData {
    fn type_info() -> &'static TypeInfo {
        DENSITYMAPRASTERDATA_TYPE_INFO
    }
}


pub const DENSITYMAPRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMapRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DensityMapRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BiomeRasterData {
    pub biomes: Vec<BiomeSpec>,
}

pub const BIOMERASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(INDEXEDRASTERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Biomes",
                flags: MemberInfoFlags::new(144),
                field_type: BIOMESPEC_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(BiomeRasterData, biomes),
            },
        ],
    }),
    array_type: Some(BIOMERASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BiomeRasterData {
    fn type_info() -> &'static TypeInfo {
        BIOMERASTERDATA_TYPE_INFO
    }
}


pub const BIOMERASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("BiomeRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IndexedRasterData {
}

pub const INDEXEDRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexedRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INDEXEDRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IndexedRasterData {
    fn type_info() -> &'static TypeInfo {
        INDEXEDRASTERDATA_TYPE_INFO
    }
}


pub const INDEXEDRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexedRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("IndexedRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TileBiomeList {
    pub hash: i32,
    pub biomes: Vec<u8>,
}

pub const TILEBIOMELIST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TileBiomeList",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Hash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TileBiomeList, hash),
            },
            FieldInfoData {
                name: "Biomes",
                flags: MemberInfoFlags::new(144),
                field_type: UINT8_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TileBiomeList, biomes),
            },
        ],
    }),
    array_type: Some(TILEBIOMELIST_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TileBiomeList {
    fn type_info() -> &'static TypeInfo {
        TILEBIOMELIST_TYPE_INFO
    }
}


pub const TILEBIOMELIST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TileBiomeList-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TileBiomeList-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BiomeSpec {
    pub name: String,
    pub value: u32,
}

pub const BIOMESPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeSpec",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BiomeSpec, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BiomeSpec, value),
            },
        ],
    }),
    array_type: Some(BIOMESPEC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BiomeSpec {
    fn type_info() -> &'static TypeInfo {
        BIOMESPEC_TYPE_INFO
    }
}


pub const BIOMESPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("BiomeSpec-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FlowMapRasterData {
}

pub const FLOWMAPRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlowMapRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOWMAPRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FlowMapRasterData {
    fn type_info() -> &'static TypeInfo {
        FLOWMAPRASTERDATA_TYPE_INFO
    }
}


pub const FLOWMAPRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlowMapRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("FlowMapRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestructionDepthRasterData {
}

pub const DESTRUCTIONDEPTHRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONDEPTHRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionDepthRasterData {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONDEPTHRASTERDATA_TYPE_INFO
    }
}


pub const DESTRUCTIONDEPTHRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DestructionDepthRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestructionDepthGenerateOptions {
}

pub const DESTRUCTIONDEPTHGENERATEOPTIONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateOptions",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONDEPTHGENERATEOPTIONS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionDepthGenerateOptions {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONDEPTHGENERATEOPTIONS_TYPE_INFO
    }
}


pub const DESTRUCTIONDEPTHGENERATEOPTIONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateOptions-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DestructionDepthGenerateOptions-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DestructionDepthGenerateSource {
    #[default]
    DestructionDepthGenerateSource_None = 0,
    DestructionDepthGenerateSource_Collisions = 1,
    DestructionDepthGenerateSource_PhysicsMaterials = 2,
    DestructionDepthGenerateSource_Oceans = 3,
    DestructionDepthGenerateSource_DestructionMasks = 4,
}

pub const DESTRUCTIONDEPTHGENERATESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateSource",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(DESTRUCTIONDEPTHGENERATESOURCE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DestructionDepthGenerateSource {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONDEPTHGENERATESOURCE_TYPE_INFO
    }
}


pub const DESTRUCTIONDEPTHGENERATESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateSource-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DestructionDepthGenerateSource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsMaterialsRasterData {
}

pub const PHYSICSMATERIALSRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsMaterialsRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSMATERIALSRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsMaterialsRasterData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSMATERIALSRASTERDATA_TYPE_INFO
    }
}


pub const PHYSICSMATERIALSRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsMaterialsRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PhysicsMaterialsRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ByteRasterData {
}

pub const BYTERASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ByteRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BYTERASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ByteRasterData {
    fn type_info() -> &'static TypeInfo {
        BYTERASTERDATA_TYPE_INFO
    }
}


pub const BYTERASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ByteRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ByteRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RGBRasterData {
}

pub const RGBRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RGBRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RGBRasterData {
    fn type_info() -> &'static TypeInfo {
        RGBRASTERDATA_TYPE_INFO
    }
}


pub const RGBRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RGBRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RGBARasterData {
}

pub const RGBARASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBARasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RGBARASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RGBARasterData {
    fn type_info() -> &'static TypeInfo {
        RGBARASTERDATA_TYPE_INFO
    }
}


pub const RGBARASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBARasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RGBARasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HeightfieldRasterData {
}

pub const HEIGHTFIELDRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldRasterData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(HEIGHTFIELDRASTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HeightfieldRasterData {
    fn type_info() -> &'static TypeInfo {
        HEIGHTFIELDRASTERDATA_TYPE_INFO
    }
}


pub const HEIGHTFIELDRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldRasterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HeightfieldRasterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DensityMap_FilterType {
    #[default]
    DensityMapFilter_SecondOrderDifference = 0,
    DensityMapFilter_GaussianCurvature = 1,
    DensityMapFilter_MeanCurvature = 2,
    DensityMapFilter_LaplaceBeltrami = 3,
    DensityMapFilter_LaplaceBeltramiNoVoronoi = 4,
}

pub const DENSITYMAP_FILTERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMap_FilterType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(DENSITYMAP_FILTERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DensityMap_FilterType {
    fn type_info() -> &'static TypeInfo {
        DENSITYMAP_FILTERTYPE_TYPE_INFO
    }
}


pub const DENSITYMAP_FILTERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMap_FilterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DensityMap_FilterType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VirtualRasterQuadtreeData {
}

pub const VIRTUALRASTERQUADTREEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VirtualRasterQuadtreeData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VIRTUALRASTERQUADTREEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VirtualRasterQuadtreeData {
    fn type_info() -> &'static TypeInfo {
        VIRTUALRASTERQUADTREEDATA_TYPE_INFO
    }
}


pub const VIRTUALRASTERQUADTREEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VirtualRasterQuadtreeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VirtualRasterQuadtreeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RasterQuadtreeData {
}

pub const RASTERQUADTREEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RASTERQUADTREEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RasterQuadtreeData {
    fn type_info() -> &'static TypeInfo {
        RASTERQUADTREEDATA_TYPE_INFO
    }
}


pub const RASTERQUADTREEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterQuadtreeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SampleCenter {
    #[default]
    SampleCenter_Center = 0,
    SampleCenter_TopLeft = 1,
}

pub const SAMPLECENTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SampleCenter",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(SAMPLECENTER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SampleCenter {
    fn type_info() -> &'static TypeInfo {
        SAMPLECENTER_TYPE_INFO
    }
}


pub const SAMPLECENTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SampleCenter-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SampleCenter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RasterQuadtreeNodeData {
}

pub const RASTERQUADTREENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeNodeData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RASTERQUADTREENODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RasterQuadtreeNodeData {
    fn type_info() -> &'static TypeInfo {
        RASTERQUADTREENODEDATA_TYPE_INFO
    }
}


pub const RASTERQUADTREENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterQuadtreeNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum StyleTransferTexture {
    #[default]
    StyleTransferTexture_CedarMountain = 0,
    StyleTransferTexture_Canyon = 1,
    StyleTransferTexture_Glacier = 2,
    StyleTransferTexture_Custom = 3,
}

pub const STYLETRANSFERTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StyleTransferTexture",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(STYLETRANSFERTEXTURE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StyleTransferTexture {
    fn type_info() -> &'static TypeInfo {
        STYLETRANSFERTEXTURE_TYPE_INFO
    }
}


pub const STYLETRANSFERTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StyleTransferTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("StyleTransferTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum OverlayType {
    #[default]
    OverlayType_Base = 0,
    OverlayType_Effect = 1,
    OverlayType_Autopaint = 2,
    OverlayType_Paint = 3,
    OverlayType_Folder = 4,
    OverlayType_Synthetic = 5,
}

pub const OVERLAYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OverlayType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(OVERLAYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OverlayType {
    fn type_info() -> &'static TypeInfo {
        OVERLAYTYPE_TYPE_INFO
    }
}


pub const OVERLAYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OverlayType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("OverlayType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EffectOverlayType {
    #[default]
    EffectOverlayType_Filter = 0,
    EffectOverlayType_Generator = 1,
}

pub const EFFECTOVERLAYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectOverlayType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(EFFECTOVERLAYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EffectOverlayType {
    fn type_info() -> &'static TypeInfo {
        EFFECTOVERLAYTYPE_TYPE_INFO
    }
}


pub const EFFECTOVERLAYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectOverlayType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("EffectOverlayType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RasterNodeUsage {
    #[default]
    RasterNodeUsage_Default = 0,
    RasterNodeUsage_Disabled = 1,
    RasterNodeUsage_Persistent = 2,
    RasterNodeUsage_PersistentDedicatedServer = 3,
    RasterNodeUsage_Skipped = 4,
}

pub const RASTERNODEUSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterNodeUsage",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(RASTERNODEUSAGE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RasterNodeUsage {
    fn type_info() -> &'static TypeInfo {
        RASTERNODEUSAGE_TYPE_INFO
    }
}


pub const RASTERNODEUSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterNodeUsage-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterNodeUsage-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RectangularCoverageData {
}

pub const RECTANGULARCOVERAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RectangularCoverageData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RECTANGULARCOVERAGEDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RectangularCoverageData {
    fn type_info() -> &'static TypeInfo {
        RECTANGULARCOVERAGEDATA_TYPE_INFO
    }
}


pub const RECTANGULARCOVERAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RectangularCoverageData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RectangularCoverageData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutopaintOutput {
}

pub const AUTOPAINTOUTPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutput",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPAINTOUTPUTBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTOUTPUT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutopaintOutput {
    fn type_info() -> &'static TypeInfo {
        AUTOPAINTOUTPUT_TYPE_INFO
    }
}


pub const AUTOPAINTOUTPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutput-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintOutput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutopaintOutputOverride {
}

pub const AUTOPAINTOUTPUTOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputOverride",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPAINTOUTPUTOVERRIDEBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTOUTPUTOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutopaintOutputOverride {
    fn type_info() -> &'static TypeInfo {
        AUTOPAINTOUTPUTOVERRIDE_TYPE_INFO
    }
}


pub const AUTOPAINTOUTPUTOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintOutputOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutopaintOutputs {
}

pub const AUTOPAINTOUTPUTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputs",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPAINTOUTPUTSBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTOUTPUTS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutopaintOutputs {
    fn type_info() -> &'static TypeInfo {
        AUTOPAINTOUTPUTS_TYPE_INFO
    }
}


pub const AUTOPAINTOUTPUTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputs-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintOutputs-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutopaintConfigs {
}

pub const AUTOPAINTCONFIGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintConfigs",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTCONFIGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutopaintConfigs {
    fn type_info() -> &'static TypeInfo {
        AUTOPAINTCONFIGS_TYPE_INFO
    }
}


pub const AUTOPAINTCONFIGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintConfigs-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintConfigs-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RasterTypeToRasterFormat {
    pub raster_type: super::entity::RasterType,
    pub raster_format: super::entity::RasterFormat,
}

pub const RASTERTYPETORASTERFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTypeToRasterFormat",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "RasterType",
                flags: MemberInfoFlags::new(0),
                field_type: RASTERTYPE_TYPE_INFO,
                rust_offset: offset_of!(RasterTypeToRasterFormat, raster_type),
            },
            FieldInfoData {
                name: "RasterFormat",
                flags: MemberInfoFlags::new(0),
                field_type: RASTERFORMAT_TYPE_INFO,
                rust_offset: offset_of!(RasterTypeToRasterFormat, raster_format),
            },
        ],
    }),
    array_type: Some(RASTERTYPETORASTERFORMAT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RasterTypeToRasterFormat {
    fn type_info() -> &'static TypeInfo {
        RASTERTYPETORASTERFORMAT_TYPE_INFO
    }
}


pub const RASTERTYPETORASTERFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTypeToRasterFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterTypeToRasterFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClassTypeAutopaintOutputsMap {
    pub class_type: String,
    pub autopaint_outputs: AutopaintOutputs,
}

pub const CLASSTYPEAUTOPAINTOUTPUTSMAP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassTypeAutopaintOutputsMap",
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ClassType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ClassTypeAutopaintOutputsMap, class_type),
            },
            FieldInfoData {
                name: "AutopaintOutputs",
                flags: MemberInfoFlags::new(0),
                field_type: AUTOPAINTOUTPUTS_TYPE_INFO,
                rust_offset: offset_of!(ClassTypeAutopaintOutputsMap, autopaint_outputs),
            },
        ],
    }),
    array_type: Some(CLASSTYPEAUTOPAINTOUTPUTSMAP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClassTypeAutopaintOutputsMap {
    fn type_info() -> &'static TypeInfo {
        CLASSTYPEAUTOPAINTOUTPUTSMAP_TYPE_INFO
    }
}


pub const CLASSTYPEAUTOPAINTOUTPUTSMAP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassTypeAutopaintOutputsMap-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ClassTypeAutopaintOutputsMap-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPaintMeshData {
}

pub const AUTOPAINTMESHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintMeshData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTMESHDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AutoPaintMeshData {
    fn type_info() -> &'static TypeInfo {
        AUTOPAINTMESHDATA_TYPE_INFO
    }
}


pub const AUTOPAINTMESHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintMeshData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutoPaintMeshData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPaintRoadData {
}

pub const AUTOPAINTROADDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintRoadData",
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTROADDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AutoPaintRoadData {
    fn type_info() -> &'static TypeInfo {
        AUTOPAINTROADDATA_TYPE_INFO
    }
}


pub const AUTOPAINTROADDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintRoadData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutoPaintRoadData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum OutputType {
    #[default]
    OutputType_Mesh = 0,
    OutputType_Quad = 1,
    OutputType_Count = 2,
}

pub const OUTPUTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutputType",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(OUTPUTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OutputType {
    fn type_info() -> &'static TypeInfo {
        OUTPUTTYPE_TYPE_INFO
    }
}


pub const OUTPUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutputType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("OutputType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const TPABLENDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TPABlendMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(TPABLENDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TPABlendMode {
    fn type_info() -> &'static TypeInfo {
        TPABLENDMODE_TYPE_INFO
    }
}


pub const TPABLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TPABlendMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TPABlendMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DepthBuffer {
    #[default]
    DepthBuffer_Disabled = 0,
    DepthBuffer_Increasing = 1,
    DepthBuffer_Decreasing = 2,
}

pub const DEPTHBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DepthBuffer",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(DEPTHBUFFER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DepthBuffer {
    fn type_info() -> &'static TypeInfo {
        DEPTHBUFFER_TYPE_INFO
    }
}


pub const DEPTHBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DepthBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DepthBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FaceCulling {
    #[default]
    FaceCulling_Back = 0,
    FaceCulling_Front = 1,
    FaceCulling_None = 2,
    FaceCulling_Count = 3,
}

pub const FACECULLING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceCulling",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(FACECULLING_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FaceCulling {
    fn type_info() -> &'static TypeInfo {
        FACECULLING_TYPE_INFO
    }
}


pub const FACECULLING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceCulling-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("FaceCulling-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainStreamingTree {
}

pub const TERRAINSTREAMINGTREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTree",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(TERRAINSTREAMINGTREE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TerrainStreamingTree {
    fn type_info() -> &'static TypeInfo {
        TERRAINSTREAMINGTREE_TYPE_INFO
    }
}


pub const TERRAINSTREAMINGTREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTree-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainStreamingTree-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Terrain {
}

pub const TERRAIN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Terrain",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ITERRAIN_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TERRAIN_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Terrain {
    fn type_info() -> &'static TypeInfo {
        TERRAIN_TYPE_INFO
    }
}


pub const TERRAIN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Terrain-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Terrain-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ITerrain {
}

pub const ITERRAIN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITerrain",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ITERRAIN_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ITerrain {
    fn type_info() -> &'static TypeInfo {
        ITERRAIN_TYPE_INFO
    }
}


pub const ITERRAIN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITerrain-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ITerrain-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HeightfieldDecal {
}

pub const HEIGHTFIELDDECAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecal",
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(HEIGHTFIELDDECAL_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HeightfieldDecal {
    fn type_info() -> &'static TypeInfo {
        HEIGHTFIELDDECAL_TYPE_INFO
    }
}


pub const HEIGHTFIELDDECAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecal-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HeightfieldDecal-Array"),
    array_type: None,
    alignment: 8,
};


