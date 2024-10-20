use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_emitter_types(registry: &mut TypeRegistry) {
    registry.register_type(MESHEMITTERRESOURCE_TYPE_INFO);
    registry.register_type(MESHEMITTERRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(MESHEMITTERMASKRESOURCE_TYPE_INFO);
    registry.register_type(MESHEMITTERMASKRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHRESOURCE_TYPE_INFO);
    registry.register_type(EMITTERGRAPHRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSYSTEMSETTINGS_TYPE_INFO);
    registry.register_type(EMITTERSYSTEMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(FLATEMITTERDOCUMENT_TYPE_INFO);
    registry.register_type(FLATEMITTERDOCUMENT_ARRAY_TYPE_INFO);
    registry.register_type(SCALABLEEMITTERDOCUMENT_TYPE_INFO);
    registry.register_type(SCALABLEEMITTERDOCUMENT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERDOCUMENT_TYPE_INFO);
    registry.register_type(EMITTERDOCUMENT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERTEMPLATEDATA_TYPE_INFO);
    registry.register_type(EMITTERTEMPLATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PERPARTICLEPARAMS_TYPE_INFO);
    registry.register_type(PERPARTICLEPARAMS_ARRAY_TYPE_INFO);
    registry.register_type(PROCESSORDATA_TYPE_INFO);
    registry.register_type(PROCESSORDATA_ARRAY_TYPE_INFO);
    registry.register_type(EVALUATORDATA_TYPE_INFO);
    registry.register_type(EVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTICLESORTING_TYPE_INFO);
    registry.register_type(PARTICLESORTING_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERDRAWORDER_TYPE_INFO);
    registry.register_type(EMITTERDRAWORDER_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSCHEMATICINPUTPARAMETER_TYPE_INFO);
    registry.register_type(EMITTERSCHEMATICINPUTPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(PROCESSORTYPE_TYPE_INFO);
    registry.register_type(PROCESSORTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EVALUATORTYPE_TYPE_INFO);
    registry.register_type(EVALUATORTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMFREQUENCY_TYPE_INFO);
    registry.register_type(RANDOMFREQUENCY_ARRAY_TYPE_INFO);
    registry.register_type(EMITTABLEALIGNMENT_TYPE_INFO);
    registry.register_type(EMITTABLEALIGNMENT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTABLETYPE_TYPE_INFO);
    registry.register_type(EMITTABLETYPE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTABLEFIELD_TYPE_INFO);
    registry.register_type(EMITTABLEFIELD_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUME_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUME_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPH_TYPE_INFO);
    registry.register_type(EMITTERGRAPH_ARRAY_TYPE_INFO);
    registry.register_type(RUNTIMEPARTICLEDATABUFFER_TYPE_INFO);
    registry.register_type(RUNTIMEPARTICLEDATABUFFER_ARRAY_TYPE_INFO);
    registry.register_type(RUNTIMESAMPLER_TYPE_INFO);
    registry.register_type(RUNTIMESAMPLER_ARRAY_TYPE_INFO);
    registry.register_type(RUNTIMETEXTURE_TYPE_INFO);
    registry.register_type(RUNTIMETEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHMESH_TYPE_INFO);
    registry.register_type(EMITTERGRAPHMESH_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHUSERBUFFER_TYPE_INFO);
    registry.register_type(EMITTERGRAPHUSERBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHSPAWNMODE_TYPE_INFO);
    registry.register_type(EMITTERGRAPHSPAWNMODE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHSORTMODE_TYPE_INFO);
    registry.register_type(EMITTERGRAPHSORTMODE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHDRAWLAYER_TYPE_INFO);
    registry.register_type(EMITTERGRAPHDRAWLAYER_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHDRAWPASS_TYPE_INFO);
    registry.register_type(EMITTERGRAPHDRAWPASS_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHCULLEDBEHAVIOR_TYPE_INFO);
    registry.register_type(EMITTERGRAPHCULLEDBEHAVIOR_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHCONFIG_TYPE_INFO);
    registry.register_type(EMITTERGRAPHCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMSPAWNRATEMODIFIER_TYPE_INFO);
    registry.register_type(RANDOMSPAWNRATEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHRANDOMFREQUENCY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHRANDOMFREQUENCY_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSPAWNRATEMODIFIER_TYPE_INFO);
    registry.register_type(EMITTERSPAWNRATEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERSPAWNRATEMODIFIER_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERSPAWNRATEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNRATEMODIFIER_TYPE_INFO);
    registry.register_type(SPAWNRATEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHSPAWNPROPERTY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHSPAWNPROPERTY_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNMODECONTINUOUS_TYPE_INFO);
    registry.register_type(SPAWNMODECONTINUOUS_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNMODEBURST_TYPE_INFO);
    registry.register_type(SPAWNMODEBURST_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNMODEINFO_TYPE_INFO);
    registry.register_type(SPAWNMODEINFO_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXSHADERPARAM_TYPE_INFO);
    registry.register_type(VERTEXSHADERPARAM_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXSHADERTEXTUREPARAM_TYPE_INFO);
    registry.register_type(VERTEXSHADERTEXTUREPARAM_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHPARTICLEDATATYPE_TYPE_INFO);
    registry.register_type(EMITTERGRAPHPARTICLEDATATYPE_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURENODEFILTER_TYPE_INFO);
    registry.register_type(TEXTURENODEFILTER_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURENODEADDRESS_TYPE_INFO);
    registry.register_type(TEXTURENODEADDRESS_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMESASSET_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMESASSET_ARRAY_TYPE_INFO);
    registry.register_type(MESHEMITTERMASKASSET_TYPE_INFO);
    registry.register_type(MESHEMITTERMASKASSET_ARRAY_TYPE_INFO);
    registry.register_type(MESHEMITTERASSET_TYPE_INFO);
    registry.register_type(MESHEMITTERASSET_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERASSET_TYPE_INFO);
    registry.register_type(EMITTERASSET_ARRAY_TYPE_INFO);
    registry.register_type(UPDATESTENCILMASKDATA_TYPE_INFO);
    registry.register_type(UPDATESTENCILMASKDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSTENCILMASK_TYPE_INFO);
    registry.register_type(EMITTERSTENCILMASK_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEVOLUMEMASKDATA_TYPE_INFO);
    registry.register_type(UPDATEVOLUMEMASKDATA_ARRAY_TYPE_INFO);
    registry.register_type(VOLUMEMASKTYPE_TYPE_INFO);
    registry.register_type(VOLUMEMASKTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEVOLUMETRICDATA_TYPE_INFO);
    registry.register_type(UPDATEVOLUMETRICDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEMESHEMITTERMASKDATA_TYPE_INFO);
    registry.register_type(UPDATEMESHEMITTERMASKDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEMESHEMITTERSOURCEDATA_TYPE_INFO);
    registry.register_type(UPDATEMESHEMITTERSOURCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEVERTEXANIMDATA_TYPE_INFO);
    registry.register_type(UPDATEVERTEXANIMDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEBEAMPOINTDATA_TYPE_INFO);
    registry.register_type(UPDATEBEAMPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(BEAMINTERPOLATION_TYPE_INFO);
    registry.register_type(BEAMINTERPOLATION_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEBEAMTARGETDATA_TYPE_INFO);
    registry.register_type(UPDATEBEAMTARGETDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEBEAMSOURCEDATA_TYPE_INFO);
    registry.register_type(UPDATEBEAMSOURCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARAMOVERRIDESELECTION_TYPE_INFO);
    registry.register_type(PARAMOVERRIDESELECTION_ARRAY_TYPE_INFO);
    registry.register_type(LOCATIONSELECTION_TYPE_INFO);
    registry.register_type(LOCATIONSELECTION_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEQUADBENDINGANGLEDATA_TYPE_INFO);
    registry.register_type(UPDATEQUADBENDINGANGLEDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATECLIPSCALEDATA_TYPE_INFO);
    registry.register_type(UPDATECLIPSCALEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SNAPTOWATERDATA_TYPE_INFO);
    registry.register_type(SNAPTOWATERDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATECOLLISIONDATA_TYPE_INFO);
    registry.register_type(UPDATECOLLISIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERCOLLISIONEFFECTORIENTATION_TYPE_INFO);
    registry.register_type(EMITTERCOLLISIONEFFECTORIENTATION_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERCOLLISIONTYPE_TYPE_INFO);
    registry.register_type(EMITTERCOLLISIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERCOLLISIONPRIORITY_TYPE_INFO);
    registry.register_type(EMITTERCOLLISIONPRIORITY_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSNAPVELOCITYTYPE_TYPE_INFO);
    registry.register_type(EMITTERSNAPVELOCITYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERTERRAINSNAPTYPE_TYPE_INFO);
    registry.register_type(EMITTERTERRAINSNAPTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERCOLLISIONMETHOD_TYPE_INFO);
    registry.register_type(EMITTERCOLLISIONMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(UPDATECAMERAPROXIMITYDATA_TYPE_INFO);
    registry.register_type(UPDATECAMERAPROXIMITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATETRAPEZOIDSHAPEDATA_TYPE_INFO);
    registry.register_type(UPDATETRAPEZOIDSHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEBACKLIGHTDATA_TYPE_INFO);
    registry.register_type(UPDATEBACKLIGHTDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATELIGHTWRAPAROUNDDATA_TYPE_INFO);
    registry.register_type(UPDATELIGHTWRAPAROUNDDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATETEXTURECOLORLERPDATA_TYPE_INFO);
    registry.register_type(UPDATETEXTURECOLORLERPDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEALPHALEVELSCALEDATA_TYPE_INFO);
    registry.register_type(UPDATEALPHALEVELSCALEDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEALPHALEVELMAXDATA_TYPE_INFO);
    registry.register_type(UPDATEALPHALEVELMAXDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEALPHALEVELMINDATA_TYPE_INFO);
    registry.register_type(UPDATEALPHALEVELMINDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATERIBBONTEXTUREDATA_TYPE_INFO);
    registry.register_type(UPDATERIBBONTEXTUREDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATERIBBONFADEDATA_TYPE_INFO);
    registry.register_type(UPDATERIBBONFADEDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATETRANSPARENCYSECONDARYDATA_TYPE_INFO);
    registry.register_type(UPDATETRANSPARENCYSECONDARYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATETRANSPARENCYDATA_TYPE_INFO);
    registry.register_type(UPDATETRANSPARENCYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATECOLORSECONDARYDATA_TYPE_INFO);
    registry.register_type(UPDATECOLORSECONDARYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATECOLORDATA_TYPE_INFO);
    registry.register_type(UPDATECOLORDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATECUSTOMPARAMSDATA_TYPE_INFO);
    registry.register_type(UPDATECUSTOMPARAMSDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATESIZEZDATA_TYPE_INFO);
    registry.register_type(UPDATESIZEZDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATESIZEYDATA_TYPE_INFO);
    registry.register_type(UPDATESIZEYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATESIZEXDATA_TYPE_INFO);
    registry.register_type(UPDATESIZEXDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATESIZEDATA_TYPE_INFO);
    registry.register_type(UPDATESIZEDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEROTATIONDATA_TYPE_INFO);
    registry.register_type(UPDATEROTATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATETEXTURECOORDSDATA_TYPE_INFO);
    registry.register_type(UPDATETEXTURECOORDSDATA_ARRAY_TYPE_INFO);
    registry.register_type(TEXCOORDMODIFIER_TYPE_INFO);
    registry.register_type(TEXCOORDMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEMIMICOVERRIDESDATA_TYPE_INFO);
    registry.register_type(UPDATEMIMICOVERRIDESDATA_ARRAY_TYPE_INFO);
    registry.register_type(MIMICEMITTERDATA_TYPE_INFO);
    registry.register_type(MIMICEMITTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERDATA_TYPE_INFO);
    registry.register_type(EMITTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(TURBULANCEDATA_TYPE_INFO);
    registry.register_type(TURBULANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(TURBULENCENOISETYPE_TYPE_INFO);
    registry.register_type(TURBULENCENOISETYPE_ARRAY_TYPE_INFO);
    registry.register_type(AIRRESISTANCEDATA_TYPE_INFO);
    registry.register_type(AIRRESISTANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(WORLDFORCESDATA_TYPE_INFO);
    registry.register_type(WORLDFORCESDATA_ARRAY_TYPE_INFO);
    registry.register_type(WORLDWINDDATA_TYPE_INFO);
    registry.register_type(WORLDWINDDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALFORCEDATA_TYPE_INFO);
    registry.register_type(LOCALFORCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(GRAVITYDATA_TYPE_INFO);
    registry.register_type(GRAVITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEAGEDATA_TYPE_INFO);
    registry.register_type(UPDATEAGEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNCOLORRANDOMDATA_TYPE_INFO);
    registry.register_type(SPAWNCOLORRANDOMDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNROTATIONSPEEDDATA_TYPE_INFO);
    registry.register_type(SPAWNROTATIONSPEEDDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNORIENTATIONDATA_TYPE_INFO);
    registry.register_type(SPAWNORIENTATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNROTATIONDATA_TYPE_INFO);
    registry.register_type(SPAWNROTATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNANIMATIONFRAMEDATA_TYPE_INFO);
    registry.register_type(SPAWNANIMATIONFRAMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNANIMATIONDATA_TYPE_INFO);
    registry.register_type(SPAWNANIMATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNPOSITIONDATA_TYPE_INFO);
    registry.register_type(SPAWNPOSITIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNSIZEDATA_TYPE_INFO);
    registry.register_type(SPAWNSIZEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNSPEEDDATA_TYPE_INFO);
    registry.register_type(SPAWNSPEEDDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNDIRECTIONDATA_TYPE_INFO);
    registry.register_type(SPAWNDIRECTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNPOINTCLOUDDATA_TYPE_INFO);
    registry.register_type(SPAWNPOINTCLOUDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PREROLLDATA_TYPE_INFO);
    registry.register_type(PREROLLDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNRIBBONRATEDATA_TYPE_INFO);
    registry.register_type(SPAWNRIBBONRATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNRATEDATA_TYPE_INFO);
    registry.register_type(SPAWNRATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMSHADERDATA_TYPE_INFO);
    registry.register_type(CUSTOMSHADERDATA_ARRAY_TYPE_INFO);
    registry.register_type(BASEEMITTERDATA_TYPE_INFO);
    registry.register_type(BASEEMITTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(POLYNOMIALXYZWEVALUATORDATA_TYPE_INFO);
    registry.register_type(POLYNOMIALXYZWEVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(MULTICOLORINTERPDATA_TYPE_INFO);
    registry.register_type(MULTICOLORINTERPDATA_ARRAY_TYPE_INFO);
    registry.register_type(MULTICOLORGRADIENT_TYPE_INFO);
    registry.register_type(MULTICOLORGRADIENT_ARRAY_TYPE_INFO);
    registry.register_type(MULTICOLORGRADIENTKEYPOINT_TYPE_INFO);
    registry.register_type(MULTICOLORGRADIENTKEYPOINT_ARRAY_TYPE_INFO);
    registry.register_type(POLYNOMIALCOLORINTERPDATA_TYPE_INFO);
    registry.register_type(POLYNOMIALCOLORINTERPDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONSTANTEVALUATORDATA_TYPE_INFO);
    registry.register_type(CONSTANTEVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(CAMERAPROXIMITYEVALUATORDATA_TYPE_INFO);
    registry.register_type(CAMERAPROXIMITYEVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(SUPERSPHEREEVALUATORDATA_TYPE_INFO);
    registry.register_type(SUPERSPHEREEVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPHEREEVALUATORDATA_TYPE_INFO);
    registry.register_type(SPHEREEVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOXEVALUATORDATA_TYPE_INFO);
    registry.register_type(BOXEVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMXYZWEVALUATORDATA_TYPE_INFO);
    registry.register_type(RANDOMXYZWEVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMXYZEVALUATORDATA_TYPE_INFO);
    registry.register_type(RANDOMXYZEVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMEVALUATORDATA_TYPE_INFO);
    registry.register_type(RANDOMEVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(ROTATEVECTORDATA_TYPE_INFO);
    registry.register_type(ROTATEVECTORDATA_ARRAY_TYPE_INFO);
    registry.register_type(SAMPLETEXTUREDATA_TYPE_INFO);
    registry.register_type(SAMPLETEXTUREDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPLINEDATA_TYPE_INFO);
    registry.register_type(SPLINEDATA_ARRAY_TYPE_INFO);
    registry.register_type(POLYNOMIALOPERATORDATA_TYPE_INFO);
    registry.register_type(POLYNOMIALOPERATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(POLYNOMIALOPERATION_TYPE_INFO);
    registry.register_type(POLYNOMIALOPERATION_ARRAY_TYPE_INFO);
    registry.register_type(POLYNOMIALTEMPDATA_TYPE_INFO);
    registry.register_type(POLYNOMIALTEMPDATA_ARRAY_TYPE_INFO);
    registry.register_type(POLYNOMIALDATA_TYPE_INFO);
    registry.register_type(POLYNOMIALDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEFAULTEVALUATORDATA_TYPE_INFO);
    registry.register_type(DEFAULTEVALUATORDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshEmitterResource {
}

pub const MESHEMITTERRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterResource",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(MESHEMITTERRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshEmitterResource {
    fn type_info() -> &'static TypeInfo {
        MESHEMITTERRESOURCE_TYPE_INFO
    }
}


pub const MESHEMITTERRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MeshEmitterResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshEmitterMaskResource {
}

pub const MESHEMITTERMASKRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskResource",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(MESHEMITTERMASKRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshEmitterMaskResource {
    fn type_info() -> &'static TypeInfo {
        MESHEMITTERMASKRESOURCE_TYPE_INFO
    }
}


pub const MESHEMITTERMASKRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MeshEmitterMaskResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterGraphResource {
}

pub const EMITTERGRAPHRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphResource",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(EMITTERGRAPHRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EmitterGraphResource {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHRESOURCE_TYPE_INFO
    }
}


pub const EMITTERGRAPHRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterSystemSettings {
    pub enable: bool,
    pub update_job_enable: bool,
    pub skip_update_max_count: u32,
    pub force_job_count: i32,
    pub time_scale: f32,
    pub global_reset_start_time_interval: f32,
    pub enable_fixed_time_step: bool,
    pub enable_fixed_delta: bool,
    pub enable_jobs: bool,
    pub collision_ray_cast_enable: bool,
    pub collision_ray_cast_max_count: u32,
    pub proximity_physics_entities_max_count: u32,
    pub draw_debug_ray_cast_collision: bool,
    pub emitter_quality_level: super::core::QualityLevel,
    pub template_timeout_time: f32,
    pub precise_wind_and_force_max_distance: f32,
    pub turbulence_max_distance: f32,
    pub screen_area_culling_start: f32,
    pub screen_area_culling_end: f32,
    pub screen_area_culling_min_total_area: f32,
    pub screen_area_culling_max_total_area: f32,
    pub screen_area_culling_max_multiplier: f32,
    pub process_job_yield_time: f32,
    pub visible_job_yield_time: f32,
    pub mesh_emitter_motion_blur_enable: bool,
    pub enable_rendering: bool,
    pub draw_stats: u32,
    pub collect_performance_stats: bool,
    pub collect_performance_stats_time: i32,
    pub draw_mem_stats: u32,
    pub draw_stats_sampling_period: f32,
    pub draw_stats_entries_per_page: u32,
    pub draw_stats_page_index: u32,
    pub draw_stats_filter: String,
    pub hide_inactive_stats: bool,
    pub save_list_active_emitters: bool,
    pub draw_emitter_name: bool,
    pub z_buffer_cull_enable: bool,
    pub draw_projected_boxes: bool,
    pub draw_bounding_boxes: u32,
    pub min_screen_area: f32,
    pub min_screen_area_threshold: f32,
    pub force_cull_distance: f32,
    pub force_cull_fade_far_distance: f32,
    pub draw_transforms: bool,
    pub draw_light_probe_sample_transforms: bool,
    pub draw_debug_base_atlas: bool,
    pub draw_debug_normal_atlas: bool,
    pub draw_debug_atlas_miplevel: u32,
    pub draw_debug_atlas_texture_index: i32,
    pub draw_debug_atlas_alpha: bool,
    pub draw_debug_emitter_exclusion_volumes: i32,
    pub draw_debug_atlas_page_index: i32,
    pub draw_debug_emitter_sun_transmittance_map_group: bool,
    pub draw_debug_emitter_sun_transmittance_maps_links: bool,
    pub force_sun_transmittance_on_all_emitters: bool,
    pub emitter_render_sun_transmittance_views_first: bool,
    pub draw_debug_emitter_vertex_buffer_usage: bool,
    pub emitter_gpu_lighting_enable: bool,
    pub walrus_lighting_enable: bool,
    pub emitter_gpu_lighting_pipeline_shaders_enabled: bool,
    pub system_shaders_path: String,
    pub system_v_s_f_path: String,
    pub crossfire_driver_profile_available: bool,
    pub quad_clip_scale_enable: bool,
    pub quad_enable_rendering: bool,
    pub quad_nice_rendering_enable: bool,
    pub quad_simple_rendering_enable: bool,
    pub quad_enable_opaque: bool,
    pub quad_enable_custom_shader: bool,
    pub quad_color_shader_costs_enable: bool,
    pub quad_enable_sorting: bool,
    pub quad_enable_wireframe: bool,
    pub quad_half_res_enable: bool,
    pub quad_groups_join_all: bool,
    pub quad_groups_join_none: bool,
    pub quad_groups_join_nice_and_simple: bool,
    pub quad_technique: i32,
    pub quad_vertex_shadows_enable: bool,
    pub quad_cloud_vertex_shadows_enable: bool,
    pub quad_planar_reflection_enable: bool,
    pub quad_point_lights_enable: bool,
    pub quad_spot_lights_enable: bool,
    pub punctual_light_threshold_squared: f32,
    pub quad_near_fade_distance: f32,
    pub custom_emitter_position_sorting: bool,
    pub quad_max_count: u32,
    pub mesh_rendering_enable: bool,
    pub mesh_draw_transforms: bool,
    pub mesh_draw_bounding_boxes: bool,
    pub mesh_shadow_enable: bool,
    pub mesh_planar_reflection_enable: bool,
    pub mesh_culling_distance: f32,
    pub mesh_draw_count_limit: u32,
    pub mesh_streaming_priority_multiplier: f32,
    pub mesh_draw_cull_stats: bool,
    pub mesh_max_count: u32,
    pub skip_render_if_probe_is_uninitialized: bool,
    pub batch_update_light_probes_enable: bool,
    pub quad_light_probe_max_update_count: u32,
    pub graph_light_probe_max_update_count: u32,
    pub mesh_light_probe_max_update_count: u32,
    pub graph_emitter_enabled: bool,
    pub graph_emitter_draw_debug_stats: bool,
    pub graph_emitter_draw_debug_buffers: bool,
    pub graph_emitter_draw_debug_view_visible_instances: bool,
    pub graph_emitter_overlapped_compute_enable: bool,
    pub emitter_graph_block_allocator_max_byte_count: u32,
    pub emitter_graph_block_allocator_block_max_count: u32,
    pub emitter_graph_max_defrag_operations_per_frame: u32,
    pub emitter_graph_draw_debug_uber_buffer: bool,
    pub emitter_graph_uber_buffer_defrag_enable: bool,
}

pub const EMITTERSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, enable),
            },
            FieldInfoData {
                name: "UpdateJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, update_job_enable),
            },
            FieldInfoData {
                name: "SkipUpdateMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, skip_update_max_count),
            },
            FieldInfoData {
                name: "ForceJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, force_job_count),
            },
            FieldInfoData {
                name: "TimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, time_scale),
            },
            FieldInfoData {
                name: "GlobalResetStartTimeInterval",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, global_reset_start_time_interval),
            },
            FieldInfoData {
                name: "EnableFixedTimeStep",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, enable_fixed_time_step),
            },
            FieldInfoData {
                name: "EnableFixedDelta",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, enable_fixed_delta),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, enable_jobs),
            },
            FieldInfoData {
                name: "CollisionRayCastEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, collision_ray_cast_enable),
            },
            FieldInfoData {
                name: "CollisionRayCastMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, collision_ray_cast_max_count),
            },
            FieldInfoData {
                name: "ProximityPhysicsEntitiesMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, proximity_physics_entities_max_count),
            },
            FieldInfoData {
                name: "DrawDebugRayCastCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_ray_cast_collision),
            },
            FieldInfoData {
                name: "EmitterQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, emitter_quality_level),
            },
            FieldInfoData {
                name: "TemplateTimeoutTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, template_timeout_time),
            },
            FieldInfoData {
                name: "PreciseWindAndForceMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, precise_wind_and_force_max_distance),
            },
            FieldInfoData {
                name: "TurbulenceMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, turbulence_max_distance),
            },
            FieldInfoData {
                name: "ScreenAreaCullingStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, screen_area_culling_start),
            },
            FieldInfoData {
                name: "ScreenAreaCullingEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, screen_area_culling_end),
            },
            FieldInfoData {
                name: "ScreenAreaCullingMinTotalArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, screen_area_culling_min_total_area),
            },
            FieldInfoData {
                name: "ScreenAreaCullingMaxTotalArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, screen_area_culling_max_total_area),
            },
            FieldInfoData {
                name: "ScreenAreaCullingMaxMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, screen_area_culling_max_multiplier),
            },
            FieldInfoData {
                name: "ProcessJobYieldTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, process_job_yield_time),
            },
            FieldInfoData {
                name: "VisibleJobYieldTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, visible_job_yield_time),
            },
            FieldInfoData {
                name: "MeshEmitterMotionBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_emitter_motion_blur_enable),
            },
            FieldInfoData {
                name: "EnableRendering",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, enable_rendering),
            },
            FieldInfoData {
                name: "DrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_stats),
            },
            FieldInfoData {
                name: "CollectPerformanceStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, collect_performance_stats),
            },
            FieldInfoData {
                name: "CollectPerformanceStatsTime",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, collect_performance_stats_time),
            },
            FieldInfoData {
                name: "DrawMemStats",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_mem_stats),
            },
            FieldInfoData {
                name: "DrawStatsSamplingPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_stats_sampling_period),
            },
            FieldInfoData {
                name: "DrawStatsEntriesPerPage",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_stats_entries_per_page),
            },
            FieldInfoData {
                name: "DrawStatsPageIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_stats_page_index),
            },
            FieldInfoData {
                name: "DrawStatsFilter",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_stats_filter),
            },
            FieldInfoData {
                name: "HideInactiveStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, hide_inactive_stats),
            },
            FieldInfoData {
                name: "SaveListActiveEmitters",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, save_list_active_emitters),
            },
            FieldInfoData {
                name: "DrawEmitterName",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_emitter_name),
            },
            FieldInfoData {
                name: "ZBufferCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, z_buffer_cull_enable),
            },
            FieldInfoData {
                name: "DrawProjectedBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_projected_boxes),
            },
            FieldInfoData {
                name: "DrawBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_bounding_boxes),
            },
            FieldInfoData {
                name: "MinScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, min_screen_area),
            },
            FieldInfoData {
                name: "MinScreenAreaThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, min_screen_area_threshold),
            },
            FieldInfoData {
                name: "ForceCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, force_cull_distance),
            },
            FieldInfoData {
                name: "ForceCullFadeFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, force_cull_fade_far_distance),
            },
            FieldInfoData {
                name: "DrawTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_transforms),
            },
            FieldInfoData {
                name: "DrawLightProbeSampleTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_light_probe_sample_transforms),
            },
            FieldInfoData {
                name: "DrawDebugBaseAtlas",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_base_atlas),
            },
            FieldInfoData {
                name: "DrawDebugNormalAtlas",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_normal_atlas),
            },
            FieldInfoData {
                name: "DrawDebugAtlasMiplevel",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_atlas_miplevel),
            },
            FieldInfoData {
                name: "DrawDebugAtlasTextureIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_atlas_texture_index),
            },
            FieldInfoData {
                name: "DrawDebugAtlasAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_atlas_alpha),
            },
            FieldInfoData {
                name: "DrawDebugEmitterExclusionVolumes",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_emitter_exclusion_volumes),
            },
            FieldInfoData {
                name: "DrawDebugAtlasPageIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_atlas_page_index),
            },
            FieldInfoData {
                name: "DrawDebugEmitterSunTransmittanceMapGroup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_emitter_sun_transmittance_map_group),
            },
            FieldInfoData {
                name: "DrawDebugEmitterSunTransmittanceMapsLinks",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_emitter_sun_transmittance_maps_links),
            },
            FieldInfoData {
                name: "ForceSunTransmittanceOnAllEmitters",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, force_sun_transmittance_on_all_emitters),
            },
            FieldInfoData {
                name: "EmitterRenderSunTransmittanceViewsFirst",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, emitter_render_sun_transmittance_views_first),
            },
            FieldInfoData {
                name: "DrawDebugEmitterVertexBufferUsage",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_emitter_vertex_buffer_usage),
            },
            FieldInfoData {
                name: "EmitterGpuLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, emitter_gpu_lighting_enable),
            },
            FieldInfoData {
                name: "WalrusLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, walrus_lighting_enable),
            },
            FieldInfoData {
                name: "EmitterGpuLightingPipelineShadersEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, emitter_gpu_lighting_pipeline_shaders_enabled),
            },
            FieldInfoData {
                name: "SystemShadersPath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, system_shaders_path),
            },
            FieldInfoData {
                name: "SystemVSFPath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, system_v_s_f_path),
            },
            FieldInfoData {
                name: "CrossfireDriverProfileAvailable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, crossfire_driver_profile_available),
            },
            FieldInfoData {
                name: "QuadClipScaleEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_clip_scale_enable),
            },
            FieldInfoData {
                name: "QuadEnableRendering",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_enable_rendering),
            },
            FieldInfoData {
                name: "QuadNiceRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_nice_rendering_enable),
            },
            FieldInfoData {
                name: "QuadSimpleRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_simple_rendering_enable),
            },
            FieldInfoData {
                name: "QuadEnableOpaque",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_enable_opaque),
            },
            FieldInfoData {
                name: "QuadEnableCustomShader",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_enable_custom_shader),
            },
            FieldInfoData {
                name: "QuadColorShaderCostsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_color_shader_costs_enable),
            },
            FieldInfoData {
                name: "QuadEnableSorting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_enable_sorting),
            },
            FieldInfoData {
                name: "QuadEnableWireframe",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_enable_wireframe),
            },
            FieldInfoData {
                name: "QuadHalfResEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_half_res_enable),
            },
            FieldInfoData {
                name: "QuadGroupsJoinAll",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_groups_join_all),
            },
            FieldInfoData {
                name: "QuadGroupsJoinNone",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_groups_join_none),
            },
            FieldInfoData {
                name: "QuadGroupsJoinNiceAndSimple",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_groups_join_nice_and_simple),
            },
            FieldInfoData {
                name: "QuadTechnique",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_technique),
            },
            FieldInfoData {
                name: "QuadVertexShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_vertex_shadows_enable),
            },
            FieldInfoData {
                name: "QuadCloudVertexShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_cloud_vertex_shadows_enable),
            },
            FieldInfoData {
                name: "QuadPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_planar_reflection_enable),
            },
            FieldInfoData {
                name: "QuadPointLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_point_lights_enable),
            },
            FieldInfoData {
                name: "QuadSpotLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_spot_lights_enable),
            },
            FieldInfoData {
                name: "PunctualLightThresholdSquared",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, punctual_light_threshold_squared),
            },
            FieldInfoData {
                name: "QuadNearFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_near_fade_distance),
            },
            FieldInfoData {
                name: "CustomEmitterPositionSorting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, custom_emitter_position_sorting),
            },
            FieldInfoData {
                name: "QuadMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_max_count),
            },
            FieldInfoData {
                name: "MeshRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_rendering_enable),
            },
            FieldInfoData {
                name: "MeshDrawTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_draw_transforms),
            },
            FieldInfoData {
                name: "MeshDrawBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_draw_bounding_boxes),
            },
            FieldInfoData {
                name: "MeshShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_shadow_enable),
            },
            FieldInfoData {
                name: "MeshPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_planar_reflection_enable),
            },
            FieldInfoData {
                name: "MeshCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_culling_distance),
            },
            FieldInfoData {
                name: "MeshDrawCountLimit",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_draw_count_limit),
            },
            FieldInfoData {
                name: "MeshStreamingPriorityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_streaming_priority_multiplier),
            },
            FieldInfoData {
                name: "MeshDrawCullStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_draw_cull_stats),
            },
            FieldInfoData {
                name: "MeshMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_max_count),
            },
            FieldInfoData {
                name: "SkipRenderIfProbeIsUninitialized",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, skip_render_if_probe_is_uninitialized),
            },
            FieldInfoData {
                name: "BatchUpdateLightProbesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, batch_update_light_probes_enable),
            },
            FieldInfoData {
                name: "QuadLightProbeMaxUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, quad_light_probe_max_update_count),
            },
            FieldInfoData {
                name: "GraphLightProbeMaxUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, graph_light_probe_max_update_count),
            },
            FieldInfoData {
                name: "MeshLightProbeMaxUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, mesh_light_probe_max_update_count),
            },
            FieldInfoData {
                name: "GraphEmitterEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, graph_emitter_enabled),
            },
            FieldInfoData {
                name: "GraphEmitterDrawDebugStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, graph_emitter_draw_debug_stats),
            },
            FieldInfoData {
                name: "GraphEmitterDrawDebugBuffers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, graph_emitter_draw_debug_buffers),
            },
            FieldInfoData {
                name: "GraphEmitterDrawDebugViewVisibleInstances",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, graph_emitter_draw_debug_view_visible_instances),
            },
            FieldInfoData {
                name: "GraphEmitterOverlappedComputeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, graph_emitter_overlapped_compute_enable),
            },
            FieldInfoData {
                name: "EmitterGraphBlockAllocatorMaxByteCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, emitter_graph_block_allocator_max_byte_count),
            },
            FieldInfoData {
                name: "EmitterGraphBlockAllocatorBlockMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, emitter_graph_block_allocator_block_max_count),
            },
            FieldInfoData {
                name: "EmitterGraphMaxDefragOperationsPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, emitter_graph_max_defrag_operations_per_frame),
            },
            FieldInfoData {
                name: "EmitterGraphDrawDebugUberBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, emitter_graph_draw_debug_uber_buffer),
            },
            FieldInfoData {
                name: "EmitterGraphUberBufferDefragEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemSettings, emitter_graph_uber_buffer_defrag_enable),
            },
        ],
    }),
    array_type: Some(EMITTERSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterSystemSettings {
    fn type_info() -> &'static TypeInfo {
        EMITTERSYSTEMSETTINGS_TYPE_INFO
    }
}


