use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct VisualTerrainDynamicState {
    pub visible: bool,
    pub draw_enable: bool,
    pub override_draw_enable: bool,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait VisualTerrainDynamicStateTrait: TypeObject {
    fn visible(&self) -> &bool;
    fn visible_mut(&mut self) -> &mut bool;
    fn draw_enable(&self) -> &bool;
    fn draw_enable_mut(&mut self) -> &mut bool;
    fn override_draw_enable(&self) -> &bool;
    fn override_draw_enable_mut(&mut self) -> &mut bool;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_override0_mut(&mut self) -> &mut u8;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl VisualTerrainDynamicStateTrait for VisualTerrainDynamicState {
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn draw_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_enable
    }
    fn override_draw_enable(&self) -> &bool {
        &self.override_draw_enable
    }
    fn override_draw_enable_mut(&mut self) -> &mut bool {
        &mut self.override_draw_enable
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static VISUALTERRAINDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainDynamicState",
    name_hash: 3037994820,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualTerrainDynamicState as Default>::default())),
            create_boxed: || Box::new(<VisualTerrainDynamicState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Visible",
                name_hash: 901540267,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainDynamicState, visible),
            },
            FieldInfoData {
                name: "DrawEnable",
                name_hash: 1347356004,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainDynamicState, draw_enable),
            },
            FieldInfoData {
                name: "OverrideDrawEnable",
                name_hash: 705365584,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainDynamicState, override_draw_enable),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                name_hash: 3558987183,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VisualTerrainDynamicState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static VISUALTERRAINDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainDynamicState-Array",
    name_hash: 1319166960,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VisualTerrainStaticState {
    pub terrain: Option<LockedTypeObject /* TerrainBaseAsset */>,
    pub decals_resource_handle: super::world_base::ResourceRefHandle,
    pub settings: Option<LockedTypeObject /* VisualTerrainBaseSettings */>,
    pub field_flag_changed0: u8,
}

pub trait VisualTerrainStaticStateTrait: TypeObject {
    fn terrain(&self) -> &Option<LockedTypeObject /* TerrainBaseAsset */>;
    fn terrain_mut(&mut self) -> &mut Option<LockedTypeObject /* TerrainBaseAsset */>;
    fn decals_resource_handle(&self) -> &super::world_base::ResourceRefHandle;
    fn decals_resource_handle_mut(&mut self) -> &mut super::world_base::ResourceRefHandle;
    fn settings(&self) -> &Option<LockedTypeObject /* VisualTerrainBaseSettings */>;
    fn settings_mut(&mut self) -> &mut Option<LockedTypeObject /* VisualTerrainBaseSettings */>;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl VisualTerrainStaticStateTrait for VisualTerrainStaticState {
    fn terrain(&self) -> &Option<LockedTypeObject /* TerrainBaseAsset */> {
        &self.terrain
    }
    fn terrain_mut(&mut self) -> &mut Option<LockedTypeObject /* TerrainBaseAsset */> {
        &mut self.terrain
    }
    fn decals_resource_handle(&self) -> &super::world_base::ResourceRefHandle {
        &self.decals_resource_handle
    }
    fn decals_resource_handle_mut(&mut self) -> &mut super::world_base::ResourceRefHandle {
        &mut self.decals_resource_handle
    }
    fn settings(&self) -> &Option<LockedTypeObject /* VisualTerrainBaseSettings */> {
        &self.settings
    }
    fn settings_mut(&mut self) -> &mut Option<LockedTypeObject /* VisualTerrainBaseSettings */> {
        &mut self.settings
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static VISUALTERRAINSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainStaticState",
    name_hash: 286008137,
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualTerrainStaticState as Default>::default())),
            create_boxed: || Box::new(<VisualTerrainStaticState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Terrain",
                name_hash: 3173545970,
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainBaseAsset",
                rust_offset: offset_of!(VisualTerrainStaticState, terrain),
            },
            FieldInfoData {
                name: "DecalsResourceHandle",
                name_hash: 335042489,
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRefHandle",
                rust_offset: offset_of!(VisualTerrainStaticState, decals_resource_handle),
            },
            FieldInfoData {
                name: "Settings",
                name_hash: 649772672,
                flags: MemberInfoFlags::new(0),
                field_type: "VisualTerrainBaseSettings",
                rust_offset: offset_of!(VisualTerrainStaticState, settings),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static VISUALTERRAINSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainStaticState-Array",
    name_hash: 2030987645,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VisualTerrainHandle {
}

pub trait VisualTerrainHandleTrait: TypeObject {
}

impl VisualTerrainHandleTrait for VisualTerrainHandle {
}

pub static VISUALTERRAINHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainHandle",
    name_hash: 881839212,
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualTerrainHandle as Default>::default())),
            create_boxed: || Box::new(<VisualTerrainHandle as Default>::default()),
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


pub static VISUALTERRAINHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainHandle-Array",
    name_hash: 2634091608,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VisualTerrainBaseSettings {
    pub _glacier_base: super::core::DataContainer,
}

pub trait VisualTerrainBaseSettingsTrait: super::core::DataContainerTrait {
}

impl VisualTerrainBaseSettingsTrait for VisualTerrainBaseSettings {
}

impl super::core::DataContainerTrait for VisualTerrainBaseSettings {
}

pub static VISUALTERRAINBASESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainBaseSettings",
    name_hash: 1956583894,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(VisualTerrainBaseSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualTerrainBaseSettings as Default>::default())),
            create_boxed: || Box::new(<VisualTerrainBaseSettings as Default>::default()),
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


pub static VISUALTERRAINBASESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainBaseSettings-Array",
    name_hash: 1407778658,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualTerrainBaseSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn data_load_job_count_mut(&mut self) -> &mut u32;
    fn active_free_streaming_data_load_job_count(&self) -> &u32;
    fn active_free_streaming_data_load_job_count_mut(&mut self) -> &mut u32;
    fn load_occluder_data_enable(&self) -> &bool;
    fn load_occluder_data_enable_mut(&mut self) -> &mut bool;
    fn additional_blurriness(&self) -> &u32;
    fn additional_blurriness_mut(&mut self) -> &mut u32;
    fn invisible_detail_reduction_factor(&self) -> &f32;
    fn invisible_detail_reduction_factor_mut(&mut self) -> &mut f32;
    fn occluded_detail_reduction_factor(&self) -> &f32;
    fn occluded_detail_reduction_factor_mut(&mut self) -> &mut f32;
    fn keep_pool_full_enable(&self) -> &bool;
    fn keep_pool_full_enable_mut(&mut self) -> &mut bool;
    fn heightfield_atlas_sample_count_x_factor(&self) -> &u32;
    fn heightfield_atlas_sample_count_x_factor_mut(&mut self) -> &mut u32;
    fn heightfield_atlas_sample_count_y_factor(&self) -> &u32;
    fn heightfield_atlas_sample_count_y_factor_mut(&mut self) -> &mut u32;
    fn mask_atlas_sample_count_x_factor(&self) -> &u32;
    fn mask_atlas_sample_count_x_factor_mut(&mut self) -> &mut u32;
    fn mask_atlas_sample_count_y_factor(&self) -> &u32;
    fn mask_atlas_sample_count_y_factor_mut(&mut self) -> &mut u32;
    fn mask_additional_blurriness(&self) -> &u32;
    fn mask_additional_blurriness_mut(&mut self) -> &mut u32;
    fn color_atlas_sample_count_x_factor(&self) -> &u32;
    fn color_atlas_sample_count_x_factor_mut(&mut self) -> &mut u32;
    fn color_atlas_sample_count_y_factor(&self) -> &u32;
    fn color_atlas_sample_count_y_factor_mut(&mut self) -> &mut u32;
    fn color_additional_blurriness(&self) -> &u32;
    fn color_additional_blurriness_mut(&mut self) -> &mut u32;
}

impl TerrainStreamingSettingsTrait for TerrainStreamingSettings {
    fn data_load_job_count(&self) -> &u32 {
        &self.data_load_job_count
    }
    fn data_load_job_count_mut(&mut self) -> &mut u32 {
        &mut self.data_load_job_count
    }
    fn active_free_streaming_data_load_job_count(&self) -> &u32 {
        &self.active_free_streaming_data_load_job_count
    }
    fn active_free_streaming_data_load_job_count_mut(&mut self) -> &mut u32 {
        &mut self.active_free_streaming_data_load_job_count
    }
    fn load_occluder_data_enable(&self) -> &bool {
        &self.load_occluder_data_enable
    }
    fn load_occluder_data_enable_mut(&mut self) -> &mut bool {
        &mut self.load_occluder_data_enable
    }
    fn additional_blurriness(&self) -> &u32 {
        &self.additional_blurriness
    }
    fn additional_blurriness_mut(&mut self) -> &mut u32 {
        &mut self.additional_blurriness
    }
    fn invisible_detail_reduction_factor(&self) -> &f32 {
        &self.invisible_detail_reduction_factor
    }
    fn invisible_detail_reduction_factor_mut(&mut self) -> &mut f32 {
        &mut self.invisible_detail_reduction_factor
    }
    fn occluded_detail_reduction_factor(&self) -> &f32 {
        &self.occluded_detail_reduction_factor
    }
    fn occluded_detail_reduction_factor_mut(&mut self) -> &mut f32 {
        &mut self.occluded_detail_reduction_factor
    }
    fn keep_pool_full_enable(&self) -> &bool {
        &self.keep_pool_full_enable
    }
    fn keep_pool_full_enable_mut(&mut self) -> &mut bool {
        &mut self.keep_pool_full_enable
    }
    fn heightfield_atlas_sample_count_x_factor(&self) -> &u32 {
        &self.heightfield_atlas_sample_count_x_factor
    }
    fn heightfield_atlas_sample_count_x_factor_mut(&mut self) -> &mut u32 {
        &mut self.heightfield_atlas_sample_count_x_factor
    }
    fn heightfield_atlas_sample_count_y_factor(&self) -> &u32 {
        &self.heightfield_atlas_sample_count_y_factor
    }
    fn heightfield_atlas_sample_count_y_factor_mut(&mut self) -> &mut u32 {
        &mut self.heightfield_atlas_sample_count_y_factor
    }
    fn mask_atlas_sample_count_x_factor(&self) -> &u32 {
        &self.mask_atlas_sample_count_x_factor
    }
    fn mask_atlas_sample_count_x_factor_mut(&mut self) -> &mut u32 {
        &mut self.mask_atlas_sample_count_x_factor
    }
    fn mask_atlas_sample_count_y_factor(&self) -> &u32 {
        &self.mask_atlas_sample_count_y_factor
    }
    fn mask_atlas_sample_count_y_factor_mut(&mut self) -> &mut u32 {
        &mut self.mask_atlas_sample_count_y_factor
    }
    fn mask_additional_blurriness(&self) -> &u32 {
        &self.mask_additional_blurriness
    }
    fn mask_additional_blurriness_mut(&mut self) -> &mut u32 {
        &mut self.mask_additional_blurriness
    }
    fn color_atlas_sample_count_x_factor(&self) -> &u32 {
        &self.color_atlas_sample_count_x_factor
    }
    fn color_atlas_sample_count_x_factor_mut(&mut self) -> &mut u32 {
        &mut self.color_atlas_sample_count_x_factor
    }
    fn color_atlas_sample_count_y_factor(&self) -> &u32 {
        &self.color_atlas_sample_count_y_factor
    }
    fn color_atlas_sample_count_y_factor_mut(&mut self) -> &mut u32 {
        &mut self.color_atlas_sample_count_y_factor
    }
    fn color_additional_blurriness(&self) -> &u32 {
        &self.color_additional_blurriness
    }
    fn color_additional_blurriness_mut(&mut self) -> &mut u32 {
        &mut self.color_additional_blurriness
    }
}

impl super::core::DataContainerTrait for TerrainStreamingSettings {
}

pub static TERRAINSTREAMINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingSettings",
    name_hash: 324263211,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TerrainStreamingSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainStreamingSettings as Default>::default())),
            create_boxed: || Box::new(<TerrainStreamingSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DataLoadJobCount",
                name_hash: 2206629719,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, data_load_job_count),
            },
            FieldInfoData {
                name: "ActiveFreeStreamingDataLoadJobCount",
                name_hash: 1513849907,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, active_free_streaming_data_load_job_count),
            },
            FieldInfoData {
                name: "LoadOccluderDataEnable",
                name_hash: 1587906935,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainStreamingSettings, load_occluder_data_enable),
            },
            FieldInfoData {
                name: "AdditionalBlurriness",
                name_hash: 3610471397,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, additional_blurriness),
            },
            FieldInfoData {
                name: "InvisibleDetailReductionFactor",
                name_hash: 770364681,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainStreamingSettings, invisible_detail_reduction_factor),
            },
            FieldInfoData {
                name: "OccludedDetailReductionFactor",
                name_hash: 1482388499,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainStreamingSettings, occluded_detail_reduction_factor),
            },
            FieldInfoData {
                name: "KeepPoolFullEnable",
                name_hash: 2806586800,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainStreamingSettings, keep_pool_full_enable),
            },
            FieldInfoData {
                name: "HeightfieldAtlasSampleCountXFactor",
                name_hash: 2700717027,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, heightfield_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "HeightfieldAtlasSampleCountYFactor",
                name_hash: 3835784866,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, heightfield_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "MaskAtlasSampleCountXFactor",
                name_hash: 3363793834,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, mask_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "MaskAtlasSampleCountYFactor",
                name_hash: 2228725995,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, mask_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "MaskAdditionalBlurriness",
                name_hash: 1316541521,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, mask_additional_blurriness),
            },
            FieldInfoData {
                name: "ColorAtlasSampleCountXFactor",
                name_hash: 2883811075,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, color_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "ColorAtlasSampleCountYFactor",
                name_hash: 3608111938,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainStreamingSettings, color_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "ColorAdditionalBlurriness",
                name_hash: 3507421848,
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


pub static TERRAINSTREAMINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingSettings-Array",
    name_hash: 886226079,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainStreamingSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainQuadDecalData {
    pub _glacier_base: VisualVectorShapeData,
    pub shader2d: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader2d_mesh_scattering_mask: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader2d_single_layer_mask: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader3d_z_only: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader2d_displacement: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub stick_to_terrain: bool,
    pub user_masks: super::core::Vec4,
    pub tangent_space_enable: bool,
    pub atlas_tile_template: Option<LockedTypeObject /* TerrainQuadDecalAtlasTileTemplateData */>,
    pub atlas_tile: TerrainQuadDecalAtlasTile,
}

pub trait TerrainQuadDecalDataTrait: VisualVectorShapeDataTrait {
    fn shader2d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_mesh_scattering_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_mesh_scattering_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_single_layer_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_single_layer_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader3d_z_only(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader3d_z_only_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_displacement(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_displacement_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn stick_to_terrain(&self) -> &bool;
    fn stick_to_terrain_mut(&mut self) -> &mut bool;
    fn user_masks(&self) -> &super::core::Vec4;
    fn user_masks_mut(&mut self) -> &mut super::core::Vec4;
    fn tangent_space_enable(&self) -> &bool;
    fn tangent_space_enable_mut(&mut self) -> &mut bool;
    fn atlas_tile_template(&self) -> &Option<LockedTypeObject /* TerrainQuadDecalAtlasTileTemplateData */>;
    fn atlas_tile_template_mut(&mut self) -> &mut Option<LockedTypeObject /* TerrainQuadDecalAtlasTileTemplateData */>;
    fn atlas_tile(&self) -> &TerrainQuadDecalAtlasTile;
    fn atlas_tile_mut(&mut self) -> &mut TerrainQuadDecalAtlasTile;
}

impl TerrainQuadDecalDataTrait for TerrainQuadDecalData {
    fn shader2d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d
    }
    fn shader2d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d
    }
    fn shader2d_mesh_scattering_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d_mesh_scattering_mask
    }
    fn shader2d_mesh_scattering_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d_mesh_scattering_mask
    }
    fn shader2d_single_layer_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d_single_layer_mask
    }
    fn shader2d_single_layer_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d_single_layer_mask
    }
    fn shader3d_z_only(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader3d_z_only
    }
    fn shader3d_z_only_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader3d_z_only
    }
    fn shader2d_displacement(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d_displacement
    }
    fn shader2d_displacement_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d_displacement
    }
    fn stick_to_terrain(&self) -> &bool {
        &self.stick_to_terrain
    }
    fn stick_to_terrain_mut(&mut self) -> &mut bool {
        &mut self.stick_to_terrain
    }
    fn user_masks(&self) -> &super::core::Vec4 {
        &self.user_masks
    }
    fn user_masks_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.user_masks
    }
    fn tangent_space_enable(&self) -> &bool {
        &self.tangent_space_enable
    }
    fn tangent_space_enable_mut(&mut self) -> &mut bool {
        &mut self.tangent_space_enable
    }
    fn atlas_tile_template(&self) -> &Option<LockedTypeObject /* TerrainQuadDecalAtlasTileTemplateData */> {
        &self.atlas_tile_template
    }
    fn atlas_tile_template_mut(&mut self) -> &mut Option<LockedTypeObject /* TerrainQuadDecalAtlasTileTemplateData */> {
        &mut self.atlas_tile_template
    }
    fn atlas_tile(&self) -> &TerrainQuadDecalAtlasTile {
        &self.atlas_tile
    }
    fn atlas_tile_mut(&mut self) -> &mut TerrainQuadDecalAtlasTile {
        &mut self.atlas_tile
    }
}

impl VisualVectorShapeDataTrait for TerrainQuadDecalData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn error_tolerance_mut(&mut self) -> &mut f32 {
        self._glacier_base.error_tolerance_mut()
    }
    fn shader3d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d()
    }
    fn shader3d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d_mut()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn draw_order_index_mut(&mut self) -> &mut u32 {
        self._glacier_base.draw_order_index_mut()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn tessellation_triangle_size_mut(&mut self) -> &mut f32 {
        self._glacier_base.tessellation_triangle_size_mut()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
    fn split_to_match_heightfield_mut(&mut self) -> &mut bool {
        self._glacier_base.split_to_match_heightfield_mut()
    }
}

impl super::entity::VectorShapeDataTrait for TerrainQuadDecalData {
    fn points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points()
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points_mut()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn tension_mut(&mut self) -> &mut f32 {
        self._glacier_base.tension_mut()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn is_closed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_closed_mut()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_roll_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_roll_mut()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
    fn allow_yaw_pitch_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_yaw_pitch_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TerrainQuadDecalData {
}

impl super::core::DataContainerTrait for TerrainQuadDecalData {
}

pub static TERRAINQUADDECALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalData",
    name_hash: 2226659916,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        super_class_offset: offset_of!(TerrainQuadDecalData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainQuadDecalData as Default>::default())),
            create_boxed: || Box::new(<TerrainQuadDecalData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Shader2d",
                name_hash: 596681178,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d),
            },
            FieldInfoData {
                name: "Shader2dMeshScatteringMask",
                name_hash: 2043204219,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d_mesh_scattering_mask),
            },
            FieldInfoData {
                name: "Shader2dSingleLayerMask",
                name_hash: 413897143,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d_single_layer_mask),
            },
            FieldInfoData {
                name: "Shader3dZOnly",
                name_hash: 585356309,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainQuadDecalData, shader3d_z_only),
            },
            FieldInfoData {
                name: "Shader2dDisplacement",
                name_hash: 4098644109,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainQuadDecalData, shader2d_displacement),
            },
            FieldInfoData {
                name: "StickToTerrain",
                name_hash: 633294575,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainQuadDecalData, stick_to_terrain),
            },
            FieldInfoData {
                name: "UserMasks",
                name_hash: 1589111411,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(TerrainQuadDecalData, user_masks),
            },
            FieldInfoData {
                name: "TangentSpaceEnable",
                name_hash: 397952163,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainQuadDecalData, tangent_space_enable),
            },
            FieldInfoData {
                name: "AtlasTileTemplate",
                name_hash: 3181192042,
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainQuadDecalAtlasTileTemplateData",
                rust_offset: offset_of!(TerrainQuadDecalData, atlas_tile_template),
            },
            FieldInfoData {
                name: "AtlasTile",
                name_hash: 3027817338,
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


pub static TERRAINQUADDECALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalData-Array",
    name_hash: 2054849272,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainQuadDecalData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainQuadDecalAtlasTileTemplateData {
    pub _glacier_base: super::core::Asset,
    pub atlas_tile: TerrainQuadDecalAtlasTile,
}

pub trait TerrainQuadDecalAtlasTileTemplateDataTrait: super::core::AssetTrait {
    fn atlas_tile(&self) -> &TerrainQuadDecalAtlasTile;
    fn atlas_tile_mut(&mut self) -> &mut TerrainQuadDecalAtlasTile;
}

impl TerrainQuadDecalAtlasTileTemplateDataTrait for TerrainQuadDecalAtlasTileTemplateData {
    fn atlas_tile(&self) -> &TerrainQuadDecalAtlasTile {
        &self.atlas_tile
    }
    fn atlas_tile_mut(&mut self) -> &mut TerrainQuadDecalAtlasTile {
        &mut self.atlas_tile
    }
}

impl super::core::AssetTrait for TerrainQuadDecalAtlasTileTemplateData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TerrainQuadDecalAtlasTileTemplateData {
}

pub static TERRAINQUADDECALATLASTILETEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTileTemplateData",
    name_hash: 2147093827,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(TerrainQuadDecalAtlasTileTemplateData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainQuadDecalAtlasTileTemplateData as Default>::default())),
            create_boxed: || Box::new(<TerrainQuadDecalAtlasTileTemplateData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AtlasTile",
                name_hash: 3027817338,
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


pub static TERRAINQUADDECALATLASTILETEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTileTemplateData-Array",
    name_hash: 3094392951,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainQuadDecalAtlasTileTemplateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn tile_index_x_mut(&mut self) -> &mut u32;
    fn tile_index_y(&self) -> &u32;
    fn tile_index_y_mut(&mut self) -> &mut u32;
    fn tile_count_x(&self) -> &u32;
    fn tile_count_x_mut(&mut self) -> &mut u32;
    fn tile_count_y(&self) -> &u32;
    fn tile_count_y_mut(&mut self) -> &mut u32;
    fn flip_x(&self) -> &bool;
    fn flip_x_mut(&mut self) -> &mut bool;
    fn flip_y(&self) -> &bool;
    fn flip_y_mut(&mut self) -> &mut bool;
}

impl TerrainQuadDecalAtlasTileTrait for TerrainQuadDecalAtlasTile {
    fn tile_index_x(&self) -> &u32 {
        &self.tile_index_x
    }
    fn tile_index_x_mut(&mut self) -> &mut u32 {
        &mut self.tile_index_x
    }
    fn tile_index_y(&self) -> &u32 {
        &self.tile_index_y
    }
    fn tile_index_y_mut(&mut self) -> &mut u32 {
        &mut self.tile_index_y
    }
    fn tile_count_x(&self) -> &u32 {
        &self.tile_count_x
    }
    fn tile_count_x_mut(&mut self) -> &mut u32 {
        &mut self.tile_count_x
    }
    fn tile_count_y(&self) -> &u32 {
        &self.tile_count_y
    }
    fn tile_count_y_mut(&mut self) -> &mut u32 {
        &mut self.tile_count_y
    }
    fn flip_x(&self) -> &bool {
        &self.flip_x
    }
    fn flip_x_mut(&mut self) -> &mut bool {
        &mut self.flip_x
    }
    fn flip_y(&self) -> &bool {
        &self.flip_y
    }
    fn flip_y_mut(&mut self) -> &mut bool {
        &mut self.flip_y
    }
}

pub static TERRAINQUADDECALATLASTILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTile",
    name_hash: 566325539,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainQuadDecalAtlasTile as Default>::default())),
            create_boxed: || Box::new(<TerrainQuadDecalAtlasTile as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TileIndexX",
                name_hash: 2534612119,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_index_x),
            },
            FieldInfoData {
                name: "TileIndexY",
                name_hash: 2534612118,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_index_y),
            },
            FieldInfoData {
                name: "TileCountX",
                name_hash: 2473222698,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_count_x),
            },
            FieldInfoData {
                name: "TileCountY",
                name_hash: 2473222699,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, tile_count_y),
            },
            FieldInfoData {
                name: "FlipX",
                name_hash: 207056974,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainQuadDecalAtlasTile, flip_x),
            },
            FieldInfoData {
                name: "FlipY",
                name_hash: 207056975,
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


pub static TERRAINQUADDECALATLASTILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainQuadDecalAtlasTile-Array",
    name_hash: 161477783,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainQuadDecalAtlasTile"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainFillDecalData {
    pub _glacier_base: VisualVectorShapeData,
    pub shader2d: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader2d_mesh_scattering_mask: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader2d_single_layer_mask: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader3d_z_only: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
}

pub trait TerrainFillDecalDataTrait: VisualVectorShapeDataTrait {
    fn shader2d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_mesh_scattering_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_mesh_scattering_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_single_layer_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_single_layer_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader3d_z_only(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader3d_z_only_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
}

impl TerrainFillDecalDataTrait for TerrainFillDecalData {
    fn shader2d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d
    }
    fn shader2d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d
    }
    fn shader2d_mesh_scattering_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d_mesh_scattering_mask
    }
    fn shader2d_mesh_scattering_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d_mesh_scattering_mask
    }
    fn shader2d_single_layer_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d_single_layer_mask
    }
    fn shader2d_single_layer_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d_single_layer_mask
    }
    fn shader3d_z_only(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader3d_z_only
    }
    fn shader3d_z_only_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader3d_z_only
    }
}

impl VisualVectorShapeDataTrait for TerrainFillDecalData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn error_tolerance_mut(&mut self) -> &mut f32 {
        self._glacier_base.error_tolerance_mut()
    }
    fn shader3d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d()
    }
    fn shader3d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d_mut()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn draw_order_index_mut(&mut self) -> &mut u32 {
        self._glacier_base.draw_order_index_mut()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn tessellation_triangle_size_mut(&mut self) -> &mut f32 {
        self._glacier_base.tessellation_triangle_size_mut()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
    fn split_to_match_heightfield_mut(&mut self) -> &mut bool {
        self._glacier_base.split_to_match_heightfield_mut()
    }
}

impl super::entity::VectorShapeDataTrait for TerrainFillDecalData {
    fn points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points()
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points_mut()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn tension_mut(&mut self) -> &mut f32 {
        self._glacier_base.tension_mut()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn is_closed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_closed_mut()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_roll_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_roll_mut()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
    fn allow_yaw_pitch_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_yaw_pitch_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TerrainFillDecalData {
}

impl super::core::DataContainerTrait for TerrainFillDecalData {
}

pub static TERRAINFILLDECALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainFillDecalData",
    name_hash: 1970219650,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        super_class_offset: offset_of!(TerrainFillDecalData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainFillDecalData as Default>::default())),
            create_boxed: || Box::new(<TerrainFillDecalData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Shader2d",
                name_hash: 596681178,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainFillDecalData, shader2d),
            },
            FieldInfoData {
                name: "Shader2dMeshScatteringMask",
                name_hash: 2043204219,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainFillDecalData, shader2d_mesh_scattering_mask),
            },
            FieldInfoData {
                name: "Shader2dSingleLayerMask",
                name_hash: 413897143,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(TerrainFillDecalData, shader2d_single_layer_mask),
            },
            FieldInfoData {
                name: "Shader3dZOnly",
                name_hash: 585356309,
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


pub static TERRAINFILLDECALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainFillDecalData-Array",
    name_hash: 370141622,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainFillDecalData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn error_tolerance_mut(&mut self) -> &mut f32 {
        self._glacier_base.error_tolerance_mut()
    }
    fn shader3d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d()
    }
    fn shader3d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d_mut()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn draw_order_index_mut(&mut self) -> &mut u32 {
        self._glacier_base.draw_order_index_mut()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn tessellation_triangle_size_mut(&mut self) -> &mut f32 {
        self._glacier_base.tessellation_triangle_size_mut()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
    fn split_to_match_heightfield_mut(&mut self) -> &mut bool {
        self._glacier_base.split_to_match_heightfield_mut()
    }
}

impl super::entity::VectorShapeDataTrait for LakeData {
    fn points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points()
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points_mut()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn tension_mut(&mut self) -> &mut f32 {
        self._glacier_base.tension_mut()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn is_closed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_closed_mut()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_roll_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_roll_mut()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
    fn allow_yaw_pitch_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_yaw_pitch_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LakeData {
}

impl super::core::DataContainerTrait for LakeData {
}

pub static LAKEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LakeData",
    name_hash: 3686446134,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        super_class_offset: offset_of!(LakeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LakeData as Default>::default())),
            create_boxed: || Box::new(<LakeData as Default>::default()),
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


pub static LAKEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LakeData-Array",
    name_hash: 876259458,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("LakeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RiverData {
    pub _glacier_base: RibbonData,
}

pub trait RiverDataTrait: RibbonDataTrait {
}

impl RiverDataTrait for RiverData {
}

impl RibbonDataTrait for RiverData {
    fn ribbon_points(&self) -> &Vec<BoxedTypeObject /* RibbonPointData */> {
        self._glacier_base.ribbon_points()
    }
    fn ribbon_points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* RibbonPointData */> {
        self._glacier_base.ribbon_points_mut()
    }
}

impl VisualVectorShapeDataTrait for RiverData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn error_tolerance_mut(&mut self) -> &mut f32 {
        self._glacier_base.error_tolerance_mut()
    }
    fn shader3d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d()
    }
    fn shader3d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d_mut()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn draw_order_index_mut(&mut self) -> &mut u32 {
        self._glacier_base.draw_order_index_mut()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn tessellation_triangle_size_mut(&mut self) -> &mut f32 {
        self._glacier_base.tessellation_triangle_size_mut()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
    fn split_to_match_heightfield_mut(&mut self) -> &mut bool {
        self._glacier_base.split_to_match_heightfield_mut()
    }
}

impl super::entity::VectorShapeDataTrait for RiverData {
    fn points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points()
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points_mut()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn tension_mut(&mut self) -> &mut f32 {
        self._glacier_base.tension_mut()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn is_closed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_closed_mut()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_roll_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_roll_mut()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
    fn allow_yaw_pitch_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_yaw_pitch_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RiverData {
}

impl super::core::DataContainerTrait for RiverData {
}

pub static RIVERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RiverData",
    name_hash: 2813589135,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RIBBONDATA_TYPE_INFO),
        super_class_offset: offset_of!(RiverData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RiverData as Default>::default())),
            create_boxed: || Box::new(<RiverData as Default>::default()),
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


pub static RIVERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RiverData-Array",
    name_hash: 1943262011,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RiverData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RoadData {
    pub _glacier_base: RibbonData,
    pub shader2d: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader2d_mesh_scattering_mask: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader2d_single_layer_mask: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader3d_z_only: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub shader2d_displacement: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub stick_to_terrain: bool,
    pub uv_tile_factor: f32,
    pub tangent_space_enable: bool,
}

pub trait RoadDataTrait: RibbonDataTrait {
    fn shader2d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_mesh_scattering_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_mesh_scattering_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_single_layer_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_single_layer_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader3d_z_only(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader3d_z_only_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_displacement(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader2d_displacement_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn stick_to_terrain(&self) -> &bool;
    fn stick_to_terrain_mut(&mut self) -> &mut bool;
    fn uv_tile_factor(&self) -> &f32;
    fn uv_tile_factor_mut(&mut self) -> &mut f32;
    fn tangent_space_enable(&self) -> &bool;
    fn tangent_space_enable_mut(&mut self) -> &mut bool;
}

impl RoadDataTrait for RoadData {
    fn shader2d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d
    }
    fn shader2d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d
    }
    fn shader2d_mesh_scattering_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d_mesh_scattering_mask
    }
    fn shader2d_mesh_scattering_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d_mesh_scattering_mask
    }
    fn shader2d_single_layer_mask(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d_single_layer_mask
    }
    fn shader2d_single_layer_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d_single_layer_mask
    }
    fn shader3d_z_only(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader3d_z_only
    }
    fn shader3d_z_only_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader3d_z_only
    }
    fn shader2d_displacement(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader2d_displacement
    }
    fn shader2d_displacement_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader2d_displacement
    }
    fn stick_to_terrain(&self) -> &bool {
        &self.stick_to_terrain
    }
    fn stick_to_terrain_mut(&mut self) -> &mut bool {
        &mut self.stick_to_terrain
    }
    fn uv_tile_factor(&self) -> &f32 {
        &self.uv_tile_factor
    }
    fn uv_tile_factor_mut(&mut self) -> &mut f32 {
        &mut self.uv_tile_factor
    }
    fn tangent_space_enable(&self) -> &bool {
        &self.tangent_space_enable
    }
    fn tangent_space_enable_mut(&mut self) -> &mut bool {
        &mut self.tangent_space_enable
    }
}