pub const EMITTERSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterSystemSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FlatEmitterDocument {
    pub template_data: EmitterTemplateData,
}

pub const FLATEMITTERDOCUMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlatEmitterDocument",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERDOCUMENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TemplateData",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERTEMPLATEDATA_TYPE_INFO,
                rust_offset: offset_of!(FlatEmitterDocument, template_data),
            },
        ],
    }),
    array_type: Some(FLATEMITTERDOCUMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FlatEmitterDocument {
    fn type_info() -> &'static TypeInfo {
        FLATEMITTERDOCUMENT_TYPE_INFO
    }
}


pub const FLATEMITTERDOCUMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlatEmitterDocument-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("FlatEmitterDocument-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ScalableEmitterDocument {
    pub template_data_low: EmitterTemplateData,
    pub template_data_medium: EmitterTemplateData,
    pub template_data_high: EmitterTemplateData,
    pub template_data_ultra: EmitterTemplateData,
}

pub const SCALABLEEMITTERDOCUMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScalableEmitterDocument",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERDOCUMENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TemplateDataLow",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERTEMPLATEDATA_TYPE_INFO,
                rust_offset: offset_of!(ScalableEmitterDocument, template_data_low),
            },
            FieldInfoData {
                name: "TemplateDataMedium",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERTEMPLATEDATA_TYPE_INFO,
                rust_offset: offset_of!(ScalableEmitterDocument, template_data_medium),
            },
            FieldInfoData {
                name: "TemplateDataHigh",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERTEMPLATEDATA_TYPE_INFO,
                rust_offset: offset_of!(ScalableEmitterDocument, template_data_high),
            },
            FieldInfoData {
                name: "TemplateDataUltra",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERTEMPLATEDATA_TYPE_INFO,
                rust_offset: offset_of!(ScalableEmitterDocument, template_data_ultra),
            },
        ],
    }),
    array_type: Some(SCALABLEEMITTERDOCUMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScalableEmitterDocument {
    fn type_info() -> &'static TypeInfo {
        SCALABLEEMITTERDOCUMENT_TYPE_INFO
    }
}


pub const SCALABLEEMITTERDOCUMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScalableEmitterDocument-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ScalableEmitterDocument-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterDocument {
}

pub const EMITTERDOCUMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDocument",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EMITTERDOCUMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterDocument {
    fn type_info() -> &'static TypeInfo {
        EMITTERDOCUMENT_TYPE_INFO
    }
}


pub const EMITTERDOCUMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDocument-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterDocument-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterTemplateData {
    pub root_processor: ProcessorData,
    pub max_count: u32,
    pub lifetime: f32,
    pub time_scale: f32,
    pub repeat_particle_spawning: bool,
    pub lifetime_frame_count: u32,
    pub follow_spawn_source: bool,
    pub follow_spawn_source_velocity: bool,
    pub kill_particles_with_emitter: bool,
    pub kill_ribbon_tail_distance: f32,
    pub smooth_ribbon_spawn: bool,
    pub exclusion_volume_cull_enable: bool,
    pub emittable_type: EmittableType,
    pub emittable_alignment: EmittableAlignment,
    pub world_alignment_direction: super::core::Vec3,
    pub motion_stretch_multiplier: f32,
    pub motion_stretch_view_multiplier: f32,
    pub motion_stretch_length_clamp: f32,
    pub motion_stretch_relative_length_clamp: f32,
    pub orient_to_position: super::core::Vec3,
    pub mesh: super::render::MeshAsset,
    pub object_variation_name_hash: u32,
    pub emissive: bool,
    pub emissive_exposure_factor: f32,
    pub opaque: bool,
    pub mesh_particles_motion_blur: bool,
    pub vertex_pixel_lighting_blend_factor: f32,
    pub global_local_normal_blend_factor: f32,
    pub soft_particles_fade_distance_multiplier: f32,
    pub light_wrap_around_factor: f32,
    pub bent_normal_factor: f32,
    pub light_multiplier: f32,
    pub light_multiplier_dynamic: f32,
    pub receive_sun_shadow: bool,
    pub bending_factor: f32,
    pub micro_variation_smoothing_factor: f32,
    pub force_nice_sorting: bool,
    pub local_space: bool,
    pub allow_scale: bool,
    pub camera_space: bool,
    pub transparency_sun_shadow_enable: bool,
    pub sun_volumetric_shadow_enable: bool,
    pub sun_volumetric_shadow_absorption_scale: f32,
    pub sun_volumetric_shadow_absorption_r: f32,
    pub sun_volumetric_shadow_absorption_g: f32,
    pub sun_volumetric_shadow_absorption_b: f32,
    pub sun_volumetric_shadow_phase_g0: f32,
    pub sun_volumetric_shadow_phase_g1: f32,
    pub sun_volumetric_shadow_offset: f32,
    pub enable_pyro_shader: bool,
    pub gnomon_light_rig_index: i32,
    pub use_right_texture_tile: bool,
    pub cast_planar_reflection_enable: bool,
    pub force_full_res: bool,
    pub fog_fade: bool,
    pub camera_bias: f32,
    pub emitter_draw_order: EmitterDrawOrder,
    pub flip_u_probability: f32,
    pub flip_v_probability: f32,
    pub lock_ribbon_direction: bool,
    pub particle_culling_factor: f32,
    pub instanced: bool,
    pub alpha_cull_threshold: f32,
    pub min_spawn_distance: f32,
    pub max_spawn_distance: f32,
    pub min_screen_area: f32,
    pub mesh_culling_distance: f32,
    pub pause_simulation_when_culled: bool,
    pub skip_update_max_count: i32,
    pub skip_simulation_distance: f32,
    pub precise_wind_and_force_max_distance: f32,
    pub turbulence_max_distance: f32,
    pub distance_scale_length: f32,
    pub distance_scale_near_value: f32,
    pub distance_scale_far_value: f32,
    pub speed_normalization_value: f32,
    pub wind_speed_normalization_value: f32,
    pub travelled_distance_normalization_value: f32,
    pub accept_global_parameter1: bool,
    pub accept_global_parameter2: bool,
    pub accept_global_parameter3: bool,
    pub per_particle_effect_parameters: Vec<super::effect_base::EffectParameter>,
    pub min_distance_travelled_before_spawn: f32,
    pub cull_fade_near_distance: f32,
    pub cull_fade_near_range: f32,
    pub cull_fade_far_distance: f32,
    pub cull_fade_far_range: f32,
    pub skip_near_camera_fade: bool,
    pub emitter_wind_evaluation_enable: bool,
    pub emittable_wind_evaluation_enable: bool,
    pub debug_name: String,
    pub tweak_inherited_emitter: EmitterDocument,
}