impl RibbonDataTrait for RoadData {
    fn ribbon_points(&self) -> &Vec<BoxedTypeObject /* RibbonPointData */> {
        self._glacier_base.ribbon_points()
    }
    fn ribbon_points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* RibbonPointData */> {
        self._glacier_base.ribbon_points_mut()
    }
}

impl VisualVectorShapeDataTrait for RoadData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn error_tolerance_mut(&mut self) -> &mut f32 {
        self._glacier_base.error_tolerance_mut()
    }
    fn shader3d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d()
    }
    fn shader3d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d_mut()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn draw_order_index_mut(&mut self) -> &mut u32 {
        self._glacier_base.draw_order_index_mut()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn tessellation_triangle_size_mut(&mut self) -> &mut f32 {
        self._glacier_base.tessellation_triangle_size_mut()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
    fn split_to_match_heightfield_mut(&mut self) -> &mut bool {
        self._glacier_base.split_to_match_heightfield_mut()
    }
}

impl super::entity::VectorShapeDataTrait for RoadData {
    fn points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points()
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points_mut()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn tension_mut(&mut self) -> &mut f32 {
        self._glacier_base.tension_mut()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn is_closed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_closed_mut()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_roll_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_roll_mut()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
    fn allow_yaw_pitch_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_yaw_pitch_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RoadData {
}

impl super::core::DataContainerTrait for RoadData {
}

pub static ROADDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RoadData",
    name_hash: 488110701,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RIBBONDATA_TYPE_INFO),
        super_class_offset: offset_of!(RoadData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RoadData as Default>::default())),
            create_boxed: || Box::new(<RoadData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Shader2d",
                name_hash: 596681178,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(RoadData, shader2d),
            },
            FieldInfoData {
                name: "Shader2dMeshScatteringMask",
                name_hash: 2043204219,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(RoadData, shader2d_mesh_scattering_mask),
            },
            FieldInfoData {
                name: "Shader2dSingleLayerMask",
                name_hash: 413897143,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(RoadData, shader2d_single_layer_mask),
            },
            FieldInfoData {
                name: "Shader3dZOnly",
                name_hash: 585356309,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(RoadData, shader3d_z_only),
            },
            FieldInfoData {
                name: "Shader2dDisplacement",
                name_hash: 4098644109,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(RoadData, shader2d_displacement),
            },
            FieldInfoData {
                name: "StickToTerrain",
                name_hash: 633294575,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RoadData, stick_to_terrain),
            },
            FieldInfoData {
                name: "UvTileFactor",
                name_hash: 1731623903,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RoadData, uv_tile_factor),
            },
            FieldInfoData {
                name: "TangentSpaceEnable",
                name_hash: 397952163,
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


pub static ROADDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RoadData-Array",
    name_hash: 2841591641,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RoadData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RibbonData {
    pub _glacier_base: VisualVectorShapeData,
    pub ribbon_points: Vec<BoxedTypeObject /* RibbonPointData */>,
}

pub trait RibbonDataTrait: VisualVectorShapeDataTrait {
    fn ribbon_points(&self) -> &Vec<BoxedTypeObject /* RibbonPointData */>;
    fn ribbon_points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* RibbonPointData */>;
}

impl RibbonDataTrait for RibbonData {
    fn ribbon_points(&self) -> &Vec<BoxedTypeObject /* RibbonPointData */> {
        &self.ribbon_points
    }
    fn ribbon_points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* RibbonPointData */> {
        &mut self.ribbon_points
    }
}

impl VisualVectorShapeDataTrait for RibbonData {
    fn error_tolerance(&self) -> &f32 {
        self._glacier_base.error_tolerance()
    }
    fn error_tolerance_mut(&mut self) -> &mut f32 {
        self._glacier_base.error_tolerance_mut()
    }
    fn shader3d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d()
    }
    fn shader3d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        self._glacier_base.shader3d_mut()
    }
    fn draw_order_index(&self) -> &u32 {
        self._glacier_base.draw_order_index()
    }
    fn draw_order_index_mut(&mut self) -> &mut u32 {
        self._glacier_base.draw_order_index_mut()
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        self._glacier_base.tessellation_triangle_size()
    }
    fn tessellation_triangle_size_mut(&mut self) -> &mut f32 {
        self._glacier_base.tessellation_triangle_size_mut()
    }
    fn split_to_match_heightfield(&self) -> &bool {
        self._glacier_base.split_to_match_heightfield()
    }
    fn split_to_match_heightfield_mut(&mut self) -> &mut bool {
        self._glacier_base.split_to_match_heightfield_mut()
    }
}

impl super::entity::VectorShapeDataTrait for RibbonData {
    fn points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points()
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points_mut()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn tension_mut(&mut self) -> &mut f32 {
        self._glacier_base.tension_mut()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn is_closed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_closed_mut()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_roll_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_roll_mut()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
    fn allow_yaw_pitch_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_yaw_pitch_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RibbonData {
}

impl super::core::DataContainerTrait for RibbonData {
}

pub static RIBBONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonData",
    name_hash: 1306758767,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALVECTORSHAPEDATA_TYPE_INFO),
        super_class_offset: offset_of!(RibbonData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RibbonData as Default>::default())),
            create_boxed: || Box::new(<RibbonData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RibbonPoints",
                name_hash: 935977920,
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


pub static RIBBONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonData-Array",
    name_hash: 1261833307,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RibbonData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RibbonPointData {
    pub left: f32,
    pub right: f32,
    pub user_mask_left: super::core::Vec4,
    pub user_mask_right: super::core::Vec4,
}

pub trait RibbonPointDataTrait: TypeObject {
    fn left(&self) -> &f32;
    fn left_mut(&mut self) -> &mut f32;
    fn right(&self) -> &f32;
    fn right_mut(&mut self) -> &mut f32;
    fn user_mask_left(&self) -> &super::core::Vec4;
    fn user_mask_left_mut(&mut self) -> &mut super::core::Vec4;
    fn user_mask_right(&self) -> &super::core::Vec4;
    fn user_mask_right_mut(&mut self) -> &mut super::core::Vec4;
}

impl RibbonPointDataTrait for RibbonPointData {
    fn left(&self) -> &f32 {
        &self.left
    }
    fn left_mut(&mut self) -> &mut f32 {
        &mut self.left
    }
    fn right(&self) -> &f32 {
        &self.right
    }
    fn right_mut(&mut self) -> &mut f32 {
        &mut self.right
    }
    fn user_mask_left(&self) -> &super::core::Vec4 {
        &self.user_mask_left
    }
    fn user_mask_left_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.user_mask_left
    }
    fn user_mask_right(&self) -> &super::core::Vec4 {
        &self.user_mask_right
    }
    fn user_mask_right_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.user_mask_right
    }
}

pub static RIBBONPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonPointData",
    name_hash: 2351552803,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RibbonPointData as Default>::default())),
            create_boxed: || Box::new(<RibbonPointData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Left",
                name_hash: 2089021886,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RibbonPointData, left),
            },
            FieldInfoData {
                name: "Right",
                name_hash: 230390021,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RibbonPointData, right),
            },
            FieldInfoData {
                name: "UserMaskLeft",
                name_hash: 2010169083,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(RibbonPointData, user_mask_left),
            },
            FieldInfoData {
                name: "UserMaskRight",
                name_hash: 1923208736,
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


pub static RIBBONPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RibbonPointData-Array",
    name_hash: 1253465239,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RibbonPointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VisualVectorShapeData {
    pub _glacier_base: super::entity::VectorShapeData,
    pub error_tolerance: f32,
    pub shader3d: Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>,
    pub draw_order_index: u32,
    pub tessellation_triangle_size: f32,
    pub split_to_match_heightfield: bool,
}

pub trait VisualVectorShapeDataTrait: super::entity::VectorShapeDataTrait {
    fn error_tolerance(&self) -> &f32;
    fn error_tolerance_mut(&mut self) -> &mut f32;
    fn shader3d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn shader3d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */>;
    fn draw_order_index(&self) -> &u32;
    fn draw_order_index_mut(&mut self) -> &mut u32;
    fn tessellation_triangle_size(&self) -> &f32;
    fn tessellation_triangle_size_mut(&mut self) -> &mut f32;
    fn split_to_match_heightfield(&self) -> &bool;
    fn split_to_match_heightfield_mut(&mut self) -> &mut bool;
}

impl VisualVectorShapeDataTrait for VisualVectorShapeData {
    fn error_tolerance(&self) -> &f32 {
        &self.error_tolerance
    }
    fn error_tolerance_mut(&mut self) -> &mut f32 {
        &mut self.error_tolerance
    }
    fn shader3d(&self) -> &Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &self.shader3d
    }
    fn shader3d_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::SurfaceShaderBaseAsset */> {
        &mut self.shader3d
    }
    fn draw_order_index(&self) -> &u32 {
        &self.draw_order_index
    }
    fn draw_order_index_mut(&mut self) -> &mut u32 {
        &mut self.draw_order_index
    }
    fn tessellation_triangle_size(&self) -> &f32 {
        &self.tessellation_triangle_size
    }
    fn tessellation_triangle_size_mut(&mut self) -> &mut f32 {
        &mut self.tessellation_triangle_size
    }
    fn split_to_match_heightfield(&self) -> &bool {
        &self.split_to_match_heightfield
    }
    fn split_to_match_heightfield_mut(&mut self) -> &mut bool {
        &mut self.split_to_match_heightfield
    }
}

impl super::entity::VectorShapeDataTrait for VisualVectorShapeData {
    fn points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points()
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points_mut()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn tension_mut(&mut self) -> &mut f32 {
        self._glacier_base.tension_mut()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn is_closed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_closed_mut()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_roll_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_roll_mut()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
    fn allow_yaw_pitch_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_yaw_pitch_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VisualVectorShapeData {
}

impl super::core::DataContainerTrait for VisualVectorShapeData {
}

pub static VISUALVECTORSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualVectorShapeData",
    name_hash: 2703618359,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::VECTORSHAPEDATA_TYPE_INFO),
        super_class_offset: offset_of!(VisualVectorShapeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualVectorShapeData as Default>::default())),
            create_boxed: || Box::new(<VisualVectorShapeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ErrorTolerance",
                name_hash: 3302753588,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualVectorShapeData, error_tolerance),
            },
            FieldInfoData {
                name: "Shader3d",
                name_hash: 596681147,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(VisualVectorShapeData, shader3d),
            },
            FieldInfoData {
                name: "DrawOrderIndex",
                name_hash: 274360149,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualVectorShapeData, draw_order_index),
            },
            FieldInfoData {
                name: "TessellationTriangleSize",
                name_hash: 22509191,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualVectorShapeData, tessellation_triangle_size),
            },
            FieldInfoData {
                name: "SplitToMatchHeightfield",
                name_hash: 477252130,
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


pub static VISUALVECTORSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualVectorShapeData-Array",
    name_hash: 2469849731,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VisualVectorShapeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainShaderParameterBlockDynamicState {
}

pub trait TerrainShaderParameterBlockDynamicStateTrait: TypeObject {
}

impl TerrainShaderParameterBlockDynamicStateTrait for TerrainShaderParameterBlockDynamicState {
}

pub static TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockDynamicState",
    name_hash: 1093877113,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainShaderParameterBlockDynamicState as Default>::default())),
            create_boxed: || Box::new(<TerrainShaderParameterBlockDynamicState as Default>::default()),
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


pub static TERRAINSHADERPARAMETERBLOCKDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockDynamicState-Array",
    name_hash: 1643371597,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainShaderParameterBlockDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainShaderParameterBlockStaticState {
    pub shader_block_handle: super::render_base::ShaderParameterBlockHandle,
    pub field_flag_changed0: u8,
}

pub trait TerrainShaderParameterBlockStaticStateTrait: TypeObject {
    fn shader_block_handle(&self) -> &super::render_base::ShaderParameterBlockHandle;
    fn shader_block_handle_mut(&mut self) -> &mut super::render_base::ShaderParameterBlockHandle;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TerrainShaderParameterBlockStaticStateTrait for TerrainShaderParameterBlockStaticState {
    fn shader_block_handle(&self) -> &super::render_base::ShaderParameterBlockHandle {
        &self.shader_block_handle
    }
    fn shader_block_handle_mut(&mut self) -> &mut super::render_base::ShaderParameterBlockHandle {
        &mut self.shader_block_handle
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static TERRAINSHADERPARAMETERBLOCKSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockStaticState",
    name_hash: 1678295828,
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainShaderParameterBlockStaticState as Default>::default())),
            create_boxed: || Box::new(<TerrainShaderParameterBlockStaticState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ShaderBlockHandle",
                name_hash: 1372241359,
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderParameterBlockHandle",
                rust_offset: offset_of!(TerrainShaderParameterBlockStaticState, shader_block_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static TERRAINSHADERPARAMETERBLOCKSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainShaderParameterBlockStaticState-Array",
    name_hash: 1002446624,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainShaderParameterBlockStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn height_query_cache_size_mut(&mut self) -> &mut u32;
    fn modifiers_enable(&self) -> &bool;
    fn modifiers_enable_mut(&mut self) -> &mut bool;
    fn modifiers_capacity(&self) -> &u32;
    fn modifiers_capacity_mut(&mut self) -> &mut u32;
    fn intersecting_modifiers_max(&self) -> &u32;
    fn intersecting_modifiers_max_mut(&mut self) -> &mut u32;
    fn modifier_slope_max(&self) -> &f32;
    fn modifier_slope_max_mut(&mut self) -> &mut f32;
    fn modifier_depth_factor(&self) -> &f32;
    fn modifier_depth_factor_mut(&mut self) -> &mut f32;
    fn modifiers_applied_per_frame_max(&self) -> &u32;
    fn modifiers_applied_per_frame_max_mut(&mut self) -> &mut u32;
    fn prioritization_on_several_frames(&self) -> &bool;
    fn prioritization_on_several_frames_mut(&mut self) -> &mut bool;
    fn refining_during_prioritization(&self) -> &bool;
    fn refining_during_prioritization_mut(&mut self) -> &mut bool;
    fn refining_during_prioritization_min_priority(&self) -> &f32;
    fn refining_during_prioritization_min_priority_mut(&mut self) -> &mut f32;
}

impl TerrainSettingsTrait for TerrainSettings {
    fn height_query_cache_size(&self) -> &u32 {
        &self.height_query_cache_size
    }
    fn height_query_cache_size_mut(&mut self) -> &mut u32 {
        &mut self.height_query_cache_size
    }
    fn modifiers_enable(&self) -> &bool {
        &self.modifiers_enable
    }
    fn modifiers_enable_mut(&mut self) -> &mut bool {
        &mut self.modifiers_enable
    }
    fn modifiers_capacity(&self) -> &u32 {
        &self.modifiers_capacity
    }
    fn modifiers_capacity_mut(&mut self) -> &mut u32 {
        &mut self.modifiers_capacity
    }
    fn intersecting_modifiers_max(&self) -> &u32 {
        &self.intersecting_modifiers_max
    }
    fn intersecting_modifiers_max_mut(&mut self) -> &mut u32 {
        &mut self.intersecting_modifiers_max
    }
    fn modifier_slope_max(&self) -> &f32 {
        &self.modifier_slope_max
    }
    fn modifier_slope_max_mut(&mut self) -> &mut f32 {
        &mut self.modifier_slope_max
    }
    fn modifier_depth_factor(&self) -> &f32 {
        &self.modifier_depth_factor
    }
    fn modifier_depth_factor_mut(&mut self) -> &mut f32 {
        &mut self.modifier_depth_factor
    }
    fn modifiers_applied_per_frame_max(&self) -> &u32 {
        &self.modifiers_applied_per_frame_max
    }
    fn modifiers_applied_per_frame_max_mut(&mut self) -> &mut u32 {
        &mut self.modifiers_applied_per_frame_max
    }
    fn prioritization_on_several_frames(&self) -> &bool {
        &self.prioritization_on_several_frames
    }
    fn prioritization_on_several_frames_mut(&mut self) -> &mut bool {
        &mut self.prioritization_on_several_frames
    }
    fn refining_during_prioritization(&self) -> &bool {
        &self.refining_during_prioritization
    }
    fn refining_during_prioritization_mut(&mut self) -> &mut bool {
        &mut self.refining_during_prioritization
    }
    fn refining_during_prioritization_min_priority(&self) -> &f32 {
        &self.refining_during_prioritization_min_priority
    }
    fn refining_during_prioritization_min_priority_mut(&mut self) -> &mut f32 {
        &mut self.refining_during_prioritization_min_priority
    }
}

impl super::core::DataContainerTrait for TerrainSettings {
}

pub static TERRAINSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSettings",
    name_hash: 3254599735,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TerrainSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainSettings as Default>::default())),
            create_boxed: || Box::new(<TerrainSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HeightQueryCacheSize",
                name_hash: 360991385,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainSettings, height_query_cache_size),
            },
            FieldInfoData {
                name: "ModifiersEnable",
                name_hash: 3614191008,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainSettings, modifiers_enable),
            },
            FieldInfoData {
                name: "ModifiersCapacity",
                name_hash: 3948255029,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainSettings, modifiers_capacity),
            },
            FieldInfoData {
                name: "IntersectingModifiersMax",
                name_hash: 3775873328,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainSettings, intersecting_modifiers_max),
            },
            FieldInfoData {
                name: "ModifierSlopeMax",
                name_hash: 2295593955,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainSettings, modifier_slope_max),
            },
            FieldInfoData {
                name: "ModifierDepthFactor",
                name_hash: 2305338610,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainSettings, modifier_depth_factor),
            },
            FieldInfoData {
                name: "ModifiersAppliedPerFrameMax",
                name_hash: 2590709066,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainSettings, modifiers_applied_per_frame_max),
            },
            FieldInfoData {
                name: "PrioritizationOnSeveralFrames",
                name_hash: 2324831317,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainSettings, prioritization_on_several_frames),
            },
            FieldInfoData {
                name: "RefiningDuringPrioritization",
                name_hash: 2401351605,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainSettings, refining_during_prioritization),
            },
            FieldInfoData {
                name: "RefiningDuringPrioritizationMinPriority",
                name_hash: 833921773,
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


pub static TERRAINSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSettings-Array",
    name_hash: 4060969347,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainModificationDynamicState {
    pub burn_map: Vec<u8>,
    pub field_flag_changed0: u8,
}

pub trait TerrainModificationDynamicStateTrait: TypeObject {
    fn burn_map(&self) -> &Vec<u8>;
    fn burn_map_mut(&mut self) -> &mut Vec<u8>;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TerrainModificationDynamicStateTrait for TerrainModificationDynamicState {
    fn burn_map(&self) -> &Vec<u8> {
        &self.burn_map
    }
    fn burn_map_mut(&mut self) -> &mut Vec<u8> {
        &mut self.burn_map
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static TERRAINMODIFICATIONDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationDynamicState",
    name_hash: 3908319534,
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainModificationDynamicState as Default>::default())),
            create_boxed: || Box::new(<TerrainModificationDynamicState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BurnMap",
                name_hash: 2742368018,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint8-Array",
                rust_offset: offset_of!(TerrainModificationDynamicState, burn_map),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static TERRAINMODIFICATIONDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationDynamicState-Array",
    name_hash: 4239425946,
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
    name_hash: 3452491092,
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


pub static TERRAINMODIFICATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainModificationType-Array",
    name_hash: 3575177696,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainModificationType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainEditingDynamicState {
}

pub trait TerrainEditingDynamicStateTrait: TypeObject {
}

impl TerrainEditingDynamicStateTrait for TerrainEditingDynamicState {
}

pub static TERRAINEDITINGDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingDynamicState",
    name_hash: 1243982188,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainEditingDynamicState as Default>::default())),
            create_boxed: || Box::new(<TerrainEditingDynamicState as Default>::default()),
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


pub static TERRAINEDITINGDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingDynamicState-Array",
    name_hash: 3569920344,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainEditingDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainEditingStaticState {
    pub event_type: TerrainEditingEvent,
    pub mesh_scattering_type: Option<LockedTypeObject /* TerrainMeshScatteringType */>,
    pub mesh_scattering_field_number: u32,
    pub decals: Vec<Option<LockedTypeObject /* VisualVectorShapeData */>>,
    pub field_flag_changed0: u8,
}

pub trait TerrainEditingStaticStateTrait: TypeObject {
    fn event_type(&self) -> &TerrainEditingEvent;
    fn event_type_mut(&mut self) -> &mut TerrainEditingEvent;
    fn mesh_scattering_type(&self) -> &Option<LockedTypeObject /* TerrainMeshScatteringType */>;
    fn mesh_scattering_type_mut(&mut self) -> &mut Option<LockedTypeObject /* TerrainMeshScatteringType */>;
    fn mesh_scattering_field_number(&self) -> &u32;
    fn mesh_scattering_field_number_mut(&mut self) -> &mut u32;
    fn decals(&self) -> &Vec<Option<LockedTypeObject /* VisualVectorShapeData */>>;
    fn decals_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* VisualVectorShapeData */>>;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TerrainEditingStaticStateTrait for TerrainEditingStaticState {
    fn event_type(&self) -> &TerrainEditingEvent {
        &self.event_type
    }
    fn event_type_mut(&mut self) -> &mut TerrainEditingEvent {
        &mut self.event_type
    }
    fn mesh_scattering_type(&self) -> &Option<LockedTypeObject /* TerrainMeshScatteringType */> {
        &self.mesh_scattering_type
    }
    fn mesh_scattering_type_mut(&mut self) -> &mut Option<LockedTypeObject /* TerrainMeshScatteringType */> {
        &mut self.mesh_scattering_type
    }
    fn mesh_scattering_field_number(&self) -> &u32 {
        &self.mesh_scattering_field_number
    }
    fn mesh_scattering_field_number_mut(&mut self) -> &mut u32 {
        &mut self.mesh_scattering_field_number
    }
    fn decals(&self) -> &Vec<Option<LockedTypeObject /* VisualVectorShapeData */>> {
        &self.decals
    }
    fn decals_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* VisualVectorShapeData */>> {
        &mut self.decals
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static TERRAINEDITINGSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingStaticState",
    name_hash: 2519809953,
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainEditingStaticState as Default>::default())),
            create_boxed: || Box::new(<TerrainEditingStaticState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EventType",
                name_hash: 4133155473,
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainEditingEvent",
                rust_offset: offset_of!(TerrainEditingStaticState, event_type),
            },
            FieldInfoData {
                name: "MeshScatteringType",
                name_hash: 1570628328,
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainMeshScatteringType",
                rust_offset: offset_of!(TerrainEditingStaticState, mesh_scattering_type),
            },
            FieldInfoData {
                name: "MeshScatteringFieldNumber",
                name_hash: 3401970961,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainEditingStaticState, mesh_scattering_field_number),
            },
            FieldInfoData {
                name: "Decals",
                name_hash: 2594137241,
                flags: MemberInfoFlags::new(144),
                field_type: "VisualVectorShapeData-Array",
                rust_offset: offset_of!(TerrainEditingStaticState, decals),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static TERRAINEDITINGSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingStaticState-Array",
    name_hash: 353905173,
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
    name_hash: 4077519778,
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


pub static TERRAINEDITINGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainEditingEvent-Array",
    name_hash: 265626134,
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
    name_hash: 3221520412,
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


pub static PLAYABLEPIXELSPERMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayablePixelsPerMeter-Array",
    name_hash: 1767176232,
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
    name_hash: 2817629483,
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


pub static TERRAINANCHOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainAnchor-Array",
    name_hash: 3099622559,
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
    name_hash: 2344784311,
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


pub static TERRAINSIZE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainSize-Array",
    name_hash: 3857312515,
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
    name_hash: 786642237,
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


pub static HIGHRESTERRAINSIZE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HighResTerrainSize-Array",
    name_hash: 3676180873,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HighResTerrainSize"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TerrainLayerCombinationsData {
}

pub static TERRAINLAYERCOMBINATIONSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationsData",
    name_hash: 3302289931,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(TerrainLayerCombinationsData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerCombinationsData as Default>::default())),
            create_boxed: || Box::new(<TerrainLayerCombinationsData as Default>::default()),
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


pub static TERRAINLAYERCOMBINATIONSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationsData-Array",
    name_hash: 2657337535,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerCombinationsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainLayerShaderData {
}

pub trait TerrainLayerShaderDataTrait: TypeObject {
}

impl TerrainLayerShaderDataTrait for TerrainLayerShaderData {
}

pub static TERRAINLAYERSHADERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerShaderData",
    name_hash: 2297862408,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerShaderData as Default>::default())),
            create_boxed: || Box::new(<TerrainLayerShaderData as Default>::default()),
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


pub static TERRAINLAYERSHADERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerShaderData-Array",
    name_hash: 518216508,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerShaderData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MeshScatteringSpawnData {
}

pub trait MeshScatteringSpawnDataTrait: TypeObject {
}

impl MeshScatteringSpawnDataTrait for MeshScatteringSpawnData {
}

pub static MESHSCATTERINGSPAWNDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringSpawnData",
    name_hash: 1185490395,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshScatteringSpawnData as Default>::default())),
            create_boxed: || Box::new(<MeshScatteringSpawnData as Default>::default()),
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


pub static MESHSCATTERINGSPAWNDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringSpawnData-Array",
    name_hash: 2336378095,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringSpawnData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TerrainStreamingTreeAsset {
}

pub static TERRAINSTREAMINGTREEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTreeAsset",
    name_hash: 629342072,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(TerrainStreamingTreeAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainStreamingTreeAsset as Default>::default())),
            create_boxed: || Box::new(<TerrainStreamingTreeAsset as Default>::default()),
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


pub static TERRAINSTREAMINGTREEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTreeAsset-Array",
    name_hash: 3231816012,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainStreamingTreeAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainLayerMaskData {
}

pub trait TerrainLayerMaskDataTrait: TypeObject {
}

impl TerrainLayerMaskDataTrait for TerrainLayerMaskData {
}

pub static TERRAINLAYERMASKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerMaskData",
    name_hash: 2272926261,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerMaskData as Default>::default())),
            create_boxed: || Box::new(<TerrainLayerMaskData as Default>::default()),
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


pub static TERRAINLAYERMASKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerMaskData-Array",
    name_hash: 1645042305,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerMaskData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PathfindingMaskRasterData {
}

pub static PATHFINDINGMASKRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingMaskRasterData",
    name_hash: 3168712330,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        super_class_offset: offset_of!(PathfindingMaskRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingMaskRasterData as Default>::default())),
            create_boxed: || Box::new(<PathfindingMaskRasterData as Default>::default()),
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