pub const EMITTERTEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTemplateData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RootProcessor",
                flags: MemberInfoFlags::new(0),
                field_type: PROCESSORDATA_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, root_processor),
            },
            FieldInfoData {
                name: "MaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, max_count),
            },
            FieldInfoData {
                name: "Lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, lifetime),
            },
            FieldInfoData {
                name: "TimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, time_scale),
            },
            FieldInfoData {
                name: "RepeatParticleSpawning",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, repeat_particle_spawning),
            },
            FieldInfoData {
                name: "LifetimeFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, lifetime_frame_count),
            },
            FieldInfoData {
                name: "FollowSpawnSource",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, follow_spawn_source),
            },
            FieldInfoData {
                name: "FollowSpawnSourceVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, follow_spawn_source_velocity),
            },
            FieldInfoData {
                name: "KillParticlesWithEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, kill_particles_with_emitter),
            },
            FieldInfoData {
                name: "KillRibbonTailDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, kill_ribbon_tail_distance),
            },
            FieldInfoData {
                name: "SmoothRibbonSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, smooth_ribbon_spawn),
            },
            FieldInfoData {
                name: "ExclusionVolumeCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, exclusion_volume_cull_enable),
            },
            FieldInfoData {
                name: "EmittableType",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTABLETYPE_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, emittable_type),
            },
            FieldInfoData {
                name: "EmittableAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTABLEALIGNMENT_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, emittable_alignment),
            },
            FieldInfoData {
                name: "WorldAlignmentDirection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, world_alignment_direction),
            },
            FieldInfoData {
                name: "MotionStretchMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, motion_stretch_multiplier),
            },
            FieldInfoData {
                name: "MotionStretchViewMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, motion_stretch_view_multiplier),
            },
            FieldInfoData {
                name: "MotionStretchLengthClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, motion_stretch_length_clamp),
            },
            FieldInfoData {
                name: "MotionStretchRelativeLengthClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, motion_stretch_relative_length_clamp),
            },
            FieldInfoData {
                name: "OrientToPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, orient_to_position),
            },
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, mesh),
            },
            FieldInfoData {
                name: "ObjectVariationNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, object_variation_name_hash),
            },
            FieldInfoData {
                name: "Emissive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, emissive),
            },
            FieldInfoData {
                name: "EmissiveExposureFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, emissive_exposure_factor),
            },
            FieldInfoData {
                name: "Opaque",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, opaque),
            },
            FieldInfoData {
                name: "MeshParticlesMotionBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, mesh_particles_motion_blur),
            },
            FieldInfoData {
                name: "VertexPixelLightingBlendFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, vertex_pixel_lighting_blend_factor),
            },
            FieldInfoData {
                name: "GlobalLocalNormalBlendFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, global_local_normal_blend_factor),
            },
            FieldInfoData {
                name: "SoftParticlesFadeDistanceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, soft_particles_fade_distance_multiplier),
            },
            FieldInfoData {
                name: "LightWrapAroundFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, light_wrap_around_factor),
            },
            FieldInfoData {
                name: "BentNormalFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, bent_normal_factor),
            },
            FieldInfoData {
                name: "LightMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, light_multiplier),
            },
            FieldInfoData {
                name: "LightMultiplierDynamic",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, light_multiplier_dynamic),
            },
            FieldInfoData {
                name: "ReceiveSunShadow",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, receive_sun_shadow),
            },
            FieldInfoData {
                name: "BendingFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, bending_factor),
            },
            FieldInfoData {
                name: "MicroVariationSmoothingFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, micro_variation_smoothing_factor),
            },
            FieldInfoData {
                name: "ForceNiceSorting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, force_nice_sorting),
            },
            FieldInfoData {
                name: "LocalSpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, local_space),
            },
            FieldInfoData {
                name: "AllowScale",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, allow_scale),
            },
            FieldInfoData {
                name: "CameraSpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, camera_space),
            },
            FieldInfoData {
                name: "TransparencySunShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, transparency_sun_shadow_enable),
            },
            FieldInfoData {
                name: "SunVolumetricShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_enable),
            },
            FieldInfoData {
                name: "SunVolumetricShadowAbsorptionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_absorption_scale),
            },
            FieldInfoData {
                name: "SunVolumetricShadowAbsorptionR",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_absorption_r),
            },
            FieldInfoData {
                name: "SunVolumetricShadowAbsorptionG",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_absorption_g),
            },
            FieldInfoData {
                name: "SunVolumetricShadowAbsorptionB",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_absorption_b),
            },
            FieldInfoData {
                name: "SunVolumetricShadowPhaseG0",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_phase_g0),
            },
            FieldInfoData {
                name: "SunVolumetricShadowPhaseG1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_phase_g1),
            },
            FieldInfoData {
                name: "SunVolumetricShadowOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_offset),
            },
            FieldInfoData {
                name: "EnablePyroShader",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, enable_pyro_shader),
            },
            FieldInfoData {
                name: "GnomonLightRigIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, gnomon_light_rig_index),
            },
            FieldInfoData {
                name: "UseRightTextureTile",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, use_right_texture_tile),
            },
            FieldInfoData {
                name: "CastPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, cast_planar_reflection_enable),
            },
            FieldInfoData {
                name: "ForceFullRes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, force_full_res),
            },
            FieldInfoData {
                name: "FogFade",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, fog_fade),
            },
            FieldInfoData {
                name: "CameraBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, camera_bias),
            },
            FieldInfoData {
                name: "EmitterDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERDRAWORDER_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, emitter_draw_order),
            },
            FieldInfoData {
                name: "FlipUProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, flip_u_probability),
            },
            FieldInfoData {
                name: "FlipVProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, flip_v_probability),
            },
            FieldInfoData {
                name: "LockRibbonDirection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, lock_ribbon_direction),
            },
            FieldInfoData {
                name: "ParticleCullingFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, particle_culling_factor),
            },
            FieldInfoData {
                name: "Instanced",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, instanced),
            },
            FieldInfoData {
                name: "AlphaCullThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, alpha_cull_threshold),
            },
            FieldInfoData {
                name: "MinSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, min_spawn_distance),
            },
            FieldInfoData {
                name: "MaxSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, max_spawn_distance),
            },
            FieldInfoData {
                name: "MinScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, min_screen_area),
            },
            FieldInfoData {
                name: "MeshCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, mesh_culling_distance),
            },
            FieldInfoData {
                name: "PauseSimulationWhenCulled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, pause_simulation_when_culled),
            },
            FieldInfoData {
                name: "SkipUpdateMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, skip_update_max_count),
            },
            FieldInfoData {
                name: "SkipSimulationDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, skip_simulation_distance),
            },
            FieldInfoData {
                name: "PreciseWindAndForceMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, precise_wind_and_force_max_distance),
            },
            FieldInfoData {
                name: "TurbulenceMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, turbulence_max_distance),
            },
            FieldInfoData {
                name: "DistanceScaleLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, distance_scale_length),
            },
            FieldInfoData {
                name: "DistanceScaleNearValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, distance_scale_near_value),
            },
            FieldInfoData {
                name: "DistanceScaleFarValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, distance_scale_far_value),
            },
            FieldInfoData {
                name: "SpeedNormalizationValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, speed_normalization_value),
            },
            FieldInfoData {
                name: "WindSpeedNormalizationValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, wind_speed_normalization_value),
            },
            FieldInfoData {
                name: "TravelledDistanceNormalizationValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, travelled_distance_normalization_value),
            },
            FieldInfoData {
                name: "AcceptGlobalParameter1",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, accept_global_parameter1),
            },
            FieldInfoData {
                name: "AcceptGlobalParameter2",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, accept_global_parameter2),
            },
            FieldInfoData {
                name: "AcceptGlobalParameter3",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, accept_global_parameter3),
            },
            FieldInfoData {
                name: "PerParticleEffectParameters",
                flags: MemberInfoFlags::new(144),
                field_type: EFFECTPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, per_particle_effect_parameters),
            },
            FieldInfoData {
                name: "MinDistanceTravelledBeforeSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, min_distance_travelled_before_spawn),
            },
            FieldInfoData {
                name: "CullFadeNearDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, cull_fade_near_distance),
            },
            FieldInfoData {
                name: "CullFadeNearRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, cull_fade_near_range),
            },
            FieldInfoData {
                name: "CullFadeFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, cull_fade_far_distance),
            },
            FieldInfoData {
                name: "CullFadeFarRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, cull_fade_far_range),
            },
            FieldInfoData {
                name: "SkipNearCameraFade",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, skip_near_camera_fade),
            },
            FieldInfoData {
                name: "EmitterWindEvaluationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, emitter_wind_evaluation_enable),
            },
            FieldInfoData {
                name: "EmittableWindEvaluationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, emittable_wind_evaluation_enable),
            },
            FieldInfoData {
                name: "DebugName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, debug_name),
            },
            FieldInfoData {
                name: "TweakInheritedEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERDOCUMENT_TYPE_INFO,
                rust_offset: offset_of!(EmitterTemplateData, tweak_inherited_emitter),
            },
        ],
    }),
    array_type: Some(EMITTERTEMPLATEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterTemplateData {
    fn type_info() -> &'static TypeInfo {
        EMITTERTEMPLATEDATA_TYPE_INFO
    }
}


pub const EMITTERTEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTemplateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterTemplateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PerParticleParams {
    #[default]
    FloatCount = 4,
}

pub const PERPARTICLEPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerParticleParams",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(PERPARTICLEPARAMS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PerParticleParams {
    fn type_info() -> &'static TypeInfo {
        PERPARTICLEPARAMS_TYPE_INFO
    }
}


pub const PERPARTICLEPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerParticleParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PerParticleParams-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProcessorData {
    pub pre: EvaluatorData,
    pub next_processor: ProcessorData,
    pub evaluator_input: EmittableField,
    pub evaluator_input_param: super::effect_base::EffectParameter,
    pub schematics_enable: bool,
}

pub const PROCESSORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProcessorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Pre",
                flags: MemberInfoFlags::new(0),
                field_type: EVALUATORDATA_TYPE_INFO,
                rust_offset: offset_of!(ProcessorData, pre),
            },
            FieldInfoData {
                name: "NextProcessor",
                flags: MemberInfoFlags::new(0),
                field_type: PROCESSORDATA_TYPE_INFO,
                rust_offset: offset_of!(ProcessorData, next_processor),
            },
            FieldInfoData {
                name: "EvaluatorInput",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTABLEFIELD_TYPE_INFO,
                rust_offset: offset_of!(ProcessorData, evaluator_input),
            },
            FieldInfoData {
                name: "EvaluatorInputParam",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(ProcessorData, evaluator_input_param),
            },
            FieldInfoData {
                name: "SchematicsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProcessorData, schematics_enable),
            },
        ],
    }),
    array_type: Some(PROCESSORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProcessorData {
    fn type_info() -> &'static TypeInfo {
        PROCESSORDATA_TYPE_INFO
    }
}


pub const PROCESSORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProcessorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ProcessorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EvaluatorData {
    pub parameter: super::effect_base::EffectParameter,
    pub schematics_enable: bool,
}

pub const EVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Parameter",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(EvaluatorData, parameter),
            },
            FieldInfoData {
                name: "SchematicsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EvaluatorData, schematics_enable),
            },
        ],
    }),
    array_type: Some(EVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EvaluatorData {
    fn type_info() -> &'static TypeInfo {
        EVALUATORDATA_TYPE_INFO
    }
}


pub const EVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ParticleSorting {
    #[default]
    ParticleSorting_CameraDistance = 0,
    ParticleSorting_NewToOld = 1,
    ParticleSorting_OldToNew = 2,
}

pub const PARTICLESORTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticleSorting",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(PARTICLESORTING_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ParticleSorting {
    fn type_info() -> &'static TypeInfo {
        PARTICLESORTING_TYPE_INFO
    }
}


pub const PARTICLESORTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticleSorting-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ParticleSorting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterDrawOrder {
    #[default]
    EmitterDrawOrder_Default = 0,
    EmitterDrawOrder_Foreground = 1,
    EmitterDrawOrder_Background = 2,
}

pub const EMITTERDRAWORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDrawOrder",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERDRAWORDER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterDrawOrder {
    fn type_info() -> &'static TypeInfo {
        EMITTERDRAWORDER_TYPE_INFO
    }
}


pub const EMITTERDRAWORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDrawOrder-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterDrawOrder-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterSchematicInputParameter {
    #[default]
    EsiColor = 0,
    EsiSpawnRate = 1,
    EsiDefaultValues = 2,
    EsiDimentions = 3,
    EsiPivot = 4,
    EsiPolynomialScaleValue = 5,
    EsiSuperSphereDistributionScale = 6,
    EmitterSchematicInputCount = 7,
}

pub const EMITTERSCHEMATICINPUTPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSchematicInputParameter",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERSCHEMATICINPUTPARAMETER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterSchematicInputParameter {
    fn type_info() -> &'static TypeInfo {
        EMITTERSCHEMATICINPUTPARAMETER_TYPE_INFO
    }
}


pub const EMITTERSCHEMATICINPUTPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSchematicInputParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterSchematicInputParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ProcessorType {
    #[default]
    PtBaseEmitter = 0,
    PtCustomShader = 1,
    PtPreRoll = 2,
    PtSpawnRate = 3,
    PtSpawnRibbonRate = 4,
    PtSpawnPointCloud = 5,
    PtSpawnSpeed = 6,
    PtSpawnPosition = 7,
    PtSpawnDirection = 8,
    PtSpawnSize = 9,
    PtSpawnAnimation = 10,
    PtSpawnAnimationFrame = 11,
    PtSpawnRotation = 12,
    PtSpawnOrientation = 13,
    PtSpawnRotationSpeed = 14,
    PtSpawnColorRandom = 15,
    PtUpdatePosition = 16,
    PtUpdateAge = 17,
    PtTurbulance = 18,
    PtGravity = 19,
    PtLocalForce = 20,
    PtAirResistance = 21,
    PtEmitter = 22,
    PtMimicEmitter = 23,
    PtUpdateMimicOverrides = 24,
    PtUpdateColor = 25,
    PtUpdateColorSecondary = 26,
    PtUpdateColorLeaf = 27,
    PtUpdateTransparency = 28,
    PtUpdateTransparencySecondary = 29,
    PtUpdateTextureCoords = 30,
    PtUpdateRotation = 31,
    PtUpdateSizeX = 32,
    PtUpdateSizeY = 33,
    PtUpdateSizeZ = 34,
    PtUpdateSize = 35,
    PtUpdateAlphaLevelMin = 36,
    PtUpdateAlphaLevelMax = 37,
    PtUpdateAlphaLevelScale = 38,
    PtUpdateClipScale = 39,
    PtUpdateCameraProximity = 40,
    PtUpdateRibbonFade = 41,
    PtUpdateRibbonTexture = 42,
    PtWorldWind = 43,
    PtWorldForces = 44,
    PtUpdateCollision = 45,
    PtSnapToWater = 46,
    PtUpdateQuadBendingAngle = 47,
    PtUpdateBeamSource = 48,
    PtUpdateBeamTarget = 49,
    PtUpdateBeamPoint = 50,
    PtUpdateCustomParams = 51,
    PtUpdateVertexAnim = 52,
    PtUpdateMeshEmitterSource = 53,
    PtUpdateMeshEmitterMask = 54,
    PtUpdateVolumetric = 55,
    PtUpdateVolumeMask = 56,
    PtUpdateStencilMask = 57,
    PtUpdateTextureColorLerp = 58,
    PtUpdateLightWrapAround = 59,
    PtUpdateTrapezoidShape = 60,
    PtUpdateBackLight = 61,
    ProcessorTypeCount = 62,
}

pub const PROCESSORTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProcessorType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(PROCESSORTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProcessorType {
    fn type_info() -> &'static TypeInfo {
        PROCESSORTYPE_TYPE_INFO
    }
}


pub const PROCESSORTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProcessorType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ProcessorType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EvaluatorType {
    #[default]
    EtNone = 0,
    EtDefault = 1,
    EtBox = 2,
    EtPolynomial = 3,
    EtPolynomialOperator = 4,
    EtSpline = 5,
    EtRandom = 6,
    EtRandomXYZ = 7,
    EtRotateVector = 8,
    EtSampleTexture = 9,
    EtSphere = 10,
    EtSuperSphere = 11,
    EtConstant = 12,
    EtPolynomialColorInterp = 13,
    EtMultiColorInterp = 14,
    EtCamProx = 15,
    EtRandomXYZW = 16,
    EtPolynomialXYZW = 17,
    EvaluatorTypeCount = 18,
}

pub const EVALUATORTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluatorType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EVALUATORTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EvaluatorType {
    fn type_info() -> &'static TypeInfo {
        EVALUATORTYPE_TYPE_INFO
    }
}


pub const EVALUATORTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluatorType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EvaluatorType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RandomFrequency {
    #[default]
    RandomPerFrame = 0,
    RandomPerEmittable = 1,
    RandomPerInstance = 2,
}

pub const RANDOMFREQUENCY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomFrequency",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(RANDOMFREQUENCY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RandomFrequency {
    fn type_info() -> &'static TypeInfo {
        RANDOMFREQUENCY_TYPE_INFO
    }
}


pub const RANDOMFREQUENCY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomFrequency-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RandomFrequency-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmittableAlignment {
    #[default]
    EmittableAlignment_Screen = 0,
    EmittableAlignment_Emitter = 1,
    EmittableAlignment_Emittable = 2,
    EmittableAlignment_Direction = 3,
    EmittableAlignment_WorldFixedRotation = 4,
    EmittableAlignment_World = 5,
    EmittableAlignment_WorldPerpendicular = 6,
    EmittableAlignment_ScreenMotionStretch = 7,
    EmittableAlignment_DirectionMotionStretch = 8,
    EmittableAlignment_MotionStretchScreen = 9,
    EmittableAlignment_OrientationToPosition = 10,
}

pub const EMITTABLEALIGNMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableAlignment",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTABLEALIGNMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmittableAlignment {
    fn type_info() -> &'static TypeInfo {
        EMITTABLEALIGNMENT_TYPE_INFO
    }
}


pub const EMITTABLEALIGNMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableAlignment-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmittableAlignment-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmittableType {
    #[default]
    EmittableType_Point = 0,
    EmittableType_Quad = 100,
    EmittableType_Mesh = 200,
    EmittableType_Ribbon = 300,
    EmittableType_Beam = 400,
    EmittableType_Decal = 800,
    Point = 801,
    Quad = 802,
    ScreenAlignedQuad = 803,
    DirectionAlignedQuad = 804,
    WorldAlignedQuad = 805,
    ParticleMesh = 806,
    Ribbon = 807,
    Trail = 808,
}

pub const EMITTABLETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTABLETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmittableType {
    fn type_info() -> &'static TypeInfo {
        EMITTABLETYPE_TYPE_INFO
    }
}