pub static PATHFINDINGMASKRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingMaskRasterData-Array",
    name_hash: 276287934,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PathfindingMaskRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainData {
    pub _glacier_base: TerrainBaseAsset,
    pub terrain_layers: Vec<Option<LockedTypeObject /* TerrainLayerData */>>,
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
    fn terrain_layers(&self) -> &Vec<Option<LockedTypeObject /* TerrainLayerData */>>;
    fn terrain_layers_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TerrainLayerData */>>;
    fn dynamic_mask_enable(&self) -> &bool;
    fn dynamic_mask_enable_mut(&mut self) -> &mut bool;
    fn detail_displacement_max_level_diff(&self) -> &u32;
    fn detail_displacement_max_level_diff_mut(&mut self) -> &mut u32;
    fn detail_displacement_indirection_texture_tile_x(&self) -> &u32;
    fn detail_displacement_indirection_texture_tile_x_mut(&mut self) -> &mut u32;
    fn override_occluder_settings(&self) -> &bool;
    fn override_occluder_settings_mut(&mut self) -> &mut bool;
    fn occluder_enable(&self) -> &bool;
    fn occluder_enable_mut(&mut self) -> &mut bool;
    fn occluder_patch_faces_per_side(&self) -> &u32;
    fn occluder_patch_faces_per_side_mut(&mut self) -> &mut u32;
    fn occluder_lod_scale(&self) -> &f32;
    fn occluder_lod_scale_mut(&mut self) -> &mut f32;
    fn terrain_streaming_tree_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn terrain_streaming_tree_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
    fn visual_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn visual_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
    fn terrain_layer_combinations_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn terrain_layer_combinations_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl TerrainDataTrait for TerrainData {
    fn terrain_layers(&self) -> &Vec<Option<LockedTypeObject /* TerrainLayerData */>> {
        &self.terrain_layers
    }
    fn terrain_layers_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TerrainLayerData */>> {
        &mut self.terrain_layers
    }
    fn dynamic_mask_enable(&self) -> &bool {
        &self.dynamic_mask_enable
    }
    fn dynamic_mask_enable_mut(&mut self) -> &mut bool {
        &mut self.dynamic_mask_enable
    }
    fn detail_displacement_max_level_diff(&self) -> &u32 {
        &self.detail_displacement_max_level_diff
    }
    fn detail_displacement_max_level_diff_mut(&mut self) -> &mut u32 {
        &mut self.detail_displacement_max_level_diff
    }
    fn detail_displacement_indirection_texture_tile_x(&self) -> &u32 {
        &self.detail_displacement_indirection_texture_tile_x
    }
    fn detail_displacement_indirection_texture_tile_x_mut(&mut self) -> &mut u32 {
        &mut self.detail_displacement_indirection_texture_tile_x
    }
    fn override_occluder_settings(&self) -> &bool {
        &self.override_occluder_settings
    }
    fn override_occluder_settings_mut(&mut self) -> &mut bool {
        &mut self.override_occluder_settings
    }
    fn occluder_enable(&self) -> &bool {
        &self.occluder_enable
    }
    fn occluder_enable_mut(&mut self) -> &mut bool {
        &mut self.occluder_enable
    }
    fn occluder_patch_faces_per_side(&self) -> &u32 {
        &self.occluder_patch_faces_per_side
    }
    fn occluder_patch_faces_per_side_mut(&mut self) -> &mut u32 {
        &mut self.occluder_patch_faces_per_side
    }
    fn occluder_lod_scale(&self) -> &f32 {
        &self.occluder_lod_scale
    }
    fn occluder_lod_scale_mut(&mut self) -> &mut f32 {
        &mut self.occluder_lod_scale
    }
    fn terrain_streaming_tree_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.terrain_streaming_tree_resource
    }
    fn terrain_streaming_tree_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.terrain_streaming_tree_resource
    }
    fn visual_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.visual_resource
    }
    fn visual_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.visual_resource
    }
    fn terrain_layer_combinations_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.terrain_layer_combinations_resource
    }
    fn terrain_layer_combinations_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.terrain_layer_combinations_resource
    }
}

impl TerrainBaseAssetTrait for TerrainData {
}

impl super::core::AssetTrait for TerrainData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TerrainData {
}

pub static TERRAINDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainData",
    name_hash: 2345466050,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TERRAINBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(TerrainData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainData as Default>::default())),
            create_boxed: || Box::new(<TerrainData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TerrainLayers",
                name_hash: 3314924962,
                flags: MemberInfoFlags::new(144),
                field_type: "TerrainLayerData-Array",
                rust_offset: offset_of!(TerrainData, terrain_layers),
            },
            FieldInfoData {
                name: "DynamicMaskEnable",
                name_hash: 755434949,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainData, dynamic_mask_enable),
            },
            FieldInfoData {
                name: "DetailDisplacementMaxLevelDiff",
                name_hash: 2993616588,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainData, detail_displacement_max_level_diff),
            },
            FieldInfoData {
                name: "DetailDisplacementIndirectionTextureTileX",
                name_hash: 1797376690,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainData, detail_displacement_indirection_texture_tile_x),
            },
            FieldInfoData {
                name: "OverrideOccluderSettings",
                name_hash: 4005666641,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainData, override_occluder_settings),
            },
            FieldInfoData {
                name: "OccluderEnable",
                name_hash: 4076210433,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainData, occluder_enable),
            },
            FieldInfoData {
                name: "OccluderPatchFacesPerSide",
                name_hash: 1520778816,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainData, occluder_patch_faces_per_side),
            },
            FieldInfoData {
                name: "OccluderLodScale",
                name_hash: 967251551,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainData, occluder_lod_scale),
            },
            FieldInfoData {
                name: "TerrainStreamingTreeResource",
                name_hash: 2943630818,
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(TerrainData, terrain_streaming_tree_resource),
            },
            FieldInfoData {
                name: "VisualResource",
                name_hash: 2957853819,
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(TerrainData, visual_resource),
            },
            FieldInfoData {
                name: "TerrainLayerCombinationsResource",
                name_hash: 4115938481,
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


pub static TERRAINDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainData-Array",
    name_hash: 1659873398,
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
    name_hash: 2258401200,
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


pub static ENLIGHTENMESHFILTERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenMeshFilterType-Array",
    name_hash: 566813956,
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
    name_hash: 586276181,
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


pub static RASTERTREEBUILDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTreeBuildMode-Array",
    name_hash: 556449121,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterTreeBuildMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainLayerCombinationDrawData {
}

pub trait TerrainLayerCombinationDrawDataTrait: TypeObject {
}

impl TerrainLayerCombinationDrawDataTrait for TerrainLayerCombinationDrawData {
}

pub static TERRAINLAYERCOMBINATIONDRAWDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawData",
    name_hash: 2117423672,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerCombinationDrawData as Default>::default())),
            create_boxed: || Box::new(<TerrainLayerCombinationDrawData as Default>::default()),
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


pub static TERRAINLAYERCOMBINATIONDRAWDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawData-Array",
    name_hash: 1119550348,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerCombinationDrawData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Surface3dDrawMethodData {
}

pub trait Surface3dDrawMethodDataTrait: TypeObject {
}

impl Surface3dDrawMethodDataTrait for Surface3dDrawMethodData {
}

pub static SURFACE3DDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface3dDrawMethodData",
    name_hash: 1307966664,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Surface3dDrawMethodData as Default>::default())),
            create_boxed: || Box::new(<Surface3dDrawMethodData as Default>::default()),
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


pub static SURFACE3DDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface3dDrawMethodData-Array",
    name_hash: 4044525436,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Surface3dDrawMethodData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SingleLayerMaskDrawMethodData {
}

pub trait SingleLayerMaskDrawMethodDataTrait: TypeObject {
}

impl SingleLayerMaskDrawMethodDataTrait for SingleLayerMaskDrawMethodData {
}

pub static SINGLELAYERMASKDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawMethodData",
    name_hash: 2291637255,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SingleLayerMaskDrawMethodData as Default>::default())),
            create_boxed: || Box::new(<SingleLayerMaskDrawMethodData as Default>::default()),
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


pub static SINGLELAYERMASKDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawMethodData-Array",
    name_hash: 3065920691,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SingleLayerMaskDrawMethodData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SingleLayerMaskDrawPassData {
}

pub trait SingleLayerMaskDrawPassDataTrait: TypeObject {
}

impl SingleLayerMaskDrawPassDataTrait for SingleLayerMaskDrawPassData {
}

pub static SINGLELAYERMASKDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawPassData",
    name_hash: 4134876841,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SingleLayerMaskDrawPassData as Default>::default())),
            create_boxed: || Box::new(<SingleLayerMaskDrawPassData as Default>::default()),
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


pub static SINGLELAYERMASKDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleLayerMaskDrawPassData-Array",
    name_hash: 246988061,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SingleLayerMaskDrawPassData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MeshScatteringMaskScaleDrawMethodData {
}

pub trait MeshScatteringMaskScaleDrawMethodDataTrait: TypeObject {
}

impl MeshScatteringMaskScaleDrawMethodDataTrait for MeshScatteringMaskScaleDrawMethodData {
}

pub static MESHSCATTERINGMASKSCALEDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringMaskScaleDrawMethodData",
    name_hash: 3564080211,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshScatteringMaskScaleDrawMethodData as Default>::default())),
            create_boxed: || Box::new(<MeshScatteringMaskScaleDrawMethodData as Default>::default()),
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


pub static MESHSCATTERINGMASKSCALEDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringMaskScaleDrawMethodData-Array",
    name_hash: 3294377831,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringMaskScaleDrawMethodData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Displacement2dDrawMethodData {
}

pub trait Displacement2dDrawMethodDataTrait: TypeObject {
}

impl Displacement2dDrawMethodDataTrait for Displacement2dDrawMethodData {
}

pub static DISPLACEMENT2DDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawMethodData",
    name_hash: 287120267,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Displacement2dDrawMethodData as Default>::default())),
            create_boxed: || Box::new(<Displacement2dDrawMethodData as Default>::default()),
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


pub static DISPLACEMENT2DDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawMethodData-Array",
    name_hash: 2441415231,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Displacement2dDrawMethodData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Displacement2dDrawPassData {
}

pub trait Displacement2dDrawPassDataTrait: TypeObject {
}

impl Displacement2dDrawPassDataTrait for Displacement2dDrawPassData {
}

pub static DISPLACEMENT2DDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawPassData",
    name_hash: 2216819237,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Displacement2dDrawPassData as Default>::default())),
            create_boxed: || Box::new(<Displacement2dDrawPassData as Default>::default()),
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


pub static DISPLACEMENT2DDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Displacement2dDrawPassData-Array",
    name_hash: 629830289,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Displacement2dDrawPassData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Surface2dDrawMethodData {
}

pub trait Surface2dDrawMethodDataTrait: TypeObject {
}

impl Surface2dDrawMethodDataTrait for Surface2dDrawMethodData {
}

pub static SURFACE2DDRAWMETHODDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawMethodData",
    name_hash: 1799213353,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Surface2dDrawMethodData as Default>::default())),
            create_boxed: || Box::new(<Surface2dDrawMethodData as Default>::default()),
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


pub static SURFACE2DDRAWMETHODDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawMethodData-Array",
    name_hash: 4281022365,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Surface2dDrawMethodData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Surface2dDrawPassData {
}

pub trait Surface2dDrawPassDataTrait: TypeObject {
}

impl Surface2dDrawPassDataTrait for Surface2dDrawPassData {
}

pub static SURFACE2DDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawPassData",
    name_hash: 2897881671,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Surface2dDrawPassData as Default>::default())),
            create_boxed: || Box::new(<Surface2dDrawPassData as Default>::default()),
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


pub static SURFACE2DDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Surface2dDrawPassData-Array",
    name_hash: 4015143795,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Surface2dDrawPassData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainLayerCombinationDrawPassData {
}

pub trait TerrainLayerCombinationDrawPassDataTrait: TypeObject {
}

impl TerrainLayerCombinationDrawPassDataTrait for TerrainLayerCombinationDrawPassData {
}

pub static TERRAINLAYERCOMBINATIONDRAWPASSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawPassData",
    name_hash: 828413673,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerCombinationDrawPassData as Default>::default())),
            create_boxed: || Box::new(<TerrainLayerCombinationDrawPassData as Default>::default()),
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


pub static TERRAINLAYERCOMBINATIONDRAWPASSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinationDrawPassData-Array",
    name_hash: 2894826973,
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
    name_hash: 4132800027,
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


pub static TERRAINDRAWPASSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDrawPassType-Array",
    name_hash: 1788222639,
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
    name_hash: 2196067720,
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


pub static TERRAINBRUSHDETAILOPERATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBrushDetailOperation-Array",
    name_hash: 2500687804,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainBrushDetailOperation"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SingleTerrainLayerData {
    pub _glacier_base: TerrainLayerData,
    pub mesh_scattering_types: Vec<Option<LockedTypeObject /* TerrainMeshScatteringType */>>,
}

pub trait SingleTerrainLayerDataTrait: TerrainLayerDataTrait {
    fn mesh_scattering_types(&self) -> &Vec<Option<LockedTypeObject /* TerrainMeshScatteringType */>>;
    fn mesh_scattering_types_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TerrainMeshScatteringType */>>;
}

impl SingleTerrainLayerDataTrait for SingleTerrainLayerData {
    fn mesh_scattering_types(&self) -> &Vec<Option<LockedTypeObject /* TerrainMeshScatteringType */>> {
        &self.mesh_scattering_types
    }
    fn mesh_scattering_types_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TerrainMeshScatteringType */>> {
        &mut self.mesh_scattering_types
    }
}

impl TerrainLayerDataTrait for SingleTerrainLayerData {
}

impl super::core::DataContainerTrait for SingleTerrainLayerData {
}

pub static SINGLETERRAINLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleTerrainLayerData",
    name_hash: 3602684891,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TERRAINLAYERDATA_TYPE_INFO),
        super_class_offset: offset_of!(SingleTerrainLayerData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SingleTerrainLayerData as Default>::default())),
            create_boxed: || Box::new(<SingleTerrainLayerData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MeshScatteringTypes",
                name_hash: 291127195,
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


pub static SINGLETERRAINLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SingleTerrainLayerData-Array",
    name_hash: 4120265455,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SingleTerrainLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainLayerData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait TerrainLayerDataTrait: super::core::DataContainerTrait {
}

impl TerrainLayerDataTrait for TerrainLayerData {
}

impl super::core::DataContainerTrait for TerrainLayerData {
}

pub static TERRAINLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerData",
    name_hash: 3245002433,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TerrainLayerData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerData as Default>::default())),
            create_boxed: || Box::new(<TerrainLayerData as Default>::default()),
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


pub static TERRAINLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerData-Array",
    name_hash: 3642775797,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ColorImportSettings {
}

pub trait ColorImportSettingsTrait: TypeObject {
}

impl ColorImportSettingsTrait for ColorImportSettings {
}

pub static COLORIMPORTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorImportSettings",
    name_hash: 1193066656,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ColorImportSettings as Default>::default())),
            create_boxed: || Box::new(<ColorImportSettings as Default>::default()),
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


pub static COLORIMPORTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorImportSettings-Array",
    name_hash: 3456706068,
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
    name_hash: 3244453705,
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


pub static TERRAINLAYERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerType-Array",
    name_hash: 3689788797,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainLayerProceduralMask {
    pub altitude_min: f32,
}

pub trait TerrainLayerProceduralMaskTrait: TypeObject {
    fn altitude_min(&self) -> &f32;
    fn altitude_min_mut(&mut self) -> &mut f32;
}

impl TerrainLayerProceduralMaskTrait for TerrainLayerProceduralMask {
    fn altitude_min(&self) -> &f32 {
        &self.altitude_min
    }
    fn altitude_min_mut(&mut self) -> &mut f32 {
        &mut self.altitude_min
    }
}

pub static TERRAINLAYERPROCEDURALMASK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerProceduralMask",
    name_hash: 503185632,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerProceduralMask as Default>::default())),
            create_boxed: || Box::new(<TerrainLayerProceduralMask as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AltitudeMin",
                name_hash: 4056047967,
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


pub static TERRAINLAYERPROCEDURALMASK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerProceduralMask-Array",
    name_hash: 1319474900,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainLayerProceduralMask"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainGeoTexture {
}

pub trait TerrainGeoTextureTrait: TypeObject {
}

impl TerrainGeoTextureTrait for TerrainGeoTexture {
}

pub static TERRAINGEOTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainGeoTexture",
    name_hash: 3607725312,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainGeoTexture as Default>::default())),
            create_boxed: || Box::new(<TerrainGeoTexture as Default>::default()),
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


pub static TERRAINGEOTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainGeoTexture-Array",
    name_hash: 2683824948,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainGeoTexture"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn identifier_mut(&mut self) -> &mut u32;
    fn min_scale(&self) -> &super::core::Vec2;
    fn min_scale_mut(&mut self) -> &mut super::core::Vec2;
    fn max_scale(&self) -> &super::core::Vec2;
    fn max_scale_mut(&mut self) -> &mut super::core::Vec2;
    fn scale_randomness(&self) -> &f32;
    fn scale_randomness_mut(&mut self) -> &mut f32;
    fn lod0_dissolve_out_distance_factor(&self) -> &f32;
    fn lod0_dissolve_out_distance_factor_mut(&mut self) -> &mut f32;
    fn lod1_dissolve_in_distance_factor(&self) -> &f32;
    fn lod1_dissolve_in_distance_factor_mut(&mut self) -> &mut f32;
    fn lod1_dissolve_out_distance_factor(&self) -> &f32;
    fn lod1_dissolve_out_distance_factor_mut(&mut self) -> &mut f32;
    fn lod2_dissolve_in_distance_factor(&self) -> &f32;
    fn lod2_dissolve_in_distance_factor_mut(&mut self) -> &mut f32;
    fn lod2_dissolve_out_distance_factor(&self) -> &f32;
    fn lod2_dissolve_out_distance_factor_mut(&mut self) -> &mut f32;
    fn lod3_dissolve_in_distance_factor(&self) -> &f32;
    fn lod3_dissolve_in_distance_factor_mut(&mut self) -> &mut f32;
    fn density(&self) -> &super::core::QualityScalableFloat;
    fn density_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn first_spawn_level(&self) -> &u32;
    fn first_spawn_level_mut(&mut self) -> &mut u32;
    fn wind_scale(&self) -> &f32;
    fn wind_scale_mut(&mut self) -> &mut f32;
    fn stiffness(&self) -> &f32;
    fn stiffness_mut(&mut self) -> &mut f32;
    fn damping(&self) -> &f32;
    fn damping_mut(&mut self) -> &mut f32;
    fn mass(&self) -> &f32;
    fn mass_mut(&mut self) -> &mut f32;
    fn wind_wiggle(&self) -> &f32;
    fn wind_wiggle_mut(&mut self) -> &mut f32;
    fn use_vertex_color_weights(&self) -> &bool;
    fn use_vertex_color_weights_mut(&mut self) -> &mut bool;
    fn dissolve_range_ratio(&self) -> &f32;
    fn dissolve_range_ratio_mut(&mut self) -> &mut f32;
}

impl TerrainMeshScatteringTypeTrait for TerrainMeshScatteringType {
    fn identifier(&self) -> &u32 {
        &self.identifier
    }
    fn identifier_mut(&mut self) -> &mut u32 {
        &mut self.identifier
    }
    fn min_scale(&self) -> &super::core::Vec2 {
        &self.min_scale
    }
    fn min_scale_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.min_scale
    }
    fn max_scale(&self) -> &super::core::Vec2 {
        &self.max_scale
    }
    fn max_scale_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.max_scale
    }
    fn scale_randomness(&self) -> &f32 {
        &self.scale_randomness
    }
    fn scale_randomness_mut(&mut self) -> &mut f32 {
        &mut self.scale_randomness
    }
    fn lod0_dissolve_out_distance_factor(&self) -> &f32 {
        &self.lod0_dissolve_out_distance_factor
    }
    fn lod0_dissolve_out_distance_factor_mut(&mut self) -> &mut f32 {
        &mut self.lod0_dissolve_out_distance_factor
    }
    fn lod1_dissolve_in_distance_factor(&self) -> &f32 {
        &self.lod1_dissolve_in_distance_factor
    }
    fn lod1_dissolve_in_distance_factor_mut(&mut self) -> &mut f32 {
        &mut self.lod1_dissolve_in_distance_factor
    }
    fn lod1_dissolve_out_distance_factor(&self) -> &f32 {
        &self.lod1_dissolve_out_distance_factor
    }
    fn lod1_dissolve_out_distance_factor_mut(&mut self) -> &mut f32 {
        &mut self.lod1_dissolve_out_distance_factor
    }
    fn lod2_dissolve_in_distance_factor(&self) -> &f32 {
        &self.lod2_dissolve_in_distance_factor
    }
    fn lod2_dissolve_in_distance_factor_mut(&mut self) -> &mut f32 {
        &mut self.lod2_dissolve_in_distance_factor
    }
    fn lod2_dissolve_out_distance_factor(&self) -> &f32 {
        &self.lod2_dissolve_out_distance_factor
    }
    fn lod2_dissolve_out_distance_factor_mut(&mut self) -> &mut f32 {
        &mut self.lod2_dissolve_out_distance_factor
    }
    fn lod3_dissolve_in_distance_factor(&self) -> &f32 {
        &self.lod3_dissolve_in_distance_factor
    }
    fn lod3_dissolve_in_distance_factor_mut(&mut self) -> &mut f32 {
        &mut self.lod3_dissolve_in_distance_factor
    }
    fn density(&self) -> &super::core::QualityScalableFloat {
        &self.density
    }
    fn density_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.density
    }
    fn first_spawn_level(&self) -> &u32 {
        &self.first_spawn_level
    }
    fn first_spawn_level_mut(&mut self) -> &mut u32 {
        &mut self.first_spawn_level
    }
    fn wind_scale(&self) -> &f32 {
        &self.wind_scale
    }
    fn wind_scale_mut(&mut self) -> &mut f32 {
        &mut self.wind_scale
    }
    fn stiffness(&self) -> &f32 {
        &self.stiffness
    }
    fn stiffness_mut(&mut self) -> &mut f32 {
        &mut self.stiffness
    }
    fn damping(&self) -> &f32 {
        &self.damping
    }
    fn damping_mut(&mut self) -> &mut f32 {
        &mut self.damping
    }
    fn mass(&self) -> &f32 {
        &self.mass
    }
    fn mass_mut(&mut self) -> &mut f32 {
        &mut self.mass
    }
    fn wind_wiggle(&self) -> &f32 {
        &self.wind_wiggle
    }
    fn wind_wiggle_mut(&mut self) -> &mut f32 {
        &mut self.wind_wiggle
    }
    fn use_vertex_color_weights(&self) -> &bool {
        &self.use_vertex_color_weights
    }
    fn use_vertex_color_weights_mut(&mut self) -> &mut bool {
        &mut self.use_vertex_color_weights
    }
    fn dissolve_range_ratio(&self) -> &f32 {
        &self.dissolve_range_ratio
    }
    fn dissolve_range_ratio_mut(&mut self) -> &mut f32 {
        &mut self.dissolve_range_ratio
    }
}

impl super::core::DataContainerTrait for TerrainMeshScatteringType {
}

pub static TERRAINMESHSCATTERINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainMeshScatteringType",
    name_hash: 490066143,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TerrainMeshScatteringType, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainMeshScatteringType as Default>::default())),
            create_boxed: || Box::new(<TerrainMeshScatteringType as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Identifier",
                name_hash: 3512790342,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainMeshScatteringType, identifier),
            },
            FieldInfoData {
                name: "MinScale",
                name_hash: 3368655127,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(TerrainMeshScatteringType, min_scale),
            },
            FieldInfoData {
                name: "MaxScale",
                name_hash: 395677513,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(TerrainMeshScatteringType, max_scale),
            },
            FieldInfoData {
                name: "ScaleRandomness",
                name_hash: 2420946861,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, scale_randomness),
            },
            FieldInfoData {
                name: "Lod0DissolveOutDistanceFactor",
                name_hash: 3263877551,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod0_dissolve_out_distance_factor),
            },
            FieldInfoData {
                name: "Lod1DissolveInDistanceFactor",
                name_hash: 4186735975,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod1_dissolve_in_distance_factor),
            },
            FieldInfoData {
                name: "Lod1DissolveOutDistanceFactor",
                name_hash: 2473794894,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod1_dissolve_out_distance_factor),
            },
            FieldInfoData {
                name: "Lod2DissolveInDistanceFactor",
                name_hash: 4102077156,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod2_dissolve_in_distance_factor),
            },
            FieldInfoData {
                name: "Lod2DissolveOutDistanceFactor",
                name_hash: 3780871661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod2_dissolve_out_distance_factor),
            },
            FieldInfoData {
                name: "Lod3DissolveInDistanceFactor",
                name_hash: 4226584933,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, lod3_dissolve_in_distance_factor),
            },
            FieldInfoData {
                name: "Density",
                name_hash: 4008572221,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(TerrainMeshScatteringType, density),
            },
            FieldInfoData {
                name: "FirstSpawnLevel",
                name_hash: 2271926258,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TerrainMeshScatteringType, first_spawn_level),
            },
            FieldInfoData {
                name: "WindScale",
                name_hash: 3183611401,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, wind_scale),
            },
            FieldInfoData {
                name: "Stiffness",
                name_hash: 721813632,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, stiffness),
            },
            FieldInfoData {
                name: "Damping",
                name_hash: 3862601053,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, damping),
            },
            FieldInfoData {
                name: "Mass",
                name_hash: 2088779625,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, mass),
            },
            FieldInfoData {
                name: "WindWiggle",
                name_hash: 2152607398,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainMeshScatteringType, wind_wiggle),
            },
            FieldInfoData {
                name: "UseVertexColorWeights",
                name_hash: 2359073376,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainMeshScatteringType, use_vertex_color_weights),
            },
            FieldInfoData {
                name: "DissolveRangeRatio",
                name_hash: 3356179078,
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


pub static TERRAINMESHSCATTERINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainMeshScatteringType-Array",
    name_hash: 1654842859,
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
    name_hash: 700226826,
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


pub static MESHSCATTERINGINSTANCEDATAMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringInstanceDataMode-Array",
    name_hash: 1053333566,
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
    name_hash: 1778122697,
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


pub static UNDERGROWTHORIENTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UndergrowthOrientationMode-Array",
    name_hash: 311535101,
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
    name_hash: 3376960514,
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


pub static MESHSCATTERINGBILLBOARDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringBillboardMode-Array",
    name_hash: 3112317750,
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
    name_hash: 2354164613,
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


pub static MESHSCATTERINGORIENTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringOrientationMode-Array",
    name_hash: 2085845809,
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
    name_hash: 3164319691,
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


pub static UNDERGROWTHROTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UndergrowthRotationMode-Array",
    name_hash: 1138855679,
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
    name_hash: 811866247,
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


pub static MESHSCATTERINGROTATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringRotationMode-Array",
    name_hash: 3122010419,
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
    name_hash: 2241351988,
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


pub static MESHSCATTERINGELEVATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringElevationMode-Array",
    name_hash: 3974204544,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("MeshScatteringElevationMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainHeightfieldData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait TerrainHeightfieldDataTrait: super::core::DataContainerTrait {
}

impl TerrainHeightfieldDataTrait for TerrainHeightfieldData {
}

impl super::core::DataContainerTrait for TerrainHeightfieldData {
}

pub static TERRAINHEIGHTFIELDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainHeightfieldData",
    name_hash: 2172755071,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TerrainHeightfieldData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainHeightfieldData as Default>::default())),
            create_boxed: || Box::new(<TerrainHeightfieldData as Default>::default()),
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


pub static TERRAINHEIGHTFIELDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainHeightfieldData-Array",
    name_hash: 3157877323,
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
    name_hash: 3594673716,
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


pub static TERRAINBRUSHTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBrushType-Array",
    name_hash: 1243699584,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainBrushType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainDynamicDecalTemplateData {
    pub _glacier_base: super::core::Asset,
    pub width: f32,
    pub relative_width_deviation: f32,
    pub depth: f32,
    pub relative_depth_deviation: f32,
    pub rotation_random_amount: f32,
    pub slope_max: f32,
    pub depth_mask: Option<LockedTypeObject /* HeightfieldDecalAsset */>,
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
    fn width_mut(&mut self) -> &mut f32;
    fn relative_width_deviation(&self) -> &f32;
    fn relative_width_deviation_mut(&mut self) -> &mut f32;
    fn depth(&self) -> &f32;
    fn depth_mut(&mut self) -> &mut f32;
    fn relative_depth_deviation(&self) -> &f32;
    fn relative_depth_deviation_mut(&mut self) -> &mut f32;
    fn rotation_random_amount(&self) -> &f32;
    fn rotation_random_amount_mut(&mut self) -> &mut f32;
    fn slope_max(&self) -> &f32;
    fn slope_max_mut(&mut self) -> &mut f32;
    fn depth_mask(&self) -> &Option<LockedTypeObject /* HeightfieldDecalAsset */>;
    fn depth_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* HeightfieldDecalAsset */>;
    fn mask_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn mask_shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct;
    fn displacement_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn displacement_shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct;
    fn dynamic_mask_decal_width_scale(&self) -> &f32;
    fn dynamic_mask_decal_width_scale_mut(&mut self) -> &mut f32;
    fn tangent_space_enable(&self) -> &bool;
    fn tangent_space_enable_mut(&mut self) -> &mut bool;
    fn scale_with_destruction_depth(&self) -> &bool;
    fn scale_with_destruction_depth_mut(&mut self) -> &mut bool;
    fn force_up_scale(&self) -> &bool;
    fn force_up_scale_mut(&mut self) -> &mut bool;
    fn slope_min_threshold(&self) -> &f32;
    fn slope_min_threshold_mut(&mut self) -> &mut f32;
    fn slope_scalar_max(&self) -> &f32;
    fn slope_scalar_max_mut(&mut self) -> &mut f32;
    fn slope_multiplier(&self) -> &f32;
    fn slope_multiplier_mut(&mut self) -> &mut f32;
    fn max_opposing_slopes(&self) -> &f32;
    fn max_opposing_slopes_mut(&mut self) -> &mut f32;
    fn min_weight_threshold(&self) -> &f32;
    fn min_weight_threshold_mut(&mut self) -> &mut f32;
}