pub const EMITTABLETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmittableType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmittableField {
    #[default]
    EfZero = 0,
    EfOne = 1,
    EfNormTime = 2,
    EfNormSpeed = 3,
    EfNormWindSpeed = 4,
    EfNormMicroVariation = 5,
    EfCameraFacing = 6,
    EfEmitterNormTime = 7,
    EfEmitterNormWindSpeed = 8,
    EfEmitterNormMicroVariation = 9,
    EfEmitterNormTravelledDistance = 10,
    EfMeshPosition = 11,
    EfMeshUV = 12,
    EfMeshNormal = 13,
    EfNone = 14,
    EfCount = 15,
    EfSpawnAnimationSpeed = 16,
    EfSpawnAnimationFrameIndex = 17,
    EfVelocity = 18,
    EfParameters = 19,
    EfRotation = 20,
    EfSpeed = 21,
    EfUserDefined = 22,
    EfConstantFloat = 23,
    EfConstantVec = 24,
}

pub const EMITTABLEFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableField",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTABLEFIELD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmittableField {
    fn type_info() -> &'static TypeInfo {
        EMITTABLEFIELD_TYPE_INFO
    }
}


pub const EMITTABLEFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableField-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmittableField-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterExclusionVolumeBoundingSphereSoA {
    pub pos_x: super::core::Vec4,
    pub pos_y: super::core::Vec4,
    pub pos_z: super::core::Vec4,
    pub radius_sqr: super::core::Vec4,
}

pub const EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeBoundingSphereSoA",
    flags: MemberInfoFlags::new(36937),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PosX",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumeBoundingSphereSoA, pos_x),
            },
            FieldInfoData {
                name: "PosY",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumeBoundingSphereSoA, pos_y),
            },
            FieldInfoData {
                name: "PosZ",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumeBoundingSphereSoA, pos_z),
            },
            FieldInfoData {
                name: "RadiusSqr",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumeBoundingSphereSoA, radius_sqr),
            },
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterExclusionVolumeBoundingSphereSoA {
    fn type_info() -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_TYPE_INFO
    }
}


pub const EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeBoundingSphereSoA-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterExclusionVolumeBoundingSphereSoA-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterExclusionVolume {
    pub left: super::core::Vec4,
    pub up: super::core::Vec4,
    pub forward: super::core::Vec4,
    pub half_extents: super::core::Vec4,
    pub id: u32,
}

pub const EMITTEREXCLUSIONVOLUME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolume",
    flags: MemberInfoFlags::new(36937),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Left",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolume, left),
            },
            FieldInfoData {
                name: "Up",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolume, up),
            },
            FieldInfoData {
                name: "Forward",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolume, forward),
            },
            FieldInfoData {
                name: "HalfExtents",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolume, half_extents),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolume, id),
            },
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUME_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterExclusionVolume {
    fn type_info() -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUME_TYPE_INFO
    }
}


pub const EMITTEREXCLUSIONVOLUME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolume-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterExclusionVolume-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterGraph {
    pub spawn_mode2: SpawnModeInfo,
    pub use_node_graph: bool,
    pub graph_data: super::expression::ExpressionNodeGraphData,
    pub spawn_mode: EmitterGraphSpawnMode,
    pub spawn_rate: super::core::QualityScalableFloat,
    pub particle_max_count: super::core::QualityScalableInt,
    pub particle_life_span: super::core::QualityScalableFloat,
    pub planar_reflections_enabled: bool,
    pub normalize_mesh_start_ids: bool,
    pub meshes: Vec<EmitterGraphMesh>,
    pub object_variation_name_hash: u32,
    pub requires_per_root_view_duplication: bool,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub draw_pass: EmitterGraphDrawPass,
    pub draw_layer: EmitterGraphDrawLayer,
    pub sort_mode: EmitterGraphSortMode,
    pub user_buffers: Vec<EmitterGraphUserBuffer>,
    pub spawn_shader_override: super::core::FileRef,
    pub simulate_shader_override: super::core::FileRef,
    pub texture0: super::render_base::TextureBaseAsset,
    pub texture1: super::render_base::TextureBaseAsset,
    pub z_buffer_enable: bool,
    pub emitter_life_span: super::core::QualityScalableFloat,
    pub kill_on_stop: bool,
    pub emitter_min_spawn_distance: super::core::QualityScalableFloat,
    pub emitter_max_spawn_distance: super::core::QualityScalableFloat,
    pub spawn_outside_view_radius: f32,
    pub bounding_box_min: super::core::Vec3,
    pub bounding_box_max: super::core::Vec3,
    pub culled_behavior: EmitterGraphCulledBehavior,
    pub skip_update_max_count: i32,
    pub emitter_mesh_culling_distance: f32,
    pub min_screen_area: f32,
    pub gpu_particle_culling_enable: bool,
    pub gpu_particle_culling_radius: f32,
    pub gpu_particle_culling_distance: super::core::QualityScalableFloat,
    pub mesh_vertex_shader_fragment_code_file: super::core::FileRef,
    pub effect_params: Vec<super::effect_base::EffectParameter>,
    pub emitter_graph_params: Vec<super::effect_base::EmitterExposedInput>,
    pub is_using_opaque_lit_root_node: bool,
    pub is_using_lit_root_node: bool,
    pub is_using_gpu_lighting: bool,
    pub compiled_spawn_graph_compute_shader: super::core::ResourceRef,
    pub compiled_simulate_graph_compute_shader: super::core::ResourceRef,
    pub vertex_shader_fragment_asset_name: String,
    pub mesh_vertex_shader_fragment_asset_name: String,
    pub particle_data_byte_stride: u32,
    pub particle_data_buffer_layout_hash: u32,
    pub simulate_runtime_textures: Vec<RuntimeTexture>,
    pub simulate_runtime_samplers: Vec<RuntimeSampler>,
    pub spawn_runtime_textures: Vec<RuntimeTexture>,
    pub spawn_runtime_samplers: Vec<RuntimeSampler>,
    pub vertex_shader_runtime_textures: Vec<RuntimeTexture>,
    pub runtime_particle_data_buffers: Vec<RuntimeParticleDataBuffer>,
}

pub const EMITTERGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraph",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERGRAPHBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SpawnMode2",
                flags: MemberInfoFlags::new(0),
                field_type: SPAWNMODEINFO_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, spawn_mode2),
            },
            FieldInfoData {
                name: "UseNodeGraph",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, use_node_graph),
            },
            FieldInfoData {
                name: "GraphData",
                flags: MemberInfoFlags::new(0),
                field_type: EXPRESSIONNODEGRAPHDATA_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, graph_data),
            },
            FieldInfoData {
                name: "SpawnMode",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHSPAWNMODE_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, spawn_mode),
            },
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, spawn_rate),
            },
            FieldInfoData {
                name: "ParticleMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, particle_max_count),
            },
            FieldInfoData {
                name: "ParticleLifeSpan",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, particle_life_span),
            },
            FieldInfoData {
                name: "PlanarReflectionsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, planar_reflections_enabled),
            },
            FieldInfoData {
                name: "NormalizeMeshStartIds",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, normalize_mesh_start_ids),
            },
            FieldInfoData {
                name: "Meshes",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTERGRAPHMESH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, meshes),
            },
            FieldInfoData {
                name: "ObjectVariationNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, object_variation_name_hash),
            },
            FieldInfoData {
                name: "RequiresPerRootViewDuplication",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, requires_per_root_view_duplication),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, shader),
            },
            FieldInfoData {
                name: "DrawPass",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHDRAWPASS_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, draw_pass),
            },
            FieldInfoData {
                name: "DrawLayer",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHDRAWLAYER_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, draw_layer),
            },
            FieldInfoData {
                name: "SortMode",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHSORTMODE_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, sort_mode),
            },
            FieldInfoData {
                name: "UserBuffers",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTERGRAPHUSERBUFFER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, user_buffers),
            },
            FieldInfoData {
                name: "SpawnShaderOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FILEREF_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, spawn_shader_override),
            },
            FieldInfoData {
                name: "SimulateShaderOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FILEREF_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, simulate_shader_override),
            },
            FieldInfoData {
                name: "Texture0",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, texture0),
            },
            FieldInfoData {
                name: "Texture1",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, texture1),
            },
            FieldInfoData {
                name: "ZBufferEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, z_buffer_enable),
            },
            FieldInfoData {
                name: "EmitterLifeSpan",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, emitter_life_span),
            },
            FieldInfoData {
                name: "KillOnStop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, kill_on_stop),
            },
            FieldInfoData {
                name: "EmitterMinSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, emitter_min_spawn_distance),
            },
            FieldInfoData {
                name: "EmitterMaxSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, emitter_max_spawn_distance),
            },
            FieldInfoData {
                name: "SpawnOutsideViewRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, spawn_outside_view_radius),
            },
            FieldInfoData {
                name: "BoundingBoxMin",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, bounding_box_min),
            },
            FieldInfoData {
                name: "BoundingBoxMax",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, bounding_box_max),
            },
            FieldInfoData {
                name: "CulledBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHCULLEDBEHAVIOR_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, culled_behavior),
            },
            FieldInfoData {
                name: "SkipUpdateMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, skip_update_max_count),
            },
            FieldInfoData {
                name: "EmitterMeshCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, emitter_mesh_culling_distance),
            },
            FieldInfoData {
                name: "MinScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, min_screen_area),
            },
            FieldInfoData {
                name: "GpuParticleCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, gpu_particle_culling_enable),
            },
            FieldInfoData {
                name: "GpuParticleCullingRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, gpu_particle_culling_radius),
            },
            FieldInfoData {
                name: "GpuParticleCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, gpu_particle_culling_distance),
            },
            FieldInfoData {
                name: "MeshVertexShaderFragmentCodeFile",
                flags: MemberInfoFlags::new(0),
                field_type: FILEREF_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, mesh_vertex_shader_fragment_code_file),
            },
            FieldInfoData {
                name: "EffectParams",
                flags: MemberInfoFlags::new(144),
                field_type: EFFECTPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, effect_params),
            },
            FieldInfoData {
                name: "EmitterGraphParams",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTEREXPOSEDINPUT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, emitter_graph_params),
            },
            FieldInfoData {
                name: "IsUsingOpaqueLitRootNode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, is_using_opaque_lit_root_node),
            },
            FieldInfoData {
                name: "IsUsingLitRootNode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, is_using_lit_root_node),
            },
            FieldInfoData {
                name: "IsUsingGpuLighting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, is_using_gpu_lighting),
            },
            FieldInfoData {
                name: "CompiledSpawnGraphComputeShader",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, compiled_spawn_graph_compute_shader),
            },
            FieldInfoData {
                name: "CompiledSimulateGraphComputeShader",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, compiled_simulate_graph_compute_shader),
            },
            FieldInfoData {
                name: "VertexShaderFragmentAssetName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, vertex_shader_fragment_asset_name),
            },
            FieldInfoData {
                name: "MeshVertexShaderFragmentAssetName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, mesh_vertex_shader_fragment_asset_name),
            },
            FieldInfoData {
                name: "ParticleDataByteStride",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, particle_data_byte_stride),
            },
            FieldInfoData {
                name: "ParticleDataBufferLayoutHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, particle_data_buffer_layout_hash),
            },
            FieldInfoData {
                name: "SimulateRuntimeTextures",
                flags: MemberInfoFlags::new(144),
                field_type: RUNTIMETEXTURE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, simulate_runtime_textures),
            },
            FieldInfoData {
                name: "SimulateRuntimeSamplers",
                flags: MemberInfoFlags::new(144),
                field_type: RUNTIMESAMPLER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, simulate_runtime_samplers),
            },
            FieldInfoData {
                name: "SpawnRuntimeTextures",
                flags: MemberInfoFlags::new(144),
                field_type: RUNTIMETEXTURE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, spawn_runtime_textures),
            },
            FieldInfoData {
                name: "SpawnRuntimeSamplers",
                flags: MemberInfoFlags::new(144),
                field_type: RUNTIMESAMPLER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, spawn_runtime_samplers),
            },
            FieldInfoData {
                name: "VertexShaderRuntimeTextures",
                flags: MemberInfoFlags::new(144),
                field_type: RUNTIMETEXTURE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, vertex_shader_runtime_textures),
            },
            FieldInfoData {
                name: "RuntimeParticleDataBuffers",
                flags: MemberInfoFlags::new(144),
                field_type: RUNTIMEPARTICLEDATABUFFER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraph, runtime_particle_data_buffers),
            },
        ],
    }),
    array_type: Some(EMITTERGRAPH_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterGraph {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPH_TYPE_INFO
    }
}


pub const EMITTERGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraph-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraph-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RuntimeParticleDataBuffer {
    pub shader_parameter_handle: u32,
    pub bind_point: u8,
    pub buffer_type: EmitterGraphParticleDataType,
}

pub const RUNTIMEPARTICLEDATABUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeParticleDataBuffer",
    flags: MemberInfoFlags::new(32841),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ShaderParameterHandle",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RuntimeParticleDataBuffer, shader_parameter_handle),
            },
            FieldInfoData {
                name: "BindPoint",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RuntimeParticleDataBuffer, bind_point),
            },
            FieldInfoData {
                name: "BufferType",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHPARTICLEDATATYPE_TYPE_INFO,
                rust_offset: offset_of!(RuntimeParticleDataBuffer, buffer_type),
            },
        ],
    }),
    array_type: Some(RUNTIMEPARTICLEDATABUFFER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RuntimeParticleDataBuffer {
    fn type_info() -> &'static TypeInfo {
        RUNTIMEPARTICLEDATABUFFER_TYPE_INFO
    }
}


pub const RUNTIMEPARTICLEDATABUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeParticleDataBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RuntimeParticleDataBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RuntimeSampler {
    pub bind_point: u8,
    pub filter: super::render::TextureFilter,
    pub address: super::render_base::TextureAddress,
}

pub const RUNTIMESAMPLER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeSampler",
    flags: MemberInfoFlags::new(36937),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BindPoint",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RuntimeSampler, bind_point),
            },
            FieldInfoData {
                name: "Filter",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREFILTER_TYPE_INFO,
                rust_offset: offset_of!(RuntimeSampler, filter),
            },
            FieldInfoData {
                name: "Address",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(RuntimeSampler, address),
            },
        ],
    }),
    array_type: Some(RUNTIMESAMPLER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RuntimeSampler {
    fn type_info() -> &'static TypeInfo {
        RUNTIMESAMPLER_TYPE_INFO
    }
}


pub const RUNTIMESAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeSampler-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RuntimeSampler-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RuntimeTexture {
    pub bind_point: u8,
    pub shader_parameter_handle: u32,
    pub texture_type: super::render::TextureType,
    pub texture: super::render_base::TextureBaseAsset,
}

pub const RUNTIMETEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeTexture",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BindPoint",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RuntimeTexture, bind_point),
            },
            FieldInfoData {
                name: "ShaderParameterHandle",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RuntimeTexture, shader_parameter_handle),
            },
            FieldInfoData {
                name: "TextureType",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURETYPE_TYPE_INFO,
                rust_offset: offset_of!(RuntimeTexture, texture_type),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(RuntimeTexture, texture),
            },
        ],
    }),
    array_type: Some(RUNTIMETEXTURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RuntimeTexture {
    fn type_info() -> &'static TypeInfo {
        RUNTIMETEXTURE_TYPE_INFO
    }
}


pub const RUNTIMETEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RuntimeTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterGraphMesh {
    pub mesh: super::render::MeshAsset,
    pub object_variation: super::core::Asset,
    pub object_variation_name_hash: u32,
    pub start_id: u32,
}

pub const EMITTERGRAPHMESH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphMesh",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphMesh, mesh),
            },
            FieldInfoData {
                name: "ObjectVariation",
                flags: MemberInfoFlags::new(0),
                field_type: ASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphMesh, object_variation),
            },
            FieldInfoData {
                name: "ObjectVariationNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphMesh, object_variation_name_hash),
            },
            FieldInfoData {
                name: "StartId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphMesh, start_id),
            },
        ],
    }),
    array_type: Some(EMITTERGRAPHMESH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterGraphMesh {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHMESH_TYPE_INFO
    }
}


pub const EMITTERGRAPHMESH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphMesh-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphMesh-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterGraphUserBuffer {
    pub bind_point: u8,
    pub name: String,
}

pub const EMITTERGRAPHUSERBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphUserBuffer",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BindPoint",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphUserBuffer, bind_point),
            },
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphUserBuffer, name),
            },
        ],
    }),
    array_type: Some(EMITTERGRAPHUSERBUFFER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterGraphUserBuffer {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHUSERBUFFER_TYPE_INFO
    }
}


pub const EMITTERGRAPHUSERBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphUserBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphUserBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterGraphSpawnMode {
    #[default]
    EmitterGraphSpawnMode_Continuous = 0,
    EmitterGraphSpawnMode_SingleBurst = 1,
}

pub const EMITTERGRAPHSPAWNMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSpawnMode",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHSPAWNMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphSpawnMode {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHSPAWNMODE_TYPE_INFO
    }
}


pub const EMITTERGRAPHSPAWNMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSpawnMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphSpawnMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterGraphSortMode {
    #[default]
    EmitterGraphSortMode_Default = 0,
    EmitterGraphSortMode_Disable = 1,
    EmitterGraphSortMode_BackToFront = 2,
    EmitterGraphSortMode_Lifetime = 3,
    EmitterGraphSortMode_LifetimeInverse = 4,
}

pub const EMITTERGRAPHSORTMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSortMode",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHSORTMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphSortMode {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHSORTMODE_TYPE_INFO
    }
}


pub const EMITTERGRAPHSORTMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSortMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphSortMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterGraphDrawLayer {
    #[default]
    EmitterGraphDrawLayer_Background = 0,
    EmitterGraphDrawLayer_Default = 1,
    EmitterGraphDrawLayer_Foreground = 2,
}

pub const EMITTERGRAPHDRAWLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphDrawLayer",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHDRAWLAYER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphDrawLayer {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHDRAWLAYER_TYPE_INFO
    }
}


pub const EMITTERGRAPHDRAWLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphDrawLayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphDrawLayer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterGraphDrawPass {
    #[default]
    EmitterGraphDrawPass_Main = 0,
    EmitterGraphDrawPass_HalfResolution = 1,
    EmitterGraphDrawPass_Foreground = 2,
    EmitterGraphDrawPass_Hologram = 3,
}

pub const EMITTERGRAPHDRAWPASS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphDrawPass",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHDRAWPASS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphDrawPass {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHDRAWPASS_TYPE_INFO
    }
}


pub const EMITTERGRAPHDRAWPASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphDrawPass-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphDrawPass-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterGraphCulledBehavior {
    #[default]
    EmitterGraphCulledBehavior_Pause = 0,
    EmitterGraphCulledBehavior_SkipUpdates = 1,
    EmitterGraphCulledBehavior_Kill = 2,
}

pub const EMITTERGRAPHCULLEDBEHAVIOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphCulledBehavior",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHCULLEDBEHAVIOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphCulledBehavior {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHCULLEDBEHAVIOR_TYPE_INFO
    }
}


pub const EMITTERGRAPHCULLEDBEHAVIOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphCulledBehavior-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphCulledBehavior-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterGraphConfig {
    #[default]
    EmitterGraphConfig_EmitterGraphParamMaxCount = 32,
    EmitterGraphConfig_EffectParamMaxCount = 8,
    EmitterGraphConfig_ExclusionVolumesMaxCount = 1024,
    EmitterGraphConfig_MaxParticleCount = 65535,
}

pub const EMITTERGRAPHCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphConfig",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHCONFIG_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphConfig {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHCONFIG_TYPE_INFO
    }
}


pub const EMITTERGRAPHCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RandomSpawnRateModifier {
    pub frequency: EmitterGraphRandomFrequency,
    pub min: f32,
    pub max: f32,
}

pub const RANDOMSPAWNRATEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomSpawnRateModifier",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPAWNRATEMODIFIER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Frequency",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHRANDOMFREQUENCY_TYPE_INFO,
                rust_offset: offset_of!(RandomSpawnRateModifier, frequency),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomSpawnRateModifier, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomSpawnRateModifier, max),
            },
        ],
    }),
    array_type: Some(RANDOMSPAWNRATEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomSpawnRateModifier {
    fn type_info() -> &'static TypeInfo {
        RANDOMSPAWNRATEMODIFIER_TYPE_INFO
    }
}


pub const RANDOMSPAWNRATEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomSpawnRateModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RandomSpawnRateModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterGraphRandomFrequency {
    #[default]
    EmitterGraphRandomFrequency_RandomPerFrame = 0,
    EmitterGraphRandomFrequency_RandomPerEmitter = 1,
}

pub const EMITTERGRAPHRANDOMFREQUENCY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphRandomFrequency",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHRANDOMFREQUENCY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphRandomFrequency {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHRANDOMFREQUENCY_TYPE_INFO
    }
}


pub const EMITTERGRAPHRANDOMFREQUENCY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphRandomFrequency-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphRandomFrequency-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterSpawnRateModifier {
    pub emitter_property: EmitterGraphSpawnProperty,
    pub curve: super::core::FloatCurve,
}

pub const EMITTERSPAWNRATEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSpawnRateModifier",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPAWNRATEMODIFIER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EmitterProperty",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHSPAWNPROPERTY_TYPE_INFO,
                rust_offset: offset_of!(EmitterSpawnRateModifier, emitter_property),
            },
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(EmitterSpawnRateModifier, curve),
            },
        ],
    }),
    array_type: Some(EMITTERSPAWNRATEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterSpawnRateModifier {
    fn type_info() -> &'static TypeInfo {
        EMITTERSPAWNRATEMODIFIER_TYPE_INFO
    }
}


pub const EMITTERSPAWNRATEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSpawnRateModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterSpawnRateModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EffectParameterSpawnRateModifier {
    pub effect_parameter: super::effect_base::EffectParameter,
    pub curve: super::core::FloatCurve,
}

pub const EFFECTPARAMETERSPAWNRATEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterSpawnRateModifier",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPAWNRATEMODIFIER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EffectParameter",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(EffectParameterSpawnRateModifier, effect_parameter),
            },
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(EffectParameterSpawnRateModifier, curve),
            },
        ],
    }),
    array_type: Some(EFFECTPARAMETERSPAWNRATEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectParameterSpawnRateModifier {
    fn type_info() -> &'static TypeInfo {
        EFFECTPARAMETERSPAWNRATEMODIFIER_TYPE_INFO
    }
}


pub const EFFECTPARAMETERSPAWNRATEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterSpawnRateModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EffectParameterSpawnRateModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpawnRateModifier {
}

pub const SPAWNRATEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRateModifier",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SPAWNRATEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnRateModifier {
    fn type_info() -> &'static TypeInfo {
        SPAWNRATEMODIFIER_TYPE_INFO
    }
}


pub const SPAWNRATEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRateModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnRateModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterGraphSpawnProperty {
    #[default]
    EmitterGraphSpawnProperty_EmitterLifetimeNorm = 0,
    EmitterGraphSpawnProperty_EmitterAgeNorm = 1,
    EmitterGraphSpawnProperty_Speed = 2,
}

pub const EMITTERGRAPHSPAWNPROPERTY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSpawnProperty",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHSPAWNPROPERTY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphSpawnProperty {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHSPAWNPROPERTY_TYPE_INFO
    }
}


pub const EMITTERGRAPHSPAWNPROPERTY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSpawnProperty-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphSpawnProperty-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpawnModeContinuous {
    pub spawn_rate: super::core::QualityScalableFloat,
    pub min_spawn_rate: super::core::QualityScalableFloat,
    pub max_spawn_rate: super::core::QualityScalableFloat,
    pub spawn_rate_multipliers: Vec<SpawnRateModifier>,
}

pub const SPAWNMODECONTINUOUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeContinuous",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPAWNMODEINFO_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(SpawnModeContinuous, spawn_rate),
            },
            FieldInfoData {
                name: "MinSpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(SpawnModeContinuous, min_spawn_rate),
            },
            FieldInfoData {
                name: "MaxSpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(SpawnModeContinuous, max_spawn_rate),
            },
            FieldInfoData {
                name: "SpawnRateMultipliers",
                flags: MemberInfoFlags::new(144),
                field_type: SPAWNRATEMODIFIER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SpawnModeContinuous, spawn_rate_multipliers),
            },
        ],
    }),
    array_type: Some(SPAWNMODECONTINUOUS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnModeContinuous {
    fn type_info() -> &'static TypeInfo {
        SPAWNMODECONTINUOUS_TYPE_INFO
    }
}


pub const SPAWNMODECONTINUOUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeContinuous-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnModeContinuous-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpawnModeBurst {
}

pub const SPAWNMODEBURST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeBurst",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPAWNMODEINFO_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SPAWNMODEBURST_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnModeBurst {
    fn type_info() -> &'static TypeInfo {
        SPAWNMODEBURST_TYPE_INFO
    }
}


pub const SPAWNMODEBURST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeBurst-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnModeBurst-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpawnModeInfo {
    pub particle_max_count: super::core::QualityScalableInt,
    pub spawn_mode_enum: EmitterGraphSpawnMode,
}

pub const SPAWNMODEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeInfo",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ParticleMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(SpawnModeInfo, particle_max_count),
            },
            FieldInfoData {
                name: "SpawnModeEnum",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHSPAWNMODE_TYPE_INFO,
                rust_offset: offset_of!(SpawnModeInfo, spawn_mode_enum),
            },
        ],
    }),
    array_type: Some(SPAWNMODEINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SpawnModeInfo {
    fn type_info() -> &'static TypeInfo {
        SPAWNMODEINFO_TYPE_INFO
    }
}


pub const SPAWNMODEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnModeInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VertexShaderParam {
    pub param_name: String,
    pub value: super::core::Vec4,
    pub param_type: super::effect_base::EmitterGraphParamType,
    pub value_type: super::emitter_base::EmitterExposableType,
}

pub const VERTEXSHADERPARAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderParam",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParamName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderParam, param_name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderParam, value),
            },
            FieldInfoData {
                name: "ParamType",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHPARAMTYPE_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderParam, param_type),
            },
            FieldInfoData {
                name: "ValueType",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTEREXPOSABLETYPE_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderParam, value_type),
            },
        ],
    }),
    array_type: Some(VERTEXSHADERPARAM_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VertexShaderParam {
    fn type_info() -> &'static TypeInfo {
        VERTEXSHADERPARAM_TYPE_INFO
    }
}


pub const VERTEXSHADERPARAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderParam-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("VertexShaderParam-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VertexShaderTextureParam {
    pub param_name: String,
    pub value: super::render_base::TextureBaseAsset,
    pub address_mode: TextureNodeAddress,
    pub filter_mode: TextureNodeFilter,
}

pub const VERTEXSHADERTEXTUREPARAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderTextureParam",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParamName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderTextureParam, param_name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderTextureParam, value),
            },
            FieldInfoData {
                name: "AddressMode",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURENODEADDRESS_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderTextureParam, address_mode),
            },
            FieldInfoData {
                name: "FilterMode",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURENODEFILTER_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderTextureParam, filter_mode),
            },
        ],
    }),
    array_type: Some(VERTEXSHADERTEXTUREPARAM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VertexShaderTextureParam {
    fn type_info() -> &'static TypeInfo {
        VERTEXSHADERTEXTUREPARAM_TYPE_INFO
    }
}


pub const VERTEXSHADERTEXTUREPARAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderTextureParam-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("VertexShaderTextureParam-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterGraphParticleDataType {
    #[default]
    EmitterGraphParticleDataType_Float = 0,
    EmitterGraphParticleDataType_Float2 = 1,
    EmitterGraphParticleDataType_Float3 = 2,
    EmitterGraphParticleDataType_Float4 = 3,
    EmitterGraphParticleDataType_Half2 = 4,
    EmitterGraphParticleDataType_Half3 = 5,
    EmitterGraphParticleDataType_Half4 = 6,
    EmitterGraphParticleDataType_Float3x3 = 7,
    EmitterGraphParticleDataType_Float4x4 = 8,
}

pub const EMITTERGRAPHPARTICLEDATATYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphParticleDataType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHPARTICLEDATATYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphParticleDataType {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHPARTICLEDATATYPE_TYPE_INFO
    }
}


pub const EMITTERGRAPHPARTICLEDATATYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphParticleDataType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphParticleDataType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TextureNodeFilter {
    #[default]
    TnfPoint = 0,
    TnfLinear = 1,
}

pub const TEXTURENODEFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureNodeFilter",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTURENODEFILTER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureNodeFilter {
    fn type_info() -> &'static TypeInfo {
        TEXTURENODEFILTER_TYPE_INFO
    }
}


pub const TEXTURENODEFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureNodeFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("TextureNodeFilter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TextureNodeAddress {
    #[default]
    TnaWrap = 0,
    TnaClamp = 1,
}

pub const TEXTURENODEADDRESS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureNodeAddress",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTURENODEADDRESS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureNodeAddress {
    fn type_info() -> &'static TypeInfo {
        TEXTURENODEADDRESS_TYPE_INFO
    }
}


pub const TEXTURENODEADDRESS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureNodeAddress-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("TextureNodeAddress-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterExclusionVolumesAsset {
    pub exclusion_volumes_count: u32,
    pub exclusion_volumes: Vec<EmitterExclusionVolume>,
    pub exclusion_volume_bounding_spheres: Vec<EmitterExclusionVolumeBoundingSphereSoA>,
}

pub const EMITTEREXCLUSIONVOLUMESASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumesAsset",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTEREXCLUSIONVOLUMESBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExclusionVolumesCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumesAsset, exclusion_volumes_count),
            },
            FieldInfoData {
                name: "ExclusionVolumes",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTEREXCLUSIONVOLUME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumesAsset, exclusion_volumes),
            },
            FieldInfoData {
                name: "ExclusionVolumeBoundingSpheres",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumesAsset, exclusion_volume_bounding_spheres),
            },
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUMESASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterExclusionVolumesAsset {
    fn type_info() -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUMESASSET_TYPE_INFO
    }
}


pub const EMITTEREXCLUSIONVOLUMESASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumesAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterExclusionVolumesAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshEmitterMaskAsset {
    pub mesh_emitter_mask_resource: super::core::ResourceRef,
}

pub const MESHEMITTERMASKASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskAsset",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHEMITTERMASKBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MeshEmitterMaskResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(MeshEmitterMaskAsset, mesh_emitter_mask_resource),
            },
        ],
    }),
    array_type: Some(MESHEMITTERMASKASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshEmitterMaskAsset {
    fn type_info() -> &'static TypeInfo {
        MESHEMITTERMASKASSET_TYPE_INFO
    }
}


pub const MESHEMITTERMASKASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MeshEmitterMaskAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshEmitterAsset {
    pub mesh_emitter_resource: super::core::ResourceRef,
}

pub const MESHEMITTERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterAsset",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHEMITTERBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MeshEmitterResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(MeshEmitterAsset, mesh_emitter_resource),
            },
        ],
    }),
    array_type: Some(MESHEMITTERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshEmitterAsset {
    fn type_info() -> &'static TypeInfo {
        MESHEMITTERASSET_TYPE_INFO
    }
}


pub const MESHEMITTERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MeshEmitterAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterAsset {
}

pub const EMITTERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterAsset",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERBASEASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EMITTERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterAsset {
    fn type_info() -> &'static TypeInfo {
        EMITTERASSET_TYPE_INFO
    }
}


pub const EMITTERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateStencilMaskData {
    pub mask: EmitterStencilMask,
}

pub const UPDATESTENCILMASKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateStencilMaskData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Mask",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERSTENCILMASK_TYPE_INFO,
                rust_offset: offset_of!(UpdateStencilMaskData, mask),
            },
        ],
    }),
    array_type: Some(UPDATESTENCILMASKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateStencilMaskData {
    fn type_info() -> &'static TypeInfo {
        UPDATESTENCILMASKDATA_TYPE_INFO
    }
}


pub const UPDATESTENCILMASKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateStencilMaskData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateStencilMaskData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterStencilMask {
    #[default]
    EmitterStencilMask_None = 0,
    EmitterStencilMask_Static = 1,
    EmitterStencilMask_Dynamic = 2,
}

pub const EMITTERSTENCILMASK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterStencilMask",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERSTENCILMASK_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterStencilMask {
    fn type_info() -> &'static TypeInfo {
        EMITTERSTENCILMASK_TYPE_INFO
    }
}


pub const EMITTERSTENCILMASK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterStencilMask-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterStencilMask-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateVolumeMaskData {
    pub mask_type: VolumeMaskType,
}

pub const UPDATEVOLUMEMASKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVolumeMaskData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaskType",
                flags: MemberInfoFlags::new(0),
                field_type: VOLUMEMASKTYPE_TYPE_INFO,
                rust_offset: offset_of!(UpdateVolumeMaskData, mask_type),
            },
        ],
    }),
    array_type: Some(UPDATEVOLUMEMASKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateVolumeMaskData {
    fn type_info() -> &'static TypeInfo {
        UPDATEVOLUMEMASKDATA_TYPE_INFO
    }
}


pub const UPDATEVOLUMEMASKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVolumeMaskData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateVolumeMaskData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VolumeMaskType {
    #[default]
    VolumeMaskType_None = 0,
    VolumeMaskType_Static = 1,
    VolumeMaskType_Dynamic = 2,
}

pub const VOLUMEMASKTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumeMaskType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(VOLUMEMASKTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VolumeMaskType {
    fn type_info() -> &'static TypeInfo {
        VOLUMEMASKTYPE_TYPE_INFO
    }
}


pub const VOLUMEMASKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumeMaskType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("VolumeMaskType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateVolumetricData {
    pub absorption: f32,
    pub high_quality_injection: bool,
}

pub const UPDATEVOLUMETRICDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVolumetricData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateVolumetricData, absorption),
            },
            FieldInfoData {
                name: "HighQualityInjection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateVolumetricData, high_quality_injection),
            },
        ],
    }),
    array_type: Some(UPDATEVOLUMETRICDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateVolumetricData {
    fn type_info() -> &'static TypeInfo {
        UPDATEVOLUMETRICDATA_TYPE_INFO
    }
}


pub const UPDATEVOLUMETRICDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVolumetricData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateVolumetricData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateMeshEmitterMaskData {
    pub mesh_emitter_mask: MeshEmitterMaskAsset,
}

pub const UPDATEMESHEMITTERMASKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMeshEmitterMaskData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MeshEmitterMask",
                flags: MemberInfoFlags::new(0),
                field_type: MESHEMITTERMASKASSET_TYPE_INFO,
                rust_offset: offset_of!(UpdateMeshEmitterMaskData, mesh_emitter_mask),
            },
        ],
    }),
    array_type: Some(UPDATEMESHEMITTERMASKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateMeshEmitterMaskData {
    fn type_info() -> &'static TypeInfo {
        UPDATEMESHEMITTERMASKDATA_TYPE_INFO
    }
}


pub const UPDATEMESHEMITTERMASKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMeshEmitterMaskData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateMeshEmitterMaskData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateMeshEmitterSourceData {
    pub mesh_emitter: MeshEmitterAsset,
    pub generate_position: bool,
    pub generate_normal: bool,
    pub generate_u_vs: bool,
    pub send_mesh_uvs_to_shader_graph: bool,
    pub sequential_emission: bool,
    pub particles_per_primitive: u32,
    pub random_position: f32,
}

pub const UPDATEMESHEMITTERSOURCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMeshEmitterSourceData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MeshEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: MESHEMITTERASSET_TYPE_INFO,
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, mesh_emitter),
            },
            FieldInfoData {
                name: "GeneratePosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, generate_position),
            },
            FieldInfoData {
                name: "GenerateNormal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, generate_normal),
            },
            FieldInfoData {
                name: "GenerateUVs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, generate_u_vs),
            },
            FieldInfoData {
                name: "SendMeshUvsToShaderGraph",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, send_mesh_uvs_to_shader_graph),
            },
            FieldInfoData {
                name: "SequentialEmission",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, sequential_emission),
            },
            FieldInfoData {
                name: "ParticlesPerPrimitive",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, particles_per_primitive),
            },
            FieldInfoData {
                name: "RandomPosition",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, random_position),
            },
        ],
    }),
    array_type: Some(UPDATEMESHEMITTERSOURCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateMeshEmitterSourceData {
    fn type_info() -> &'static TypeInfo {
        UPDATEMESHEMITTERSOURCEDATA_TYPE_INFO
    }
}


pub const UPDATEMESHEMITTERSOURCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMeshEmitterSourceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateMeshEmitterSourceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateVertexAnimData {
    pub shader_fragment: super::render::VertexShaderFragmentAsset,
    pub per_particle_randomness: f32,
    pub frequency_multiplier: f32,
    pub animation_parameters: super::core::Vec4,
    pub animation_texture: super::render_base::TextureBaseAsset,
}

pub const UPDATEVERTEXANIMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVertexAnimData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ShaderFragment",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXSHADERFRAGMENTASSET_TYPE_INFO,
                rust_offset: offset_of!(UpdateVertexAnimData, shader_fragment),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateVertexAnimData, per_particle_randomness),
            },
            FieldInfoData {
                name: "FrequencyMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateVertexAnimData, frequency_multiplier),
            },
            FieldInfoData {
                name: "AnimationParameters",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UpdateVertexAnimData, animation_parameters),
            },
            FieldInfoData {
                name: "AnimationTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(UpdateVertexAnimData, animation_texture),
            },
        ],
    }),
    array_type: Some(UPDATEVERTEXANIMDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateVertexAnimData {
    fn type_info() -> &'static TypeInfo {
        UPDATEVERTEXANIMDATA_TYPE_INFO
    }
}


pub const UPDATEVERTEXANIMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVertexAnimData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateVertexAnimData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateBeamPointData {
    pub num_points: u32,
    pub num_ctrl_points: u32,
    pub taper_coefficients: super::core::Vec4,
    pub attractor: LocationSelection,
    pub attractor_coefficients: super::core::Vec4,
    pub param_override: ParamOverrideSelection,
    pub param_coefficients: super::core::Vec4,
    pub beam_interpolation: BeamInterpolation,
    pub coefficient: f32,
}

pub const UPDATEBEAMPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamPointData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "NumPoints",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamPointData, num_points),
            },
            FieldInfoData {
                name: "NumCtrlPoints",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamPointData, num_ctrl_points),
            },
            FieldInfoData {
                name: "TaperCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamPointData, taper_coefficients),
            },
            FieldInfoData {
                name: "Attractor",
                flags: MemberInfoFlags::new(0),
                field_type: LOCATIONSELECTION_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamPointData, attractor),
            },
            FieldInfoData {
                name: "AttractorCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamPointData, attractor_coefficients),
            },
            FieldInfoData {
                name: "ParamOverride",
                flags: MemberInfoFlags::new(0),
                field_type: PARAMOVERRIDESELECTION_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamPointData, param_override),
            },
            FieldInfoData {
                name: "ParamCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamPointData, param_coefficients),
            },
            FieldInfoData {
                name: "BeamInterpolation",
                flags: MemberInfoFlags::new(0),
                field_type: BEAMINTERPOLATION_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamPointData, beam_interpolation),
            },
            FieldInfoData {
                name: "Coefficient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamPointData, coefficient),
            },
        ],
    }),
    array_type: Some(UPDATEBEAMPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateBeamPointData {
    fn type_info() -> &'static TypeInfo {
        UPDATEBEAMPOINTDATA_TYPE_INFO
    }
}


pub const UPDATEBEAMPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamPointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateBeamPointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BeamInterpolation {
    #[default]
    BeamInterpolation_Linear = 0,
    BeamInterpolation_Spline = 1,
    BeamInterpolation_Curve = 2,
}

pub const BEAMINTERPOLATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BeamInterpolation",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(BEAMINTERPOLATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BeamInterpolation {
    fn type_info() -> &'static TypeInfo {
        BEAMINTERPOLATION_TYPE_INFO
    }
}