impl TerrainDynamicDecalTemplateDataTrait for TerrainDynamicDecalTemplateData {
    fn width(&self) -> &f32 {
        &self.width
    }
    fn width_mut(&mut self) -> &mut f32 {
        &mut self.width
    }
    fn relative_width_deviation(&self) -> &f32 {
        &self.relative_width_deviation
    }
    fn relative_width_deviation_mut(&mut self) -> &mut f32 {
        &mut self.relative_width_deviation
    }
    fn depth(&self) -> &f32 {
        &self.depth
    }
    fn depth_mut(&mut self) -> &mut f32 {
        &mut self.depth
    }
    fn relative_depth_deviation(&self) -> &f32 {
        &self.relative_depth_deviation
    }
    fn relative_depth_deviation_mut(&mut self) -> &mut f32 {
        &mut self.relative_depth_deviation
    }
    fn rotation_random_amount(&self) -> &f32 {
        &self.rotation_random_amount
    }
    fn rotation_random_amount_mut(&mut self) -> &mut f32 {
        &mut self.rotation_random_amount
    }
    fn slope_max(&self) -> &f32 {
        &self.slope_max
    }
    fn slope_max_mut(&mut self) -> &mut f32 {
        &mut self.slope_max
    }
    fn depth_mask(&self) -> &Option<LockedTypeObject /* HeightfieldDecalAsset */> {
        &self.depth_mask
    }
    fn depth_mask_mut(&mut self) -> &mut Option<LockedTypeObject /* HeightfieldDecalAsset */> {
        &mut self.depth_mask
    }
    fn mask_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.mask_shader
    }
    fn mask_shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct {
        &mut self.mask_shader
    }
    fn displacement_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.displacement_shader
    }
    fn displacement_shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct {
        &mut self.displacement_shader
    }
    fn dynamic_mask_decal_width_scale(&self) -> &f32 {
        &self.dynamic_mask_decal_width_scale
    }
    fn dynamic_mask_decal_width_scale_mut(&mut self) -> &mut f32 {
        &mut self.dynamic_mask_decal_width_scale
    }
    fn tangent_space_enable(&self) -> &bool {
        &self.tangent_space_enable
    }
    fn tangent_space_enable_mut(&mut self) -> &mut bool {
        &mut self.tangent_space_enable
    }
    fn scale_with_destruction_depth(&self) -> &bool {
        &self.scale_with_destruction_depth
    }
    fn scale_with_destruction_depth_mut(&mut self) -> &mut bool {
        &mut self.scale_with_destruction_depth
    }
    fn force_up_scale(&self) -> &bool {
        &self.force_up_scale
    }
    fn force_up_scale_mut(&mut self) -> &mut bool {
        &mut self.force_up_scale
    }
    fn slope_min_threshold(&self) -> &f32 {
        &self.slope_min_threshold
    }
    fn slope_min_threshold_mut(&mut self) -> &mut f32 {
        &mut self.slope_min_threshold
    }
    fn slope_scalar_max(&self) -> &f32 {
        &self.slope_scalar_max
    }
    fn slope_scalar_max_mut(&mut self) -> &mut f32 {
        &mut self.slope_scalar_max
    }
    fn slope_multiplier(&self) -> &f32 {
        &self.slope_multiplier
    }
    fn slope_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.slope_multiplier
    }
    fn max_opposing_slopes(&self) -> &f32 {
        &self.max_opposing_slopes
    }
    fn max_opposing_slopes_mut(&mut self) -> &mut f32 {
        &mut self.max_opposing_slopes
    }
    fn min_weight_threshold(&self) -> &f32 {
        &self.min_weight_threshold
    }
    fn min_weight_threshold_mut(&mut self) -> &mut f32 {
        &mut self.min_weight_threshold
    }
}

impl super::core::AssetTrait for TerrainDynamicDecalTemplateData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TerrainDynamicDecalTemplateData {
}

pub static TERRAINDYNAMICDECALTEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDynamicDecalTemplateData",
    name_hash: 4221238472,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(TerrainDynamicDecalTemplateData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainDynamicDecalTemplateData as Default>::default())),
            create_boxed: || Box::new(<TerrainDynamicDecalTemplateData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Width",
                name_hash: 226981187,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, width),
            },
            FieldInfoData {
                name: "RelativeWidthDeviation",
                name_hash: 2091224244,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, relative_width_deviation),
            },
            FieldInfoData {
                name: "Depth",
                name_hash: 208780552,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, depth),
            },
            FieldInfoData {
                name: "RelativeDepthDeviation",
                name_hash: 1779196895,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, relative_depth_deviation),
            },
            FieldInfoData {
                name: "RotationRandomAmount",
                name_hash: 1670238502,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, rotation_random_amount),
            },
            FieldInfoData {
                name: "SlopeMax",
                name_hash: 3923137364,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_max),
            },
            FieldInfoData {
                name: "DepthMask",
                name_hash: 968418876,
                flags: MemberInfoFlags::new(0),
                field_type: "HeightfieldDecalAsset",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, depth_mask),
            },
            FieldInfoData {
                name: "MaskShader",
                name_hash: 3852583160,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, mask_shader),
            },
            FieldInfoData {
                name: "DisplacementShader",
                name_hash: 2077400859,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, displacement_shader),
            },
            FieldInfoData {
                name: "DynamicMaskDecalWidthScale",
                name_hash: 3977963893,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, dynamic_mask_decal_width_scale),
            },
            FieldInfoData {
                name: "TangentSpaceEnable",
                name_hash: 397952163,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, tangent_space_enable),
            },
            FieldInfoData {
                name: "ScaleWithDestructionDepth",
                name_hash: 3553243852,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, scale_with_destruction_depth),
            },
            FieldInfoData {
                name: "ForceUpScale",
                name_hash: 2140767269,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, force_up_scale),
            },
            FieldInfoData {
                name: "SlopeMinThreshold",
                name_hash: 979450301,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_min_threshold),
            },
            FieldInfoData {
                name: "SlopeScalarMax",
                name_hash: 2708827162,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_scalar_max),
            },
            FieldInfoData {
                name: "SlopeMultiplier",
                name_hash: 3031010539,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, slope_multiplier),
            },
            FieldInfoData {
                name: "MaxOpposingSlopes",
                name_hash: 2190796660,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TerrainDynamicDecalTemplateData, max_opposing_slopes),
            },
            FieldInfoData {
                name: "MinWeightThreshold",
                name_hash: 3084168440,
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


pub static TERRAINDYNAMICDECALTEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDynamicDecalTemplateData-Array",
    name_hash: 2255982460,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainDynamicDecalTemplateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HeightfieldDecalAsset {
    pub _glacier_base: super::core::Asset,
    pub resource: glacier_reflect::builtin::ResourceRef,
    pub mid_point128: bool,
}

pub trait HeightfieldDecalAssetTrait: super::core::AssetTrait {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
    fn mid_point128(&self) -> &bool;
    fn mid_point128_mut(&mut self) -> &mut bool;
}

impl HeightfieldDecalAssetTrait for HeightfieldDecalAsset {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.resource
    }
    fn mid_point128(&self) -> &bool {
        &self.mid_point128
    }
    fn mid_point128_mut(&mut self) -> &mut bool {
        &mut self.mid_point128
    }
}

impl super::core::AssetTrait for HeightfieldDecalAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for HeightfieldDecalAsset {
}

pub static HEIGHTFIELDDECALASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecalAsset",
    name_hash: 2724985703,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(HeightfieldDecalAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HeightfieldDecalAsset as Default>::default())),
            create_boxed: || Box::new(<HeightfieldDecalAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Resource",
                name_hash: 74513935,
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(HeightfieldDecalAsset, resource),
            },
            FieldInfoData {
                name: "MidPoint128",
                name_hash: 853483410,
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


pub static HEIGHTFIELDDECALASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecalAsset-Array",
    name_hash: 790420819,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HeightfieldDecalAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TerrainBaseAsset {
}

pub static TERRAINBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBaseAsset",
    name_hash: 1390778135,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(TerrainBaseAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainBaseAsset as Default>::default())),
            create_boxed: || Box::new(<TerrainBaseAsset as Default>::default()),
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


pub static TERRAINBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainBaseAsset-Array",
    name_hash: 1877950371,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PathfindingRasterData {
}

pub static PATHFINDINGRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRasterData",
    name_hash: 2191858046,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RGBRASTERDATA_TYPE_INFO),
        super_class_offset: offset_of!(PathfindingRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingRasterData as Default>::default())),
            create_boxed: || Box::new(<PathfindingRasterData as Default>::default()),
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


pub static PATHFINDINGRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRasterData-Array",
    name_hash: 341514826,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PathfindingRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RasterCoverageData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait RasterCoverageDataTrait: super::core::DataContainerTrait {
}

impl RasterCoverageDataTrait for RasterCoverageData {
}

impl super::core::DataContainerTrait for RasterCoverageData {
}

pub static RASTERCOVERAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterCoverageData",
    name_hash: 885503416,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(RasterCoverageData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RasterCoverageData as Default>::default())),
            create_boxed: || Box::new(<RasterCoverageData as Default>::default()),
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


pub static RASTERCOVERAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterCoverageData-Array",
    name_hash: 1634336524,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterCoverageData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DensityMapRasterData {
}

pub static DENSITYMAPRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMapRasterData",
    name_hash: 4087409714,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        super_class_offset: offset_of!(DensityMapRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DensityMapRasterData as Default>::default())),
            create_boxed: || Box::new(<DensityMapRasterData as Default>::default()),
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


pub static DENSITYMAPRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMapRasterData-Array",
    name_hash: 3165336198,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DensityMapRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BiomeRasterData {
    pub _glacier_base: IndexedRasterData,
    pub biomes: Vec<BoxedTypeObject /* BiomeSpec */>,
}

pub trait BiomeRasterDataTrait: IndexedRasterDataTrait {
    fn biomes(&self) -> &Vec<BoxedTypeObject /* BiomeSpec */>;
    fn biomes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* BiomeSpec */>;
}

impl BiomeRasterDataTrait for BiomeRasterData {
    fn biomes(&self) -> &Vec<BoxedTypeObject /* BiomeSpec */> {
        &self.biomes
    }
    fn biomes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* BiomeSpec */> {
        &mut self.biomes
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for BiomeRasterData {
}

pub static BIOMERASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeRasterData",
    name_hash: 331544954,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(INDEXEDRASTERDATA_TYPE_INFO),
        super_class_offset: offset_of!(BiomeRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BiomeRasterData as Default>::default())),
            create_boxed: || Box::new(<BiomeRasterData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Biomes",
                name_hash: 2681472698,
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


pub static BIOMERASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeRasterData-Array",
    name_hash: 455167054,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("BiomeRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for IndexedRasterData {
}

pub static INDEXEDRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexedRasterData",
    name_hash: 714420329,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        super_class_offset: offset_of!(IndexedRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IndexedRasterData as Default>::default())),
            create_boxed: || Box::new(<IndexedRasterData as Default>::default()),
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


pub static INDEXEDRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndexedRasterData-Array",
    name_hash: 3358410589,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("IndexedRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TileBiomeList {
    pub hash: i32,
    pub biomes: Vec<u8>,
}

pub trait TileBiomeListTrait: TypeObject {
    fn hash(&self) -> &i32;
    fn hash_mut(&mut self) -> &mut i32;
    fn biomes(&self) -> &Vec<u8>;
    fn biomes_mut(&mut self) -> &mut Vec<u8>;
}

impl TileBiomeListTrait for TileBiomeList {
    fn hash(&self) -> &i32 {
        &self.hash
    }
    fn hash_mut(&mut self) -> &mut i32 {
        &mut self.hash
    }
    fn biomes(&self) -> &Vec<u8> {
        &self.biomes
    }
    fn biomes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.biomes
    }
}

pub static TILEBIOMELIST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TileBiomeList",
    name_hash: 1382742559,
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TileBiomeList as Default>::default())),
            create_boxed: || Box::new(<TileBiomeList as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Hash",
                name_hash: 2089161879,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TileBiomeList, hash),
            },
            FieldInfoData {
                name: "Biomes",
                name_hash: 2681472698,
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


pub static TILEBIOMELIST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TileBiomeList-Array",
    name_hash: 1194475179,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TileBiomeList"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BiomeSpec {
    pub name: String,
    pub value: u32,
}

pub trait BiomeSpecTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn value(&self) -> &u32;
    fn value_mut(&mut self) -> &mut u32;
}

impl BiomeSpecTrait for BiomeSpec {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn value(&self) -> &u32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut u32 {
        &mut self.value
    }
}

pub static BIOMESPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeSpec",
    name_hash: 2196889612,
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BiomeSpec as Default>::default())),
            create_boxed: || Box::new(<BiomeSpec as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BiomeSpec, name),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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


pub static BIOMESPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BiomeSpec-Array",
    name_hash: 4046950456,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("BiomeSpec"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for FlowMapRasterData {
}

pub static FLOWMAPRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlowMapRasterData",
    name_hash: 228118488,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        super_class_offset: offset_of!(FlowMapRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FlowMapRasterData as Default>::default())),
            create_boxed: || Box::new(<FlowMapRasterData as Default>::default()),
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


pub static FLOWMAPRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlowMapRasterData-Array",
    name_hash: 1910139500,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("FlowMapRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DestructionDepthRasterData {
}

pub static DESTRUCTIONDEPTHRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthRasterData",
    name_hash: 2622500677,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BYTERASTERDATA_TYPE_INFO),
        super_class_offset: offset_of!(DestructionDepthRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionDepthRasterData as Default>::default())),
            create_boxed: || Box::new(<DestructionDepthRasterData as Default>::default()),
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


pub static DESTRUCTIONDEPTHRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthRasterData-Array",
    name_hash: 15452017,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DestructionDepthRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DestructionDepthGenerateOptions {
    pub _glacier_base: super::core::DataContainer,
}

pub trait DestructionDepthGenerateOptionsTrait: super::core::DataContainerTrait {
}

impl DestructionDepthGenerateOptionsTrait for DestructionDepthGenerateOptions {
}

impl super::core::DataContainerTrait for DestructionDepthGenerateOptions {
}

pub static DESTRUCTIONDEPTHGENERATEOPTIONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateOptions",
    name_hash: 3099401421,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(DestructionDepthGenerateOptions, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionDepthGenerateOptions as Default>::default())),
            create_boxed: || Box::new(<DestructionDepthGenerateOptions as Default>::default()),
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


pub static DESTRUCTIONDEPTHGENERATEOPTIONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateOptions-Array",
    name_hash: 674509049,
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
    name_hash: 3457310656,
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


pub static DESTRUCTIONDEPTHGENERATESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionDepthGenerateSource-Array",
    name_hash: 2569918068,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DestructionDepthGenerateSource"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PhysicsMaterialsRasterData {
}

pub static PHYSICSMATERIALSRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsMaterialsRasterData",
    name_hash: 3319507973,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PhysicsMaterialsRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsMaterialsRasterData as Default>::default())),
            create_boxed: || Box::new(<PhysicsMaterialsRasterData as Default>::default()),
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


pub static PHYSICSMATERIALSRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsMaterialsRasterData-Array",
    name_hash: 3767915441,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("PhysicsMaterialsRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ByteRasterData {
}

pub static BYTERASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ByteRasterData",
    name_hash: 3744804348,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        super_class_offset: offset_of!(ByteRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ByteRasterData as Default>::default())),
            create_boxed: || Box::new(<ByteRasterData as Default>::default()),
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


pub static BYTERASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ByteRasterData-Array",
    name_hash: 219960776,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ByteRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for RGBRasterData {
}

pub static RGBRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBRasterData",
    name_hash: 2877370241,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        super_class_offset: offset_of!(RGBRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RGBRasterData as Default>::default())),
            create_boxed: || Box::new(<RGBRasterData as Default>::default()),
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


pub static RGBRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBRasterData-Array",
    name_hash: 384846133,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RGBRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for RGBARasterData {
}

pub static RGBARASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBARasterData",
    name_hash: 4242486656,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        super_class_offset: offset_of!(RGBARasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RGBARasterData as Default>::default())),
            create_boxed: || Box::new(<RGBARasterData as Default>::default()),
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


pub static RGBARASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RGBARasterData-Array",
    name_hash: 2557475764,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RGBARasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for HeightfieldRasterData {
}

pub static HEIGHTFIELDRASTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldRasterData",
    name_hash: 901403019,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        super_class_offset: offset_of!(HeightfieldRasterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HeightfieldRasterData as Default>::default())),
            create_boxed: || Box::new(<HeightfieldRasterData as Default>::default()),
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


pub static HEIGHTFIELDRASTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldRasterData-Array",
    name_hash: 3232940095,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HeightfieldRasterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DensityMapFilterType {
    #[default]
    DensityMapFilter_SecondOrderDifference = 0,
    DensityMapFilter_GaussianCurvature = 1,
    DensityMapFilter_MeanCurvature = 2,
    DensityMapFilter_LaplaceBeltrami = 3,
    DensityMapFilter_LaplaceBeltramiNoVoronoi = 4,
}

pub static DENSITYMAP_FILTERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMap_FilterType",
    name_hash: 547086694,
    flags: MemberInfoFlags::new(49429),
    module: "TerrainBase",
    data: TypeInfoData::Enum,
    array_type: Some(DENSITYMAP_FILTERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DensityMapFilterType {
    fn type_info(&self) -> &'static TypeInfo {
        DENSITYMAP_FILTERTYPE_TYPE_INFO
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


pub static DENSITYMAP_FILTERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DensityMap_FilterType-Array",
    name_hash: 1077263954,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("DensityMap_FilterType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for VirtualRasterQuadtreeData {
}

pub static VIRTUALRASTERQUADTREEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VirtualRasterQuadtreeData",
    name_hash: 4129094192,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RASTERQUADTREEDATA_TYPE_INFO),
        super_class_offset: offset_of!(VirtualRasterQuadtreeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VirtualRasterQuadtreeData as Default>::default())),
            create_boxed: || Box::new(<VirtualRasterQuadtreeData as Default>::default()),
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


pub static VIRTUALRASTERQUADTREEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VirtualRasterQuadtreeData-Array",
    name_hash: 3242280836,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("VirtualRasterQuadtreeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for RasterQuadtreeData {
}

pub static RASTERQUADTREEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeData",
    name_hash: 1287771729,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(RasterQuadtreeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RasterQuadtreeData as Default>::default())),
            create_boxed: || Box::new(<RasterQuadtreeData as Default>::default()),
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


pub static RASTERQUADTREEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeData-Array",
    name_hash: 1846659173,
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
    name_hash: 874053704,
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


pub static SAMPLECENTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SampleCenter-Array",
    name_hash: 290075388,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("SampleCenter"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RasterQuadtreeNodeData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait RasterQuadtreeNodeDataTrait: super::core::DataContainerTrait {
}

impl RasterQuadtreeNodeDataTrait for RasterQuadtreeNodeData {
}

impl super::core::DataContainerTrait for RasterQuadtreeNodeData {
}

pub static RASTERQUADTREENODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeNodeData",
    name_hash: 2180542705,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(RasterQuadtreeNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RasterQuadtreeNodeData as Default>::default())),
            create_boxed: || Box::new(<RasterQuadtreeNodeData as Default>::default()),
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


pub static RASTERQUADTREENODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterQuadtreeNodeData-Array",
    name_hash: 1999786949,
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
    name_hash: 46608166,
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


pub static STYLETRANSFERTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StyleTransferTexture-Array",
    name_hash: 3756301202,
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
    name_hash: 2657205479,
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


pub static OVERLAYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OverlayType-Array",
    name_hash: 3015556819,
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
    name_hash: 898913072,
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


pub static EFFECTOVERLAYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectOverlayType-Array",
    name_hash: 2098684036,
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
    name_hash: 284171971,
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


pub static RASTERNODEUSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterNodeUsage-Array",
    name_hash: 3306021367,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterNodeUsage"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RectangularCoverageData {
}

pub trait RectangularCoverageDataTrait: TypeObject {
}

impl RectangularCoverageDataTrait for RectangularCoverageData {
}

pub static RECTANGULARCOVERAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RectangularCoverageData",
    name_hash: 2710185113,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RectangularCoverageData as Default>::default())),
            create_boxed: || Box::new(<RectangularCoverageData as Default>::default()),
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


pub static RECTANGULARCOVERAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RectangularCoverageData-Array",
    name_hash: 19866157,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RectangularCoverageData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for AutopaintOutput {
}

pub static AUTOPAINTOUTPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutput",
    name_hash: 1919363191,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::AUTOPAINTOUTPUTBASE_TYPE_INFO),
        super_class_offset: offset_of!(AutopaintOutput, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutopaintOutput as Default>::default())),
            create_boxed: || Box::new(<AutopaintOutput as Default>::default()),
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


pub static AUTOPAINTOUTPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutput-Array",
    name_hash: 2610467395,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintOutput"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
}

pub static AUTOPAINTOUTPUTOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputOverride",
    name_hash: 2137825795,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::AUTOPAINTOUTPUTOVERRIDEBASE_TYPE_INFO),
        super_class_offset: offset_of!(AutopaintOutputOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutopaintOutputOverride as Default>::default())),
            create_boxed: || Box::new(<AutopaintOutputOverride as Default>::default()),
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


pub static AUTOPAINTOUTPUTOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputOverride-Array",
    name_hash: 1971328183,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintOutputOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AutopaintOutputs {
    pub _glacier_base: super::entity::AutopaintOutputsBase,
}

pub trait AutopaintOutputsTrait: super::entity::AutopaintOutputsBaseTrait {
}

impl AutopaintOutputsTrait for AutopaintOutputs {
}

impl super::entity::AutopaintOutputsBaseTrait for AutopaintOutputs {
    fn outputs(&self) -> &Vec<Option<LockedTypeObject /* super::entity::AutopaintOutputBase */>> {
        self._glacier_base.outputs()
    }
    fn outputs_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::AutopaintOutputBase */>> {
        self._glacier_base.outputs_mut()
    }
}

impl super::core::AssetTrait for AutopaintOutputs {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for AutopaintOutputs {
}

pub static AUTOPAINTOUTPUTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputs",
    name_hash: 3209443108,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::AUTOPAINTOUTPUTSBASE_TYPE_INFO),
        super_class_offset: offset_of!(AutopaintOutputs, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutopaintOutputs as Default>::default())),
            create_boxed: || Box::new(<AutopaintOutputs as Default>::default()),
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


pub static AUTOPAINTOUTPUTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputs-Array",
    name_hash: 1675045520,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintOutputs"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for AutopaintConfigs {
}

pub static AUTOPAINTCONFIGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintConfigs",
    name_hash: 3553159953,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(AutopaintConfigs, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutopaintConfigs as Default>::default())),
            create_boxed: || Box::new(<AutopaintConfigs as Default>::default()),
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


pub static AUTOPAINTCONFIGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintConfigs-Array",
    name_hash: 797535397,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutopaintConfigs"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RasterTypeToRasterFormat {
    pub raster_type: super::entity::RasterType,
    pub raster_format: super::entity::RasterFormat,
}

pub trait RasterTypeToRasterFormatTrait: TypeObject {
    fn raster_type(&self) -> &super::entity::RasterType;
    fn raster_type_mut(&mut self) -> &mut super::entity::RasterType;
    fn raster_format(&self) -> &super::entity::RasterFormat;
    fn raster_format_mut(&mut self) -> &mut super::entity::RasterFormat;
}

impl RasterTypeToRasterFormatTrait for RasterTypeToRasterFormat {
    fn raster_type(&self) -> &super::entity::RasterType {
        &self.raster_type
    }
    fn raster_type_mut(&mut self) -> &mut super::entity::RasterType {
        &mut self.raster_type
    }
    fn raster_format(&self) -> &super::entity::RasterFormat {
        &self.raster_format
    }
    fn raster_format_mut(&mut self) -> &mut super::entity::RasterFormat {
        &mut self.raster_format
    }
}

pub static RASTERTYPETORASTERFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTypeToRasterFormat",
    name_hash: 2379833605,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RasterTypeToRasterFormat as Default>::default())),
            create_boxed: || Box::new(<RasterTypeToRasterFormat as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RasterType",
                name_hash: 2617211102,
                flags: MemberInfoFlags::new(0),
                field_type: "RasterType",
                rust_offset: offset_of!(RasterTypeToRasterFormat, raster_type),
            },
            FieldInfoData {
                name: "RasterFormat",
                name_hash: 1744737829,
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


pub static RASTERTYPETORASTERFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterTypeToRasterFormat-Array",
    name_hash: 3745990833,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("RasterTypeToRasterFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClassTypeAutopaintOutputsMap {
    pub class_type: String,
    pub autopaint_outputs: Option<LockedTypeObject /* AutopaintOutputs */>,
}

pub trait ClassTypeAutopaintOutputsMapTrait: TypeObject {
    fn class_type(&self) -> &String;
    fn class_type_mut(&mut self) -> &mut String;
    fn autopaint_outputs(&self) -> &Option<LockedTypeObject /* AutopaintOutputs */>;
    fn autopaint_outputs_mut(&mut self) -> &mut Option<LockedTypeObject /* AutopaintOutputs */>;
}

impl ClassTypeAutopaintOutputsMapTrait for ClassTypeAutopaintOutputsMap {
    fn class_type(&self) -> &String {
        &self.class_type
    }
    fn class_type_mut(&mut self) -> &mut String {
        &mut self.class_type
    }
    fn autopaint_outputs(&self) -> &Option<LockedTypeObject /* AutopaintOutputs */> {
        &self.autopaint_outputs
    }
    fn autopaint_outputs_mut(&mut self) -> &mut Option<LockedTypeObject /* AutopaintOutputs */> {
        &mut self.autopaint_outputs
    }
}

pub static CLASSTYPEAUTOPAINTOUTPUTSMAP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassTypeAutopaintOutputsMap",
    name_hash: 1329196910,
    flags: MemberInfoFlags::new(73),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClassTypeAutopaintOutputsMap as Default>::default())),
            create_boxed: || Box::new(<ClassTypeAutopaintOutputsMap as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ClassType",
                name_hash: 3204124979,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ClassTypeAutopaintOutputsMap, class_type),
            },
            FieldInfoData {
                name: "AutopaintOutputs",
                name_hash: 3209443108,
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


pub static CLASSTYPEAUTOPAINTOUTPUTSMAP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassTypeAutopaintOutputsMap-Array",
    name_hash: 1596303450,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ClassTypeAutopaintOutputsMap"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AutoPaintMeshData {
}

pub trait AutoPaintMeshDataTrait: TypeObject {
}

impl AutoPaintMeshDataTrait for AutoPaintMeshData {
}

pub static AUTOPAINTMESHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintMeshData",
    name_hash: 3384379115,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPaintMeshData as Default>::default())),
            create_boxed: || Box::new(<AutoPaintMeshData as Default>::default()),
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


pub static AUTOPAINTMESHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintMeshData-Array",
    name_hash: 1501869791,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("AutoPaintMeshData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AutoPaintRoadData {
}

pub trait AutoPaintRoadDataTrait: TypeObject {
}

impl AutoPaintRoadDataTrait for AutoPaintRoadData {
}

pub static AUTOPAINTROADDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintRoadData",
    name_hash: 3399598784,
    flags: MemberInfoFlags::new(36937),
    module: "TerrainBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPaintRoadData as Default>::default())),
            create_boxed: || Box::new(<AutoPaintRoadData as Default>::default()),
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


pub static AUTOPAINTROADDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPaintRoadData-Array",
    name_hash: 1249606004,
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
    name_hash: 542986178,
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


pub static OUTPUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutputType-Array",
    name_hash: 774193014,
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
    name_hash: 446189538,
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


pub static TPABLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TPABlendMode-Array",
    name_hash: 2838191830,
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
    name_hash: 2785138248,
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


pub static DEPTHBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DepthBuffer-Array",
    name_hash: 2147191036,
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
    name_hash: 3167605106,
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


pub static FACECULLING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceCulling-Array",
    name_hash: 3295594566,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("FaceCulling"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TerrainStreamingTree {
}

pub trait TerrainStreamingTreeTrait: TypeObject {
}

impl TerrainStreamingTreeTrait for TerrainStreamingTree {
}

pub static TERRAINSTREAMINGTREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTree",
    name_hash: 608875176,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainStreamingTree as Default>::default())),
            create_boxed: || Box::new(<TerrainStreamingTree as Default>::default()),
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


pub static TERRAINSTREAMINGTREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainStreamingTree-Array",
    name_hash: 3907253276,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("TerrainStreamingTree"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3173545970,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ITERRAIN_TYPE_INFO),
        super_class_offset: offset_of!(Terrain, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Terrain as Default>::default())),
            create_boxed: || Box::new(<Terrain as Default>::default()),
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


pub static TERRAIN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Terrain-Array",
    name_hash: 3771485894,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("Terrain"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ITerrain {
}

pub trait ITerrainTrait: TypeObject {
}

impl ITerrainTrait for ITerrain {
}

pub static ITERRAIN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITerrain",
    name_hash: 2910641339,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ITerrain as Default>::default())),
            create_boxed: || Box::new(<ITerrain as Default>::default()),
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


pub static ITERRAIN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITerrain-Array",
    name_hash: 1546658831,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("ITerrain"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HeightfieldDecal {
}

pub trait HeightfieldDecalTrait: TypeObject {
}

impl HeightfieldDecalTrait for HeightfieldDecal {
}

pub static HEIGHTFIELDDECAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecal",
    name_hash: 2054852567,
    flags: MemberInfoFlags::new(101),
    module: "TerrainBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HeightfieldDecal as Default>::default())),
            create_boxed: || Box::new(<HeightfieldDecal as Default>::default()),
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


pub static HEIGHTFIELDDECAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HeightfieldDecal-Array",
    name_hash: 331057379,
    flags: MemberInfoFlags::new(145),
    module: "TerrainBase",
    data: TypeInfoData::Array("HeightfieldDecal"),
    array_type: None,
    alignment: 8,
};