pub const BEAMINTERPOLATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BeamInterpolation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("BeamInterpolation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateBeamTargetData {
    pub target: LocationSelection,
}

pub const UPDATEBEAMTARGETDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamTargetData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Target",
                flags: MemberInfoFlags::new(0),
                field_type: LOCATIONSELECTION_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamTargetData, target),
            },
        ],
    }),
    array_type: Some(UPDATEBEAMTARGETDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateBeamTargetData {
    fn type_info() -> &'static TypeInfo {
        UPDATEBEAMTARGETDATA_TYPE_INFO
    }
}


pub const UPDATEBEAMTARGETDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamTargetData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateBeamTargetData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateBeamSourceData {
    pub source: LocationSelection,
}

pub const UPDATEBEAMSOURCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamSourceData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Source",
                flags: MemberInfoFlags::new(0),
                field_type: LOCATIONSELECTION_TYPE_INFO,
                rust_offset: offset_of!(UpdateBeamSourceData, source),
            },
        ],
    }),
    array_type: Some(UPDATEBEAMSOURCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateBeamSourceData {
    fn type_info() -> &'static TypeInfo {
        UPDATEBEAMSOURCEDATA_TYPE_INFO
    }
}


pub const UPDATEBEAMSOURCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamSourceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateBeamSourceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ParamOverrideSelection {
    #[default]
    ParamOverride_None = 0,
    ParamOverride_R = 1,
    ParamOverride_G = 2,
    ParamOverride_B = 3,
    ParamOverride_A = 4,
}

pub const PARAMOVERRIDESELECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParamOverrideSelection",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(PARAMOVERRIDESELECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ParamOverrideSelection {
    fn type_info() -> &'static TypeInfo {
        PARAMOVERRIDESELECTION_TYPE_INFO
    }
}


pub const PARAMOVERRIDESELECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParamOverrideSelection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ParamOverrideSelection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LocationSelection {
    #[default]
    LsEmitter = 0,
    LsParticle = 1,
    LsParticlePlusVelocity = 2,
    LsParticleMinusVelocity = 3,
    LsSource = 4,
    LsTarget = 5,
    LsOther = 6,
}

pub const LOCATIONSELECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationSelection",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(LOCATIONSELECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocationSelection {
    fn type_info() -> &'static TypeInfo {
        LOCATIONSELECTION_TYPE_INFO
    }
}


pub const LOCATIONSELECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationSelection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("LocationSelection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateQuadBendingAngleData {
}

pub const UPDATEQUADBENDINGANGLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateQuadBendingAngleData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UPDATEQUADBENDINGANGLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateQuadBendingAngleData {
    fn type_info() -> &'static TypeInfo {
        UPDATEQUADBENDINGANGLEDATA_TYPE_INFO
    }
}


pub const UPDATEQUADBENDINGANGLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateQuadBendingAngleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateQuadBendingAngleData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateClipScaleData {
    pub lookup: Vec<i16>,
}

pub const UPDATECLIPSCALEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateClipScaleData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Lookup",
                flags: MemberInfoFlags::new(144),
                field_type: INT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UpdateClipScaleData, lookup),
            },
        ],
    }),
    array_type: Some(UPDATECLIPSCALEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateClipScaleData {
    fn type_info() -> &'static TypeInfo {
        UPDATECLIPSCALEDATA_TYPE_INFO
    }
}


pub const UPDATECLIPSCALEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateClipScaleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateClipScaleData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SnapToWaterData {
    pub offset: f32,
}

pub const SNAPTOWATERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapToWaterData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SnapToWaterData, offset),
            },
        ],
    }),
    array_type: Some(SNAPTOWATERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SnapToWaterData {
    fn type_info() -> &'static TypeInfo {
        SNAPTOWATERDATA_TYPE_INFO
    }
}


pub const SNAPTOWATERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapToWaterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SnapToWaterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateCollisionData {
    pub restitution: f32,
    pub reflection_bias: f32,
    pub rest_speed_threshold: f32,
    pub randomness: f32,
    pub kill_on_collision: bool,
    pub death_effect_orientation: EmitterCollisionEffectOrientation,
    pub collision_type: EmitterCollisionType,
    pub collision_radius_factor: f32,
    pub method: EmitterCollisionMethod,
    pub priority: EmitterCollisionPriority,
    pub snap_on_terrain: bool,
    pub snap_offset_based_on_emitter: f32,
    pub snap_relative_offset_factor: f32,
    pub snap_range: f32,
    pub snap_velocity: EmitterSnapVelocityType,
    pub snap_type: EmitterTerrainSnapType,
    pub repel_factor: f32,
    pub repel_height: f32,
    pub material_pair: super::entity::MaterialDecl,
    pub throttle: f32,
    pub throttle_far_distance: f32,
    pub throttle_envelope: super::core::Vec4,
    pub check_water: bool,
    pub check_terrain: bool,
    pub check_ragdoll: bool,
    pub check_character: bool,
    pub check_group: bool,
    pub check_phantoms: bool,
    pub check_simple_shape: bool,
}

pub const UPDATECOLLISIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCollisionData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Restitution",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, restitution),
            },
            FieldInfoData {
                name: "ReflectionBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, reflection_bias),
            },
            FieldInfoData {
                name: "RestSpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, rest_speed_threshold),
            },
            FieldInfoData {
                name: "Randomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, randomness),
            },
            FieldInfoData {
                name: "KillOnCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, kill_on_collision),
            },
            FieldInfoData {
                name: "DeathEffectOrientation",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERCOLLISIONEFFECTORIENTATION_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, death_effect_orientation),
            },
            FieldInfoData {
                name: "CollisionType",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERCOLLISIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, collision_type),
            },
            FieldInfoData {
                name: "CollisionRadiusFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, collision_radius_factor),
            },
            FieldInfoData {
                name: "Method",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERCOLLISIONMETHOD_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, method),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERCOLLISIONPRIORITY_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, priority),
            },
            FieldInfoData {
                name: "SnapOnTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, snap_on_terrain),
            },
            FieldInfoData {
                name: "SnapOffsetBasedOnEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, snap_offset_based_on_emitter),
            },
            FieldInfoData {
                name: "SnapRelativeOffsetFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, snap_relative_offset_factor),
            },
            FieldInfoData {
                name: "SnapRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, snap_range),
            },
            FieldInfoData {
                name: "SnapVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERSNAPVELOCITYTYPE_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, snap_velocity),
            },
            FieldInfoData {
                name: "SnapType",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERTERRAINSNAPTYPE_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, snap_type),
            },
            FieldInfoData {
                name: "RepelFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, repel_factor),
            },
            FieldInfoData {
                name: "RepelHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, repel_height),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, material_pair),
            },
            FieldInfoData {
                name: "Throttle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, throttle),
            },
            FieldInfoData {
                name: "ThrottleFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, throttle_far_distance),
            },
            FieldInfoData {
                name: "ThrottleEnvelope",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, throttle_envelope),
            },
            FieldInfoData {
                name: "CheckWater",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, check_water),
            },
            FieldInfoData {
                name: "CheckTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, check_terrain),
            },
            FieldInfoData {
                name: "CheckRagdoll",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, check_ragdoll),
            },
            FieldInfoData {
                name: "CheckCharacter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, check_character),
            },
            FieldInfoData {
                name: "CheckGroup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, check_group),
            },
            FieldInfoData {
                name: "CheckPhantoms",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, check_phantoms),
            },
            FieldInfoData {
                name: "CheckSimpleShape",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateCollisionData, check_simple_shape),
            },
        ],
    }),
    array_type: Some(UPDATECOLLISIONDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateCollisionData {
    fn type_info() -> &'static TypeInfo {
        UPDATECOLLISIONDATA_TYPE_INFO
    }
}


pub const UPDATECOLLISIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCollisionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateCollisionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterCollisionEffectOrientation {
    #[default]
    EmitterCollisionEffectOrientation_BounceDirection = 0,
    EmitterCollisionEffectOrientation_Normal = 1,
    EmitterCollisionEffectOrientation_ImpactDirection = 2,
}

pub const EMITTERCOLLISIONEFFECTORIENTATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionEffectOrientation",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERCOLLISIONEFFECTORIENTATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterCollisionEffectOrientation {
    fn type_info() -> &'static TypeInfo {
        EMITTERCOLLISIONEFFECTORIENTATION_TYPE_INFO
    }
}


pub const EMITTERCOLLISIONEFFECTORIENTATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionEffectOrientation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterCollisionEffectOrientation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterCollisionType {
    #[default]
    EmitterCollisionType_PassThrough = 0,
    EmitterCollisionType_Stick = 1,
    EmitterCollisionType_Bounce = 2,
    EmitterCollisionType_Count = 3,
}

pub const EMITTERCOLLISIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERCOLLISIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterCollisionType {
    fn type_info() -> &'static TypeInfo {
        EMITTERCOLLISIONTYPE_TYPE_INFO
    }
}


pub const EMITTERCOLLISIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterCollisionType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterCollisionPriority {
    #[default]
    EmitterCollisionPriority_Low = 0,
    EmitterCollisionPriority_Medium = 1,
    EmitterCollisionPriority_High = 2,
}

pub const EMITTERCOLLISIONPRIORITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionPriority",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERCOLLISIONPRIORITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterCollisionPriority {
    fn type_info() -> &'static TypeInfo {
        EMITTERCOLLISIONPRIORITY_TYPE_INFO
    }
}


pub const EMITTERCOLLISIONPRIORITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionPriority-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterCollisionPriority-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterSnapVelocityType {
    #[default]
    EmitterSnapVelocityType_Disabled = 0,
    EmitterSnapVelocityType_MaintainMagnitude = 1,
    EmitterSnapVelocityType_MaintainDirection = 2,
}

pub const EMITTERSNAPVELOCITYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSnapVelocityType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERSNAPVELOCITYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterSnapVelocityType {
    fn type_info() -> &'static TypeInfo {
        EMITTERSNAPVELOCITYTYPE_TYPE_INFO
    }
}


pub const EMITTERSNAPVELOCITYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSnapVelocityType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterSnapVelocityType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterTerrainSnapType {
    #[default]
    EmitterTerrainSnapType_SpawnOnly = 0,
    EmitterTerrainSnapType_RenderingOnly = 1,
    EmitterTerrainSnapType_SpawnAndRendering = 2,
    EmitterTerrainSnapType_SpawnAndRepel = 3,
}

pub const EMITTERTERRAINSNAPTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTerrainSnapType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERTERRAINSNAPTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterTerrainSnapType {
    fn type_info() -> &'static TypeInfo {
        EMITTERTERRAINSNAPTYPE_TYPE_INFO
    }
}


pub const EMITTERTERRAINSNAPTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTerrainSnapType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterTerrainSnapType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterCollisionMethod {
    #[default]
    EmitterCollisionMethod_TerrainHeightMap = 0,
    EmitterCollisionMethod_RayCast = 1,
    EmitterCollisionMethod_RayCastDetailed = 2,
}

pub const EMITTERCOLLISIONMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERCOLLISIONMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterCollisionMethod {
    fn type_info() -> &'static TypeInfo {
        EMITTERCOLLISIONMETHOD_TYPE_INFO
    }
}


pub const EMITTERCOLLISIONMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterCollisionMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateCameraProximityData {
    pub size: super::core::Vec3,
    pub fade_distance: super::core::Vec3,
    pub forward_offset: f32,
}

pub const UPDATECAMERAPROXIMITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCameraProximityData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(UpdateCameraProximityData, size),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(UpdateCameraProximityData, fade_distance),
            },
            FieldInfoData {
                name: "ForwardOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateCameraProximityData, forward_offset),
            },
        ],
    }),
    array_type: Some(UPDATECAMERAPROXIMITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateCameraProximityData {
    fn type_info() -> &'static TypeInfo {
        UPDATECAMERAPROXIMITYDATA_TYPE_INFO
    }
}


pub const UPDATECAMERAPROXIMITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCameraProximityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateCameraProximityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateTrapezoidShapeData {
    pub scale: f32,
}

pub const UPDATETRAPEZOIDSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTrapezoidShapeData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateTrapezoidShapeData, scale),
            },
        ],
    }),
    array_type: Some(UPDATETRAPEZOIDSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateTrapezoidShapeData {
    fn type_info() -> &'static TypeInfo {
        UPDATETRAPEZOIDSHAPEDATA_TYPE_INFO
    }
}


pub const UPDATETRAPEZOIDSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTrapezoidShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateTrapezoidShapeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateBackLightData {
    pub vertex_back_light: f32,
    pub gnomon_back_light: f32,
    pub pixel_contrast: f32,
    pub view_independent_contrast: f32,
}

pub const UPDATEBACKLIGHTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBackLightData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "VertexBackLight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateBackLightData, vertex_back_light),
            },
            FieldInfoData {
                name: "GnomonBackLight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateBackLightData, gnomon_back_light),
            },
            FieldInfoData {
                name: "PixelContrast",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateBackLightData, pixel_contrast),
            },
            FieldInfoData {
                name: "ViewIndependentContrast",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateBackLightData, view_independent_contrast),
            },
        ],
    }),
    array_type: Some(UPDATEBACKLIGHTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateBackLightData {
    fn type_info() -> &'static TypeInfo {
        UPDATEBACKLIGHTDATA_TYPE_INFO
    }
}


pub const UPDATEBACKLIGHTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBackLightData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateBackLightData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateLightWrapAroundData {
}

pub const UPDATELIGHTWRAPAROUNDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateLightWrapAroundData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UPDATELIGHTWRAPAROUNDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateLightWrapAroundData {
    fn type_info() -> &'static TypeInfo {
        UPDATELIGHTWRAPAROUNDDATA_TYPE_INFO
    }
}


pub const UPDATELIGHTWRAPAROUNDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateLightWrapAroundData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateLightWrapAroundData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateTextureColorLerpData {
    pub texture_color_strength: f32,
}

pub const UPDATETEXTURECOLORLERPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTextureColorLerpData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TextureColorStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureColorLerpData, texture_color_strength),
            },
        ],
    }),
    array_type: Some(UPDATETEXTURECOLORLERPDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateTextureColorLerpData {
    fn type_info() -> &'static TypeInfo {
        UPDATETEXTURECOLORLERPDATA_TYPE_INFO
    }
}


pub const UPDATETEXTURECOLORLERPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTextureColorLerpData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateTextureColorLerpData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateAlphaLevelScaleData {
    pub exponent: f32,
}

pub const UPDATEALPHALEVELSCALEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelScaleData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Exponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateAlphaLevelScaleData, exponent),
            },
        ],
    }),
    array_type: Some(UPDATEALPHALEVELSCALEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateAlphaLevelScaleData {
    fn type_info() -> &'static TypeInfo {
        UPDATEALPHALEVELSCALEDATA_TYPE_INFO
    }
}


pub const UPDATEALPHALEVELSCALEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelScaleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateAlphaLevelScaleData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateAlphaLevelMaxData {
    pub max_level: f32,
}

pub const UPDATEALPHALEVELMAXDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelMaxData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxLevel",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateAlphaLevelMaxData, max_level),
            },
        ],
    }),
    array_type: Some(UPDATEALPHALEVELMAXDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateAlphaLevelMaxData {
    fn type_info() -> &'static TypeInfo {
        UPDATEALPHALEVELMAXDATA_TYPE_INFO
    }
}


pub const UPDATEALPHALEVELMAXDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelMaxData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateAlphaLevelMaxData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateAlphaLevelMinData {
    pub min_level: f32,
}

pub const UPDATEALPHALEVELMINDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelMinData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MinLevel",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateAlphaLevelMinData, min_level),
            },
        ],
    }),
    array_type: Some(UPDATEALPHALEVELMINDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateAlphaLevelMinData {
    fn type_info() -> &'static TypeInfo {
        UPDATEALPHALEVELMINDATA_TYPE_INFO
    }
}


pub const UPDATEALPHALEVELMINDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelMinData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateAlphaLevelMinData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateRibbonTextureData {
    pub texture_particle_count: i32,
    pub mirror_texture: bool,
    pub beam_like_coords: bool,
}

pub const UPDATERIBBONTEXTUREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRibbonTextureData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TextureParticleCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateRibbonTextureData, texture_particle_count),
            },
            FieldInfoData {
                name: "MirrorTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateRibbonTextureData, mirror_texture),
            },
            FieldInfoData {
                name: "BeamLikeCoords",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateRibbonTextureData, beam_like_coords),
            },
        ],
    }),
    array_type: Some(UPDATERIBBONTEXTUREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateRibbonTextureData {
    fn type_info() -> &'static TypeInfo {
        UPDATERIBBONTEXTUREDATA_TYPE_INFO
    }
}


pub const UPDATERIBBONTEXTUREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRibbonTextureData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateRibbonTextureData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateRibbonFadeData {
    pub fade_in_particle_count: i32,
    pub fade_out_particle_count: i32,
}

pub const UPDATERIBBONFADEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRibbonFadeData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FadeInParticleCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateRibbonFadeData, fade_in_particle_count),
            },
            FieldInfoData {
                name: "FadeOutParticleCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateRibbonFadeData, fade_out_particle_count),
            },
        ],
    }),
    array_type: Some(UPDATERIBBONFADEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateRibbonFadeData {
    fn type_info() -> &'static TypeInfo {
        UPDATERIBBONFADEDATA_TYPE_INFO
    }
}


pub const UPDATERIBBONFADEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRibbonFadeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateRibbonFadeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateTransparencySecondaryData {
}

pub const UPDATETRANSPARENCYSECONDARYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTransparencySecondaryData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UPDATETRANSPARENCYSECONDARYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateTransparencySecondaryData {
    fn type_info() -> &'static TypeInfo {
        UPDATETRANSPARENCYSECONDARYDATA_TYPE_INFO
    }
}


pub const UPDATETRANSPARENCYSECONDARYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTransparencySecondaryData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateTransparencySecondaryData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateTransparencyData {
}

pub const UPDATETRANSPARENCYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTransparencyData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UPDATETRANSPARENCYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateTransparencyData {
    fn type_info() -> &'static TypeInfo {
        UPDATETRANSPARENCYDATA_TYPE_INFO
    }
}


pub const UPDATETRANSPARENCYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTransparencyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateTransparencyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateColorSecondaryData {
    pub color: super::core::Vec3,
}

pub const UPDATECOLORSECONDARYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateColorSecondaryData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(UpdateColorSecondaryData, color),
            },
        ],
    }),
    array_type: Some(UPDATECOLORSECONDARYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateColorSecondaryData {
    fn type_info() -> &'static TypeInfo {
        UPDATECOLORSECONDARYDATA_TYPE_INFO
    }
}


pub const UPDATECOLORSECONDARYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateColorSecondaryData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateColorSecondaryData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateColorData {
    pub color: super::core::Vec3,
}

pub const UPDATECOLORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateColorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(UpdateColorData, color),
            },
        ],
    }),
    array_type: Some(UPDATECOLORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateColorData {
    fn type_info() -> &'static TypeInfo {
        UPDATECOLORDATA_TYPE_INFO
    }
}


pub const UPDATECOLORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateColorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateColorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateCustomParamsData {
}

pub const UPDATECUSTOMPARAMSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCustomParamsData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UPDATECUSTOMPARAMSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateCustomParamsData {
    fn type_info() -> &'static TypeInfo {
        UPDATECUSTOMPARAMSDATA_TYPE_INFO
    }
}


pub const UPDATECUSTOMPARAMSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCustomParamsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateCustomParamsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateSizeZData {
}

pub const UPDATESIZEZDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeZData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UPDATESIZEZDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateSizeZData {
    fn type_info() -> &'static TypeInfo {
        UPDATESIZEZDATA_TYPE_INFO
    }
}


pub const UPDATESIZEZDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeZData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateSizeZData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateSizeYData {
}

pub const UPDATESIZEYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeYData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UPDATESIZEYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateSizeYData {
    fn type_info() -> &'static TypeInfo {
        UPDATESIZEYDATA_TYPE_INFO
    }
}


pub const UPDATESIZEYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeYData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateSizeYData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateSizeXData {
}

pub const UPDATESIZEXDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeXData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UPDATESIZEXDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateSizeXData {
    fn type_info() -> &'static TypeInfo {
        UPDATESIZEXDATA_TYPE_INFO
    }
}


pub const UPDATESIZEXDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeXData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateSizeXData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateSizeData {
    pub pivot: super::core::Vec2,
    pub multiply_with_size_x_y_z: bool,
}

pub const UPDATESIZEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Pivot",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(UpdateSizeData, pivot),
            },
            FieldInfoData {
                name: "MultiplyWithSizeXYZ",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateSizeData, multiply_with_size_x_y_z),
            },
        ],
    }),
    array_type: Some(UPDATESIZEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateSizeData {
    fn type_info() -> &'static TypeInfo {
        UPDATESIZEDATA_TYPE_INFO
    }
}


pub const UPDATESIZEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateSizeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpdateRotationData {
}

pub const UPDATEROTATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRotationData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UPDATEROTATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateRotationData {
    fn type_info() -> &'static TypeInfo {
        UPDATEROTATIONDATA_TYPE_INFO
    }
}


pub const UPDATEROTATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRotationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateRotationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateTextureCoordsData {
    pub base_texture: super::render::AtlasTextureAsset,
    pub normal_texture: super::render::AtlasTextureAsset,
    pub disable_clip_scale_optimization: bool,
    pub modifier_u: TexCoordModifier,
    pub modifier_v: TexCoordModifier,
    pub scale_u: f32,
    pub scale_v: f32,
    pub bias_u: f32,
    pub bias_v: f32,
    pub direct_texture_frame_lookup: bool,
    pub input_start_min: f32,
    pub input_start_max: f32,
    pub enable_frame_blending: bool,
}

pub const UPDATETEXTURECOORDSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTextureCoordsData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BaseTexture",
                flags: MemberInfoFlags::new(0),
                field_type: ATLASTEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, base_texture),
            },
            FieldInfoData {
                name: "NormalTexture",
                flags: MemberInfoFlags::new(0),
                field_type: ATLASTEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, normal_texture),
            },
            FieldInfoData {
                name: "DisableClipScaleOptimization",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, disable_clip_scale_optimization),
            },
            FieldInfoData {
                name: "ModifierU",
                flags: MemberInfoFlags::new(0),
                field_type: TEXCOORDMODIFIER_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, modifier_u),
            },
            FieldInfoData {
                name: "ModifierV",
                flags: MemberInfoFlags::new(0),
                field_type: TEXCOORDMODIFIER_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, modifier_v),
            },
            FieldInfoData {
                name: "ScaleU",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, scale_u),
            },
            FieldInfoData {
                name: "ScaleV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, scale_v),
            },
            FieldInfoData {
                name: "BiasU",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, bias_u),
            },
            FieldInfoData {
                name: "BiasV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, bias_v),
            },
            FieldInfoData {
                name: "DirectTextureFrameLookup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, direct_texture_frame_lookup),
            },
            FieldInfoData {
                name: "InputStartMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, input_start_min),
            },
            FieldInfoData {
                name: "InputStartMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, input_start_max),
            },
            FieldInfoData {
                name: "EnableFrameBlending",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateTextureCoordsData, enable_frame_blending),
            },
        ],
    }),
    array_type: Some(UPDATETEXTURECOORDSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateTextureCoordsData {
    fn type_info() -> &'static TypeInfo {
        UPDATETEXTURECOORDSDATA_TYPE_INFO
    }
}


pub const UPDATETEXTURECOORDSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTextureCoordsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateTextureCoordsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TexCoordModifier {
    #[default]
    TCM_None = 0,
    TCM_Absolute = 1,
    TCM_WorldSpaceExtent = 2,
    TCM_PerBeamPoint = 3,
}

pub const TEXCOORDMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TexCoordModifier",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(TEXCOORDMODIFIER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TexCoordModifier {
    fn type_info() -> &'static TypeInfo {
        TEXCOORDMODIFIER_TYPE_INFO
    }
}


pub const TEXCOORDMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TexCoordModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("TexCoordModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateMimicOverridesData {
    pub lifetime_scale: f32,
    pub unique_random: bool,
    pub size_scale: f32,
    pub size_y_scale_multiplier: f32,
    pub uniform_scale_original: bool,
    pub override_rotation: bool,
}

pub const UPDATEMIMICOVERRIDESDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMimicOverridesData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LifetimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateMimicOverridesData, lifetime_scale),
            },
            FieldInfoData {
                name: "UniqueRandom",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateMimicOverridesData, unique_random),
            },
            FieldInfoData {
                name: "SizeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateMimicOverridesData, size_scale),
            },
            FieldInfoData {
                name: "SizeYScaleMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateMimicOverridesData, size_y_scale_multiplier),
            },
            FieldInfoData {
                name: "UniformScaleOriginal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateMimicOverridesData, uniform_scale_original),
            },
            FieldInfoData {
                name: "OverrideRotation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UpdateMimicOverridesData, override_rotation),
            },
        ],
    }),
    array_type: Some(UPDATEMIMICOVERRIDESDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateMimicOverridesData {
    fn type_info() -> &'static TypeInfo {
        UPDATEMIMICOVERRIDESDATA_TYPE_INFO
    }
}


pub const UPDATEMIMICOVERRIDESDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMimicOverridesData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateMimicOverridesData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MimicEmitterData {
    pub emitter_assets: Vec<EmitterDocument>,
    pub kill_mimics_when_deactivated: bool,
}

pub const MIMICEMITTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MimicEmitterData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EmitterAssets",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTERDOCUMENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MimicEmitterData, emitter_assets),
            },
            FieldInfoData {
                name: "KillMimicsWhenDeactivated",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MimicEmitterData, kill_mimics_when_deactivated),
            },
        ],
    }),
    array_type: Some(MIMICEMITTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MimicEmitterData {
    fn type_info() -> &'static TypeInfo {
        MIMICEMITTERDATA_TYPE_INFO
    }
}


pub const MIMICEMITTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MimicEmitterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MimicEmitterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterData {
    pub emitter_assets: Vec<EmitterDocument>,
    pub emitter_graph_assets: Vec<EmitterGraph>,
}

pub const EMITTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EmitterAssets",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTERDOCUMENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterData, emitter_assets),
            },
            FieldInfoData {
                name: "EmitterGraphAssets",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTERGRAPH_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterData, emitter_graph_assets),
            },
        ],
    }),
    array_type: Some(EMITTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterData {
    fn type_info() -> &'static TypeInfo {
        EMITTERDATA_TYPE_INFO
    }
}


pub const EMITTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TurbulanceData {
    pub intensity: f32,
    pub noise_type: TurbulenceNoiseType,
    pub period_space: f32,
    pub turbulence_force_as_instant_velocity: f32,
    pub octaves: i32,
    pub octave_persistence: f32,
    pub per_particle_randomness: f32,
    pub multiplier: super::core::Vec3,
}

pub const TURBULANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurbulanceData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TurbulanceData, intensity),
            },
            FieldInfoData {
                name: "NoiseType",
                flags: MemberInfoFlags::new(0),
                field_type: TURBULENCENOISETYPE_TYPE_INFO,
                rust_offset: offset_of!(TurbulanceData, noise_type),
            },
            FieldInfoData {
                name: "PeriodSpace",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TurbulanceData, period_space),
            },
            FieldInfoData {
                name: "TurbulenceForceAsInstantVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TurbulanceData, turbulence_force_as_instant_velocity),
            },
            FieldInfoData {
                name: "Octaves",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TurbulanceData, octaves),
            },
            FieldInfoData {
                name: "OctavePersistence",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TurbulanceData, octave_persistence),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TurbulanceData, per_particle_randomness),
            },
            FieldInfoData {
                name: "Multiplier",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TurbulanceData, multiplier),
            },
        ],
    }),
    array_type: Some(TURBULANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TurbulanceData {
    fn type_info() -> &'static TypeInfo {
        TURBULANCEDATA_TYPE_INFO
    }
}


pub const TURBULANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurbulanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("TurbulanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TurbulenceNoiseType {
    #[default]
    TurbulenceNoiseType_Random = 0,
    TurbulenceNoiseType_Perlin = 1,
    TurbulenceNoiseType_PerlinSimplex = 2,
    TurbulenceNoiseType_PerlinCurl = 3,
}

pub const TURBULENCENOISETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurbulenceNoiseType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(TURBULENCENOISETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TurbulenceNoiseType {
    fn type_info() -> &'static TypeInfo {
        TURBULENCENOISETYPE_TYPE_INFO
    }
}


pub const TURBULENCENOISETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurbulenceNoiseType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("TurbulenceNoiseType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AirResistanceData {
    pub drag_factor: f32,
}

pub const AIRRESISTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AirResistanceData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DragFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AirResistanceData, drag_factor),
            },
        ],
    }),
    array_type: Some(AIRRESISTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AirResistanceData {
    fn type_info() -> &'static TypeInfo {
        AIRRESISTANCEDATA_TYPE_INFO
    }
}


pub const AIRRESISTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AirResistanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("AirResistanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WorldForcesData {
    pub forces_multiplier: f32,
    pub per_particle_randomness: f32,
}

pub const WORLDFORCESDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldForcesData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ForcesMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldForcesData, forces_multiplier),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldForcesData, per_particle_randomness),
            },
        ],
    }),
    array_type: Some(WORLDFORCESDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldForcesData {
    fn type_info() -> &'static TypeInfo {
        WORLDFORCESDATA_TYPE_INFO
    }
}


pub const WORLDFORCESDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldForcesData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("WorldForcesData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WorldWindData {
    pub wind_multiplier: f32,
    pub per_particle_randomness: f32,
}

pub const WORLDWINDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldWindData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WindMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldWindData, wind_multiplier),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldWindData, per_particle_randomness),
            },
        ],
    }),
    array_type: Some(WORLDWINDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldWindData {
    fn type_info() -> &'static TypeInfo {
        WORLDWINDDATA_TYPE_INFO
    }
}


pub const WORLDWINDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldWindData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("WorldWindData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalForceData {
    pub local_force: super::core::Vec3,
    pub emitter_local_space_force: bool,
    pub per_particle_randomness: f32,
}

pub const LOCALFORCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalForceData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LocalForce",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LocalForceData, local_force),
            },
            FieldInfoData {
                name: "EmitterLocalSpaceForce",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalForceData, emitter_local_space_force),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalForceData, per_particle_randomness),
            },
        ],
    }),
    array_type: Some(LOCALFORCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalForceData {
    fn type_info() -> &'static TypeInfo {
        LOCALFORCEDATA_TYPE_INFO
    }
}


pub const LOCALFORCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalForceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("LocalForceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GravityData {
    pub gravity: f32,
    pub per_particle_randomness: f32,
}

pub const GRAVITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GravityData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Gravity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GravityData, gravity),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GravityData, per_particle_randomness),
            },
        ],
    }),
    array_type: Some(GRAVITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GravityData {
    fn type_info() -> &'static TypeInfo {
        GRAVITYDATA_TYPE_INFO
    }
}


pub const GRAVITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GravityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("GravityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UpdateAgeData {
    pub lifetime: f32,
    pub random_lifetime_scale: f32,
    pub max_factor: f32,
    pub death_effect: super::effect_base::EffectBlueprint,
    pub throttle: f32,
    pub throttle_far_distance: f32,
    pub throttle_envelope: super::core::Vec4,
}

pub const UPDATEAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAgeData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateAgeData, lifetime),
            },
            FieldInfoData {
                name: "RandomLifetimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateAgeData, random_lifetime_scale),
            },
            FieldInfoData {
                name: "MaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateAgeData, max_factor),
            },
            FieldInfoData {
                name: "DeathEffect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(UpdateAgeData, death_effect),
            },
            FieldInfoData {
                name: "Throttle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateAgeData, throttle),
            },
            FieldInfoData {
                name: "ThrottleFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UpdateAgeData, throttle_far_distance),
            },
            FieldInfoData {
                name: "ThrottleEnvelope",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UpdateAgeData, throttle_envelope),
            },
        ],
    }),
    array_type: Some(UPDATEAGEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateAgeData {
    fn type_info() -> &'static TypeInfo {
        UPDATEAGEDATA_TYPE_INFO
    }
}


pub const UPDATEAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAgeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateAgeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpawnColorRandomData {
    pub color0: super::core::Vec3,
    pub color1: super::core::Vec3,
}

pub const SPAWNCOLORRANDOMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnColorRandomData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Color0",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SpawnColorRandomData, color0),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SpawnColorRandomData, color1),
            },
        ],
    }),
    array_type: Some(SPAWNCOLORRANDOMDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SpawnColorRandomData {
    fn type_info() -> &'static TypeInfo {
        SPAWNCOLORRANDOMDATA_TYPE_INFO
    }
}


pub const SPAWNCOLORRANDOMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnColorRandomData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnColorRandomData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpawnRotationSpeedData {
    pub rotation_speed: f32,
}

pub const SPAWNROTATIONSPEEDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRotationSpeedData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RotationSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnRotationSpeedData, rotation_speed),
            },
        ],
    }),
    array_type: Some(SPAWNROTATIONSPEEDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnRotationSpeedData {
    fn type_info() -> &'static TypeInfo {
        SPAWNROTATIONSPEEDDATA_TYPE_INFO
    }
}


pub const SPAWNROTATIONSPEEDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRotationSpeedData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnRotationSpeedData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpawnOrientationData {
}

pub const SPAWNORIENTATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnOrientationData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SPAWNORIENTATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnOrientationData {
    fn type_info() -> &'static TypeInfo {
        SPAWNORIENTATIONDATA_TYPE_INFO
    }
}


pub const SPAWNORIENTATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnOrientationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnOrientationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpawnRotationData {
    pub rotation: f32,
}

pub const SPAWNROTATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRotationData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnRotationData, rotation),
            },
        ],
    }),
    array_type: Some(SPAWNROTATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnRotationData {
    fn type_info() -> &'static TypeInfo {
        SPAWNROTATIONDATA_TYPE_INFO
    }
}


pub const SPAWNROTATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRotationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnRotationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpawnAnimationFrameData {
    pub animation_frame: u32,
}

pub const SPAWNANIMATIONFRAMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnAnimationFrameData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AnimationFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnAnimationFrameData, animation_frame),
            },
        ],
    }),
    array_type: Some(SPAWNANIMATIONFRAMEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnAnimationFrameData {
    fn type_info() -> &'static TypeInfo {
        SPAWNANIMATIONFRAMEDATA_TYPE_INFO
    }
}


pub const SPAWNANIMATIONFRAMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnAnimationFrameData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnAnimationFrameData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpawnAnimationData {
    pub animation_speed: f32,
    pub based_on_lifetime: bool,
}

pub const SPAWNANIMATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnAnimationData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AnimationSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnAnimationData, animation_speed),
            },
            FieldInfoData {
                name: "BasedOnLifetime",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpawnAnimationData, based_on_lifetime),
            },
        ],
    }),
    array_type: Some(SPAWNANIMATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnAnimationData {
    fn type_info() -> &'static TypeInfo {
        SPAWNANIMATIONDATA_TYPE_INFO
    }
}


pub const SPAWNANIMATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnAnimationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnAnimationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpawnPositionData {
    pub apply_screen_aspect_ratio: bool,
}

pub const SPAWNPOSITIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnPositionData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ApplyScreenAspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpawnPositionData, apply_screen_aspect_ratio),
            },
        ],
    }),
    array_type: Some(SPAWNPOSITIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnPositionData {
    fn type_info() -> &'static TypeInfo {
        SPAWNPOSITIONDATA_TYPE_INFO
    }
}


pub const SPAWNPOSITIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnPositionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnPositionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpawnSizeData {
    pub size: f32,
}

pub const SPAWNSIZEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnSizeData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnSizeData, size),
            },
        ],
    }),
    array_type: Some(SPAWNSIZEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnSizeData {
    fn type_info() -> &'static TypeInfo {
        SPAWNSIZEDATA_TYPE_INFO
    }
}


pub const SPAWNSIZEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnSizeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnSizeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpawnSpeedData {
    pub speed: f32,
}

pub const SPAWNSPEEDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnSpeedData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnSpeedData, speed),
            },
        ],
    }),
    array_type: Some(SPAWNSPEEDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnSpeedData {
    fn type_info() -> &'static TypeInfo {
        SPAWNSPEEDDATA_TYPE_INFO
    }
}


pub const SPAWNSPEEDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnSpeedData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnSpeedData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpawnDirectionData {
    pub inherit_speed_and_direction_from_emitter: bool,
    pub direction_from_emitter_origin: f32,
    pub inherit_speed_amount: f32,
    pub inherit_speed_smoothing_factor: f32,
    pub inherit_speed_randomness: f32,
}

pub const SPAWNDIRECTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnDirectionData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InheritSpeedAndDirectionFromEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpawnDirectionData, inherit_speed_and_direction_from_emitter),
            },
            FieldInfoData {
                name: "DirectionFromEmitterOrigin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnDirectionData, direction_from_emitter_origin),
            },
            FieldInfoData {
                name: "InheritSpeedAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnDirectionData, inherit_speed_amount),
            },
            FieldInfoData {
                name: "InheritSpeedSmoothingFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnDirectionData, inherit_speed_smoothing_factor),
            },
            FieldInfoData {
                name: "InheritSpeedRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnDirectionData, inherit_speed_randomness),
            },
        ],
    }),
    array_type: Some(SPAWNDIRECTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnDirectionData {
    fn type_info() -> &'static TypeInfo {
        SPAWNDIRECTIONDATA_TYPE_INFO
    }
}


pub const SPAWNDIRECTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnDirectionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnDirectionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpawnPointCloudData {
}

pub const SPAWNPOINTCLOUDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnPointCloudData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SPAWNPOINTCLOUDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnPointCloudData {
    fn type_info() -> &'static TypeInfo {
        SPAWNPOINTCLOUDDATA_TYPE_INFO
    }
}


pub const SPAWNPOINTCLOUDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnPointCloudData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnPointCloudData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PreRollData {
    pub pre_roll: f32,
    pub updates_per_second: f32,
    pub skip_simulate_post_pre_roll: bool,
}

pub const PREROLLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreRollData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PreRoll",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PreRollData, pre_roll),
            },
            FieldInfoData {
                name: "UpdatesPerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PreRollData, updates_per_second),
            },
            FieldInfoData {
                name: "SkipSimulatePostPreRoll",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PreRollData, skip_simulate_post_pre_roll),
            },
        ],
    }),
    array_type: Some(PREROLLDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PreRollData {
    fn type_info() -> &'static TypeInfo {
        PREROLLDATA_TYPE_INFO
    }
}


pub const PREROLLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreRollData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PreRollData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpawnRibbonRateData {
    pub spawn_rate: f32,
    pub distribute_over_distance: bool,
    pub angle_deviation: f32,
}

pub const SPAWNRIBBONRATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRibbonRateData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnRibbonRateData, spawn_rate),
            },
            FieldInfoData {
                name: "DistributeOverDistance",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpawnRibbonRateData, distribute_over_distance),
            },
            FieldInfoData {
                name: "AngleDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnRibbonRateData, angle_deviation),
            },
        ],
    }),
    array_type: Some(SPAWNRIBBONRATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnRibbonRateData {
    fn type_info() -> &'static TypeInfo {
        SPAWNRIBBONRATEDATA_TYPE_INFO
    }
}


pub const SPAWNRIBBONRATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRibbonRateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnRibbonRateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpawnRateData {
    pub spawn_rate: f32,
    pub distribute_over_time: bool,
    pub distribute_over_distance: bool,
}

pub const SPAWNRATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRateData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpawnRateData, spawn_rate),
            },
            FieldInfoData {
                name: "DistributeOverTime",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpawnRateData, distribute_over_time),
            },
            FieldInfoData {
                name: "DistributeOverDistance",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpawnRateData, distribute_over_distance),
            },
        ],
    }),
    array_type: Some(SPAWNRATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnRateData {
    fn type_info() -> &'static TypeInfo {
        SPAWNRATEDATA_TYPE_INFO
    }
}


pub const SPAWNRATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnRateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CustomShaderData {
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub animation_frame_count: f32,
    pub animation_frame_column_count: f32,
    pub emitter_draw_order: EmitterDrawOrder,
    pub particle_sorting: ParticleSorting,
}

pub const CUSTOMSHADERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomShaderData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(CustomShaderData, shader),
            },
            FieldInfoData {
                name: "AnimationFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CustomShaderData, animation_frame_count),
            },
            FieldInfoData {
                name: "AnimationFrameColumnCount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CustomShaderData, animation_frame_column_count),
            },
            FieldInfoData {
                name: "EmitterDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERDRAWORDER_TYPE_INFO,
                rust_offset: offset_of!(CustomShaderData, emitter_draw_order),
            },
            FieldInfoData {
                name: "ParticleSorting",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICLESORTING_TYPE_INFO,
                rust_offset: offset_of!(CustomShaderData, particle_sorting),
            },
        ],
    }),
    array_type: Some(CUSTOMSHADERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomShaderData {
    fn type_info() -> &'static TypeInfo {
        CUSTOMSHADERDATA_TYPE_INFO
    }
}


pub const CUSTOMSHADERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomShaderData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("CustomShaderData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BaseEmitterData {
    pub emitter_asset: EmitterDocument,
}

pub const BASEEMITTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseEmitterData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EmitterAsset",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERDOCUMENT_TYPE_INFO,
                rust_offset: offset_of!(BaseEmitterData, emitter_asset),
            },
        ],
    }),
    array_type: Some(BASEEMITTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BaseEmitterData {
    fn type_info() -> &'static TypeInfo {
        BASEEMITTERDATA_TYPE_INFO
    }
}


pub const BASEEMITTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseEmitterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("BaseEmitterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PolynomialXYZWEvaluatorData {
    pub x_coefficients: super::core::Vec4,
    pub y_coefficients: super::core::Vec4,
    pub z_coefficients: super::core::Vec4,
    pub w_coefficients: super::core::Vec4,
    pub scale_value: super::core::Vec4,
    pub min_clamp: super::core::Vec4,
    pub max_clamp: super::core::Vec4,
}

pub const POLYNOMIALXYZWEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialXYZWEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "XCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, x_coefficients),
            },
            FieldInfoData {
                name: "YCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, y_coefficients),
            },
            FieldInfoData {
                name: "ZCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, z_coefficients),
            },
            FieldInfoData {
                name: "WCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, w_coefficients),
            },
            FieldInfoData {
                name: "ScaleValue",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, scale_value),
            },
            FieldInfoData {
                name: "MinClamp",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, min_clamp),
            },
            FieldInfoData {
                name: "MaxClamp",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, max_clamp),
            },
        ],
    }),
    array_type: Some(POLYNOMIALXYZWEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PolynomialXYZWEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        POLYNOMIALXYZWEVALUATORDATA_TYPE_INFO
    }
}


pub const POLYNOMIALXYZWEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialXYZWEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialXYZWEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MultiColorInterpData {
    pub gradient: MultiColorGradient,
}

pub const MULTICOLORINTERPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorInterpData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Gradient",
                flags: MemberInfoFlags::new(0),
                field_type: MULTICOLORGRADIENT_TYPE_INFO,
                rust_offset: offset_of!(MultiColorInterpData, gradient),
            },
        ],
    }),
    array_type: Some(MULTICOLORINTERPDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MultiColorInterpData {
    fn type_info() -> &'static TypeInfo {
        MULTICOLORINTERPDATA_TYPE_INFO
    }
}


pub const MULTICOLORINTERPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorInterpData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MultiColorInterpData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MultiColorGradient {
    pub key_points: Vec<MultiColorGradientKeyPoint>,
}

pub const MULTICOLORGRADIENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorGradient",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "KeyPoints",
                flags: MemberInfoFlags::new(144),
                field_type: MULTICOLORGRADIENTKEYPOINT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MultiColorGradient, key_points),
            },
        ],
    }),
    array_type: Some(MULTICOLORGRADIENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MultiColorGradient {
    fn type_info() -> &'static TypeInfo {
        MULTICOLORGRADIENT_TYPE_INFO
    }
}


pub const MULTICOLORGRADIENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorGradient-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MultiColorGradient-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MultiColorGradientKeyPoint {
    pub color: super::core::Vec3,
}

pub const MULTICOLORGRADIENTKEYPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorGradientKeyPoint",
    flags: MemberInfoFlags::new(36937),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(MultiColorGradientKeyPoint, color),
            },
        ],
    }),
    array_type: Some(MULTICOLORGRADIENTKEYPOINT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MultiColorGradientKeyPoint {
    fn type_info() -> &'static TypeInfo {
        MULTICOLORGRADIENTKEYPOINT_TYPE_INFO
    }
}


pub const MULTICOLORGRADIENTKEYPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorGradientKeyPoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MultiColorGradientKeyPoint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PolynomialColorInterpData {
    pub color0: super::core::Vec3,
    pub color1: super::core::Vec3,
    pub coefficients: super::core::Vec4,
}

pub const POLYNOMIALCOLORINTERPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialColorInterpData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Color0",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PolynomialColorInterpData, color0),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PolynomialColorInterpData, color1),
            },
            FieldInfoData {
                name: "Coefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PolynomialColorInterpData, coefficients),
            },
        ],
    }),
    array_type: Some(POLYNOMIALCOLORINTERPDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PolynomialColorInterpData {
    fn type_info() -> &'static TypeInfo {
        POLYNOMIALCOLORINTERPDATA_TYPE_INFO
    }
}


pub const POLYNOMIALCOLORINTERPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialColorInterpData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialColorInterpData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConstantEvaluatorData {
    pub scale: f32,
}

pub const CONSTANTEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstantEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ConstantEvaluatorData, scale),
            },
        ],
    }),
    array_type: Some(CONSTANTEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConstantEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        CONSTANTEVALUATORDATA_TYPE_INFO
    }
}


pub const CONSTANTEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstantEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ConstantEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CameraProximityEvaluatorData {
    pub size: super::core::Vec3,
    pub offset: super::core::Vec3,
    pub forward_offset: f32,
    pub inner_radius: f32,
    pub inner_radius_direction: super::core::Vec3,
}

pub const CAMERAPROXIMITYEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraProximityEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CameraProximityEvaluatorData, size),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CameraProximityEvaluatorData, offset),
            },
            FieldInfoData {
                name: "ForwardOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraProximityEvaluatorData, forward_offset),
            },
            FieldInfoData {
                name: "InnerRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraProximityEvaluatorData, inner_radius),
            },
            FieldInfoData {
                name: "InnerRadiusDirection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CameraProximityEvaluatorData, inner_radius_direction),
            },
        ],
    }),
    array_type: Some(CAMERAPROXIMITYEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CameraProximityEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        CAMERAPROXIMITYEVALUATORDATA_TYPE_INFO
    }
}


pub const CAMERAPROXIMITYEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraProximityEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("CameraProximityEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SuperSphereEvaluatorData {
    pub scale: super::core::Vec3,
    pub pivot: super::core::Vec3,
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub start_zenith_angle: f32,
    pub end_zenith_angle: f32,
    pub inner_radius_bound: f32,
    pub start_zenith_angle_bound: f32,
    pub end_zenith_angle_bound: f32,
    pub start_azimuth_angle: f32,
    pub end_azimuth_angle: f32,
    pub distribution_along_radius: f32,
    pub orient_along_z: bool,
}

pub const SUPERSPHEREEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SuperSphereEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, scale),
            },
            FieldInfoData {
                name: "Pivot",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, pivot),
            },
            FieldInfoData {
                name: "InnerRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, inner_radius),
            },
            FieldInfoData {
                name: "OuterRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, outer_radius),
            },
            FieldInfoData {
                name: "StartZenithAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, start_zenith_angle),
            },
            FieldInfoData {
                name: "EndZenithAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, end_zenith_angle),
            },
            FieldInfoData {
                name: "InnerRadiusBound",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, inner_radius_bound),
            },
            FieldInfoData {
                name: "StartZenithAngleBound",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, start_zenith_angle_bound),
            },
            FieldInfoData {
                name: "EndZenithAngleBound",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, end_zenith_angle_bound),
            },
            FieldInfoData {
                name: "StartAzimuthAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, start_azimuth_angle),
            },
            FieldInfoData {
                name: "EndAzimuthAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, end_azimuth_angle),
            },
            FieldInfoData {
                name: "DistributionAlongRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, distribution_along_radius),
            },
            FieldInfoData {
                name: "OrientAlongZ",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SuperSphereEvaluatorData, orient_along_z),
            },
        ],
    }),
    array_type: Some(SUPERSPHEREEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SuperSphereEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        SUPERSPHEREEVALUATORDATA_TYPE_INFO
    }
}


pub const SUPERSPHEREEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SuperSphereEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SuperSphereEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SphereEvaluatorData {
    pub scale: super::core::Vec3,
    pub radius: f32,
    pub pivot: super::core::Vec3,
}

pub const SPHEREEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SphereEvaluatorData, scale),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereEvaluatorData, radius),
            },
            FieldInfoData {
                name: "Pivot",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SphereEvaluatorData, pivot),
            },
        ],
    }),
    array_type: Some(SPHEREEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SphereEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        SPHEREEVALUATORDATA_TYPE_INFO
    }
}


pub const SPHEREEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SphereEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BoxEvaluatorData {
    pub dimensions: super::core::Vec3,
    pub pivot: super::core::Vec3,
}

pub const BOXEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Dimensions",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(BoxEvaluatorData, dimensions),
            },
            FieldInfoData {
                name: "Pivot",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(BoxEvaluatorData, pivot),
            },
        ],
    }),
    array_type: Some(BOXEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoxEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        BOXEVALUATORDATA_TYPE_INFO
    }
}


pub const BOXEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("BoxEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RandomXYZWEvaluatorData {
    pub random_frequency: RandomFrequency,
    pub max_x: f32,
    pub min_x: f32,
    pub max_y: f32,
    pub min_y: f32,
    pub max_z: f32,
    pub min_z: f32,
    pub max_w: f32,
    pub min_w: f32,
    pub mirror: bool,
}

pub const RANDOMXYZWEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomXYZWEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RandomFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: RANDOMFREQUENCY_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZWEvaluatorData, random_frequency),
            },
            FieldInfoData {
                name: "MaxX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZWEvaluatorData, max_x),
            },
            FieldInfoData {
                name: "MinX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZWEvaluatorData, min_x),
            },
            FieldInfoData {
                name: "MaxY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZWEvaluatorData, max_y),
            },
            FieldInfoData {
                name: "MinY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZWEvaluatorData, min_y),
            },
            FieldInfoData {
                name: "MaxZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZWEvaluatorData, max_z),
            },
            FieldInfoData {
                name: "MinZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZWEvaluatorData, min_z),
            },
            FieldInfoData {
                name: "MaxW",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZWEvaluatorData, max_w),
            },
            FieldInfoData {
                name: "MinW",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZWEvaluatorData, min_w),
            },
            FieldInfoData {
                name: "Mirror",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZWEvaluatorData, mirror),
            },
        ],
    }),
    array_type: Some(RANDOMXYZWEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomXYZWEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        RANDOMXYZWEVALUATORDATA_TYPE_INFO
    }
}


pub const RANDOMXYZWEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomXYZWEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RandomXYZWEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RandomXYZEvaluatorData {
    pub random_frequency: RandomFrequency,
    pub max_x: f32,
    pub min_x: f32,
    pub max_y: f32,
    pub min_y: f32,
    pub max_z: f32,
    pub min_z: f32,
    pub mirror: bool,
}

pub const RANDOMXYZEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomXYZEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RandomFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: RANDOMFREQUENCY_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZEvaluatorData, random_frequency),
            },
            FieldInfoData {
                name: "MaxX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZEvaluatorData, max_x),
            },
            FieldInfoData {
                name: "MinX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZEvaluatorData, min_x),
            },
            FieldInfoData {
                name: "MaxY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZEvaluatorData, max_y),
            },
            FieldInfoData {
                name: "MinY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZEvaluatorData, min_y),
            },
            FieldInfoData {
                name: "MaxZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZEvaluatorData, max_z),
            },
            FieldInfoData {
                name: "MinZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZEvaluatorData, min_z),
            },
            FieldInfoData {
                name: "Mirror",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomXYZEvaluatorData, mirror),
            },
        ],
    }),
    array_type: Some(RANDOMXYZEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomXYZEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        RANDOMXYZEVALUATORDATA_TYPE_INFO
    }
}


pub const RANDOMXYZEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomXYZEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RandomXYZEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RandomEvaluatorData {
    pub random_frequency: RandomFrequency,
    pub max: f32,
    pub min: f32,
    pub mirror: bool,
}

pub const RANDOMEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RandomFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: RANDOMFREQUENCY_TYPE_INFO,
                rust_offset: offset_of!(RandomEvaluatorData, random_frequency),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomEvaluatorData, max),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomEvaluatorData, min),
            },
            FieldInfoData {
                name: "Mirror",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomEvaluatorData, mirror),
            },
        ],
    }),
    array_type: Some(RANDOMEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        RANDOMEVALUATORDATA_TYPE_INFO
    }
}


pub const RANDOMEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RandomEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RotateVectorData {
    pub angle: f32,
    pub input_affects_phi: bool,
    pub rotate_within_plane: bool,
}

pub const ROTATEVECTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotateVectorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Angle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RotateVectorData, angle),
            },
            FieldInfoData {
                name: "InputAffectsPhi",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RotateVectorData, input_affects_phi),
            },
            FieldInfoData {
                name: "RotateWithinPlane",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RotateVectorData, rotate_within_plane),
            },
        ],
    }),
    array_type: Some(ROTATEVECTORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RotateVectorData {
    fn type_info() -> &'static TypeInfo {
        ROTATEVECTORDATA_TYPE_INFO
    }
}


pub const ROTATEVECTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotateVectorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RotateVectorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SampleTextureData {
    pub gradient_data: Vec<super::core::Vec4>,
    pub color_intensity_max: super::core::Vec3,
    pub color_intensity_min: super::core::Vec3,
    pub texture_dimensions: super::core::Vec2,
    pub texture_origin_u: f32,
    pub texture_origin_v: f32,
}

pub const SAMPLETEXTUREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SampleTextureData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "GradientData",
                flags: MemberInfoFlags::new(144),
                field_type: VEC4_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SampleTextureData, gradient_data),
            },
            FieldInfoData {
                name: "ColorIntensityMax",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SampleTextureData, color_intensity_max),
            },
            FieldInfoData {
                name: "ColorIntensityMin",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SampleTextureData, color_intensity_min),
            },
            FieldInfoData {
                name: "TextureDimensions",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SampleTextureData, texture_dimensions),
            },
            FieldInfoData {
                name: "TextureOriginU",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SampleTextureData, texture_origin_u),
            },
            FieldInfoData {
                name: "TextureOriginV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SampleTextureData, texture_origin_v),
            },
        ],
    }),
    array_type: Some(SAMPLETEXTUREDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SampleTextureData {
    fn type_info() -> &'static TypeInfo {
        SAMPLETEXTUREDATA_TYPE_INFO
    }
}


pub const SAMPLETEXTUREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SampleTextureData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SampleTextureData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SplineData {
    pub spline_curve: super::core::SplineCurve,
}

pub const SPLINEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SplineCurve",
                flags: MemberInfoFlags::new(0),
                field_type: SPLINECURVE_TYPE_INFO,
                rust_offset: offset_of!(SplineData, spline_curve),
            },
        ],
    }),
    array_type: Some(SPLINEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SplineData {
    fn type_info() -> &'static TypeInfo {
        SPLINEDATA_TYPE_INFO
    }
}


pub const SPLINEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SplineData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PolynomialOperatorData {
    pub first_operand: PolynomialTempData,
    pub second_operand: PolynomialTempData,
    pub operation: PolynomialOperation,
    pub min_clamp_result: f32,
    pub max_clamp_result: f32,
}

pub const POLYNOMIALOPERATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialOperatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FirstOperand",
                flags: MemberInfoFlags::new(0),
                field_type: POLYNOMIALTEMPDATA_TYPE_INFO,
                rust_offset: offset_of!(PolynomialOperatorData, first_operand),
            },
            FieldInfoData {
                name: "SecondOperand",
                flags: MemberInfoFlags::new(0),
                field_type: POLYNOMIALTEMPDATA_TYPE_INFO,
                rust_offset: offset_of!(PolynomialOperatorData, second_operand),
            },
            FieldInfoData {
                name: "Operation",
                flags: MemberInfoFlags::new(0),
                field_type: POLYNOMIALOPERATION_TYPE_INFO,
                rust_offset: offset_of!(PolynomialOperatorData, operation),
            },
            FieldInfoData {
                name: "MinClampResult",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PolynomialOperatorData, min_clamp_result),
            },
            FieldInfoData {
                name: "MaxClampResult",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PolynomialOperatorData, max_clamp_result),
            },
        ],
    }),
    array_type: Some(POLYNOMIALOPERATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PolynomialOperatorData {
    fn type_info() -> &'static TypeInfo {
        POLYNOMIALOPERATORDATA_TYPE_INFO
    }
}


pub const POLYNOMIALOPERATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialOperatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialOperatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PolynomialOperation {
    #[default]
    Multiplication = 0,
    Addition = 1,
    Subtraction = 2,
}

pub const POLYNOMIALOPERATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialOperation",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(POLYNOMIALOPERATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PolynomialOperation {
    fn type_info() -> &'static TypeInfo {
        POLYNOMIALOPERATION_TYPE_INFO
    }
}


pub const POLYNOMIALOPERATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialOperation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialOperation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PolynomialTempData {
    pub coefficients: super::core::Vec4,
    pub scale_value: f32,
    pub min_clamp: f32,
    pub max_clamp: f32,
}

pub const POLYNOMIALTEMPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialTempData",
    flags: MemberInfoFlags::new(36937),
    module: "Emitter",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Coefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PolynomialTempData, coefficients),
            },
            FieldInfoData {
                name: "ScaleValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PolynomialTempData, scale_value),
            },
            FieldInfoData {
                name: "MinClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PolynomialTempData, min_clamp),
            },
            FieldInfoData {
                name: "MaxClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PolynomialTempData, max_clamp),
            },
        ],
    }),
    array_type: Some(POLYNOMIALTEMPDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PolynomialTempData {
    fn type_info() -> &'static TypeInfo {
        POLYNOMIALTEMPDATA_TYPE_INFO
    }
}


pub const POLYNOMIALTEMPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialTempData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialTempData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PolynomialData {
    pub coefficients: super::core::Vec4,
    pub scale_value: f32,
    pub min_clamp: f32,
    pub max_clamp: f32,
}

pub const POLYNOMIALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Coefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PolynomialData, coefficients),
            },
            FieldInfoData {
                name: "ScaleValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PolynomialData, scale_value),
            },
            FieldInfoData {
                name: "MinClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PolynomialData, min_clamp),
            },
            FieldInfoData {
                name: "MaxClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PolynomialData, max_clamp),
            },
        ],
    }),
    array_type: Some(POLYNOMIALDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PolynomialData {
    fn type_info() -> &'static TypeInfo {
        POLYNOMIALDATA_TYPE_INFO
    }
}


pub const POLYNOMIALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DefaultEvaluatorData {
    pub values: super::core::Vec4,
}

pub const DEFAULTEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(DefaultEvaluatorData, values),
            },
        ],
    }),
    array_type: Some(DEFAULTEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DefaultEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        DEFAULTEVALUATORDATA_TYPE_INFO
    }
}


pub const DEFAULTEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("DefaultEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


