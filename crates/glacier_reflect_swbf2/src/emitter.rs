use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct MeshEmitterResource {
}

pub trait MeshEmitterResourceTrait: TypeObject {
}

impl MeshEmitterResourceTrait for MeshEmitterResource {
}

pub static MESHEMITTERRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterResource",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshEmitterResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHEMITTERRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshEmitterResource {
    fn type_info(&self) -> &'static TypeInfo {
        MESHEMITTERRESOURCE_TYPE_INFO
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


pub static MESHEMITTERRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MeshEmitterResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshEmitterMaskResource {
}

pub trait MeshEmitterMaskResourceTrait: TypeObject {
}

impl MeshEmitterMaskResourceTrait for MeshEmitterMaskResource {
}

pub static MESHEMITTERMASKRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskResource",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshEmitterMaskResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHEMITTERMASKRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshEmitterMaskResource {
    fn type_info(&self) -> &'static TypeInfo {
        MESHEMITTERMASKRESOURCE_TYPE_INFO
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


pub static MESHEMITTERMASKRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MeshEmitterMaskResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterGraphResource {
}

pub trait EmitterGraphResourceTrait: TypeObject {
}

impl EmitterGraphResourceTrait for EmitterGraphResource {
}

pub static EMITTERGRAPHRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphResource",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterGraphResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EMITTERGRAPHRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EmitterGraphResource {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHRESOURCE_TYPE_INFO
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


pub static EMITTERGRAPHRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterSystemSettings {
    pub _glacier_base: super::core::DataContainer,
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

pub trait EmitterSystemSettingsTrait: super::core::DataContainerTrait {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn update_job_enable(&self) -> &bool;
    fn update_job_enable_mut(&mut self) -> &mut bool;
    fn skip_update_max_count(&self) -> &u32;
    fn skip_update_max_count_mut(&mut self) -> &mut u32;
    fn force_job_count(&self) -> &i32;
    fn force_job_count_mut(&mut self) -> &mut i32;
    fn time_scale(&self) -> &f32;
    fn time_scale_mut(&mut self) -> &mut f32;
    fn global_reset_start_time_interval(&self) -> &f32;
    fn global_reset_start_time_interval_mut(&mut self) -> &mut f32;
    fn enable_fixed_time_step(&self) -> &bool;
    fn enable_fixed_time_step_mut(&mut self) -> &mut bool;
    fn enable_fixed_delta(&self) -> &bool;
    fn enable_fixed_delta_mut(&mut self) -> &mut bool;
    fn enable_jobs(&self) -> &bool;
    fn enable_jobs_mut(&mut self) -> &mut bool;
    fn collision_ray_cast_enable(&self) -> &bool;
    fn collision_ray_cast_enable_mut(&mut self) -> &mut bool;
    fn collision_ray_cast_max_count(&self) -> &u32;
    fn collision_ray_cast_max_count_mut(&mut self) -> &mut u32;
    fn proximity_physics_entities_max_count(&self) -> &u32;
    fn proximity_physics_entities_max_count_mut(&mut self) -> &mut u32;
    fn draw_debug_ray_cast_collision(&self) -> &bool;
    fn draw_debug_ray_cast_collision_mut(&mut self) -> &mut bool;
    fn emitter_quality_level(&self) -> &super::core::QualityLevel;
    fn emitter_quality_level_mut(&mut self) -> &mut super::core::QualityLevel;
    fn template_timeout_time(&self) -> &f32;
    fn template_timeout_time_mut(&mut self) -> &mut f32;
    fn precise_wind_and_force_max_distance(&self) -> &f32;
    fn precise_wind_and_force_max_distance_mut(&mut self) -> &mut f32;
    fn turbulence_max_distance(&self) -> &f32;
    fn turbulence_max_distance_mut(&mut self) -> &mut f32;
    fn screen_area_culling_start(&self) -> &f32;
    fn screen_area_culling_start_mut(&mut self) -> &mut f32;
    fn screen_area_culling_end(&self) -> &f32;
    fn screen_area_culling_end_mut(&mut self) -> &mut f32;
    fn screen_area_culling_min_total_area(&self) -> &f32;
    fn screen_area_culling_min_total_area_mut(&mut self) -> &mut f32;
    fn screen_area_culling_max_total_area(&self) -> &f32;
    fn screen_area_culling_max_total_area_mut(&mut self) -> &mut f32;
    fn screen_area_culling_max_multiplier(&self) -> &f32;
    fn screen_area_culling_max_multiplier_mut(&mut self) -> &mut f32;
    fn process_job_yield_time(&self) -> &f32;
    fn process_job_yield_time_mut(&mut self) -> &mut f32;
    fn visible_job_yield_time(&self) -> &f32;
    fn visible_job_yield_time_mut(&mut self) -> &mut f32;
    fn mesh_emitter_motion_blur_enable(&self) -> &bool;
    fn mesh_emitter_motion_blur_enable_mut(&mut self) -> &mut bool;
    fn enable_rendering(&self) -> &bool;
    fn enable_rendering_mut(&mut self) -> &mut bool;
    fn draw_stats(&self) -> &u32;
    fn draw_stats_mut(&mut self) -> &mut u32;
    fn collect_performance_stats(&self) -> &bool;
    fn collect_performance_stats_mut(&mut self) -> &mut bool;
    fn collect_performance_stats_time(&self) -> &i32;
    fn collect_performance_stats_time_mut(&mut self) -> &mut i32;
    fn draw_mem_stats(&self) -> &u32;
    fn draw_mem_stats_mut(&mut self) -> &mut u32;
    fn draw_stats_sampling_period(&self) -> &f32;
    fn draw_stats_sampling_period_mut(&mut self) -> &mut f32;
    fn draw_stats_entries_per_page(&self) -> &u32;
    fn draw_stats_entries_per_page_mut(&mut self) -> &mut u32;
    fn draw_stats_page_index(&self) -> &u32;
    fn draw_stats_page_index_mut(&mut self) -> &mut u32;
    fn draw_stats_filter(&self) -> &String;
    fn draw_stats_filter_mut(&mut self) -> &mut String;
    fn hide_inactive_stats(&self) -> &bool;
    fn hide_inactive_stats_mut(&mut self) -> &mut bool;
    fn save_list_active_emitters(&self) -> &bool;
    fn save_list_active_emitters_mut(&mut self) -> &mut bool;
    fn draw_emitter_name(&self) -> &bool;
    fn draw_emitter_name_mut(&mut self) -> &mut bool;
    fn z_buffer_cull_enable(&self) -> &bool;
    fn z_buffer_cull_enable_mut(&mut self) -> &mut bool;
    fn draw_projected_boxes(&self) -> &bool;
    fn draw_projected_boxes_mut(&mut self) -> &mut bool;
    fn draw_bounding_boxes(&self) -> &u32;
    fn draw_bounding_boxes_mut(&mut self) -> &mut u32;
    fn min_screen_area(&self) -> &f32;
    fn min_screen_area_mut(&mut self) -> &mut f32;
    fn min_screen_area_threshold(&self) -> &f32;
    fn min_screen_area_threshold_mut(&mut self) -> &mut f32;
    fn force_cull_distance(&self) -> &f32;
    fn force_cull_distance_mut(&mut self) -> &mut f32;
    fn force_cull_fade_far_distance(&self) -> &f32;
    fn force_cull_fade_far_distance_mut(&mut self) -> &mut f32;
    fn draw_transforms(&self) -> &bool;
    fn draw_transforms_mut(&mut self) -> &mut bool;
    fn draw_light_probe_sample_transforms(&self) -> &bool;
    fn draw_light_probe_sample_transforms_mut(&mut self) -> &mut bool;
    fn draw_debug_base_atlas(&self) -> &bool;
    fn draw_debug_base_atlas_mut(&mut self) -> &mut bool;
    fn draw_debug_normal_atlas(&self) -> &bool;
    fn draw_debug_normal_atlas_mut(&mut self) -> &mut bool;
    fn draw_debug_atlas_miplevel(&self) -> &u32;
    fn draw_debug_atlas_miplevel_mut(&mut self) -> &mut u32;
    fn draw_debug_atlas_texture_index(&self) -> &i32;
    fn draw_debug_atlas_texture_index_mut(&mut self) -> &mut i32;
    fn draw_debug_atlas_alpha(&self) -> &bool;
    fn draw_debug_atlas_alpha_mut(&mut self) -> &mut bool;
    fn draw_debug_emitter_exclusion_volumes(&self) -> &i32;
    fn draw_debug_emitter_exclusion_volumes_mut(&mut self) -> &mut i32;
    fn draw_debug_atlas_page_index(&self) -> &i32;
    fn draw_debug_atlas_page_index_mut(&mut self) -> &mut i32;
    fn draw_debug_emitter_sun_transmittance_map_group(&self) -> &bool;
    fn draw_debug_emitter_sun_transmittance_map_group_mut(&mut self) -> &mut bool;
    fn draw_debug_emitter_sun_transmittance_maps_links(&self) -> &bool;
    fn draw_debug_emitter_sun_transmittance_maps_links_mut(&mut self) -> &mut bool;
    fn force_sun_transmittance_on_all_emitters(&self) -> &bool;
    fn force_sun_transmittance_on_all_emitters_mut(&mut self) -> &mut bool;
    fn emitter_render_sun_transmittance_views_first(&self) -> &bool;
    fn emitter_render_sun_transmittance_views_first_mut(&mut self) -> &mut bool;
    fn draw_debug_emitter_vertex_buffer_usage(&self) -> &bool;
    fn draw_debug_emitter_vertex_buffer_usage_mut(&mut self) -> &mut bool;
    fn emitter_gpu_lighting_enable(&self) -> &bool;
    fn emitter_gpu_lighting_enable_mut(&mut self) -> &mut bool;
    fn walrus_lighting_enable(&self) -> &bool;
    fn walrus_lighting_enable_mut(&mut self) -> &mut bool;
    fn emitter_gpu_lighting_pipeline_shaders_enabled(&self) -> &bool;
    fn emitter_gpu_lighting_pipeline_shaders_enabled_mut(&mut self) -> &mut bool;
    fn system_shaders_path(&self) -> &String;
    fn system_shaders_path_mut(&mut self) -> &mut String;
    fn system_v_s_f_path(&self) -> &String;
    fn system_v_s_f_path_mut(&mut self) -> &mut String;
    fn crossfire_driver_profile_available(&self) -> &bool;
    fn crossfire_driver_profile_available_mut(&mut self) -> &mut bool;
    fn quad_clip_scale_enable(&self) -> &bool;
    fn quad_clip_scale_enable_mut(&mut self) -> &mut bool;
    fn quad_enable_rendering(&self) -> &bool;
    fn quad_enable_rendering_mut(&mut self) -> &mut bool;
    fn quad_nice_rendering_enable(&self) -> &bool;
    fn quad_nice_rendering_enable_mut(&mut self) -> &mut bool;
    fn quad_simple_rendering_enable(&self) -> &bool;
    fn quad_simple_rendering_enable_mut(&mut self) -> &mut bool;
    fn quad_enable_opaque(&self) -> &bool;
    fn quad_enable_opaque_mut(&mut self) -> &mut bool;
    fn quad_enable_custom_shader(&self) -> &bool;
    fn quad_enable_custom_shader_mut(&mut self) -> &mut bool;
    fn quad_color_shader_costs_enable(&self) -> &bool;
    fn quad_color_shader_costs_enable_mut(&mut self) -> &mut bool;
    fn quad_enable_sorting(&self) -> &bool;
    fn quad_enable_sorting_mut(&mut self) -> &mut bool;
    fn quad_enable_wireframe(&self) -> &bool;
    fn quad_enable_wireframe_mut(&mut self) -> &mut bool;
    fn quad_half_res_enable(&self) -> &bool;
    fn quad_half_res_enable_mut(&mut self) -> &mut bool;
    fn quad_groups_join_all(&self) -> &bool;
    fn quad_groups_join_all_mut(&mut self) -> &mut bool;
    fn quad_groups_join_none(&self) -> &bool;
    fn quad_groups_join_none_mut(&mut self) -> &mut bool;
    fn quad_groups_join_nice_and_simple(&self) -> &bool;
    fn quad_groups_join_nice_and_simple_mut(&mut self) -> &mut bool;
    fn quad_technique(&self) -> &i32;
    fn quad_technique_mut(&mut self) -> &mut i32;
    fn quad_vertex_shadows_enable(&self) -> &bool;
    fn quad_vertex_shadows_enable_mut(&mut self) -> &mut bool;
    fn quad_cloud_vertex_shadows_enable(&self) -> &bool;
    fn quad_cloud_vertex_shadows_enable_mut(&mut self) -> &mut bool;
    fn quad_planar_reflection_enable(&self) -> &bool;
    fn quad_planar_reflection_enable_mut(&mut self) -> &mut bool;
    fn quad_point_lights_enable(&self) -> &bool;
    fn quad_point_lights_enable_mut(&mut self) -> &mut bool;
    fn quad_spot_lights_enable(&self) -> &bool;
    fn quad_spot_lights_enable_mut(&mut self) -> &mut bool;
    fn punctual_light_threshold_squared(&self) -> &f32;
    fn punctual_light_threshold_squared_mut(&mut self) -> &mut f32;
    fn quad_near_fade_distance(&self) -> &f32;
    fn quad_near_fade_distance_mut(&mut self) -> &mut f32;
    fn custom_emitter_position_sorting(&self) -> &bool;
    fn custom_emitter_position_sorting_mut(&mut self) -> &mut bool;
    fn quad_max_count(&self) -> &u32;
    fn quad_max_count_mut(&mut self) -> &mut u32;
    fn mesh_rendering_enable(&self) -> &bool;
    fn mesh_rendering_enable_mut(&mut self) -> &mut bool;
    fn mesh_draw_transforms(&self) -> &bool;
    fn mesh_draw_transforms_mut(&mut self) -> &mut bool;
    fn mesh_draw_bounding_boxes(&self) -> &bool;
    fn mesh_draw_bounding_boxes_mut(&mut self) -> &mut bool;
    fn mesh_shadow_enable(&self) -> &bool;
    fn mesh_shadow_enable_mut(&mut self) -> &mut bool;
    fn mesh_planar_reflection_enable(&self) -> &bool;
    fn mesh_planar_reflection_enable_mut(&mut self) -> &mut bool;
    fn mesh_culling_distance(&self) -> &f32;
    fn mesh_culling_distance_mut(&mut self) -> &mut f32;
    fn mesh_draw_count_limit(&self) -> &u32;
    fn mesh_draw_count_limit_mut(&mut self) -> &mut u32;
    fn mesh_streaming_priority_multiplier(&self) -> &f32;
    fn mesh_streaming_priority_multiplier_mut(&mut self) -> &mut f32;
    fn mesh_draw_cull_stats(&self) -> &bool;
    fn mesh_draw_cull_stats_mut(&mut self) -> &mut bool;
    fn mesh_max_count(&self) -> &u32;
    fn mesh_max_count_mut(&mut self) -> &mut u32;
    fn skip_render_if_probe_is_uninitialized(&self) -> &bool;
    fn skip_render_if_probe_is_uninitialized_mut(&mut self) -> &mut bool;
    fn batch_update_light_probes_enable(&self) -> &bool;
    fn batch_update_light_probes_enable_mut(&mut self) -> &mut bool;
    fn quad_light_probe_max_update_count(&self) -> &u32;
    fn quad_light_probe_max_update_count_mut(&mut self) -> &mut u32;
    fn graph_light_probe_max_update_count(&self) -> &u32;
    fn graph_light_probe_max_update_count_mut(&mut self) -> &mut u32;
    fn mesh_light_probe_max_update_count(&self) -> &u32;
    fn mesh_light_probe_max_update_count_mut(&mut self) -> &mut u32;
    fn graph_emitter_enabled(&self) -> &bool;
    fn graph_emitter_enabled_mut(&mut self) -> &mut bool;
    fn graph_emitter_draw_debug_stats(&self) -> &bool;
    fn graph_emitter_draw_debug_stats_mut(&mut self) -> &mut bool;
    fn graph_emitter_draw_debug_buffers(&self) -> &bool;
    fn graph_emitter_draw_debug_buffers_mut(&mut self) -> &mut bool;
    fn graph_emitter_draw_debug_view_visible_instances(&self) -> &bool;
    fn graph_emitter_draw_debug_view_visible_instances_mut(&mut self) -> &mut bool;
    fn graph_emitter_overlapped_compute_enable(&self) -> &bool;
    fn graph_emitter_overlapped_compute_enable_mut(&mut self) -> &mut bool;
    fn emitter_graph_block_allocator_max_byte_count(&self) -> &u32;
    fn emitter_graph_block_allocator_max_byte_count_mut(&mut self) -> &mut u32;
    fn emitter_graph_block_allocator_block_max_count(&self) -> &u32;
    fn emitter_graph_block_allocator_block_max_count_mut(&mut self) -> &mut u32;
    fn emitter_graph_max_defrag_operations_per_frame(&self) -> &u32;
    fn emitter_graph_max_defrag_operations_per_frame_mut(&mut self) -> &mut u32;
    fn emitter_graph_draw_debug_uber_buffer(&self) -> &bool;
    fn emitter_graph_draw_debug_uber_buffer_mut(&mut self) -> &mut bool;
    fn emitter_graph_uber_buffer_defrag_enable(&self) -> &bool;
    fn emitter_graph_uber_buffer_defrag_enable_mut(&mut self) -> &mut bool;
}

impl EmitterSystemSettingsTrait for EmitterSystemSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn update_job_enable(&self) -> &bool {
        &self.update_job_enable
    }
    fn update_job_enable_mut(&mut self) -> &mut bool {
        &mut self.update_job_enable
    }
    fn skip_update_max_count(&self) -> &u32 {
        &self.skip_update_max_count
    }
    fn skip_update_max_count_mut(&mut self) -> &mut u32 {
        &mut self.skip_update_max_count
    }
    fn force_job_count(&self) -> &i32 {
        &self.force_job_count
    }
    fn force_job_count_mut(&mut self) -> &mut i32 {
        &mut self.force_job_count
    }
    fn time_scale(&self) -> &f32 {
        &self.time_scale
    }
    fn time_scale_mut(&mut self) -> &mut f32 {
        &mut self.time_scale
    }
    fn global_reset_start_time_interval(&self) -> &f32 {
        &self.global_reset_start_time_interval
    }
    fn global_reset_start_time_interval_mut(&mut self) -> &mut f32 {
        &mut self.global_reset_start_time_interval
    }
    fn enable_fixed_time_step(&self) -> &bool {
        &self.enable_fixed_time_step
    }
    fn enable_fixed_time_step_mut(&mut self) -> &mut bool {
        &mut self.enable_fixed_time_step
    }
    fn enable_fixed_delta(&self) -> &bool {
        &self.enable_fixed_delta
    }
    fn enable_fixed_delta_mut(&mut self) -> &mut bool {
        &mut self.enable_fixed_delta
    }
    fn enable_jobs(&self) -> &bool {
        &self.enable_jobs
    }
    fn enable_jobs_mut(&mut self) -> &mut bool {
        &mut self.enable_jobs
    }
    fn collision_ray_cast_enable(&self) -> &bool {
        &self.collision_ray_cast_enable
    }
    fn collision_ray_cast_enable_mut(&mut self) -> &mut bool {
        &mut self.collision_ray_cast_enable
    }
    fn collision_ray_cast_max_count(&self) -> &u32 {
        &self.collision_ray_cast_max_count
    }
    fn collision_ray_cast_max_count_mut(&mut self) -> &mut u32 {
        &mut self.collision_ray_cast_max_count
    }
    fn proximity_physics_entities_max_count(&self) -> &u32 {
        &self.proximity_physics_entities_max_count
    }
    fn proximity_physics_entities_max_count_mut(&mut self) -> &mut u32 {
        &mut self.proximity_physics_entities_max_count
    }
    fn draw_debug_ray_cast_collision(&self) -> &bool {
        &self.draw_debug_ray_cast_collision
    }
    fn draw_debug_ray_cast_collision_mut(&mut self) -> &mut bool {
        &mut self.draw_debug_ray_cast_collision
    }
    fn emitter_quality_level(&self) -> &super::core::QualityLevel {
        &self.emitter_quality_level
    }
    fn emitter_quality_level_mut(&mut self) -> &mut super::core::QualityLevel {
        &mut self.emitter_quality_level
    }
    fn template_timeout_time(&self) -> &f32 {
        &self.template_timeout_time
    }
    fn template_timeout_time_mut(&mut self) -> &mut f32 {
        &mut self.template_timeout_time
    }
    fn precise_wind_and_force_max_distance(&self) -> &f32 {
        &self.precise_wind_and_force_max_distance
    }
    fn precise_wind_and_force_max_distance_mut(&mut self) -> &mut f32 {
        &mut self.precise_wind_and_force_max_distance
    }
    fn turbulence_max_distance(&self) -> &f32 {
        &self.turbulence_max_distance
    }
    fn turbulence_max_distance_mut(&mut self) -> &mut f32 {
        &mut self.turbulence_max_distance
    }
    fn screen_area_culling_start(&self) -> &f32 {
        &self.screen_area_culling_start
    }
    fn screen_area_culling_start_mut(&mut self) -> &mut f32 {
        &mut self.screen_area_culling_start
    }
    fn screen_area_culling_end(&self) -> &f32 {
        &self.screen_area_culling_end
    }
    fn screen_area_culling_end_mut(&mut self) -> &mut f32 {
        &mut self.screen_area_culling_end
    }
    fn screen_area_culling_min_total_area(&self) -> &f32 {
        &self.screen_area_culling_min_total_area
    }
    fn screen_area_culling_min_total_area_mut(&mut self) -> &mut f32 {
        &mut self.screen_area_culling_min_total_area
    }
    fn screen_area_culling_max_total_area(&self) -> &f32 {
        &self.screen_area_culling_max_total_area
    }
    fn screen_area_culling_max_total_area_mut(&mut self) -> &mut f32 {
        &mut self.screen_area_culling_max_total_area
    }
    fn screen_area_culling_max_multiplier(&self) -> &f32 {
        &self.screen_area_culling_max_multiplier
    }
    fn screen_area_culling_max_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.screen_area_culling_max_multiplier
    }
    fn process_job_yield_time(&self) -> &f32 {
        &self.process_job_yield_time
    }
    fn process_job_yield_time_mut(&mut self) -> &mut f32 {
        &mut self.process_job_yield_time
    }
    fn visible_job_yield_time(&self) -> &f32 {
        &self.visible_job_yield_time
    }
    fn visible_job_yield_time_mut(&mut self) -> &mut f32 {
        &mut self.visible_job_yield_time
    }
    fn mesh_emitter_motion_blur_enable(&self) -> &bool {
        &self.mesh_emitter_motion_blur_enable
    }
    fn mesh_emitter_motion_blur_enable_mut(&mut self) -> &mut bool {
        &mut self.mesh_emitter_motion_blur_enable
    }
    fn enable_rendering(&self) -> &bool {
        &self.enable_rendering
    }
    fn enable_rendering_mut(&mut self) -> &mut bool {
        &mut self.enable_rendering
    }
    fn draw_stats(&self) -> &u32 {
        &self.draw_stats
    }
    fn draw_stats_mut(&mut self) -> &mut u32 {
        &mut self.draw_stats
    }
    fn collect_performance_stats(&self) -> &bool {
        &self.collect_performance_stats
    }
    fn collect_performance_stats_mut(&mut self) -> &mut bool {
        &mut self.collect_performance_stats
    }
    fn collect_performance_stats_time(&self) -> &i32 {
        &self.collect_performance_stats_time
    }
    fn collect_performance_stats_time_mut(&mut self) -> &mut i32 {
        &mut self.collect_performance_stats_time
    }
    fn draw_mem_stats(&self) -> &u32 {
        &self.draw_mem_stats
    }
    fn draw_mem_stats_mut(&mut self) -> &mut u32 {
        &mut self.draw_mem_stats
    }
    fn draw_stats_sampling_period(&self) -> &f32 {
        &self.draw_stats_sampling_period
    }
    fn draw_stats_sampling_period_mut(&mut self) -> &mut f32 {
        &mut self.draw_stats_sampling_period
    }
    fn draw_stats_entries_per_page(&self) -> &u32 {
        &self.draw_stats_entries_per_page
    }
    fn draw_stats_entries_per_page_mut(&mut self) -> &mut u32 {
        &mut self.draw_stats_entries_per_page
    }
    fn draw_stats_page_index(&self) -> &u32 {
        &self.draw_stats_page_index
    }
    fn draw_stats_page_index_mut(&mut self) -> &mut u32 {
        &mut self.draw_stats_page_index
    }
    fn draw_stats_filter(&self) -> &String {
        &self.draw_stats_filter
    }
    fn draw_stats_filter_mut(&mut self) -> &mut String {
        &mut self.draw_stats_filter
    }
    fn hide_inactive_stats(&self) -> &bool {
        &self.hide_inactive_stats
    }
    fn hide_inactive_stats_mut(&mut self) -> &mut bool {
        &mut self.hide_inactive_stats
    }
    fn save_list_active_emitters(&self) -> &bool {
        &self.save_list_active_emitters
    }
    fn save_list_active_emitters_mut(&mut self) -> &mut bool {
        &mut self.save_list_active_emitters
    }
    fn draw_emitter_name(&self) -> &bool {
        &self.draw_emitter_name
    }
    fn draw_emitter_name_mut(&mut self) -> &mut bool {
        &mut self.draw_emitter_name
    }
    fn z_buffer_cull_enable(&self) -> &bool {
        &self.z_buffer_cull_enable
    }
    fn z_buffer_cull_enable_mut(&mut self) -> &mut bool {
        &mut self.z_buffer_cull_enable
    }
    fn draw_projected_boxes(&self) -> &bool {
        &self.draw_projected_boxes
    }
    fn draw_projected_boxes_mut(&mut self) -> &mut bool {
        &mut self.draw_projected_boxes
    }
    fn draw_bounding_boxes(&self) -> &u32 {
        &self.draw_bounding_boxes
    }
    fn draw_bounding_boxes_mut(&mut self) -> &mut u32 {
        &mut self.draw_bounding_boxes
    }
    fn min_screen_area(&self) -> &f32 {
        &self.min_screen_area
    }
    fn min_screen_area_mut(&mut self) -> &mut f32 {
        &mut self.min_screen_area
    }
    fn min_screen_area_threshold(&self) -> &f32 {
        &self.min_screen_area_threshold
    }
    fn min_screen_area_threshold_mut(&mut self) -> &mut f32 {
        &mut self.min_screen_area_threshold
    }
    fn force_cull_distance(&self) -> &f32 {
        &self.force_cull_distance
    }
    fn force_cull_distance_mut(&mut self) -> &mut f32 {
        &mut self.force_cull_distance
    }
    fn force_cull_fade_far_distance(&self) -> &f32 {
        &self.force_cull_fade_far_distance
    }
    fn force_cull_fade_far_distance_mut(&mut self) -> &mut f32 {
        &mut self.force_cull_fade_far_distance
    }
    fn draw_transforms(&self) -> &bool {
        &self.draw_transforms
    }
    fn draw_transforms_mut(&mut self) -> &mut bool {
        &mut self.draw_transforms
    }
    fn draw_light_probe_sample_transforms(&self) -> &bool {
        &self.draw_light_probe_sample_transforms
    }
    fn draw_light_probe_sample_transforms_mut(&mut self) -> &mut bool {
        &mut self.draw_light_probe_sample_transforms
    }
    fn draw_debug_base_atlas(&self) -> &bool {
        &self.draw_debug_base_atlas
    }
    fn draw_debug_base_atlas_mut(&mut self) -> &mut bool {
        &mut self.draw_debug_base_atlas
    }
    fn draw_debug_normal_atlas(&self) -> &bool {
        &self.draw_debug_normal_atlas
    }
    fn draw_debug_normal_atlas_mut(&mut self) -> &mut bool {
        &mut self.draw_debug_normal_atlas
    }
    fn draw_debug_atlas_miplevel(&self) -> &u32 {
        &self.draw_debug_atlas_miplevel
    }
    fn draw_debug_atlas_miplevel_mut(&mut self) -> &mut u32 {
        &mut self.draw_debug_atlas_miplevel
    }
    fn draw_debug_atlas_texture_index(&self) -> &i32 {
        &self.draw_debug_atlas_texture_index
    }
    fn draw_debug_atlas_texture_index_mut(&mut self) -> &mut i32 {
        &mut self.draw_debug_atlas_texture_index
    }
    fn draw_debug_atlas_alpha(&self) -> &bool {
        &self.draw_debug_atlas_alpha
    }
    fn draw_debug_atlas_alpha_mut(&mut self) -> &mut bool {
        &mut self.draw_debug_atlas_alpha
    }
    fn draw_debug_emitter_exclusion_volumes(&self) -> &i32 {
        &self.draw_debug_emitter_exclusion_volumes
    }
    fn draw_debug_emitter_exclusion_volumes_mut(&mut self) -> &mut i32 {
        &mut self.draw_debug_emitter_exclusion_volumes
    }
    fn draw_debug_atlas_page_index(&self) -> &i32 {
        &self.draw_debug_atlas_page_index
    }
    fn draw_debug_atlas_page_index_mut(&mut self) -> &mut i32 {
        &mut self.draw_debug_atlas_page_index
    }
    fn draw_debug_emitter_sun_transmittance_map_group(&self) -> &bool {
        &self.draw_debug_emitter_sun_transmittance_map_group
    }
    fn draw_debug_emitter_sun_transmittance_map_group_mut(&mut self) -> &mut bool {
        &mut self.draw_debug_emitter_sun_transmittance_map_group
    }
    fn draw_debug_emitter_sun_transmittance_maps_links(&self) -> &bool {
        &self.draw_debug_emitter_sun_transmittance_maps_links
    }
    fn draw_debug_emitter_sun_transmittance_maps_links_mut(&mut self) -> &mut bool {
        &mut self.draw_debug_emitter_sun_transmittance_maps_links
    }
    fn force_sun_transmittance_on_all_emitters(&self) -> &bool {
        &self.force_sun_transmittance_on_all_emitters
    }
    fn force_sun_transmittance_on_all_emitters_mut(&mut self) -> &mut bool {
        &mut self.force_sun_transmittance_on_all_emitters
    }
    fn emitter_render_sun_transmittance_views_first(&self) -> &bool {
        &self.emitter_render_sun_transmittance_views_first
    }
    fn emitter_render_sun_transmittance_views_first_mut(&mut self) -> &mut bool {
        &mut self.emitter_render_sun_transmittance_views_first
    }
    fn draw_debug_emitter_vertex_buffer_usage(&self) -> &bool {
        &self.draw_debug_emitter_vertex_buffer_usage
    }
    fn draw_debug_emitter_vertex_buffer_usage_mut(&mut self) -> &mut bool {
        &mut self.draw_debug_emitter_vertex_buffer_usage
    }
    fn emitter_gpu_lighting_enable(&self) -> &bool {
        &self.emitter_gpu_lighting_enable
    }
    fn emitter_gpu_lighting_enable_mut(&mut self) -> &mut bool {
        &mut self.emitter_gpu_lighting_enable
    }
    fn walrus_lighting_enable(&self) -> &bool {
        &self.walrus_lighting_enable
    }
    fn walrus_lighting_enable_mut(&mut self) -> &mut bool {
        &mut self.walrus_lighting_enable
    }
    fn emitter_gpu_lighting_pipeline_shaders_enabled(&self) -> &bool {
        &self.emitter_gpu_lighting_pipeline_shaders_enabled
    }
    fn emitter_gpu_lighting_pipeline_shaders_enabled_mut(&mut self) -> &mut bool {
        &mut self.emitter_gpu_lighting_pipeline_shaders_enabled
    }
    fn system_shaders_path(&self) -> &String {
        &self.system_shaders_path
    }
    fn system_shaders_path_mut(&mut self) -> &mut String {
        &mut self.system_shaders_path
    }
    fn system_v_s_f_path(&self) -> &String {
        &self.system_v_s_f_path
    }
    fn system_v_s_f_path_mut(&mut self) -> &mut String {
        &mut self.system_v_s_f_path
    }
    fn crossfire_driver_profile_available(&self) -> &bool {
        &self.crossfire_driver_profile_available
    }
    fn crossfire_driver_profile_available_mut(&mut self) -> &mut bool {
        &mut self.crossfire_driver_profile_available
    }
    fn quad_clip_scale_enable(&self) -> &bool {
        &self.quad_clip_scale_enable
    }
    fn quad_clip_scale_enable_mut(&mut self) -> &mut bool {
        &mut self.quad_clip_scale_enable
    }
    fn quad_enable_rendering(&self) -> &bool {
        &self.quad_enable_rendering
    }
    fn quad_enable_rendering_mut(&mut self) -> &mut bool {
        &mut self.quad_enable_rendering
    }
    fn quad_nice_rendering_enable(&self) -> &bool {
        &self.quad_nice_rendering_enable
    }
    fn quad_nice_rendering_enable_mut(&mut self) -> &mut bool {
        &mut self.quad_nice_rendering_enable
    }
    fn quad_simple_rendering_enable(&self) -> &bool {
        &self.quad_simple_rendering_enable
    }
    fn quad_simple_rendering_enable_mut(&mut self) -> &mut bool {
        &mut self.quad_simple_rendering_enable
    }
    fn quad_enable_opaque(&self) -> &bool {
        &self.quad_enable_opaque
    }
    fn quad_enable_opaque_mut(&mut self) -> &mut bool {
        &mut self.quad_enable_opaque
    }
    fn quad_enable_custom_shader(&self) -> &bool {
        &self.quad_enable_custom_shader
    }
    fn quad_enable_custom_shader_mut(&mut self) -> &mut bool {
        &mut self.quad_enable_custom_shader
    }
    fn quad_color_shader_costs_enable(&self) -> &bool {
        &self.quad_color_shader_costs_enable
    }
    fn quad_color_shader_costs_enable_mut(&mut self) -> &mut bool {
        &mut self.quad_color_shader_costs_enable
    }
    fn quad_enable_sorting(&self) -> &bool {
        &self.quad_enable_sorting
    }
    fn quad_enable_sorting_mut(&mut self) -> &mut bool {
        &mut self.quad_enable_sorting
    }
    fn quad_enable_wireframe(&self) -> &bool {
        &self.quad_enable_wireframe
    }
    fn quad_enable_wireframe_mut(&mut self) -> &mut bool {
        &mut self.quad_enable_wireframe
    }
    fn quad_half_res_enable(&self) -> &bool {
        &self.quad_half_res_enable
    }
    fn quad_half_res_enable_mut(&mut self) -> &mut bool {
        &mut self.quad_half_res_enable
    }
    fn quad_groups_join_all(&self) -> &bool {
        &self.quad_groups_join_all
    }
    fn quad_groups_join_all_mut(&mut self) -> &mut bool {
        &mut self.quad_groups_join_all
    }
    fn quad_groups_join_none(&self) -> &bool {
        &self.quad_groups_join_none
    }
    fn quad_groups_join_none_mut(&mut self) -> &mut bool {
        &mut self.quad_groups_join_none
    }
    fn quad_groups_join_nice_and_simple(&self) -> &bool {
        &self.quad_groups_join_nice_and_simple
    }
    fn quad_groups_join_nice_and_simple_mut(&mut self) -> &mut bool {
        &mut self.quad_groups_join_nice_and_simple
    }
    fn quad_technique(&self) -> &i32 {
        &self.quad_technique
    }
    fn quad_technique_mut(&mut self) -> &mut i32 {
        &mut self.quad_technique
    }
    fn quad_vertex_shadows_enable(&self) -> &bool {
        &self.quad_vertex_shadows_enable
    }
    fn quad_vertex_shadows_enable_mut(&mut self) -> &mut bool {
        &mut self.quad_vertex_shadows_enable
    }
    fn quad_cloud_vertex_shadows_enable(&self) -> &bool {
        &self.quad_cloud_vertex_shadows_enable
    }
    fn quad_cloud_vertex_shadows_enable_mut(&mut self) -> &mut bool {
        &mut self.quad_cloud_vertex_shadows_enable
    }
    fn quad_planar_reflection_enable(&self) -> &bool {
        &self.quad_planar_reflection_enable
    }
    fn quad_planar_reflection_enable_mut(&mut self) -> &mut bool {
        &mut self.quad_planar_reflection_enable
    }
    fn quad_point_lights_enable(&self) -> &bool {
        &self.quad_point_lights_enable
    }
    fn quad_point_lights_enable_mut(&mut self) -> &mut bool {
        &mut self.quad_point_lights_enable
    }
    fn quad_spot_lights_enable(&self) -> &bool {
        &self.quad_spot_lights_enable
    }
    fn quad_spot_lights_enable_mut(&mut self) -> &mut bool {
        &mut self.quad_spot_lights_enable
    }
    fn punctual_light_threshold_squared(&self) -> &f32 {
        &self.punctual_light_threshold_squared
    }
    fn punctual_light_threshold_squared_mut(&mut self) -> &mut f32 {
        &mut self.punctual_light_threshold_squared
    }
    fn quad_near_fade_distance(&self) -> &f32 {
        &self.quad_near_fade_distance
    }
    fn quad_near_fade_distance_mut(&mut self) -> &mut f32 {
        &mut self.quad_near_fade_distance
    }
    fn custom_emitter_position_sorting(&self) -> &bool {
        &self.custom_emitter_position_sorting
    }
    fn custom_emitter_position_sorting_mut(&mut self) -> &mut bool {
        &mut self.custom_emitter_position_sorting
    }
    fn quad_max_count(&self) -> &u32 {
        &self.quad_max_count
    }
    fn quad_max_count_mut(&mut self) -> &mut u32 {
        &mut self.quad_max_count
    }
    fn mesh_rendering_enable(&self) -> &bool {
        &self.mesh_rendering_enable
    }
    fn mesh_rendering_enable_mut(&mut self) -> &mut bool {
        &mut self.mesh_rendering_enable
    }
    fn mesh_draw_transforms(&self) -> &bool {
        &self.mesh_draw_transforms
    }
    fn mesh_draw_transforms_mut(&mut self) -> &mut bool {
        &mut self.mesh_draw_transforms
    }
    fn mesh_draw_bounding_boxes(&self) -> &bool {
        &self.mesh_draw_bounding_boxes
    }
    fn mesh_draw_bounding_boxes_mut(&mut self) -> &mut bool {
        &mut self.mesh_draw_bounding_boxes
    }
    fn mesh_shadow_enable(&self) -> &bool {
        &self.mesh_shadow_enable
    }
    fn mesh_shadow_enable_mut(&mut self) -> &mut bool {
        &mut self.mesh_shadow_enable
    }
    fn mesh_planar_reflection_enable(&self) -> &bool {
        &self.mesh_planar_reflection_enable
    }
    fn mesh_planar_reflection_enable_mut(&mut self) -> &mut bool {
        &mut self.mesh_planar_reflection_enable
    }
    fn mesh_culling_distance(&self) -> &f32 {
        &self.mesh_culling_distance
    }
    fn mesh_culling_distance_mut(&mut self) -> &mut f32 {
        &mut self.mesh_culling_distance
    }
    fn mesh_draw_count_limit(&self) -> &u32 {
        &self.mesh_draw_count_limit
    }
    fn mesh_draw_count_limit_mut(&mut self) -> &mut u32 {
        &mut self.mesh_draw_count_limit
    }
    fn mesh_streaming_priority_multiplier(&self) -> &f32 {
        &self.mesh_streaming_priority_multiplier
    }
    fn mesh_streaming_priority_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.mesh_streaming_priority_multiplier
    }
    fn mesh_draw_cull_stats(&self) -> &bool {
        &self.mesh_draw_cull_stats
    }
    fn mesh_draw_cull_stats_mut(&mut self) -> &mut bool {
        &mut self.mesh_draw_cull_stats
    }
    fn mesh_max_count(&self) -> &u32 {
        &self.mesh_max_count
    }
    fn mesh_max_count_mut(&mut self) -> &mut u32 {
        &mut self.mesh_max_count
    }
    fn skip_render_if_probe_is_uninitialized(&self) -> &bool {
        &self.skip_render_if_probe_is_uninitialized
    }
    fn skip_render_if_probe_is_uninitialized_mut(&mut self) -> &mut bool {
        &mut self.skip_render_if_probe_is_uninitialized
    }
    fn batch_update_light_probes_enable(&self) -> &bool {
        &self.batch_update_light_probes_enable
    }
    fn batch_update_light_probes_enable_mut(&mut self) -> &mut bool {
        &mut self.batch_update_light_probes_enable
    }
    fn quad_light_probe_max_update_count(&self) -> &u32 {
        &self.quad_light_probe_max_update_count
    }
    fn quad_light_probe_max_update_count_mut(&mut self) -> &mut u32 {
        &mut self.quad_light_probe_max_update_count
    }
    fn graph_light_probe_max_update_count(&self) -> &u32 {
        &self.graph_light_probe_max_update_count
    }
    fn graph_light_probe_max_update_count_mut(&mut self) -> &mut u32 {
        &mut self.graph_light_probe_max_update_count
    }
    fn mesh_light_probe_max_update_count(&self) -> &u32 {
        &self.mesh_light_probe_max_update_count
    }
    fn mesh_light_probe_max_update_count_mut(&mut self) -> &mut u32 {
        &mut self.mesh_light_probe_max_update_count
    }
    fn graph_emitter_enabled(&self) -> &bool {
        &self.graph_emitter_enabled
    }
    fn graph_emitter_enabled_mut(&mut self) -> &mut bool {
        &mut self.graph_emitter_enabled
    }
    fn graph_emitter_draw_debug_stats(&self) -> &bool {
        &self.graph_emitter_draw_debug_stats
    }
    fn graph_emitter_draw_debug_stats_mut(&mut self) -> &mut bool {
        &mut self.graph_emitter_draw_debug_stats
    }
    fn graph_emitter_draw_debug_buffers(&self) -> &bool {
        &self.graph_emitter_draw_debug_buffers
    }
    fn graph_emitter_draw_debug_buffers_mut(&mut self) -> &mut bool {
        &mut self.graph_emitter_draw_debug_buffers
    }
    fn graph_emitter_draw_debug_view_visible_instances(&self) -> &bool {
        &self.graph_emitter_draw_debug_view_visible_instances
    }
    fn graph_emitter_draw_debug_view_visible_instances_mut(&mut self) -> &mut bool {
        &mut self.graph_emitter_draw_debug_view_visible_instances
    }
    fn graph_emitter_overlapped_compute_enable(&self) -> &bool {
        &self.graph_emitter_overlapped_compute_enable
    }
    fn graph_emitter_overlapped_compute_enable_mut(&mut self) -> &mut bool {
        &mut self.graph_emitter_overlapped_compute_enable
    }
    fn emitter_graph_block_allocator_max_byte_count(&self) -> &u32 {
        &self.emitter_graph_block_allocator_max_byte_count
    }
    fn emitter_graph_block_allocator_max_byte_count_mut(&mut self) -> &mut u32 {
        &mut self.emitter_graph_block_allocator_max_byte_count
    }
    fn emitter_graph_block_allocator_block_max_count(&self) -> &u32 {
        &self.emitter_graph_block_allocator_block_max_count
    }
    fn emitter_graph_block_allocator_block_max_count_mut(&mut self) -> &mut u32 {
        &mut self.emitter_graph_block_allocator_block_max_count
    }
    fn emitter_graph_max_defrag_operations_per_frame(&self) -> &u32 {
        &self.emitter_graph_max_defrag_operations_per_frame
    }
    fn emitter_graph_max_defrag_operations_per_frame_mut(&mut self) -> &mut u32 {
        &mut self.emitter_graph_max_defrag_operations_per_frame
    }
    fn emitter_graph_draw_debug_uber_buffer(&self) -> &bool {
        &self.emitter_graph_draw_debug_uber_buffer
    }
    fn emitter_graph_draw_debug_uber_buffer_mut(&mut self) -> &mut bool {
        &mut self.emitter_graph_draw_debug_uber_buffer
    }
    fn emitter_graph_uber_buffer_defrag_enable(&self) -> &bool {
        &self.emitter_graph_uber_buffer_defrag_enable
    }
    fn emitter_graph_uber_buffer_defrag_enable_mut(&mut self) -> &mut bool {
        &mut self.emitter_graph_uber_buffer_defrag_enable
    }
}

impl super::core::DataContainerTrait for EmitterSystemSettings {
}

pub static EMITTERSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterSystemSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, enable),
            },
            FieldInfoData {
                name: "UpdateJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, update_job_enable),
            },
            FieldInfoData {
                name: "SkipUpdateMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, skip_update_max_count),
            },
            FieldInfoData {
                name: "ForceJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterSystemSettings, force_job_count),
            },
            FieldInfoData {
                name: "TimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, time_scale),
            },
            FieldInfoData {
                name: "GlobalResetStartTimeInterval",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, global_reset_start_time_interval),
            },
            FieldInfoData {
                name: "EnableFixedTimeStep",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, enable_fixed_time_step),
            },
            FieldInfoData {
                name: "EnableFixedDelta",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, enable_fixed_delta),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, enable_jobs),
            },
            FieldInfoData {
                name: "CollisionRayCastEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, collision_ray_cast_enable),
            },
            FieldInfoData {
                name: "CollisionRayCastMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, collision_ray_cast_max_count),
            },
            FieldInfoData {
                name: "ProximityPhysicsEntitiesMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, proximity_physics_entities_max_count),
            },
            FieldInfoData {
                name: "DrawDebugRayCastCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_ray_cast_collision),
            },
            FieldInfoData {
                name: "EmitterQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(EmitterSystemSettings, emitter_quality_level),
            },
            FieldInfoData {
                name: "TemplateTimeoutTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, template_timeout_time),
            },
            FieldInfoData {
                name: "PreciseWindAndForceMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, precise_wind_and_force_max_distance),
            },
            FieldInfoData {
                name: "TurbulenceMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, turbulence_max_distance),
            },
            FieldInfoData {
                name: "ScreenAreaCullingStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, screen_area_culling_start),
            },
            FieldInfoData {
                name: "ScreenAreaCullingEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, screen_area_culling_end),
            },
            FieldInfoData {
                name: "ScreenAreaCullingMinTotalArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, screen_area_culling_min_total_area),
            },
            FieldInfoData {
                name: "ScreenAreaCullingMaxTotalArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, screen_area_culling_max_total_area),
            },
            FieldInfoData {
                name: "ScreenAreaCullingMaxMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, screen_area_culling_max_multiplier),
            },
            FieldInfoData {
                name: "ProcessJobYieldTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, process_job_yield_time),
            },
            FieldInfoData {
                name: "VisibleJobYieldTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, visible_job_yield_time),
            },
            FieldInfoData {
                name: "MeshEmitterMotionBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_emitter_motion_blur_enable),
            },
            FieldInfoData {
                name: "EnableRendering",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, enable_rendering),
            },
            FieldInfoData {
                name: "DrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, draw_stats),
            },
            FieldInfoData {
                name: "CollectPerformanceStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, collect_performance_stats),
            },
            FieldInfoData {
                name: "CollectPerformanceStatsTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterSystemSettings, collect_performance_stats_time),
            },
            FieldInfoData {
                name: "DrawMemStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, draw_mem_stats),
            },
            FieldInfoData {
                name: "DrawStatsSamplingPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, draw_stats_sampling_period),
            },
            FieldInfoData {
                name: "DrawStatsEntriesPerPage",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, draw_stats_entries_per_page),
            },
            FieldInfoData {
                name: "DrawStatsPageIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, draw_stats_page_index),
            },
            FieldInfoData {
                name: "DrawStatsFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EmitterSystemSettings, draw_stats_filter),
            },
            FieldInfoData {
                name: "HideInactiveStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, hide_inactive_stats),
            },
            FieldInfoData {
                name: "SaveListActiveEmitters",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, save_list_active_emitters),
            },
            FieldInfoData {
                name: "DrawEmitterName",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_emitter_name),
            },
            FieldInfoData {
                name: "ZBufferCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, z_buffer_cull_enable),
            },
            FieldInfoData {
                name: "DrawProjectedBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_projected_boxes),
            },
            FieldInfoData {
                name: "DrawBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, draw_bounding_boxes),
            },
            FieldInfoData {
                name: "MinScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, min_screen_area),
            },
            FieldInfoData {
                name: "MinScreenAreaThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, min_screen_area_threshold),
            },
            FieldInfoData {
                name: "ForceCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, force_cull_distance),
            },
            FieldInfoData {
                name: "ForceCullFadeFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, force_cull_fade_far_distance),
            },
            FieldInfoData {
                name: "DrawTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_transforms),
            },
            FieldInfoData {
                name: "DrawLightProbeSampleTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_light_probe_sample_transforms),
            },
            FieldInfoData {
                name: "DrawDebugBaseAtlas",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_base_atlas),
            },
            FieldInfoData {
                name: "DrawDebugNormalAtlas",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_normal_atlas),
            },
            FieldInfoData {
                name: "DrawDebugAtlasMiplevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_atlas_miplevel),
            },
            FieldInfoData {
                name: "DrawDebugAtlasTextureIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_atlas_texture_index),
            },
            FieldInfoData {
                name: "DrawDebugAtlasAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_atlas_alpha),
            },
            FieldInfoData {
                name: "DrawDebugEmitterExclusionVolumes",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_emitter_exclusion_volumes),
            },
            FieldInfoData {
                name: "DrawDebugAtlasPageIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_atlas_page_index),
            },
            FieldInfoData {
                name: "DrawDebugEmitterSunTransmittanceMapGroup",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_emitter_sun_transmittance_map_group),
            },
            FieldInfoData {
                name: "DrawDebugEmitterSunTransmittanceMapsLinks",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_emitter_sun_transmittance_maps_links),
            },
            FieldInfoData {
                name: "ForceSunTransmittanceOnAllEmitters",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, force_sun_transmittance_on_all_emitters),
            },
            FieldInfoData {
                name: "EmitterRenderSunTransmittanceViewsFirst",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, emitter_render_sun_transmittance_views_first),
            },
            FieldInfoData {
                name: "DrawDebugEmitterVertexBufferUsage",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, draw_debug_emitter_vertex_buffer_usage),
            },
            FieldInfoData {
                name: "EmitterGpuLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, emitter_gpu_lighting_enable),
            },
            FieldInfoData {
                name: "WalrusLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, walrus_lighting_enable),
            },
            FieldInfoData {
                name: "EmitterGpuLightingPipelineShadersEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, emitter_gpu_lighting_pipeline_shaders_enabled),
            },
            FieldInfoData {
                name: "SystemShadersPath",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EmitterSystemSettings, system_shaders_path),
            },
            FieldInfoData {
                name: "SystemVSFPath",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EmitterSystemSettings, system_v_s_f_path),
            },
            FieldInfoData {
                name: "CrossfireDriverProfileAvailable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, crossfire_driver_profile_available),
            },
            FieldInfoData {
                name: "QuadClipScaleEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_clip_scale_enable),
            },
            FieldInfoData {
                name: "QuadEnableRendering",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_enable_rendering),
            },
            FieldInfoData {
                name: "QuadNiceRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_nice_rendering_enable),
            },
            FieldInfoData {
                name: "QuadSimpleRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_simple_rendering_enable),
            },
            FieldInfoData {
                name: "QuadEnableOpaque",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_enable_opaque),
            },
            FieldInfoData {
                name: "QuadEnableCustomShader",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_enable_custom_shader),
            },
            FieldInfoData {
                name: "QuadColorShaderCostsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_color_shader_costs_enable),
            },
            FieldInfoData {
                name: "QuadEnableSorting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_enable_sorting),
            },
            FieldInfoData {
                name: "QuadEnableWireframe",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_enable_wireframe),
            },
            FieldInfoData {
                name: "QuadHalfResEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_half_res_enable),
            },
            FieldInfoData {
                name: "QuadGroupsJoinAll",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_groups_join_all),
            },
            FieldInfoData {
                name: "QuadGroupsJoinNone",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_groups_join_none),
            },
            FieldInfoData {
                name: "QuadGroupsJoinNiceAndSimple",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_groups_join_nice_and_simple),
            },
            FieldInfoData {
                name: "QuadTechnique",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterSystemSettings, quad_technique),
            },
            FieldInfoData {
                name: "QuadVertexShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_vertex_shadows_enable),
            },
            FieldInfoData {
                name: "QuadCloudVertexShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_cloud_vertex_shadows_enable),
            },
            FieldInfoData {
                name: "QuadPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_planar_reflection_enable),
            },
            FieldInfoData {
                name: "QuadPointLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_point_lights_enable),
            },
            FieldInfoData {
                name: "QuadSpotLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, quad_spot_lights_enable),
            },
            FieldInfoData {
                name: "PunctualLightThresholdSquared",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, punctual_light_threshold_squared),
            },
            FieldInfoData {
                name: "QuadNearFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, quad_near_fade_distance),
            },
            FieldInfoData {
                name: "CustomEmitterPositionSorting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, custom_emitter_position_sorting),
            },
            FieldInfoData {
                name: "QuadMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, quad_max_count),
            },
            FieldInfoData {
                name: "MeshRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_rendering_enable),
            },
            FieldInfoData {
                name: "MeshDrawTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_draw_transforms),
            },
            FieldInfoData {
                name: "MeshDrawBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_draw_bounding_boxes),
            },
            FieldInfoData {
                name: "MeshShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_shadow_enable),
            },
            FieldInfoData {
                name: "MeshPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_planar_reflection_enable),
            },
            FieldInfoData {
                name: "MeshCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_culling_distance),
            },
            FieldInfoData {
                name: "MeshDrawCountLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_draw_count_limit),
            },
            FieldInfoData {
                name: "MeshStreamingPriorityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_streaming_priority_multiplier),
            },
            FieldInfoData {
                name: "MeshDrawCullStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_draw_cull_stats),
            },
            FieldInfoData {
                name: "MeshMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_max_count),
            },
            FieldInfoData {
                name: "SkipRenderIfProbeIsUninitialized",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, skip_render_if_probe_is_uninitialized),
            },
            FieldInfoData {
                name: "BatchUpdateLightProbesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, batch_update_light_probes_enable),
            },
            FieldInfoData {
                name: "QuadLightProbeMaxUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, quad_light_probe_max_update_count),
            },
            FieldInfoData {
                name: "GraphLightProbeMaxUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, graph_light_probe_max_update_count),
            },
            FieldInfoData {
                name: "MeshLightProbeMaxUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, mesh_light_probe_max_update_count),
            },
            FieldInfoData {
                name: "GraphEmitterEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, graph_emitter_enabled),
            },
            FieldInfoData {
                name: "GraphEmitterDrawDebugStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, graph_emitter_draw_debug_stats),
            },
            FieldInfoData {
                name: "GraphEmitterDrawDebugBuffers",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, graph_emitter_draw_debug_buffers),
            },
            FieldInfoData {
                name: "GraphEmitterDrawDebugViewVisibleInstances",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, graph_emitter_draw_debug_view_visible_instances),
            },
            FieldInfoData {
                name: "GraphEmitterOverlappedComputeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, graph_emitter_overlapped_compute_enable),
            },
            FieldInfoData {
                name: "EmitterGraphBlockAllocatorMaxByteCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, emitter_graph_block_allocator_max_byte_count),
            },
            FieldInfoData {
                name: "EmitterGraphBlockAllocatorBlockMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, emitter_graph_block_allocator_block_max_count),
            },
            FieldInfoData {
                name: "EmitterGraphMaxDefragOperationsPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterSystemSettings, emitter_graph_max_defrag_operations_per_frame),
            },
            FieldInfoData {
                name: "EmitterGraphDrawDebugUberBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, emitter_graph_draw_debug_uber_buffer),
            },
            FieldInfoData {
                name: "EmitterGraphUberBufferDefragEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterSystemSettings, emitter_graph_uber_buffer_defrag_enable),
            },
        ],
    }),
    array_type: Some(EMITTERSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterSystemSettings {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERSYSTEMSETTINGS_TYPE_INFO
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


pub static EMITTERSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterSystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FlatEmitterDocument {
    pub _glacier_base: EmitterDocument,
    pub template_data: Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>,
}

pub trait FlatEmitterDocumentTrait: EmitterDocumentTrait {
    fn template_data(&self) -> &Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>;
    fn template_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>;
}

impl FlatEmitterDocumentTrait for FlatEmitterDocument {
    fn template_data(&self) -> &Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>> {
        &self.template_data
    }
    fn template_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>> {
        &mut self.template_data
    }
}

impl EmitterDocumentTrait for FlatEmitterDocument {
}

impl EmitterAssetTrait for FlatEmitterDocument {
}

impl super::emitter_base::EmitterBaseAssetTrait for FlatEmitterDocument {
}

impl super::core::AssetTrait for FlatEmitterDocument {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for FlatEmitterDocument {
}

pub static FLATEMITTERDOCUMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlatEmitterDocument",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERDOCUMENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FlatEmitterDocument as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TemplateData",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterTemplateData",
                rust_offset: offset_of!(FlatEmitterDocument, template_data),
            },
        ],
    }),
    array_type: Some(FLATEMITTERDOCUMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FlatEmitterDocument {
    fn type_info(&self) -> &'static TypeInfo {
        FLATEMITTERDOCUMENT_TYPE_INFO
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


pub static FLATEMITTERDOCUMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlatEmitterDocument-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("FlatEmitterDocument"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ScalableEmitterDocument {
    pub _glacier_base: EmitterDocument,
    pub template_data_low: Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>,
    pub template_data_medium: Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>,
    pub template_data_high: Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>,
    pub template_data_ultra: Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>,
}

pub trait ScalableEmitterDocumentTrait: EmitterDocumentTrait {
    fn template_data_low(&self) -> &Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>;
    fn template_data_low_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>;
    fn template_data_medium(&self) -> &Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>;
    fn template_data_medium_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>;
    fn template_data_high(&self) -> &Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>;
    fn template_data_high_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>;
    fn template_data_ultra(&self) -> &Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>;
    fn template_data_ultra_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>>;
}

impl ScalableEmitterDocumentTrait for ScalableEmitterDocument {
    fn template_data_low(&self) -> &Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>> {
        &self.template_data_low
    }
    fn template_data_low_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>> {
        &mut self.template_data_low
    }
    fn template_data_medium(&self) -> &Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>> {
        &self.template_data_medium
    }
    fn template_data_medium_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>> {
        &mut self.template_data_medium
    }
    fn template_data_high(&self) -> &Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>> {
        &self.template_data_high
    }
    fn template_data_high_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>> {
        &mut self.template_data_high
    }
    fn template_data_ultra(&self) -> &Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>> {
        &self.template_data_ultra
    }
    fn template_data_ultra_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterTemplateDataTrait>>> {
        &mut self.template_data_ultra
    }
}

impl EmitterDocumentTrait for ScalableEmitterDocument {
}

impl EmitterAssetTrait for ScalableEmitterDocument {
}

impl super::emitter_base::EmitterBaseAssetTrait for ScalableEmitterDocument {
}

impl super::core::AssetTrait for ScalableEmitterDocument {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ScalableEmitterDocument {
}

pub static SCALABLEEMITTERDOCUMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScalableEmitterDocument",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERDOCUMENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ScalableEmitterDocument as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TemplateDataLow",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterTemplateData",
                rust_offset: offset_of!(ScalableEmitterDocument, template_data_low),
            },
            FieldInfoData {
                name: "TemplateDataMedium",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterTemplateData",
                rust_offset: offset_of!(ScalableEmitterDocument, template_data_medium),
            },
            FieldInfoData {
                name: "TemplateDataHigh",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterTemplateData",
                rust_offset: offset_of!(ScalableEmitterDocument, template_data_high),
            },
            FieldInfoData {
                name: "TemplateDataUltra",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterTemplateData",
                rust_offset: offset_of!(ScalableEmitterDocument, template_data_ultra),
            },
        ],
    }),
    array_type: Some(SCALABLEEMITTERDOCUMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScalableEmitterDocument {
    fn type_info(&self) -> &'static TypeInfo {
        SCALABLEEMITTERDOCUMENT_TYPE_INFO
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


pub static SCALABLEEMITTERDOCUMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScalableEmitterDocument-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ScalableEmitterDocument"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterDocument {
    pub _glacier_base: EmitterAsset,
}

pub trait EmitterDocumentTrait: EmitterAssetTrait {
}

impl EmitterDocumentTrait for EmitterDocument {
}

impl EmitterAssetTrait for EmitterDocument {
}

impl super::emitter_base::EmitterBaseAssetTrait for EmitterDocument {
}

impl super::core::AssetTrait for EmitterDocument {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EmitterDocument {
}

pub static EMITTERDOCUMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDocument",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterDocument as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EMITTERDOCUMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterDocument {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERDOCUMENT_TYPE_INFO
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


pub static EMITTERDOCUMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDocument-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterDocument"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterTemplateData {
    pub _glacier_base: super::core::DataContainer,
    pub root_processor: Option<Arc<Mutex<dyn ProcessorDataTrait>>>,
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
    pub mesh: Option<Arc<Mutex<dyn super::render::MeshAssetTrait>>>,
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
    pub per_particle_effect_parameters: Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>>,
    pub min_distance_travelled_before_spawn: f32,
    pub cull_fade_near_distance: f32,
    pub cull_fade_near_range: f32,
    pub cull_fade_far_distance: f32,
    pub cull_fade_far_range: f32,
    pub skip_near_camera_fade: bool,
    pub emitter_wind_evaluation_enable: bool,
    pub emittable_wind_evaluation_enable: bool,
    pub debug_name: String,
    pub tweak_inherited_emitter: Option<Arc<Mutex<dyn EmitterDocumentTrait>>>,
}

pub trait EmitterTemplateDataTrait: super::core::DataContainerTrait {
    fn root_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>>;
    fn root_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>>;
    fn max_count(&self) -> &u32;
    fn max_count_mut(&mut self) -> &mut u32;
    fn lifetime(&self) -> &f32;
    fn lifetime_mut(&mut self) -> &mut f32;
    fn time_scale(&self) -> &f32;
    fn time_scale_mut(&mut self) -> &mut f32;
    fn repeat_particle_spawning(&self) -> &bool;
    fn repeat_particle_spawning_mut(&mut self) -> &mut bool;
    fn lifetime_frame_count(&self) -> &u32;
    fn lifetime_frame_count_mut(&mut self) -> &mut u32;
    fn follow_spawn_source(&self) -> &bool;
    fn follow_spawn_source_mut(&mut self) -> &mut bool;
    fn follow_spawn_source_velocity(&self) -> &bool;
    fn follow_spawn_source_velocity_mut(&mut self) -> &mut bool;
    fn kill_particles_with_emitter(&self) -> &bool;
    fn kill_particles_with_emitter_mut(&mut self) -> &mut bool;
    fn kill_ribbon_tail_distance(&self) -> &f32;
    fn kill_ribbon_tail_distance_mut(&mut self) -> &mut f32;
    fn smooth_ribbon_spawn(&self) -> &bool;
    fn smooth_ribbon_spawn_mut(&mut self) -> &mut bool;
    fn exclusion_volume_cull_enable(&self) -> &bool;
    fn exclusion_volume_cull_enable_mut(&mut self) -> &mut bool;
    fn emittable_type(&self) -> &EmittableType;
    fn emittable_type_mut(&mut self) -> &mut EmittableType;
    fn emittable_alignment(&self) -> &EmittableAlignment;
    fn emittable_alignment_mut(&mut self) -> &mut EmittableAlignment;
    fn world_alignment_direction(&self) -> &super::core::Vec3;
    fn world_alignment_direction_mut(&mut self) -> &mut super::core::Vec3;
    fn motion_stretch_multiplier(&self) -> &f32;
    fn motion_stretch_multiplier_mut(&mut self) -> &mut f32;
    fn motion_stretch_view_multiplier(&self) -> &f32;
    fn motion_stretch_view_multiplier_mut(&mut self) -> &mut f32;
    fn motion_stretch_length_clamp(&self) -> &f32;
    fn motion_stretch_length_clamp_mut(&mut self) -> &mut f32;
    fn motion_stretch_relative_length_clamp(&self) -> &f32;
    fn motion_stretch_relative_length_clamp_mut(&mut self) -> &mut f32;
    fn orient_to_position(&self) -> &super::core::Vec3;
    fn orient_to_position_mut(&mut self) -> &mut super::core::Vec3;
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render::MeshAssetTrait>>>;
    fn mesh_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::MeshAssetTrait>>>;
    fn object_variation_name_hash(&self) -> &u32;
    fn object_variation_name_hash_mut(&mut self) -> &mut u32;
    fn emissive(&self) -> &bool;
    fn emissive_mut(&mut self) -> &mut bool;
    fn emissive_exposure_factor(&self) -> &f32;
    fn emissive_exposure_factor_mut(&mut self) -> &mut f32;
    fn opaque(&self) -> &bool;
    fn opaque_mut(&mut self) -> &mut bool;
    fn mesh_particles_motion_blur(&self) -> &bool;
    fn mesh_particles_motion_blur_mut(&mut self) -> &mut bool;
    fn vertex_pixel_lighting_blend_factor(&self) -> &f32;
    fn vertex_pixel_lighting_blend_factor_mut(&mut self) -> &mut f32;
    fn global_local_normal_blend_factor(&self) -> &f32;
    fn global_local_normal_blend_factor_mut(&mut self) -> &mut f32;
    fn soft_particles_fade_distance_multiplier(&self) -> &f32;
    fn soft_particles_fade_distance_multiplier_mut(&mut self) -> &mut f32;
    fn light_wrap_around_factor(&self) -> &f32;
    fn light_wrap_around_factor_mut(&mut self) -> &mut f32;
    fn bent_normal_factor(&self) -> &f32;
    fn bent_normal_factor_mut(&mut self) -> &mut f32;
    fn light_multiplier(&self) -> &f32;
    fn light_multiplier_mut(&mut self) -> &mut f32;
    fn light_multiplier_dynamic(&self) -> &f32;
    fn light_multiplier_dynamic_mut(&mut self) -> &mut f32;
    fn receive_sun_shadow(&self) -> &bool;
    fn receive_sun_shadow_mut(&mut self) -> &mut bool;
    fn bending_factor(&self) -> &f32;
    fn bending_factor_mut(&mut self) -> &mut f32;
    fn micro_variation_smoothing_factor(&self) -> &f32;
    fn micro_variation_smoothing_factor_mut(&mut self) -> &mut f32;
    fn force_nice_sorting(&self) -> &bool;
    fn force_nice_sorting_mut(&mut self) -> &mut bool;
    fn local_space(&self) -> &bool;
    fn local_space_mut(&mut self) -> &mut bool;
    fn allow_scale(&self) -> &bool;
    fn allow_scale_mut(&mut self) -> &mut bool;
    fn camera_space(&self) -> &bool;
    fn camera_space_mut(&mut self) -> &mut bool;
    fn transparency_sun_shadow_enable(&self) -> &bool;
    fn transparency_sun_shadow_enable_mut(&mut self) -> &mut bool;
    fn sun_volumetric_shadow_enable(&self) -> &bool;
    fn sun_volumetric_shadow_enable_mut(&mut self) -> &mut bool;
    fn sun_volumetric_shadow_absorption_scale(&self) -> &f32;
    fn sun_volumetric_shadow_absorption_scale_mut(&mut self) -> &mut f32;
    fn sun_volumetric_shadow_absorption_r(&self) -> &f32;
    fn sun_volumetric_shadow_absorption_r_mut(&mut self) -> &mut f32;
    fn sun_volumetric_shadow_absorption_g(&self) -> &f32;
    fn sun_volumetric_shadow_absorption_g_mut(&mut self) -> &mut f32;
    fn sun_volumetric_shadow_absorption_b(&self) -> &f32;
    fn sun_volumetric_shadow_absorption_b_mut(&mut self) -> &mut f32;
    fn sun_volumetric_shadow_phase_g0(&self) -> &f32;
    fn sun_volumetric_shadow_phase_g0_mut(&mut self) -> &mut f32;
    fn sun_volumetric_shadow_phase_g1(&self) -> &f32;
    fn sun_volumetric_shadow_phase_g1_mut(&mut self) -> &mut f32;
    fn sun_volumetric_shadow_offset(&self) -> &f32;
    fn sun_volumetric_shadow_offset_mut(&mut self) -> &mut f32;
    fn enable_pyro_shader(&self) -> &bool;
    fn enable_pyro_shader_mut(&mut self) -> &mut bool;
    fn gnomon_light_rig_index(&self) -> &i32;
    fn gnomon_light_rig_index_mut(&mut self) -> &mut i32;
    fn use_right_texture_tile(&self) -> &bool;
    fn use_right_texture_tile_mut(&mut self) -> &mut bool;
    fn cast_planar_reflection_enable(&self) -> &bool;
    fn cast_planar_reflection_enable_mut(&mut self) -> &mut bool;
    fn force_full_res(&self) -> &bool;
    fn force_full_res_mut(&mut self) -> &mut bool;
    fn fog_fade(&self) -> &bool;
    fn fog_fade_mut(&mut self) -> &mut bool;
    fn camera_bias(&self) -> &f32;
    fn camera_bias_mut(&mut self) -> &mut f32;
    fn emitter_draw_order(&self) -> &EmitterDrawOrder;
    fn emitter_draw_order_mut(&mut self) -> &mut EmitterDrawOrder;
    fn flip_u_probability(&self) -> &f32;
    fn flip_u_probability_mut(&mut self) -> &mut f32;
    fn flip_v_probability(&self) -> &f32;
    fn flip_v_probability_mut(&mut self) -> &mut f32;
    fn lock_ribbon_direction(&self) -> &bool;
    fn lock_ribbon_direction_mut(&mut self) -> &mut bool;
    fn particle_culling_factor(&self) -> &f32;
    fn particle_culling_factor_mut(&mut self) -> &mut f32;
    fn instanced(&self) -> &bool;
    fn instanced_mut(&mut self) -> &mut bool;
    fn alpha_cull_threshold(&self) -> &f32;
    fn alpha_cull_threshold_mut(&mut self) -> &mut f32;
    fn min_spawn_distance(&self) -> &f32;
    fn min_spawn_distance_mut(&mut self) -> &mut f32;
    fn max_spawn_distance(&self) -> &f32;
    fn max_spawn_distance_mut(&mut self) -> &mut f32;
    fn min_screen_area(&self) -> &f32;
    fn min_screen_area_mut(&mut self) -> &mut f32;
    fn mesh_culling_distance(&self) -> &f32;
    fn mesh_culling_distance_mut(&mut self) -> &mut f32;
    fn pause_simulation_when_culled(&self) -> &bool;
    fn pause_simulation_when_culled_mut(&mut self) -> &mut bool;
    fn skip_update_max_count(&self) -> &i32;
    fn skip_update_max_count_mut(&mut self) -> &mut i32;
    fn skip_simulation_distance(&self) -> &f32;
    fn skip_simulation_distance_mut(&mut self) -> &mut f32;
    fn precise_wind_and_force_max_distance(&self) -> &f32;
    fn precise_wind_and_force_max_distance_mut(&mut self) -> &mut f32;
    fn turbulence_max_distance(&self) -> &f32;
    fn turbulence_max_distance_mut(&mut self) -> &mut f32;
    fn distance_scale_length(&self) -> &f32;
    fn distance_scale_length_mut(&mut self) -> &mut f32;
    fn distance_scale_near_value(&self) -> &f32;
    fn distance_scale_near_value_mut(&mut self) -> &mut f32;
    fn distance_scale_far_value(&self) -> &f32;
    fn distance_scale_far_value_mut(&mut self) -> &mut f32;
    fn speed_normalization_value(&self) -> &f32;
    fn speed_normalization_value_mut(&mut self) -> &mut f32;
    fn wind_speed_normalization_value(&self) -> &f32;
    fn wind_speed_normalization_value_mut(&mut self) -> &mut f32;
    fn travelled_distance_normalization_value(&self) -> &f32;
    fn travelled_distance_normalization_value_mut(&mut self) -> &mut f32;
    fn accept_global_parameter1(&self) -> &bool;
    fn accept_global_parameter1_mut(&mut self) -> &mut bool;
    fn accept_global_parameter2(&self) -> &bool;
    fn accept_global_parameter2_mut(&mut self) -> &mut bool;
    fn accept_global_parameter3(&self) -> &bool;
    fn accept_global_parameter3_mut(&mut self) -> &mut bool;
    fn per_particle_effect_parameters(&self) -> &Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>>;
    fn per_particle_effect_parameters_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>>;
    fn min_distance_travelled_before_spawn(&self) -> &f32;
    fn min_distance_travelled_before_spawn_mut(&mut self) -> &mut f32;
    fn cull_fade_near_distance(&self) -> &f32;
    fn cull_fade_near_distance_mut(&mut self) -> &mut f32;
    fn cull_fade_near_range(&self) -> &f32;
    fn cull_fade_near_range_mut(&mut self) -> &mut f32;
    fn cull_fade_far_distance(&self) -> &f32;
    fn cull_fade_far_distance_mut(&mut self) -> &mut f32;
    fn cull_fade_far_range(&self) -> &f32;
    fn cull_fade_far_range_mut(&mut self) -> &mut f32;
    fn skip_near_camera_fade(&self) -> &bool;
    fn skip_near_camera_fade_mut(&mut self) -> &mut bool;
    fn emitter_wind_evaluation_enable(&self) -> &bool;
    fn emitter_wind_evaluation_enable_mut(&mut self) -> &mut bool;
    fn emittable_wind_evaluation_enable(&self) -> &bool;
    fn emittable_wind_evaluation_enable_mut(&mut self) -> &mut bool;
    fn debug_name(&self) -> &String;
    fn debug_name_mut(&mut self) -> &mut String;
    fn tweak_inherited_emitter(&self) -> &Option<Arc<Mutex<dyn EmitterDocumentTrait>>>;
    fn tweak_inherited_emitter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterDocumentTrait>>>;
}

impl EmitterTemplateDataTrait for EmitterTemplateData {
    fn root_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        &self.root_processor
    }
    fn root_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        &mut self.root_processor
    }
    fn max_count(&self) -> &u32 {
        &self.max_count
    }
    fn max_count_mut(&mut self) -> &mut u32 {
        &mut self.max_count
    }
    fn lifetime(&self) -> &f32 {
        &self.lifetime
    }
    fn lifetime_mut(&mut self) -> &mut f32 {
        &mut self.lifetime
    }
    fn time_scale(&self) -> &f32 {
        &self.time_scale
    }
    fn time_scale_mut(&mut self) -> &mut f32 {
        &mut self.time_scale
    }
    fn repeat_particle_spawning(&self) -> &bool {
        &self.repeat_particle_spawning
    }
    fn repeat_particle_spawning_mut(&mut self) -> &mut bool {
        &mut self.repeat_particle_spawning
    }
    fn lifetime_frame_count(&self) -> &u32 {
        &self.lifetime_frame_count
    }
    fn lifetime_frame_count_mut(&mut self) -> &mut u32 {
        &mut self.lifetime_frame_count
    }
    fn follow_spawn_source(&self) -> &bool {
        &self.follow_spawn_source
    }
    fn follow_spawn_source_mut(&mut self) -> &mut bool {
        &mut self.follow_spawn_source
    }
    fn follow_spawn_source_velocity(&self) -> &bool {
        &self.follow_spawn_source_velocity
    }
    fn follow_spawn_source_velocity_mut(&mut self) -> &mut bool {
        &mut self.follow_spawn_source_velocity
    }
    fn kill_particles_with_emitter(&self) -> &bool {
        &self.kill_particles_with_emitter
    }
    fn kill_particles_with_emitter_mut(&mut self) -> &mut bool {
        &mut self.kill_particles_with_emitter
    }
    fn kill_ribbon_tail_distance(&self) -> &f32 {
        &self.kill_ribbon_tail_distance
    }
    fn kill_ribbon_tail_distance_mut(&mut self) -> &mut f32 {
        &mut self.kill_ribbon_tail_distance
    }
    fn smooth_ribbon_spawn(&self) -> &bool {
        &self.smooth_ribbon_spawn
    }
    fn smooth_ribbon_spawn_mut(&mut self) -> &mut bool {
        &mut self.smooth_ribbon_spawn
    }
    fn exclusion_volume_cull_enable(&self) -> &bool {
        &self.exclusion_volume_cull_enable
    }
    fn exclusion_volume_cull_enable_mut(&mut self) -> &mut bool {
        &mut self.exclusion_volume_cull_enable
    }
    fn emittable_type(&self) -> &EmittableType {
        &self.emittable_type
    }
    fn emittable_type_mut(&mut self) -> &mut EmittableType {
        &mut self.emittable_type
    }
    fn emittable_alignment(&self) -> &EmittableAlignment {
        &self.emittable_alignment
    }
    fn emittable_alignment_mut(&mut self) -> &mut EmittableAlignment {
        &mut self.emittable_alignment
    }
    fn world_alignment_direction(&self) -> &super::core::Vec3 {
        &self.world_alignment_direction
    }
    fn world_alignment_direction_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.world_alignment_direction
    }
    fn motion_stretch_multiplier(&self) -> &f32 {
        &self.motion_stretch_multiplier
    }
    fn motion_stretch_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.motion_stretch_multiplier
    }
    fn motion_stretch_view_multiplier(&self) -> &f32 {
        &self.motion_stretch_view_multiplier
    }
    fn motion_stretch_view_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.motion_stretch_view_multiplier
    }
    fn motion_stretch_length_clamp(&self) -> &f32 {
        &self.motion_stretch_length_clamp
    }
    fn motion_stretch_length_clamp_mut(&mut self) -> &mut f32 {
        &mut self.motion_stretch_length_clamp
    }
    fn motion_stretch_relative_length_clamp(&self) -> &f32 {
        &self.motion_stretch_relative_length_clamp
    }
    fn motion_stretch_relative_length_clamp_mut(&mut self) -> &mut f32 {
        &mut self.motion_stretch_relative_length_clamp
    }
    fn orient_to_position(&self) -> &super::core::Vec3 {
        &self.orient_to_position
    }
    fn orient_to_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.orient_to_position
    }
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render::MeshAssetTrait>>> {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::MeshAssetTrait>>> {
        &mut self.mesh
    }
    fn object_variation_name_hash(&self) -> &u32 {
        &self.object_variation_name_hash
    }
    fn object_variation_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.object_variation_name_hash
    }
    fn emissive(&self) -> &bool {
        &self.emissive
    }
    fn emissive_mut(&mut self) -> &mut bool {
        &mut self.emissive
    }
    fn emissive_exposure_factor(&self) -> &f32 {
        &self.emissive_exposure_factor
    }
    fn emissive_exposure_factor_mut(&mut self) -> &mut f32 {
        &mut self.emissive_exposure_factor
    }
    fn opaque(&self) -> &bool {
        &self.opaque
    }
    fn opaque_mut(&mut self) -> &mut bool {
        &mut self.opaque
    }
    fn mesh_particles_motion_blur(&self) -> &bool {
        &self.mesh_particles_motion_blur
    }
    fn mesh_particles_motion_blur_mut(&mut self) -> &mut bool {
        &mut self.mesh_particles_motion_blur
    }
    fn vertex_pixel_lighting_blend_factor(&self) -> &f32 {
        &self.vertex_pixel_lighting_blend_factor
    }
    fn vertex_pixel_lighting_blend_factor_mut(&mut self) -> &mut f32 {
        &mut self.vertex_pixel_lighting_blend_factor
    }
    fn global_local_normal_blend_factor(&self) -> &f32 {
        &self.global_local_normal_blend_factor
    }
    fn global_local_normal_blend_factor_mut(&mut self) -> &mut f32 {
        &mut self.global_local_normal_blend_factor
    }
    fn soft_particles_fade_distance_multiplier(&self) -> &f32 {
        &self.soft_particles_fade_distance_multiplier
    }
    fn soft_particles_fade_distance_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.soft_particles_fade_distance_multiplier
    }
    fn light_wrap_around_factor(&self) -> &f32 {
        &self.light_wrap_around_factor
    }
    fn light_wrap_around_factor_mut(&mut self) -> &mut f32 {
        &mut self.light_wrap_around_factor
    }
    fn bent_normal_factor(&self) -> &f32 {
        &self.bent_normal_factor
    }
    fn bent_normal_factor_mut(&mut self) -> &mut f32 {
        &mut self.bent_normal_factor
    }
    fn light_multiplier(&self) -> &f32 {
        &self.light_multiplier
    }
    fn light_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.light_multiplier
    }
    fn light_multiplier_dynamic(&self) -> &f32 {
        &self.light_multiplier_dynamic
    }
    fn light_multiplier_dynamic_mut(&mut self) -> &mut f32 {
        &mut self.light_multiplier_dynamic
    }
    fn receive_sun_shadow(&self) -> &bool {
        &self.receive_sun_shadow
    }
    fn receive_sun_shadow_mut(&mut self) -> &mut bool {
        &mut self.receive_sun_shadow
    }
    fn bending_factor(&self) -> &f32 {
        &self.bending_factor
    }
    fn bending_factor_mut(&mut self) -> &mut f32 {
        &mut self.bending_factor
    }
    fn micro_variation_smoothing_factor(&self) -> &f32 {
        &self.micro_variation_smoothing_factor
    }
    fn micro_variation_smoothing_factor_mut(&mut self) -> &mut f32 {
        &mut self.micro_variation_smoothing_factor
    }
    fn force_nice_sorting(&self) -> &bool {
        &self.force_nice_sorting
    }
    fn force_nice_sorting_mut(&mut self) -> &mut bool {
        &mut self.force_nice_sorting
    }
    fn local_space(&self) -> &bool {
        &self.local_space
    }
    fn local_space_mut(&mut self) -> &mut bool {
        &mut self.local_space
    }
    fn allow_scale(&self) -> &bool {
        &self.allow_scale
    }
    fn allow_scale_mut(&mut self) -> &mut bool {
        &mut self.allow_scale
    }
    fn camera_space(&self) -> &bool {
        &self.camera_space
    }
    fn camera_space_mut(&mut self) -> &mut bool {
        &mut self.camera_space
    }
    fn transparency_sun_shadow_enable(&self) -> &bool {
        &self.transparency_sun_shadow_enable
    }
    fn transparency_sun_shadow_enable_mut(&mut self) -> &mut bool {
        &mut self.transparency_sun_shadow_enable
    }
    fn sun_volumetric_shadow_enable(&self) -> &bool {
        &self.sun_volumetric_shadow_enable
    }
    fn sun_volumetric_shadow_enable_mut(&mut self) -> &mut bool {
        &mut self.sun_volumetric_shadow_enable
    }
    fn sun_volumetric_shadow_absorption_scale(&self) -> &f32 {
        &self.sun_volumetric_shadow_absorption_scale
    }
    fn sun_volumetric_shadow_absorption_scale_mut(&mut self) -> &mut f32 {
        &mut self.sun_volumetric_shadow_absorption_scale
    }
    fn sun_volumetric_shadow_absorption_r(&self) -> &f32 {
        &self.sun_volumetric_shadow_absorption_r
    }
    fn sun_volumetric_shadow_absorption_r_mut(&mut self) -> &mut f32 {
        &mut self.sun_volumetric_shadow_absorption_r
    }
    fn sun_volumetric_shadow_absorption_g(&self) -> &f32 {
        &self.sun_volumetric_shadow_absorption_g
    }
    fn sun_volumetric_shadow_absorption_g_mut(&mut self) -> &mut f32 {
        &mut self.sun_volumetric_shadow_absorption_g
    }
    fn sun_volumetric_shadow_absorption_b(&self) -> &f32 {
        &self.sun_volumetric_shadow_absorption_b
    }
    fn sun_volumetric_shadow_absorption_b_mut(&mut self) -> &mut f32 {
        &mut self.sun_volumetric_shadow_absorption_b
    }
    fn sun_volumetric_shadow_phase_g0(&self) -> &f32 {
        &self.sun_volumetric_shadow_phase_g0
    }
    fn sun_volumetric_shadow_phase_g0_mut(&mut self) -> &mut f32 {
        &mut self.sun_volumetric_shadow_phase_g0
    }
    fn sun_volumetric_shadow_phase_g1(&self) -> &f32 {
        &self.sun_volumetric_shadow_phase_g1
    }
    fn sun_volumetric_shadow_phase_g1_mut(&mut self) -> &mut f32 {
        &mut self.sun_volumetric_shadow_phase_g1
    }
    fn sun_volumetric_shadow_offset(&self) -> &f32 {
        &self.sun_volumetric_shadow_offset
    }
    fn sun_volumetric_shadow_offset_mut(&mut self) -> &mut f32 {
        &mut self.sun_volumetric_shadow_offset
    }
    fn enable_pyro_shader(&self) -> &bool {
        &self.enable_pyro_shader
    }
    fn enable_pyro_shader_mut(&mut self) -> &mut bool {
        &mut self.enable_pyro_shader
    }
    fn gnomon_light_rig_index(&self) -> &i32 {
        &self.gnomon_light_rig_index
    }
    fn gnomon_light_rig_index_mut(&mut self) -> &mut i32 {
        &mut self.gnomon_light_rig_index
    }
    fn use_right_texture_tile(&self) -> &bool {
        &self.use_right_texture_tile
    }
    fn use_right_texture_tile_mut(&mut self) -> &mut bool {
        &mut self.use_right_texture_tile
    }
    fn cast_planar_reflection_enable(&self) -> &bool {
        &self.cast_planar_reflection_enable
    }
    fn cast_planar_reflection_enable_mut(&mut self) -> &mut bool {
        &mut self.cast_planar_reflection_enable
    }
    fn force_full_res(&self) -> &bool {
        &self.force_full_res
    }
    fn force_full_res_mut(&mut self) -> &mut bool {
        &mut self.force_full_res
    }
    fn fog_fade(&self) -> &bool {
        &self.fog_fade
    }
    fn fog_fade_mut(&mut self) -> &mut bool {
        &mut self.fog_fade
    }
    fn camera_bias(&self) -> &f32 {
        &self.camera_bias
    }
    fn camera_bias_mut(&mut self) -> &mut f32 {
        &mut self.camera_bias
    }
    fn emitter_draw_order(&self) -> &EmitterDrawOrder {
        &self.emitter_draw_order
    }
    fn emitter_draw_order_mut(&mut self) -> &mut EmitterDrawOrder {
        &mut self.emitter_draw_order
    }
    fn flip_u_probability(&self) -> &f32 {
        &self.flip_u_probability
    }
    fn flip_u_probability_mut(&mut self) -> &mut f32 {
        &mut self.flip_u_probability
    }
    fn flip_v_probability(&self) -> &f32 {
        &self.flip_v_probability
    }
    fn flip_v_probability_mut(&mut self) -> &mut f32 {
        &mut self.flip_v_probability
    }
    fn lock_ribbon_direction(&self) -> &bool {
        &self.lock_ribbon_direction
    }
    fn lock_ribbon_direction_mut(&mut self) -> &mut bool {
        &mut self.lock_ribbon_direction
    }
    fn particle_culling_factor(&self) -> &f32 {
        &self.particle_culling_factor
    }
    fn particle_culling_factor_mut(&mut self) -> &mut f32 {
        &mut self.particle_culling_factor
    }
    fn instanced(&self) -> &bool {
        &self.instanced
    }
    fn instanced_mut(&mut self) -> &mut bool {
        &mut self.instanced
    }
    fn alpha_cull_threshold(&self) -> &f32 {
        &self.alpha_cull_threshold
    }
    fn alpha_cull_threshold_mut(&mut self) -> &mut f32 {
        &mut self.alpha_cull_threshold
    }
    fn min_spawn_distance(&self) -> &f32 {
        &self.min_spawn_distance
    }
    fn min_spawn_distance_mut(&mut self) -> &mut f32 {
        &mut self.min_spawn_distance
    }
    fn max_spawn_distance(&self) -> &f32 {
        &self.max_spawn_distance
    }
    fn max_spawn_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_spawn_distance
    }
    fn min_screen_area(&self) -> &f32 {
        &self.min_screen_area
    }
    fn min_screen_area_mut(&mut self) -> &mut f32 {
        &mut self.min_screen_area
    }
    fn mesh_culling_distance(&self) -> &f32 {
        &self.mesh_culling_distance
    }
    fn mesh_culling_distance_mut(&mut self) -> &mut f32 {
        &mut self.mesh_culling_distance
    }
    fn pause_simulation_when_culled(&self) -> &bool {
        &self.pause_simulation_when_culled
    }
    fn pause_simulation_when_culled_mut(&mut self) -> &mut bool {
        &mut self.pause_simulation_when_culled
    }
    fn skip_update_max_count(&self) -> &i32 {
        &self.skip_update_max_count
    }
    fn skip_update_max_count_mut(&mut self) -> &mut i32 {
        &mut self.skip_update_max_count
    }
    fn skip_simulation_distance(&self) -> &f32 {
        &self.skip_simulation_distance
    }
    fn skip_simulation_distance_mut(&mut self) -> &mut f32 {
        &mut self.skip_simulation_distance
    }
    fn precise_wind_and_force_max_distance(&self) -> &f32 {
        &self.precise_wind_and_force_max_distance
    }
    fn precise_wind_and_force_max_distance_mut(&mut self) -> &mut f32 {
        &mut self.precise_wind_and_force_max_distance
    }
    fn turbulence_max_distance(&self) -> &f32 {
        &self.turbulence_max_distance
    }
    fn turbulence_max_distance_mut(&mut self) -> &mut f32 {
        &mut self.turbulence_max_distance
    }
    fn distance_scale_length(&self) -> &f32 {
        &self.distance_scale_length
    }
    fn distance_scale_length_mut(&mut self) -> &mut f32 {
        &mut self.distance_scale_length
    }
    fn distance_scale_near_value(&self) -> &f32 {
        &self.distance_scale_near_value
    }
    fn distance_scale_near_value_mut(&mut self) -> &mut f32 {
        &mut self.distance_scale_near_value
    }
    fn distance_scale_far_value(&self) -> &f32 {
        &self.distance_scale_far_value
    }
    fn distance_scale_far_value_mut(&mut self) -> &mut f32 {
        &mut self.distance_scale_far_value
    }
    fn speed_normalization_value(&self) -> &f32 {
        &self.speed_normalization_value
    }
    fn speed_normalization_value_mut(&mut self) -> &mut f32 {
        &mut self.speed_normalization_value
    }
    fn wind_speed_normalization_value(&self) -> &f32 {
        &self.wind_speed_normalization_value
    }
    fn wind_speed_normalization_value_mut(&mut self) -> &mut f32 {
        &mut self.wind_speed_normalization_value
    }
    fn travelled_distance_normalization_value(&self) -> &f32 {
        &self.travelled_distance_normalization_value
    }
    fn travelled_distance_normalization_value_mut(&mut self) -> &mut f32 {
        &mut self.travelled_distance_normalization_value
    }
    fn accept_global_parameter1(&self) -> &bool {
        &self.accept_global_parameter1
    }
    fn accept_global_parameter1_mut(&mut self) -> &mut bool {
        &mut self.accept_global_parameter1
    }
    fn accept_global_parameter2(&self) -> &bool {
        &self.accept_global_parameter2
    }
    fn accept_global_parameter2_mut(&mut self) -> &mut bool {
        &mut self.accept_global_parameter2
    }
    fn accept_global_parameter3(&self) -> &bool {
        &self.accept_global_parameter3
    }
    fn accept_global_parameter3_mut(&mut self) -> &mut bool {
        &mut self.accept_global_parameter3
    }
    fn per_particle_effect_parameters(&self) -> &Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>> {
        &self.per_particle_effect_parameters
    }
    fn per_particle_effect_parameters_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>> {
        &mut self.per_particle_effect_parameters
    }
    fn min_distance_travelled_before_spawn(&self) -> &f32 {
        &self.min_distance_travelled_before_spawn
    }
    fn min_distance_travelled_before_spawn_mut(&mut self) -> &mut f32 {
        &mut self.min_distance_travelled_before_spawn
    }
    fn cull_fade_near_distance(&self) -> &f32 {
        &self.cull_fade_near_distance
    }
    fn cull_fade_near_distance_mut(&mut self) -> &mut f32 {
        &mut self.cull_fade_near_distance
    }
    fn cull_fade_near_range(&self) -> &f32 {
        &self.cull_fade_near_range
    }
    fn cull_fade_near_range_mut(&mut self) -> &mut f32 {
        &mut self.cull_fade_near_range
    }
    fn cull_fade_far_distance(&self) -> &f32 {
        &self.cull_fade_far_distance
    }
    fn cull_fade_far_distance_mut(&mut self) -> &mut f32 {
        &mut self.cull_fade_far_distance
    }
    fn cull_fade_far_range(&self) -> &f32 {
        &self.cull_fade_far_range
    }
    fn cull_fade_far_range_mut(&mut self) -> &mut f32 {
        &mut self.cull_fade_far_range
    }
    fn skip_near_camera_fade(&self) -> &bool {
        &self.skip_near_camera_fade
    }
    fn skip_near_camera_fade_mut(&mut self) -> &mut bool {
        &mut self.skip_near_camera_fade
    }
    fn emitter_wind_evaluation_enable(&self) -> &bool {
        &self.emitter_wind_evaluation_enable
    }
    fn emitter_wind_evaluation_enable_mut(&mut self) -> &mut bool {
        &mut self.emitter_wind_evaluation_enable
    }
    fn emittable_wind_evaluation_enable(&self) -> &bool {
        &self.emittable_wind_evaluation_enable
    }
    fn emittable_wind_evaluation_enable_mut(&mut self) -> &mut bool {
        &mut self.emittable_wind_evaluation_enable
    }
    fn debug_name(&self) -> &String {
        &self.debug_name
    }
    fn debug_name_mut(&mut self) -> &mut String {
        &mut self.debug_name
    }
    fn tweak_inherited_emitter(&self) -> &Option<Arc<Mutex<dyn EmitterDocumentTrait>>> {
        &self.tweak_inherited_emitter
    }
    fn tweak_inherited_emitter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterDocumentTrait>>> {
        &mut self.tweak_inherited_emitter
    }
}

impl super::core::DataContainerTrait for EmitterTemplateData {
}

pub static EMITTERTEMPLATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTemplateData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterTemplateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RootProcessor",
                flags: MemberInfoFlags::new(0),
                field_type: "ProcessorData",
                rust_offset: offset_of!(EmitterTemplateData, root_processor),
            },
            FieldInfoData {
                name: "MaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterTemplateData, max_count),
            },
            FieldInfoData {
                name: "Lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, lifetime),
            },
            FieldInfoData {
                name: "TimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, time_scale),
            },
            FieldInfoData {
                name: "RepeatParticleSpawning",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, repeat_particle_spawning),
            },
            FieldInfoData {
                name: "LifetimeFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterTemplateData, lifetime_frame_count),
            },
            FieldInfoData {
                name: "FollowSpawnSource",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, follow_spawn_source),
            },
            FieldInfoData {
                name: "FollowSpawnSourceVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, follow_spawn_source_velocity),
            },
            FieldInfoData {
                name: "KillParticlesWithEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, kill_particles_with_emitter),
            },
            FieldInfoData {
                name: "KillRibbonTailDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, kill_ribbon_tail_distance),
            },
            FieldInfoData {
                name: "SmoothRibbonSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, smooth_ribbon_spawn),
            },
            FieldInfoData {
                name: "ExclusionVolumeCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, exclusion_volume_cull_enable),
            },
            FieldInfoData {
                name: "EmittableType",
                flags: MemberInfoFlags::new(0),
                field_type: "EmittableType",
                rust_offset: offset_of!(EmitterTemplateData, emittable_type),
            },
            FieldInfoData {
                name: "EmittableAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: "EmittableAlignment",
                rust_offset: offset_of!(EmitterTemplateData, emittable_alignment),
            },
            FieldInfoData {
                name: "WorldAlignmentDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterTemplateData, world_alignment_direction),
            },
            FieldInfoData {
                name: "MotionStretchMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, motion_stretch_multiplier),
            },
            FieldInfoData {
                name: "MotionStretchViewMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, motion_stretch_view_multiplier),
            },
            FieldInfoData {
                name: "MotionStretchLengthClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, motion_stretch_length_clamp),
            },
            FieldInfoData {
                name: "MotionStretchRelativeLengthClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, motion_stretch_relative_length_clamp),
            },
            FieldInfoData {
                name: "OrientToPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterTemplateData, orient_to_position),
            },
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshAsset",
                rust_offset: offset_of!(EmitterTemplateData, mesh),
            },
            FieldInfoData {
                name: "ObjectVariationNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterTemplateData, object_variation_name_hash),
            },
            FieldInfoData {
                name: "Emissive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, emissive),
            },
            FieldInfoData {
                name: "EmissiveExposureFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, emissive_exposure_factor),
            },
            FieldInfoData {
                name: "Opaque",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, opaque),
            },
            FieldInfoData {
                name: "MeshParticlesMotionBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, mesh_particles_motion_blur),
            },
            FieldInfoData {
                name: "VertexPixelLightingBlendFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, vertex_pixel_lighting_blend_factor),
            },
            FieldInfoData {
                name: "GlobalLocalNormalBlendFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, global_local_normal_blend_factor),
            },
            FieldInfoData {
                name: "SoftParticlesFadeDistanceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, soft_particles_fade_distance_multiplier),
            },
            FieldInfoData {
                name: "LightWrapAroundFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, light_wrap_around_factor),
            },
            FieldInfoData {
                name: "BentNormalFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, bent_normal_factor),
            },
            FieldInfoData {
                name: "LightMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, light_multiplier),
            },
            FieldInfoData {
                name: "LightMultiplierDynamic",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, light_multiplier_dynamic),
            },
            FieldInfoData {
                name: "ReceiveSunShadow",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, receive_sun_shadow),
            },
            FieldInfoData {
                name: "BendingFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, bending_factor),
            },
            FieldInfoData {
                name: "MicroVariationSmoothingFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, micro_variation_smoothing_factor),
            },
            FieldInfoData {
                name: "ForceNiceSorting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, force_nice_sorting),
            },
            FieldInfoData {
                name: "LocalSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, local_space),
            },
            FieldInfoData {
                name: "AllowScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, allow_scale),
            },
            FieldInfoData {
                name: "CameraSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, camera_space),
            },
            FieldInfoData {
                name: "TransparencySunShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, transparency_sun_shadow_enable),
            },
            FieldInfoData {
                name: "SunVolumetricShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_enable),
            },
            FieldInfoData {
                name: "SunVolumetricShadowAbsorptionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_absorption_scale),
            },
            FieldInfoData {
                name: "SunVolumetricShadowAbsorptionR",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_absorption_r),
            },
            FieldInfoData {
                name: "SunVolumetricShadowAbsorptionG",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_absorption_g),
            },
            FieldInfoData {
                name: "SunVolumetricShadowAbsorptionB",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_absorption_b),
            },
            FieldInfoData {
                name: "SunVolumetricShadowPhaseG0",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_phase_g0),
            },
            FieldInfoData {
                name: "SunVolumetricShadowPhaseG1",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_phase_g1),
            },
            FieldInfoData {
                name: "SunVolumetricShadowOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, sun_volumetric_shadow_offset),
            },
            FieldInfoData {
                name: "EnablePyroShader",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, enable_pyro_shader),
            },
            FieldInfoData {
                name: "GnomonLightRigIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterTemplateData, gnomon_light_rig_index),
            },
            FieldInfoData {
                name: "UseRightTextureTile",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, use_right_texture_tile),
            },
            FieldInfoData {
                name: "CastPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, cast_planar_reflection_enable),
            },
            FieldInfoData {
                name: "ForceFullRes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, force_full_res),
            },
            FieldInfoData {
                name: "FogFade",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, fog_fade),
            },
            FieldInfoData {
                name: "CameraBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, camera_bias),
            },
            FieldInfoData {
                name: "EmitterDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterDrawOrder",
                rust_offset: offset_of!(EmitterTemplateData, emitter_draw_order),
            },
            FieldInfoData {
                name: "FlipUProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, flip_u_probability),
            },
            FieldInfoData {
                name: "FlipVProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, flip_v_probability),
            },
            FieldInfoData {
                name: "LockRibbonDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, lock_ribbon_direction),
            },
            FieldInfoData {
                name: "ParticleCullingFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, particle_culling_factor),
            },
            FieldInfoData {
                name: "Instanced",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, instanced),
            },
            FieldInfoData {
                name: "AlphaCullThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, alpha_cull_threshold),
            },
            FieldInfoData {
                name: "MinSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, min_spawn_distance),
            },
            FieldInfoData {
                name: "MaxSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, max_spawn_distance),
            },
            FieldInfoData {
                name: "MinScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, min_screen_area),
            },
            FieldInfoData {
                name: "MeshCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, mesh_culling_distance),
            },
            FieldInfoData {
                name: "PauseSimulationWhenCulled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, pause_simulation_when_culled),
            },
            FieldInfoData {
                name: "SkipUpdateMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterTemplateData, skip_update_max_count),
            },
            FieldInfoData {
                name: "SkipSimulationDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, skip_simulation_distance),
            },
            FieldInfoData {
                name: "PreciseWindAndForceMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, precise_wind_and_force_max_distance),
            },
            FieldInfoData {
                name: "TurbulenceMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, turbulence_max_distance),
            },
            FieldInfoData {
                name: "DistanceScaleLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, distance_scale_length),
            },
            FieldInfoData {
                name: "DistanceScaleNearValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, distance_scale_near_value),
            },
            FieldInfoData {
                name: "DistanceScaleFarValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, distance_scale_far_value),
            },
            FieldInfoData {
                name: "SpeedNormalizationValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, speed_normalization_value),
            },
            FieldInfoData {
                name: "WindSpeedNormalizationValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, wind_speed_normalization_value),
            },
            FieldInfoData {
                name: "TravelledDistanceNormalizationValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, travelled_distance_normalization_value),
            },
            FieldInfoData {
                name: "AcceptGlobalParameter1",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, accept_global_parameter1),
            },
            FieldInfoData {
                name: "AcceptGlobalParameter2",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, accept_global_parameter2),
            },
            FieldInfoData {
                name: "AcceptGlobalParameter3",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, accept_global_parameter3),
            },
            FieldInfoData {
                name: "PerParticleEffectParameters",
                flags: MemberInfoFlags::new(144),
                field_type: "EffectParameter-Array",
                rust_offset: offset_of!(EmitterTemplateData, per_particle_effect_parameters),
            },
            FieldInfoData {
                name: "MinDistanceTravelledBeforeSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, min_distance_travelled_before_spawn),
            },
            FieldInfoData {
                name: "CullFadeNearDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, cull_fade_near_distance),
            },
            FieldInfoData {
                name: "CullFadeNearRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, cull_fade_near_range),
            },
            FieldInfoData {
                name: "CullFadeFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, cull_fade_far_distance),
            },
            FieldInfoData {
                name: "CullFadeFarRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterTemplateData, cull_fade_far_range),
            },
            FieldInfoData {
                name: "SkipNearCameraFade",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, skip_near_camera_fade),
            },
            FieldInfoData {
                name: "EmitterWindEvaluationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, emitter_wind_evaluation_enable),
            },
            FieldInfoData {
                name: "EmittableWindEvaluationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterTemplateData, emittable_wind_evaluation_enable),
            },
            FieldInfoData {
                name: "DebugName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EmitterTemplateData, debug_name),
            },
            FieldInfoData {
                name: "TweakInheritedEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterDocument",
                rust_offset: offset_of!(EmitterTemplateData, tweak_inherited_emitter),
            },
        ],
    }),
    array_type: Some(EMITTERTEMPLATEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterTemplateData {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERTEMPLATEDATA_TYPE_INFO
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


pub static EMITTERTEMPLATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTemplateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterTemplateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PerParticleParams {
    #[default]
    FloatCount = 4,
}

pub static PERPARTICLEPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerParticleParams",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(PERPARTICLEPARAMS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PerParticleParams {
    fn type_info(&self) -> &'static TypeInfo {
        PERPARTICLEPARAMS_TYPE_INFO
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


pub static PERPARTICLEPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerParticleParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PerParticleParams"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProcessorData {
    pub _glacier_base: super::core::DataContainer,
    pub pre: Option<Arc<Mutex<dyn EvaluatorDataTrait>>>,
    pub next_processor: Option<Arc<Mutex<dyn ProcessorDataTrait>>>,
    pub evaluator_input: EmittableField,
    pub evaluator_input_param: Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>,
    pub schematics_enable: bool,
}

pub trait ProcessorDataTrait: super::core::DataContainerTrait {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>>;
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>>;
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>>;
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>>;
    fn evaluator_input(&self) -> &EmittableField;
    fn evaluator_input_mut(&mut self) -> &mut EmittableField;
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>;
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>;
    fn schematics_enable(&self) -> &bool;
    fn schematics_enable_mut(&mut self) -> &mut bool;
}

impl ProcessorDataTrait for ProcessorData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        &self.pre
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        &mut self.pre
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        &self.next_processor
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        &mut self.next_processor
    }
    fn evaluator_input(&self) -> &EmittableField {
        &self.evaluator_input
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        &mut self.evaluator_input
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        &self.evaluator_input_param
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        &mut self.evaluator_input_param
    }
    fn schematics_enable(&self) -> &bool {
        &self.schematics_enable
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        &mut self.schematics_enable
    }
}

impl super::core::DataContainerTrait for ProcessorData {
}

pub static PROCESSORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProcessorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProcessorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Pre",
                flags: MemberInfoFlags::new(0),
                field_type: "EvaluatorData",
                rust_offset: offset_of!(ProcessorData, pre),
            },
            FieldInfoData {
                name: "NextProcessor",
                flags: MemberInfoFlags::new(0),
                field_type: "ProcessorData",
                rust_offset: offset_of!(ProcessorData, next_processor),
            },
            FieldInfoData {
                name: "EvaluatorInput",
                flags: MemberInfoFlags::new(0),
                field_type: "EmittableField",
                rust_offset: offset_of!(ProcessorData, evaluator_input),
            },
            FieldInfoData {
                name: "EvaluatorInputParam",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParameter",
                rust_offset: offset_of!(ProcessorData, evaluator_input_param),
            },
            FieldInfoData {
                name: "SchematicsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProcessorData, schematics_enable),
            },
        ],
    }),
    array_type: Some(PROCESSORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProcessorData {
    fn type_info(&self) -> &'static TypeInfo {
        PROCESSORDATA_TYPE_INFO
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


pub static PROCESSORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProcessorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ProcessorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EvaluatorData {
    pub _glacier_base: super::core::DataContainer,
    pub parameter: Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>,
    pub schematics_enable: bool,
}

pub trait EvaluatorDataTrait: super::core::DataContainerTrait {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>;
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>;
    fn schematics_enable(&self) -> &bool;
    fn schematics_enable_mut(&mut self) -> &mut bool;
}

impl EvaluatorDataTrait for EvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        &self.parameter
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        &mut self.parameter
    }
    fn schematics_enable(&self) -> &bool {
        &self.schematics_enable
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        &mut self.schematics_enable
    }
}

impl super::core::DataContainerTrait for EvaluatorData {
}

pub static EVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Parameter",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParameter",
                rust_offset: offset_of!(EvaluatorData, parameter),
            },
            FieldInfoData {
                name: "SchematicsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EvaluatorData, schematics_enable),
            },
        ],
    }),
    array_type: Some(EVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        EVALUATORDATA_TYPE_INFO
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


pub static EVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ParticleSorting {
    #[default]
    ParticleSorting_CameraDistance = 0,
    ParticleSorting_NewToOld = 1,
    ParticleSorting_OldToNew = 2,
}

pub static PARTICLESORTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticleSorting",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(PARTICLESORTING_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ParticleSorting {
    fn type_info(&self) -> &'static TypeInfo {
        PARTICLESORTING_TYPE_INFO
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


pub static PARTICLESORTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticleSorting-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ParticleSorting"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterDrawOrder {
    #[default]
    EmitterDrawOrder_Default = 0,
    EmitterDrawOrder_Foreground = 1,
    EmitterDrawOrder_Background = 2,
}

pub static EMITTERDRAWORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDrawOrder",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERDRAWORDER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterDrawOrder {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERDRAWORDER_TYPE_INFO
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


pub static EMITTERDRAWORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDrawOrder-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterDrawOrder"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static EMITTERSCHEMATICINPUTPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSchematicInputParameter",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERSCHEMATICINPUTPARAMETER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterSchematicInputParameter {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERSCHEMATICINPUTPARAMETER_TYPE_INFO
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


pub static EMITTERSCHEMATICINPUTPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSchematicInputParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterSchematicInputParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static PROCESSORTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProcessorType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(PROCESSORTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProcessorType {
    fn type_info(&self) -> &'static TypeInfo {
        PROCESSORTYPE_TYPE_INFO
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


pub static PROCESSORTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProcessorType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ProcessorType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static EVALUATORTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluatorType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EVALUATORTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EvaluatorType {
    fn type_info(&self) -> &'static TypeInfo {
        EVALUATORTYPE_TYPE_INFO
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


pub static EVALUATORTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluatorType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EvaluatorType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RandomFrequency {
    #[default]
    RandomPerFrame = 0,
    RandomPerEmittable = 1,
    RandomPerInstance = 2,
}

pub static RANDOMFREQUENCY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomFrequency",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(RANDOMFREQUENCY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RandomFrequency {
    fn type_info(&self) -> &'static TypeInfo {
        RANDOMFREQUENCY_TYPE_INFO
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


pub static RANDOMFREQUENCY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomFrequency-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RandomFrequency"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static EMITTABLEALIGNMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableAlignment",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTABLEALIGNMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmittableAlignment {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTABLEALIGNMENT_TYPE_INFO
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


pub static EMITTABLEALIGNMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableAlignment-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmittableAlignment"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static EMITTABLETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTABLETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmittableType {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTABLETYPE_TYPE_INFO
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


pub static EMITTABLETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmittableType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static EMITTABLEFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableField",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTABLEFIELD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmittableField {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTABLEFIELD_TYPE_INFO
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


pub static EMITTABLEFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmittableField-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmittableField"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterExclusionVolumeBoundingSphereSoA {
    pub pos_x: super::core::Vec4,
    pub pos_y: super::core::Vec4,
    pub pos_z: super::core::Vec4,
    pub radius_sqr: super::core::Vec4,
}

pub trait EmitterExclusionVolumeBoundingSphereSoATrait: TypeObject {
    fn pos_x(&self) -> &super::core::Vec4;
    fn pos_x_mut(&mut self) -> &mut super::core::Vec4;
    fn pos_y(&self) -> &super::core::Vec4;
    fn pos_y_mut(&mut self) -> &mut super::core::Vec4;
    fn pos_z(&self) -> &super::core::Vec4;
    fn pos_z_mut(&mut self) -> &mut super::core::Vec4;
    fn radius_sqr(&self) -> &super::core::Vec4;
    fn radius_sqr_mut(&mut self) -> &mut super::core::Vec4;
}

impl EmitterExclusionVolumeBoundingSphereSoATrait for EmitterExclusionVolumeBoundingSphereSoA {
    fn pos_x(&self) -> &super::core::Vec4 {
        &self.pos_x
    }
    fn pos_x_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.pos_x
    }
    fn pos_y(&self) -> &super::core::Vec4 {
        &self.pos_y
    }
    fn pos_y_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.pos_y
    }
    fn pos_z(&self) -> &super::core::Vec4 {
        &self.pos_z
    }
    fn pos_z_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.pos_z
    }
    fn radius_sqr(&self) -> &super::core::Vec4 {
        &self.radius_sqr
    }
    fn radius_sqr_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.radius_sqr
    }
}

pub static EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeBoundingSphereSoA",
    flags: MemberInfoFlags::new(36937),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterExclusionVolumeBoundingSphereSoA as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PosX",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(EmitterExclusionVolumeBoundingSphereSoA, pos_x),
            },
            FieldInfoData {
                name: "PosY",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(EmitterExclusionVolumeBoundingSphereSoA, pos_y),
            },
            FieldInfoData {
                name: "PosZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(EmitterExclusionVolumeBoundingSphereSoA, pos_z),
            },
            FieldInfoData {
                name: "RadiusSqr",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(EmitterExclusionVolumeBoundingSphereSoA, radius_sqr),
            },
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterExclusionVolumeBoundingSphereSoA {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_TYPE_INFO
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


pub static EMITTEREXCLUSIONVOLUMEBOUNDINGSPHERESOA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeBoundingSphereSoA-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterExclusionVolumeBoundingSphereSoA"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterExclusionVolume {
    pub left: super::core::Vec4,
    pub up: super::core::Vec4,
    pub forward: super::core::Vec4,
    pub half_extents: super::core::Vec4,
    pub id: u32,
}

pub trait EmitterExclusionVolumeTrait: TypeObject {
    fn left(&self) -> &super::core::Vec4;
    fn left_mut(&mut self) -> &mut super::core::Vec4;
    fn up(&self) -> &super::core::Vec4;
    fn up_mut(&mut self) -> &mut super::core::Vec4;
    fn forward(&self) -> &super::core::Vec4;
    fn forward_mut(&mut self) -> &mut super::core::Vec4;
    fn half_extents(&self) -> &super::core::Vec4;
    fn half_extents_mut(&mut self) -> &mut super::core::Vec4;
    fn id(&self) -> &u32;
    fn id_mut(&mut self) -> &mut u32;
}

impl EmitterExclusionVolumeTrait for EmitterExclusionVolume {
    fn left(&self) -> &super::core::Vec4 {
        &self.left
    }
    fn left_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.left
    }
    fn up(&self) -> &super::core::Vec4 {
        &self.up
    }
    fn up_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.up
    }
    fn forward(&self) -> &super::core::Vec4 {
        &self.forward
    }
    fn forward_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.forward
    }
    fn half_extents(&self) -> &super::core::Vec4 {
        &self.half_extents
    }
    fn half_extents_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.half_extents
    }
    fn id(&self) -> &u32 {
        &self.id
    }
    fn id_mut(&mut self) -> &mut u32 {
        &mut self.id
    }
}

pub static EMITTEREXCLUSIONVOLUME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolume",
    flags: MemberInfoFlags::new(36937),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterExclusionVolume as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Left",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(EmitterExclusionVolume, left),
            },
            FieldInfoData {
                name: "Up",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(EmitterExclusionVolume, up),
            },
            FieldInfoData {
                name: "Forward",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(EmitterExclusionVolume, forward),
            },
            FieldInfoData {
                name: "HalfExtents",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(EmitterExclusionVolume, half_extents),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterExclusionVolume, id),
            },
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUME_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterExclusionVolume {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUME_TYPE_INFO
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


pub static EMITTEREXCLUSIONVOLUME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolume-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterExclusionVolume"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterGraph {
    pub _glacier_base: super::emitter_base::EmitterGraphBaseAsset,
    pub spawn_mode2: Option<Arc<Mutex<dyn SpawnModeInfoTrait>>>,
    pub use_node_graph: bool,
    pub graph_data: Option<Arc<Mutex<dyn super::expression::ExpressionNodeGraphDataTrait>>>,
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
    pub spawn_shader_override: glacier_reflect::builtin::FileRef,
    pub simulate_shader_override: glacier_reflect::builtin::FileRef,
    pub texture0: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub texture1: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
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
    pub mesh_vertex_shader_fragment_code_file: glacier_reflect::builtin::FileRef,
    pub effect_params: Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>>,
    pub emitter_graph_params: Vec<super::effect_base::EmitterExposedInput>,
    pub is_using_opaque_lit_root_node: bool,
    pub is_using_lit_root_node: bool,
    pub is_using_gpu_lighting: bool,
    pub compiled_spawn_graph_compute_shader: glacier_reflect::builtin::ResourceRef,
    pub compiled_simulate_graph_compute_shader: glacier_reflect::builtin::ResourceRef,
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

pub trait EmitterGraphTrait: super::emitter_base::EmitterGraphBaseAssetTrait {
    fn spawn_mode2(&self) -> &Option<Arc<Mutex<dyn SpawnModeInfoTrait>>>;
    fn spawn_mode2_mut(&mut self) -> &mut Option<Arc<Mutex<dyn SpawnModeInfoTrait>>>;
    fn use_node_graph(&self) -> &bool;
    fn use_node_graph_mut(&mut self) -> &mut bool;
    fn graph_data(&self) -> &Option<Arc<Mutex<dyn super::expression::ExpressionNodeGraphDataTrait>>>;
    fn graph_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::expression::ExpressionNodeGraphDataTrait>>>;
    fn spawn_mode(&self) -> &EmitterGraphSpawnMode;
    fn spawn_mode_mut(&mut self) -> &mut EmitterGraphSpawnMode;
    fn spawn_rate(&self) -> &super::core::QualityScalableFloat;
    fn spawn_rate_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn particle_max_count(&self) -> &super::core::QualityScalableInt;
    fn particle_max_count_mut(&mut self) -> &mut super::core::QualityScalableInt;
    fn particle_life_span(&self) -> &super::core::QualityScalableFloat;
    fn particle_life_span_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn planar_reflections_enabled(&self) -> &bool;
    fn planar_reflections_enabled_mut(&mut self) -> &mut bool;
    fn normalize_mesh_start_ids(&self) -> &bool;
    fn normalize_mesh_start_ids_mut(&mut self) -> &mut bool;
    fn meshes(&self) -> &Vec<EmitterGraphMesh>;
    fn meshes_mut(&mut self) -> &mut Vec<EmitterGraphMesh>;
    fn object_variation_name_hash(&self) -> &u32;
    fn object_variation_name_hash_mut(&mut self) -> &mut u32;
    fn requires_per_root_view_duplication(&self) -> &bool;
    fn requires_per_root_view_duplication_mut(&mut self) -> &mut bool;
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct;
    fn draw_pass(&self) -> &EmitterGraphDrawPass;
    fn draw_pass_mut(&mut self) -> &mut EmitterGraphDrawPass;
    fn draw_layer(&self) -> &EmitterGraphDrawLayer;
    fn draw_layer_mut(&mut self) -> &mut EmitterGraphDrawLayer;
    fn sort_mode(&self) -> &EmitterGraphSortMode;
    fn sort_mode_mut(&mut self) -> &mut EmitterGraphSortMode;
    fn user_buffers(&self) -> &Vec<EmitterGraphUserBuffer>;
    fn user_buffers_mut(&mut self) -> &mut Vec<EmitterGraphUserBuffer>;
    fn spawn_shader_override(&self) -> &glacier_reflect::builtin::FileRef;
    fn spawn_shader_override_mut(&mut self) -> &mut glacier_reflect::builtin::FileRef;
    fn simulate_shader_override(&self) -> &glacier_reflect::builtin::FileRef;
    fn simulate_shader_override_mut(&mut self) -> &mut glacier_reflect::builtin::FileRef;
    fn texture0(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn texture0_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn texture1(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn texture1_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn z_buffer_enable(&self) -> &bool;
    fn z_buffer_enable_mut(&mut self) -> &mut bool;
    fn emitter_life_span(&self) -> &super::core::QualityScalableFloat;
    fn emitter_life_span_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn kill_on_stop(&self) -> &bool;
    fn kill_on_stop_mut(&mut self) -> &mut bool;
    fn emitter_min_spawn_distance(&self) -> &super::core::QualityScalableFloat;
    fn emitter_min_spawn_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn emitter_max_spawn_distance(&self) -> &super::core::QualityScalableFloat;
    fn emitter_max_spawn_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn spawn_outside_view_radius(&self) -> &f32;
    fn spawn_outside_view_radius_mut(&mut self) -> &mut f32;
    fn bounding_box_min(&self) -> &super::core::Vec3;
    fn bounding_box_min_mut(&mut self) -> &mut super::core::Vec3;
    fn bounding_box_max(&self) -> &super::core::Vec3;
    fn bounding_box_max_mut(&mut self) -> &mut super::core::Vec3;
    fn culled_behavior(&self) -> &EmitterGraphCulledBehavior;
    fn culled_behavior_mut(&mut self) -> &mut EmitterGraphCulledBehavior;
    fn skip_update_max_count(&self) -> &i32;
    fn skip_update_max_count_mut(&mut self) -> &mut i32;
    fn emitter_mesh_culling_distance(&self) -> &f32;
    fn emitter_mesh_culling_distance_mut(&mut self) -> &mut f32;
    fn min_screen_area(&self) -> &f32;
    fn min_screen_area_mut(&mut self) -> &mut f32;
    fn gpu_particle_culling_enable(&self) -> &bool;
    fn gpu_particle_culling_enable_mut(&mut self) -> &mut bool;
    fn gpu_particle_culling_radius(&self) -> &f32;
    fn gpu_particle_culling_radius_mut(&mut self) -> &mut f32;
    fn gpu_particle_culling_distance(&self) -> &super::core::QualityScalableFloat;
    fn gpu_particle_culling_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn mesh_vertex_shader_fragment_code_file(&self) -> &glacier_reflect::builtin::FileRef;
    fn mesh_vertex_shader_fragment_code_file_mut(&mut self) -> &mut glacier_reflect::builtin::FileRef;
    fn effect_params(&self) -> &Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>>;
    fn effect_params_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>>;
    fn emitter_graph_params(&self) -> &Vec<super::effect_base::EmitterExposedInput>;
    fn emitter_graph_params_mut(&mut self) -> &mut Vec<super::effect_base::EmitterExposedInput>;
    fn is_using_opaque_lit_root_node(&self) -> &bool;
    fn is_using_opaque_lit_root_node_mut(&mut self) -> &mut bool;
    fn is_using_lit_root_node(&self) -> &bool;
    fn is_using_lit_root_node_mut(&mut self) -> &mut bool;
    fn is_using_gpu_lighting(&self) -> &bool;
    fn is_using_gpu_lighting_mut(&mut self) -> &mut bool;
    fn compiled_spawn_graph_compute_shader(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn compiled_spawn_graph_compute_shader_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
    fn compiled_simulate_graph_compute_shader(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn compiled_simulate_graph_compute_shader_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
    fn vertex_shader_fragment_asset_name(&self) -> &String;
    fn vertex_shader_fragment_asset_name_mut(&mut self) -> &mut String;
    fn mesh_vertex_shader_fragment_asset_name(&self) -> &String;
    fn mesh_vertex_shader_fragment_asset_name_mut(&mut self) -> &mut String;
    fn particle_data_byte_stride(&self) -> &u32;
    fn particle_data_byte_stride_mut(&mut self) -> &mut u32;
    fn particle_data_buffer_layout_hash(&self) -> &u32;
    fn particle_data_buffer_layout_hash_mut(&mut self) -> &mut u32;
    fn simulate_runtime_textures(&self) -> &Vec<RuntimeTexture>;
    fn simulate_runtime_textures_mut(&mut self) -> &mut Vec<RuntimeTexture>;
    fn simulate_runtime_samplers(&self) -> &Vec<RuntimeSampler>;
    fn simulate_runtime_samplers_mut(&mut self) -> &mut Vec<RuntimeSampler>;
    fn spawn_runtime_textures(&self) -> &Vec<RuntimeTexture>;
    fn spawn_runtime_textures_mut(&mut self) -> &mut Vec<RuntimeTexture>;
    fn spawn_runtime_samplers(&self) -> &Vec<RuntimeSampler>;
    fn spawn_runtime_samplers_mut(&mut self) -> &mut Vec<RuntimeSampler>;
    fn vertex_shader_runtime_textures(&self) -> &Vec<RuntimeTexture>;
    fn vertex_shader_runtime_textures_mut(&mut self) -> &mut Vec<RuntimeTexture>;
    fn runtime_particle_data_buffers(&self) -> &Vec<RuntimeParticleDataBuffer>;
    fn runtime_particle_data_buffers_mut(&mut self) -> &mut Vec<RuntimeParticleDataBuffer>;
}

impl EmitterGraphTrait for EmitterGraph {
    fn spawn_mode2(&self) -> &Option<Arc<Mutex<dyn SpawnModeInfoTrait>>> {
        &self.spawn_mode2
    }
    fn spawn_mode2_mut(&mut self) -> &mut Option<Arc<Mutex<dyn SpawnModeInfoTrait>>> {
        &mut self.spawn_mode2
    }
    fn use_node_graph(&self) -> &bool {
        &self.use_node_graph
    }
    fn use_node_graph_mut(&mut self) -> &mut bool {
        &mut self.use_node_graph
    }
    fn graph_data(&self) -> &Option<Arc<Mutex<dyn super::expression::ExpressionNodeGraphDataTrait>>> {
        &self.graph_data
    }
    fn graph_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::expression::ExpressionNodeGraphDataTrait>>> {
        &mut self.graph_data
    }
    fn spawn_mode(&self) -> &EmitterGraphSpawnMode {
        &self.spawn_mode
    }
    fn spawn_mode_mut(&mut self) -> &mut EmitterGraphSpawnMode {
        &mut self.spawn_mode
    }
    fn spawn_rate(&self) -> &super::core::QualityScalableFloat {
        &self.spawn_rate
    }
    fn spawn_rate_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.spawn_rate
    }
    fn particle_max_count(&self) -> &super::core::QualityScalableInt {
        &self.particle_max_count
    }
    fn particle_max_count_mut(&mut self) -> &mut super::core::QualityScalableInt {
        &mut self.particle_max_count
    }
    fn particle_life_span(&self) -> &super::core::QualityScalableFloat {
        &self.particle_life_span
    }
    fn particle_life_span_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.particle_life_span
    }
    fn planar_reflections_enabled(&self) -> &bool {
        &self.planar_reflections_enabled
    }
    fn planar_reflections_enabled_mut(&mut self) -> &mut bool {
        &mut self.planar_reflections_enabled
    }
    fn normalize_mesh_start_ids(&self) -> &bool {
        &self.normalize_mesh_start_ids
    }
    fn normalize_mesh_start_ids_mut(&mut self) -> &mut bool {
        &mut self.normalize_mesh_start_ids
    }
    fn meshes(&self) -> &Vec<EmitterGraphMesh> {
        &self.meshes
    }
    fn meshes_mut(&mut self) -> &mut Vec<EmitterGraphMesh> {
        &mut self.meshes
    }
    fn object_variation_name_hash(&self) -> &u32 {
        &self.object_variation_name_hash
    }
    fn object_variation_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.object_variation_name_hash
    }
    fn requires_per_root_view_duplication(&self) -> &bool {
        &self.requires_per_root_view_duplication
    }
    fn requires_per_root_view_duplication_mut(&mut self) -> &mut bool {
        &mut self.requires_per_root_view_duplication
    }
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct {
        &mut self.shader
    }
    fn draw_pass(&self) -> &EmitterGraphDrawPass {
        &self.draw_pass
    }
    fn draw_pass_mut(&mut self) -> &mut EmitterGraphDrawPass {
        &mut self.draw_pass
    }
    fn draw_layer(&self) -> &EmitterGraphDrawLayer {
        &self.draw_layer
    }
    fn draw_layer_mut(&mut self) -> &mut EmitterGraphDrawLayer {
        &mut self.draw_layer
    }
    fn sort_mode(&self) -> &EmitterGraphSortMode {
        &self.sort_mode
    }
    fn sort_mode_mut(&mut self) -> &mut EmitterGraphSortMode {
        &mut self.sort_mode
    }
    fn user_buffers(&self) -> &Vec<EmitterGraphUserBuffer> {
        &self.user_buffers
    }
    fn user_buffers_mut(&mut self) -> &mut Vec<EmitterGraphUserBuffer> {
        &mut self.user_buffers
    }
    fn spawn_shader_override(&self) -> &glacier_reflect::builtin::FileRef {
        &self.spawn_shader_override
    }
    fn spawn_shader_override_mut(&mut self) -> &mut glacier_reflect::builtin::FileRef {
        &mut self.spawn_shader_override
    }
    fn simulate_shader_override(&self) -> &glacier_reflect::builtin::FileRef {
        &self.simulate_shader_override
    }
    fn simulate_shader_override_mut(&mut self) -> &mut glacier_reflect::builtin::FileRef {
        &mut self.simulate_shader_override
    }
    fn texture0(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture0
    }
    fn texture0_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &mut self.texture0
    }
    fn texture1(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture1
    }
    fn texture1_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &mut self.texture1
    }
    fn z_buffer_enable(&self) -> &bool {
        &self.z_buffer_enable
    }
    fn z_buffer_enable_mut(&mut self) -> &mut bool {
        &mut self.z_buffer_enable
    }
    fn emitter_life_span(&self) -> &super::core::QualityScalableFloat {
        &self.emitter_life_span
    }
    fn emitter_life_span_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.emitter_life_span
    }
    fn kill_on_stop(&self) -> &bool {
        &self.kill_on_stop
    }
    fn kill_on_stop_mut(&mut self) -> &mut bool {
        &mut self.kill_on_stop
    }
    fn emitter_min_spawn_distance(&self) -> &super::core::QualityScalableFloat {
        &self.emitter_min_spawn_distance
    }
    fn emitter_min_spawn_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.emitter_min_spawn_distance
    }
    fn emitter_max_spawn_distance(&self) -> &super::core::QualityScalableFloat {
        &self.emitter_max_spawn_distance
    }
    fn emitter_max_spawn_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.emitter_max_spawn_distance
    }
    fn spawn_outside_view_radius(&self) -> &f32 {
        &self.spawn_outside_view_radius
    }
    fn spawn_outside_view_radius_mut(&mut self) -> &mut f32 {
        &mut self.spawn_outside_view_radius
    }
    fn bounding_box_min(&self) -> &super::core::Vec3 {
        &self.bounding_box_min
    }
    fn bounding_box_min_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.bounding_box_min
    }
    fn bounding_box_max(&self) -> &super::core::Vec3 {
        &self.bounding_box_max
    }
    fn bounding_box_max_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.bounding_box_max
    }
    fn culled_behavior(&self) -> &EmitterGraphCulledBehavior {
        &self.culled_behavior
    }
    fn culled_behavior_mut(&mut self) -> &mut EmitterGraphCulledBehavior {
        &mut self.culled_behavior
    }
    fn skip_update_max_count(&self) -> &i32 {
        &self.skip_update_max_count
    }
    fn skip_update_max_count_mut(&mut self) -> &mut i32 {
        &mut self.skip_update_max_count
    }
    fn emitter_mesh_culling_distance(&self) -> &f32 {
        &self.emitter_mesh_culling_distance
    }
    fn emitter_mesh_culling_distance_mut(&mut self) -> &mut f32 {
        &mut self.emitter_mesh_culling_distance
    }
    fn min_screen_area(&self) -> &f32 {
        &self.min_screen_area
    }
    fn min_screen_area_mut(&mut self) -> &mut f32 {
        &mut self.min_screen_area
    }
    fn gpu_particle_culling_enable(&self) -> &bool {
        &self.gpu_particle_culling_enable
    }
    fn gpu_particle_culling_enable_mut(&mut self) -> &mut bool {
        &mut self.gpu_particle_culling_enable
    }
    fn gpu_particle_culling_radius(&self) -> &f32 {
        &self.gpu_particle_culling_radius
    }
    fn gpu_particle_culling_radius_mut(&mut self) -> &mut f32 {
        &mut self.gpu_particle_culling_radius
    }
    fn gpu_particle_culling_distance(&self) -> &super::core::QualityScalableFloat {
        &self.gpu_particle_culling_distance
    }
    fn gpu_particle_culling_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.gpu_particle_culling_distance
    }
    fn mesh_vertex_shader_fragment_code_file(&self) -> &glacier_reflect::builtin::FileRef {
        &self.mesh_vertex_shader_fragment_code_file
    }
    fn mesh_vertex_shader_fragment_code_file_mut(&mut self) -> &mut glacier_reflect::builtin::FileRef {
        &mut self.mesh_vertex_shader_fragment_code_file
    }
    fn effect_params(&self) -> &Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>> {
        &self.effect_params
    }
    fn effect_params_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>> {
        &mut self.effect_params
    }
    fn emitter_graph_params(&self) -> &Vec<super::effect_base::EmitterExposedInput> {
        &self.emitter_graph_params
    }
    fn emitter_graph_params_mut(&mut self) -> &mut Vec<super::effect_base::EmitterExposedInput> {
        &mut self.emitter_graph_params
    }
    fn is_using_opaque_lit_root_node(&self) -> &bool {
        &self.is_using_opaque_lit_root_node
    }
    fn is_using_opaque_lit_root_node_mut(&mut self) -> &mut bool {
        &mut self.is_using_opaque_lit_root_node
    }
    fn is_using_lit_root_node(&self) -> &bool {
        &self.is_using_lit_root_node
    }
    fn is_using_lit_root_node_mut(&mut self) -> &mut bool {
        &mut self.is_using_lit_root_node
    }
    fn is_using_gpu_lighting(&self) -> &bool {
        &self.is_using_gpu_lighting
    }
    fn is_using_gpu_lighting_mut(&mut self) -> &mut bool {
        &mut self.is_using_gpu_lighting
    }
    fn compiled_spawn_graph_compute_shader(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.compiled_spawn_graph_compute_shader
    }
    fn compiled_spawn_graph_compute_shader_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.compiled_spawn_graph_compute_shader
    }
    fn compiled_simulate_graph_compute_shader(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.compiled_simulate_graph_compute_shader
    }
    fn compiled_simulate_graph_compute_shader_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.compiled_simulate_graph_compute_shader
    }
    fn vertex_shader_fragment_asset_name(&self) -> &String {
        &self.vertex_shader_fragment_asset_name
    }
    fn vertex_shader_fragment_asset_name_mut(&mut self) -> &mut String {
        &mut self.vertex_shader_fragment_asset_name
    }
    fn mesh_vertex_shader_fragment_asset_name(&self) -> &String {
        &self.mesh_vertex_shader_fragment_asset_name
    }
    fn mesh_vertex_shader_fragment_asset_name_mut(&mut self) -> &mut String {
        &mut self.mesh_vertex_shader_fragment_asset_name
    }
    fn particle_data_byte_stride(&self) -> &u32 {
        &self.particle_data_byte_stride
    }
    fn particle_data_byte_stride_mut(&mut self) -> &mut u32 {
        &mut self.particle_data_byte_stride
    }
    fn particle_data_buffer_layout_hash(&self) -> &u32 {
        &self.particle_data_buffer_layout_hash
    }
    fn particle_data_buffer_layout_hash_mut(&mut self) -> &mut u32 {
        &mut self.particle_data_buffer_layout_hash
    }
    fn simulate_runtime_textures(&self) -> &Vec<RuntimeTexture> {
        &self.simulate_runtime_textures
    }
    fn simulate_runtime_textures_mut(&mut self) -> &mut Vec<RuntimeTexture> {
        &mut self.simulate_runtime_textures
    }
    fn simulate_runtime_samplers(&self) -> &Vec<RuntimeSampler> {
        &self.simulate_runtime_samplers
    }
    fn simulate_runtime_samplers_mut(&mut self) -> &mut Vec<RuntimeSampler> {
        &mut self.simulate_runtime_samplers
    }
    fn spawn_runtime_textures(&self) -> &Vec<RuntimeTexture> {
        &self.spawn_runtime_textures
    }
    fn spawn_runtime_textures_mut(&mut self) -> &mut Vec<RuntimeTexture> {
        &mut self.spawn_runtime_textures
    }
    fn spawn_runtime_samplers(&self) -> &Vec<RuntimeSampler> {
        &self.spawn_runtime_samplers
    }
    fn spawn_runtime_samplers_mut(&mut self) -> &mut Vec<RuntimeSampler> {
        &mut self.spawn_runtime_samplers
    }
    fn vertex_shader_runtime_textures(&self) -> &Vec<RuntimeTexture> {
        &self.vertex_shader_runtime_textures
    }
    fn vertex_shader_runtime_textures_mut(&mut self) -> &mut Vec<RuntimeTexture> {
        &mut self.vertex_shader_runtime_textures
    }
    fn runtime_particle_data_buffers(&self) -> &Vec<RuntimeParticleDataBuffer> {
        &self.runtime_particle_data_buffers
    }
    fn runtime_particle_data_buffers_mut(&mut self) -> &mut Vec<RuntimeParticleDataBuffer> {
        &mut self.runtime_particle_data_buffers
    }
}

impl super::emitter_base::EmitterGraphBaseAssetTrait for EmitterGraph {
}

impl super::core::AssetTrait for EmitterGraph {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EmitterGraph {
}

pub static EMITTERGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraph",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::emitter_base::EMITTERGRAPHBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterGraph as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SpawnMode2",
                flags: MemberInfoFlags::new(0),
                field_type: "SpawnModeInfo",
                rust_offset: offset_of!(EmitterGraph, spawn_mode2),
            },
            FieldInfoData {
                name: "UseNodeGraph",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraph, use_node_graph),
            },
            FieldInfoData {
                name: "GraphData",
                flags: MemberInfoFlags::new(0),
                field_type: "ExpressionNodeGraphData",
                rust_offset: offset_of!(EmitterGraph, graph_data),
            },
            FieldInfoData {
                name: "SpawnMode",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphSpawnMode",
                rust_offset: offset_of!(EmitterGraph, spawn_mode),
            },
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraph, spawn_rate),
            },
            FieldInfoData {
                name: "ParticleMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(EmitterGraph, particle_max_count),
            },
            FieldInfoData {
                name: "ParticleLifeSpan",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraph, particle_life_span),
            },
            FieldInfoData {
                name: "PlanarReflectionsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraph, planar_reflections_enabled),
            },
            FieldInfoData {
                name: "NormalizeMeshStartIds",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraph, normalize_mesh_start_ids),
            },
            FieldInfoData {
                name: "Meshes",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterGraphMesh-Array",
                rust_offset: offset_of!(EmitterGraph, meshes),
            },
            FieldInfoData {
                name: "ObjectVariationNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterGraph, object_variation_name_hash),
            },
            FieldInfoData {
                name: "RequiresPerRootViewDuplication",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraph, requires_per_root_view_duplication),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(EmitterGraph, shader),
            },
            FieldInfoData {
                name: "DrawPass",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphDrawPass",
                rust_offset: offset_of!(EmitterGraph, draw_pass),
            },
            FieldInfoData {
                name: "DrawLayer",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphDrawLayer",
                rust_offset: offset_of!(EmitterGraph, draw_layer),
            },
            FieldInfoData {
                name: "SortMode",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphSortMode",
                rust_offset: offset_of!(EmitterGraph, sort_mode),
            },
            FieldInfoData {
                name: "UserBuffers",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterGraphUserBuffer-Array",
                rust_offset: offset_of!(EmitterGraph, user_buffers),
            },
            FieldInfoData {
                name: "SpawnShaderOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "FileRef",
                rust_offset: offset_of!(EmitterGraph, spawn_shader_override),
            },
            FieldInfoData {
                name: "SimulateShaderOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "FileRef",
                rust_offset: offset_of!(EmitterGraph, simulate_shader_override),
            },
            FieldInfoData {
                name: "Texture0",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(EmitterGraph, texture0),
            },
            FieldInfoData {
                name: "Texture1",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(EmitterGraph, texture1),
            },
            FieldInfoData {
                name: "ZBufferEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraph, z_buffer_enable),
            },
            FieldInfoData {
                name: "EmitterLifeSpan",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraph, emitter_life_span),
            },
            FieldInfoData {
                name: "KillOnStop",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraph, kill_on_stop),
            },
            FieldInfoData {
                name: "EmitterMinSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraph, emitter_min_spawn_distance),
            },
            FieldInfoData {
                name: "EmitterMaxSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraph, emitter_max_spawn_distance),
            },
            FieldInfoData {
                name: "SpawnOutsideViewRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterGraph, spawn_outside_view_radius),
            },
            FieldInfoData {
                name: "BoundingBoxMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterGraph, bounding_box_min),
            },
            FieldInfoData {
                name: "BoundingBoxMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterGraph, bounding_box_max),
            },
            FieldInfoData {
                name: "CulledBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphCulledBehavior",
                rust_offset: offset_of!(EmitterGraph, culled_behavior),
            },
            FieldInfoData {
                name: "SkipUpdateMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterGraph, skip_update_max_count),
            },
            FieldInfoData {
                name: "EmitterMeshCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterGraph, emitter_mesh_culling_distance),
            },
            FieldInfoData {
                name: "MinScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterGraph, min_screen_area),
            },
            FieldInfoData {
                name: "GpuParticleCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraph, gpu_particle_culling_enable),
            },
            FieldInfoData {
                name: "GpuParticleCullingRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterGraph, gpu_particle_culling_radius),
            },
            FieldInfoData {
                name: "GpuParticleCullingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraph, gpu_particle_culling_distance),
            },
            FieldInfoData {
                name: "MeshVertexShaderFragmentCodeFile",
                flags: MemberInfoFlags::new(0),
                field_type: "FileRef",
                rust_offset: offset_of!(EmitterGraph, mesh_vertex_shader_fragment_code_file),
            },
            FieldInfoData {
                name: "EffectParams",
                flags: MemberInfoFlags::new(144),
                field_type: "EffectParameter-Array",
                rust_offset: offset_of!(EmitterGraph, effect_params),
            },
            FieldInfoData {
                name: "EmitterGraphParams",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterExposedInput-Array",
                rust_offset: offset_of!(EmitterGraph, emitter_graph_params),
            },
            FieldInfoData {
                name: "IsUsingOpaqueLitRootNode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraph, is_using_opaque_lit_root_node),
            },
            FieldInfoData {
                name: "IsUsingLitRootNode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraph, is_using_lit_root_node),
            },
            FieldInfoData {
                name: "IsUsingGpuLighting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraph, is_using_gpu_lighting),
            },
            FieldInfoData {
                name: "CompiledSpawnGraphComputeShader",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(EmitterGraph, compiled_spawn_graph_compute_shader),
            },
            FieldInfoData {
                name: "CompiledSimulateGraphComputeShader",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(EmitterGraph, compiled_simulate_graph_compute_shader),
            },
            FieldInfoData {
                name: "VertexShaderFragmentAssetName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EmitterGraph, vertex_shader_fragment_asset_name),
            },
            FieldInfoData {
                name: "MeshVertexShaderFragmentAssetName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EmitterGraph, mesh_vertex_shader_fragment_asset_name),
            },
            FieldInfoData {
                name: "ParticleDataByteStride",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterGraph, particle_data_byte_stride),
            },
            FieldInfoData {
                name: "ParticleDataBufferLayoutHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterGraph, particle_data_buffer_layout_hash),
            },
            FieldInfoData {
                name: "SimulateRuntimeTextures",
                flags: MemberInfoFlags::new(144),
                field_type: "RuntimeTexture-Array",
                rust_offset: offset_of!(EmitterGraph, simulate_runtime_textures),
            },
            FieldInfoData {
                name: "SimulateRuntimeSamplers",
                flags: MemberInfoFlags::new(144),
                field_type: "RuntimeSampler-Array",
                rust_offset: offset_of!(EmitterGraph, simulate_runtime_samplers),
            },
            FieldInfoData {
                name: "SpawnRuntimeTextures",
                flags: MemberInfoFlags::new(144),
                field_type: "RuntimeTexture-Array",
                rust_offset: offset_of!(EmitterGraph, spawn_runtime_textures),
            },
            FieldInfoData {
                name: "SpawnRuntimeSamplers",
                flags: MemberInfoFlags::new(144),
                field_type: "RuntimeSampler-Array",
                rust_offset: offset_of!(EmitterGraph, spawn_runtime_samplers),
            },
            FieldInfoData {
                name: "VertexShaderRuntimeTextures",
                flags: MemberInfoFlags::new(144),
                field_type: "RuntimeTexture-Array",
                rust_offset: offset_of!(EmitterGraph, vertex_shader_runtime_textures),
            },
            FieldInfoData {
                name: "RuntimeParticleDataBuffers",
                flags: MemberInfoFlags::new(144),
                field_type: "RuntimeParticleDataBuffer-Array",
                rust_offset: offset_of!(EmitterGraph, runtime_particle_data_buffers),
            },
        ],
    }),
    array_type: Some(EMITTERGRAPH_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterGraph {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPH_TYPE_INFO
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


pub static EMITTERGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraph-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraph"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RuntimeParticleDataBuffer {
    pub shader_parameter_handle: u32,
    pub bind_point: u8,
    pub buffer_type: EmitterGraphParticleDataType,
}

pub trait RuntimeParticleDataBufferTrait: TypeObject {
    fn shader_parameter_handle(&self) -> &u32;
    fn shader_parameter_handle_mut(&mut self) -> &mut u32;
    fn bind_point(&self) -> &u8;
    fn bind_point_mut(&mut self) -> &mut u8;
    fn buffer_type(&self) -> &EmitterGraphParticleDataType;
    fn buffer_type_mut(&mut self) -> &mut EmitterGraphParticleDataType;
}

impl RuntimeParticleDataBufferTrait for RuntimeParticleDataBuffer {
    fn shader_parameter_handle(&self) -> &u32 {
        &self.shader_parameter_handle
    }
    fn shader_parameter_handle_mut(&mut self) -> &mut u32 {
        &mut self.shader_parameter_handle
    }
    fn bind_point(&self) -> &u8 {
        &self.bind_point
    }
    fn bind_point_mut(&mut self) -> &mut u8 {
        &mut self.bind_point
    }
    fn buffer_type(&self) -> &EmitterGraphParticleDataType {
        &self.buffer_type
    }
    fn buffer_type_mut(&mut self) -> &mut EmitterGraphParticleDataType {
        &mut self.buffer_type
    }
}

pub static RUNTIMEPARTICLEDATABUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeParticleDataBuffer",
    flags: MemberInfoFlags::new(32841),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RuntimeParticleDataBuffer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ShaderParameterHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RuntimeParticleDataBuffer, shader_parameter_handle),
            },
            FieldInfoData {
                name: "BindPoint",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RuntimeParticleDataBuffer, bind_point),
            },
            FieldInfoData {
                name: "BufferType",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphParticleDataType",
                rust_offset: offset_of!(RuntimeParticleDataBuffer, buffer_type),
            },
        ],
    }),
    array_type: Some(RUNTIMEPARTICLEDATABUFFER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RuntimeParticleDataBuffer {
    fn type_info(&self) -> &'static TypeInfo {
        RUNTIMEPARTICLEDATABUFFER_TYPE_INFO
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


pub static RUNTIMEPARTICLEDATABUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeParticleDataBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RuntimeParticleDataBuffer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RuntimeSampler {
    pub bind_point: u8,
    pub filter: super::render::TextureFilter,
    pub address: super::render_base::TextureAddress,
}

pub trait RuntimeSamplerTrait: TypeObject {
    fn bind_point(&self) -> &u8;
    fn bind_point_mut(&mut self) -> &mut u8;
    fn filter(&self) -> &super::render::TextureFilter;
    fn filter_mut(&mut self) -> &mut super::render::TextureFilter;
    fn address(&self) -> &super::render_base::TextureAddress;
    fn address_mut(&mut self) -> &mut super::render_base::TextureAddress;
}

impl RuntimeSamplerTrait for RuntimeSampler {
    fn bind_point(&self) -> &u8 {
        &self.bind_point
    }
    fn bind_point_mut(&mut self) -> &mut u8 {
        &mut self.bind_point
    }
    fn filter(&self) -> &super::render::TextureFilter {
        &self.filter
    }
    fn filter_mut(&mut self) -> &mut super::render::TextureFilter {
        &mut self.filter
    }
    fn address(&self) -> &super::render_base::TextureAddress {
        &self.address
    }
    fn address_mut(&mut self) -> &mut super::render_base::TextureAddress {
        &mut self.address
    }
}

pub static RUNTIMESAMPLER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeSampler",
    flags: MemberInfoFlags::new(36937),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RuntimeSampler as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BindPoint",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RuntimeSampler, bind_point),
            },
            FieldInfoData {
                name: "Filter",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureFilter",
                rust_offset: offset_of!(RuntimeSampler, filter),
            },
            FieldInfoData {
                name: "Address",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAddress",
                rust_offset: offset_of!(RuntimeSampler, address),
            },
        ],
    }),
    array_type: Some(RUNTIMESAMPLER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RuntimeSampler {
    fn type_info(&self) -> &'static TypeInfo {
        RUNTIMESAMPLER_TYPE_INFO
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


pub static RUNTIMESAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeSampler-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RuntimeSampler"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RuntimeTexture {
    pub bind_point: u8,
    pub shader_parameter_handle: u32,
    pub texture_type: super::render::TextureType,
    pub texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
}

pub trait RuntimeTextureTrait: TypeObject {
    fn bind_point(&self) -> &u8;
    fn bind_point_mut(&mut self) -> &mut u8;
    fn shader_parameter_handle(&self) -> &u32;
    fn shader_parameter_handle_mut(&mut self) -> &mut u32;
    fn texture_type(&self) -> &super::render::TextureType;
    fn texture_type_mut(&mut self) -> &mut super::render::TextureType;
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
}

impl RuntimeTextureTrait for RuntimeTexture {
    fn bind_point(&self) -> &u8 {
        &self.bind_point
    }
    fn bind_point_mut(&mut self) -> &mut u8 {
        &mut self.bind_point
    }
    fn shader_parameter_handle(&self) -> &u32 {
        &self.shader_parameter_handle
    }
    fn shader_parameter_handle_mut(&mut self) -> &mut u32 {
        &mut self.shader_parameter_handle
    }
    fn texture_type(&self) -> &super::render::TextureType {
        &self.texture_type
    }
    fn texture_type_mut(&mut self) -> &mut super::render::TextureType {
        &mut self.texture_type
    }
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &mut self.texture
    }
}

pub static RUNTIMETEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeTexture",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RuntimeTexture as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BindPoint",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RuntimeTexture, bind_point),
            },
            FieldInfoData {
                name: "ShaderParameterHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RuntimeTexture, shader_parameter_handle),
            },
            FieldInfoData {
                name: "TextureType",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureType",
                rust_offset: offset_of!(RuntimeTexture, texture_type),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(RuntimeTexture, texture),
            },
        ],
    }),
    array_type: Some(RUNTIMETEXTURE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RuntimeTexture {
    fn type_info(&self) -> &'static TypeInfo {
        RUNTIMETEXTURE_TYPE_INFO
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


pub static RUNTIMETEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RuntimeTexture"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterGraphMesh {
    pub mesh: Option<Arc<Mutex<dyn super::render::MeshAssetTrait>>>,
    pub object_variation: Option<Arc<Mutex<dyn super::core::AssetTrait>>>,
    pub object_variation_name_hash: u32,
    pub start_id: u32,
}

pub trait EmitterGraphMeshTrait: TypeObject {
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render::MeshAssetTrait>>>;
    fn mesh_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::MeshAssetTrait>>>;
    fn object_variation(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>>;
    fn object_variation_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::AssetTrait>>>;
    fn object_variation_name_hash(&self) -> &u32;
    fn object_variation_name_hash_mut(&mut self) -> &mut u32;
    fn start_id(&self) -> &u32;
    fn start_id_mut(&mut self) -> &mut u32;
}

impl EmitterGraphMeshTrait for EmitterGraphMesh {
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render::MeshAssetTrait>>> {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::MeshAssetTrait>>> {
        &mut self.mesh
    }
    fn object_variation(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        &self.object_variation
    }
    fn object_variation_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        &mut self.object_variation
    }
    fn object_variation_name_hash(&self) -> &u32 {
        &self.object_variation_name_hash
    }
    fn object_variation_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.object_variation_name_hash
    }
    fn start_id(&self) -> &u32 {
        &self.start_id
    }
    fn start_id_mut(&mut self) -> &mut u32 {
        &mut self.start_id
    }
}

pub static EMITTERGRAPHMESH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphMesh",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterGraphMesh as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshAsset",
                rust_offset: offset_of!(EmitterGraphMesh, mesh),
            },
            FieldInfoData {
                name: "ObjectVariation",
                flags: MemberInfoFlags::new(0),
                field_type: "Asset",
                rust_offset: offset_of!(EmitterGraphMesh, object_variation),
            },
            FieldInfoData {
                name: "ObjectVariationNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterGraphMesh, object_variation_name_hash),
            },
            FieldInfoData {
                name: "StartId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterGraphMesh, start_id),
            },
        ],
    }),
    array_type: Some(EMITTERGRAPHMESH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterGraphMesh {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHMESH_TYPE_INFO
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


pub static EMITTERGRAPHMESH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphMesh-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphMesh"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterGraphUserBuffer {
    pub bind_point: u8,
    pub name: String,
}

pub trait EmitterGraphUserBufferTrait: TypeObject {
    fn bind_point(&self) -> &u8;
    fn bind_point_mut(&mut self) -> &mut u8;
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
}

impl EmitterGraphUserBufferTrait for EmitterGraphUserBuffer {
    fn bind_point(&self) -> &u8 {
        &self.bind_point
    }
    fn bind_point_mut(&mut self) -> &mut u8 {
        &mut self.bind_point
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

pub static EMITTERGRAPHUSERBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphUserBuffer",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterGraphUserBuffer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BindPoint",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EmitterGraphUserBuffer, bind_point),
            },
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EmitterGraphUserBuffer, name),
            },
        ],
    }),
    array_type: Some(EMITTERGRAPHUSERBUFFER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterGraphUserBuffer {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHUSERBUFFER_TYPE_INFO
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


pub static EMITTERGRAPHUSERBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphUserBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphUserBuffer"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterGraphSpawnMode {
    #[default]
    EmitterGraphSpawnMode_Continuous = 0,
    EmitterGraphSpawnMode_SingleBurst = 1,
}

pub static EMITTERGRAPHSPAWNMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSpawnMode",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHSPAWNMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphSpawnMode {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHSPAWNMODE_TYPE_INFO
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


pub static EMITTERGRAPHSPAWNMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSpawnMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphSpawnMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterGraphSortMode {
    #[default]
    EmitterGraphSortMode_Default = 0,
    EmitterGraphSortMode_Disable = 1,
    EmitterGraphSortMode_BackToFront = 2,
    EmitterGraphSortMode_Lifetime = 3,
    EmitterGraphSortMode_LifetimeInverse = 4,
}

pub static EMITTERGRAPHSORTMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSortMode",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHSORTMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphSortMode {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHSORTMODE_TYPE_INFO
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


pub static EMITTERGRAPHSORTMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSortMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphSortMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterGraphDrawLayer {
    #[default]
    EmitterGraphDrawLayer_Background = 0,
    EmitterGraphDrawLayer_Default = 1,
    EmitterGraphDrawLayer_Foreground = 2,
}

pub static EMITTERGRAPHDRAWLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphDrawLayer",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHDRAWLAYER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphDrawLayer {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHDRAWLAYER_TYPE_INFO
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


pub static EMITTERGRAPHDRAWLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphDrawLayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphDrawLayer"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterGraphDrawPass {
    #[default]
    EmitterGraphDrawPass_Main = 0,
    EmitterGraphDrawPass_HalfResolution = 1,
    EmitterGraphDrawPass_Foreground = 2,
    EmitterGraphDrawPass_Hologram = 3,
}

pub static EMITTERGRAPHDRAWPASS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphDrawPass",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHDRAWPASS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphDrawPass {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHDRAWPASS_TYPE_INFO
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


pub static EMITTERGRAPHDRAWPASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphDrawPass-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphDrawPass"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterGraphCulledBehavior {
    #[default]
    EmitterGraphCulledBehavior_Pause = 0,
    EmitterGraphCulledBehavior_SkipUpdates = 1,
    EmitterGraphCulledBehavior_Kill = 2,
}

pub static EMITTERGRAPHCULLEDBEHAVIOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphCulledBehavior",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHCULLEDBEHAVIOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphCulledBehavior {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHCULLEDBEHAVIOR_TYPE_INFO
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


pub static EMITTERGRAPHCULLEDBEHAVIOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphCulledBehavior-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphCulledBehavior"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterGraphConfig {
    #[default]
    EmitterGraphConfig_EmitterGraphParamMaxCount = 32,
    EmitterGraphConfig_EffectParamMaxCount = 8,
    EmitterGraphConfig_ExclusionVolumesMaxCount = 1024,
    EmitterGraphConfig_MaxParticleCount = 65535,
}

pub static EMITTERGRAPHCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphConfig",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHCONFIG_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphConfig {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHCONFIG_TYPE_INFO
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


pub static EMITTERGRAPHCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RandomSpawnRateModifier {
    pub _glacier_base: SpawnRateModifier,
    pub frequency: EmitterGraphRandomFrequency,
    pub min: f32,
    pub max: f32,
}

pub trait RandomSpawnRateModifierTrait: SpawnRateModifierTrait {
    fn frequency(&self) -> &EmitterGraphRandomFrequency;
    fn frequency_mut(&mut self) -> &mut EmitterGraphRandomFrequency;
    fn min(&self) -> &f32;
    fn min_mut(&mut self) -> &mut f32;
    fn max(&self) -> &f32;
    fn max_mut(&mut self) -> &mut f32;
}

impl RandomSpawnRateModifierTrait for RandomSpawnRateModifier {
    fn frequency(&self) -> &EmitterGraphRandomFrequency {
        &self.frequency
    }
    fn frequency_mut(&mut self) -> &mut EmitterGraphRandomFrequency {
        &mut self.frequency
    }
    fn min(&self) -> &f32 {
        &self.min
    }
    fn min_mut(&mut self) -> &mut f32 {
        &mut self.min
    }
    fn max(&self) -> &f32 {
        &self.max
    }
    fn max_mut(&mut self) -> &mut f32 {
        &mut self.max
    }
}

impl SpawnRateModifierTrait for RandomSpawnRateModifier {
}

impl super::core::DataContainerTrait for RandomSpawnRateModifier {
}

pub static RANDOMSPAWNRATEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomSpawnRateModifier",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPAWNRATEMODIFIER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RandomSpawnRateModifier as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Frequency",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphRandomFrequency",
                rust_offset: offset_of!(RandomSpawnRateModifier, frequency),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomSpawnRateModifier, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomSpawnRateModifier, max),
            },
        ],
    }),
    array_type: Some(RANDOMSPAWNRATEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomSpawnRateModifier {
    fn type_info(&self) -> &'static TypeInfo {
        RANDOMSPAWNRATEMODIFIER_TYPE_INFO
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


pub static RANDOMSPAWNRATEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomSpawnRateModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RandomSpawnRateModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterGraphRandomFrequency {
    #[default]
    EmitterGraphRandomFrequency_RandomPerFrame = 0,
    EmitterGraphRandomFrequency_RandomPerEmitter = 1,
}

pub static EMITTERGRAPHRANDOMFREQUENCY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphRandomFrequency",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHRANDOMFREQUENCY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphRandomFrequency {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHRANDOMFREQUENCY_TYPE_INFO
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


pub static EMITTERGRAPHRANDOMFREQUENCY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphRandomFrequency-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphRandomFrequency"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterSpawnRateModifier {
    pub _glacier_base: SpawnRateModifier,
    pub emitter_property: EmitterGraphSpawnProperty,
    pub curve: Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>,
}

pub trait EmitterSpawnRateModifierTrait: SpawnRateModifierTrait {
    fn emitter_property(&self) -> &EmitterGraphSpawnProperty;
    fn emitter_property_mut(&mut self) -> &mut EmitterGraphSpawnProperty;
    fn curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>;
    fn curve_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>;
}

impl EmitterSpawnRateModifierTrait for EmitterSpawnRateModifier {
    fn emitter_property(&self) -> &EmitterGraphSpawnProperty {
        &self.emitter_property
    }
    fn emitter_property_mut(&mut self) -> &mut EmitterGraphSpawnProperty {
        &mut self.emitter_property
    }
    fn curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        &self.curve
    }
    fn curve_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        &mut self.curve
    }
}

impl SpawnRateModifierTrait for EmitterSpawnRateModifier {
}

impl super::core::DataContainerTrait for EmitterSpawnRateModifier {
}

pub static EMITTERSPAWNRATEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSpawnRateModifier",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPAWNRATEMODIFIER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterSpawnRateModifier as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EmitterProperty",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphSpawnProperty",
                rust_offset: offset_of!(EmitterSpawnRateModifier, emitter_property),
            },
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(EmitterSpawnRateModifier, curve),
            },
        ],
    }),
    array_type: Some(EMITTERSPAWNRATEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterSpawnRateModifier {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERSPAWNRATEMODIFIER_TYPE_INFO
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


pub static EMITTERSPAWNRATEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSpawnRateModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterSpawnRateModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectParameterSpawnRateModifier {
    pub _glacier_base: SpawnRateModifier,
    pub effect_parameter: Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>,
    pub curve: Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>,
}

pub trait EffectParameterSpawnRateModifierTrait: SpawnRateModifierTrait {
    fn effect_parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>;
    fn effect_parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>;
    fn curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>;
    fn curve_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>>;
}

impl EffectParameterSpawnRateModifierTrait for EffectParameterSpawnRateModifier {
    fn effect_parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        &self.effect_parameter
    }
    fn effect_parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        &mut self.effect_parameter
    }
    fn curve(&self) -> &Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        &self.curve
    }
    fn curve_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::FloatCurveTrait>>> {
        &mut self.curve
    }
}

impl SpawnRateModifierTrait for EffectParameterSpawnRateModifier {
}

impl super::core::DataContainerTrait for EffectParameterSpawnRateModifier {
}

pub static EFFECTPARAMETERSPAWNRATEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterSpawnRateModifier",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPAWNRATEMODIFIER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectParameterSpawnRateModifier as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EffectParameter",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParameter",
                rust_offset: offset_of!(EffectParameterSpawnRateModifier, effect_parameter),
            },
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurve",
                rust_offset: offset_of!(EffectParameterSpawnRateModifier, curve),
            },
        ],
    }),
    array_type: Some(EFFECTPARAMETERSPAWNRATEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectParameterSpawnRateModifier {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTPARAMETERSPAWNRATEMODIFIER_TYPE_INFO
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


pub static EFFECTPARAMETERSPAWNRATEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterSpawnRateModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EffectParameterSpawnRateModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnRateModifier {
    pub _glacier_base: super::core::DataContainer,
}

pub trait SpawnRateModifierTrait: super::core::DataContainerTrait {
}

impl SpawnRateModifierTrait for SpawnRateModifier {
}

impl super::core::DataContainerTrait for SpawnRateModifier {
}

pub static SPAWNRATEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRateModifier",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnRateModifier as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SPAWNRATEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnRateModifier {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNRATEMODIFIER_TYPE_INFO
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


pub static SPAWNRATEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRateModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnRateModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterGraphSpawnProperty {
    #[default]
    EmitterGraphSpawnProperty_EmitterLifetimeNorm = 0,
    EmitterGraphSpawnProperty_EmitterAgeNorm = 1,
    EmitterGraphSpawnProperty_Speed = 2,
}

pub static EMITTERGRAPHSPAWNPROPERTY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSpawnProperty",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHSPAWNPROPERTY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphSpawnProperty {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHSPAWNPROPERTY_TYPE_INFO
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


pub static EMITTERGRAPHSPAWNPROPERTY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphSpawnProperty-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphSpawnProperty"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnModeContinuous {
    pub _glacier_base: SpawnModeInfo,
    pub spawn_rate: super::core::QualityScalableFloat,
    pub min_spawn_rate: super::core::QualityScalableFloat,
    pub max_spawn_rate: super::core::QualityScalableFloat,
    pub spawn_rate_multipliers: Vec<Option<Arc<Mutex<dyn SpawnRateModifierTrait>>>>,
}

pub trait SpawnModeContinuousTrait: SpawnModeInfoTrait {
    fn spawn_rate(&self) -> &super::core::QualityScalableFloat;
    fn spawn_rate_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn min_spawn_rate(&self) -> &super::core::QualityScalableFloat;
    fn min_spawn_rate_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn max_spawn_rate(&self) -> &super::core::QualityScalableFloat;
    fn max_spawn_rate_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn spawn_rate_multipliers(&self) -> &Vec<Option<Arc<Mutex<dyn SpawnRateModifierTrait>>>>;
    fn spawn_rate_multipliers_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn SpawnRateModifierTrait>>>>;
}

impl SpawnModeContinuousTrait for SpawnModeContinuous {
    fn spawn_rate(&self) -> &super::core::QualityScalableFloat {
        &self.spawn_rate
    }
    fn spawn_rate_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.spawn_rate
    }
    fn min_spawn_rate(&self) -> &super::core::QualityScalableFloat {
        &self.min_spawn_rate
    }
    fn min_spawn_rate_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.min_spawn_rate
    }
    fn max_spawn_rate(&self) -> &super::core::QualityScalableFloat {
        &self.max_spawn_rate
    }
    fn max_spawn_rate_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.max_spawn_rate
    }
    fn spawn_rate_multipliers(&self) -> &Vec<Option<Arc<Mutex<dyn SpawnRateModifierTrait>>>> {
        &self.spawn_rate_multipliers
    }
    fn spawn_rate_multipliers_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn SpawnRateModifierTrait>>>> {
        &mut self.spawn_rate_multipliers
    }
}

impl SpawnModeInfoTrait for SpawnModeContinuous {
    fn particle_max_count(&self) -> &super::core::QualityScalableInt {
        self._glacier_base.particle_max_count()
    }
    fn particle_max_count_mut(&mut self) -> &mut super::core::QualityScalableInt {
        self._glacier_base.particle_max_count_mut()
    }
    fn spawn_mode_enum(&self) -> &EmitterGraphSpawnMode {
        self._glacier_base.spawn_mode_enum()
    }
    fn spawn_mode_enum_mut(&mut self) -> &mut EmitterGraphSpawnMode {
        self._glacier_base.spawn_mode_enum_mut()
    }
}

impl super::core::DataContainerTrait for SpawnModeContinuous {
}

pub static SPAWNMODECONTINUOUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeContinuous",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPAWNMODEINFO_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnModeContinuous as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(SpawnModeContinuous, spawn_rate),
            },
            FieldInfoData {
                name: "MinSpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(SpawnModeContinuous, min_spawn_rate),
            },
            FieldInfoData {
                name: "MaxSpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(SpawnModeContinuous, max_spawn_rate),
            },
            FieldInfoData {
                name: "SpawnRateMultipliers",
                flags: MemberInfoFlags::new(144),
                field_type: "SpawnRateModifier-Array",
                rust_offset: offset_of!(SpawnModeContinuous, spawn_rate_multipliers),
            },
        ],
    }),
    array_type: Some(SPAWNMODECONTINUOUS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnModeContinuous {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNMODECONTINUOUS_TYPE_INFO
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


pub static SPAWNMODECONTINUOUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeContinuous-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnModeContinuous"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnModeBurst {
    pub _glacier_base: SpawnModeInfo,
}

pub trait SpawnModeBurstTrait: SpawnModeInfoTrait {
}

impl SpawnModeBurstTrait for SpawnModeBurst {
}

impl SpawnModeInfoTrait for SpawnModeBurst {
    fn particle_max_count(&self) -> &super::core::QualityScalableInt {
        self._glacier_base.particle_max_count()
    }
    fn particle_max_count_mut(&mut self) -> &mut super::core::QualityScalableInt {
        self._glacier_base.particle_max_count_mut()
    }
    fn spawn_mode_enum(&self) -> &EmitterGraphSpawnMode {
        self._glacier_base.spawn_mode_enum()
    }
    fn spawn_mode_enum_mut(&mut self) -> &mut EmitterGraphSpawnMode {
        self._glacier_base.spawn_mode_enum_mut()
    }
}

impl super::core::DataContainerTrait for SpawnModeBurst {
}

pub static SPAWNMODEBURST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeBurst",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPAWNMODEINFO_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnModeBurst as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SPAWNMODEBURST_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnModeBurst {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNMODEBURST_TYPE_INFO
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


pub static SPAWNMODEBURST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeBurst-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnModeBurst"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnModeInfo {
    pub _glacier_base: super::core::DataContainer,
    pub particle_max_count: super::core::QualityScalableInt,
    pub spawn_mode_enum: EmitterGraphSpawnMode,
}

pub trait SpawnModeInfoTrait: super::core::DataContainerTrait {
    fn particle_max_count(&self) -> &super::core::QualityScalableInt;
    fn particle_max_count_mut(&mut self) -> &mut super::core::QualityScalableInt;
    fn spawn_mode_enum(&self) -> &EmitterGraphSpawnMode;
    fn spawn_mode_enum_mut(&mut self) -> &mut EmitterGraphSpawnMode;
}

impl SpawnModeInfoTrait for SpawnModeInfo {
    fn particle_max_count(&self) -> &super::core::QualityScalableInt {
        &self.particle_max_count
    }
    fn particle_max_count_mut(&mut self) -> &mut super::core::QualityScalableInt {
        &mut self.particle_max_count
    }
    fn spawn_mode_enum(&self) -> &EmitterGraphSpawnMode {
        &self.spawn_mode_enum
    }
    fn spawn_mode_enum_mut(&mut self) -> &mut EmitterGraphSpawnMode {
        &mut self.spawn_mode_enum
    }
}

impl super::core::DataContainerTrait for SpawnModeInfo {
}

pub static SPAWNMODEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeInfo",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnModeInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ParticleMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(SpawnModeInfo, particle_max_count),
            },
            FieldInfoData {
                name: "SpawnModeEnum",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphSpawnMode",
                rust_offset: offset_of!(SpawnModeInfo, spawn_mode_enum),
            },
        ],
    }),
    array_type: Some(SPAWNMODEINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SpawnModeInfo {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNMODEINFO_TYPE_INFO
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


pub static SPAWNMODEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnModeInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnModeInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VertexShaderParam {
    pub param_name: String,
    pub value: super::core::Vec4,
    pub param_type: super::effect_base::EmitterGraphParamType,
    pub value_type: super::emitter_base::EmitterExposableType,
}

pub trait VertexShaderParamTrait: TypeObject {
    fn param_name(&self) -> &String;
    fn param_name_mut(&mut self) -> &mut String;
    fn value(&self) -> &super::core::Vec4;
    fn value_mut(&mut self) -> &mut super::core::Vec4;
    fn param_type(&self) -> &super::effect_base::EmitterGraphParamType;
    fn param_type_mut(&mut self) -> &mut super::effect_base::EmitterGraphParamType;
    fn value_type(&self) -> &super::emitter_base::EmitterExposableType;
    fn value_type_mut(&mut self) -> &mut super::emitter_base::EmitterExposableType;
}

impl VertexShaderParamTrait for VertexShaderParam {
    fn param_name(&self) -> &String {
        &self.param_name
    }
    fn param_name_mut(&mut self) -> &mut String {
        &mut self.param_name
    }
    fn value(&self) -> &super::core::Vec4 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.value
    }
    fn param_type(&self) -> &super::effect_base::EmitterGraphParamType {
        &self.param_type
    }
    fn param_type_mut(&mut self) -> &mut super::effect_base::EmitterGraphParamType {
        &mut self.param_type
    }
    fn value_type(&self) -> &super::emitter_base::EmitterExposableType {
        &self.value_type
    }
    fn value_type_mut(&mut self) -> &mut super::emitter_base::EmitterExposableType {
        &mut self.value_type
    }
}

pub static VERTEXSHADERPARAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderParam",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VertexShaderParam as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ParamName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VertexShaderParam, param_name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(VertexShaderParam, value),
            },
            FieldInfoData {
                name: "ParamType",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterGraphParamType",
                rust_offset: offset_of!(VertexShaderParam, param_type),
            },
            FieldInfoData {
                name: "ValueType",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterExposableType",
                rust_offset: offset_of!(VertexShaderParam, value_type),
            },
        ],
    }),
    array_type: Some(VERTEXSHADERPARAM_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VertexShaderParam {
    fn type_info(&self) -> &'static TypeInfo {
        VERTEXSHADERPARAM_TYPE_INFO
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


pub static VERTEXSHADERPARAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderParam-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("VertexShaderParam"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VertexShaderTextureParam {
    pub param_name: String,
    pub value: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub address_mode: TextureNodeAddress,
    pub filter_mode: TextureNodeFilter,
}

pub trait VertexShaderTextureParamTrait: TypeObject {
    fn param_name(&self) -> &String;
    fn param_name_mut(&mut self) -> &mut String;
    fn value(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn value_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn address_mode(&self) -> &TextureNodeAddress;
    fn address_mode_mut(&mut self) -> &mut TextureNodeAddress;
    fn filter_mode(&self) -> &TextureNodeFilter;
    fn filter_mode_mut(&mut self) -> &mut TextureNodeFilter;
}

impl VertexShaderTextureParamTrait for VertexShaderTextureParam {
    fn param_name(&self) -> &String {
        &self.param_name
    }
    fn param_name_mut(&mut self) -> &mut String {
        &mut self.param_name
    }
    fn value(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.value
    }
    fn value_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &mut self.value
    }
    fn address_mode(&self) -> &TextureNodeAddress {
        &self.address_mode
    }
    fn address_mode_mut(&mut self) -> &mut TextureNodeAddress {
        &mut self.address_mode
    }
    fn filter_mode(&self) -> &TextureNodeFilter {
        &self.filter_mode
    }
    fn filter_mode_mut(&mut self) -> &mut TextureNodeFilter {
        &mut self.filter_mode
    }
}

pub static VERTEXSHADERTEXTUREPARAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderTextureParam",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VertexShaderTextureParam as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ParamName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VertexShaderTextureParam, param_name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(VertexShaderTextureParam, value),
            },
            FieldInfoData {
                name: "AddressMode",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureNodeAddress",
                rust_offset: offset_of!(VertexShaderTextureParam, address_mode),
            },
            FieldInfoData {
                name: "FilterMode",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureNodeFilter",
                rust_offset: offset_of!(VertexShaderTextureParam, filter_mode),
            },
        ],
    }),
    array_type: Some(VERTEXSHADERTEXTUREPARAM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VertexShaderTextureParam {
    fn type_info(&self) -> &'static TypeInfo {
        VERTEXSHADERTEXTUREPARAM_TYPE_INFO
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


pub static VERTEXSHADERTEXTUREPARAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderTextureParam-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("VertexShaderTextureParam"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static EMITTERGRAPHPARTICLEDATATYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphParticleDataType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHPARTICLEDATATYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphParticleDataType {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHPARTICLEDATATYPE_TYPE_INFO
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


pub static EMITTERGRAPHPARTICLEDATATYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphParticleDataType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterGraphParticleDataType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TextureNodeFilter {
    #[default]
    TnfPoint = 0,
    TnfLinear = 1,
}

pub static TEXTURENODEFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureNodeFilter",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTURENODEFILTER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureNodeFilter {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTURENODEFILTER_TYPE_INFO
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


pub static TEXTURENODEFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureNodeFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("TextureNodeFilter"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TextureNodeAddress {
    #[default]
    TnaWrap = 0,
    TnaClamp = 1,
}

pub static TEXTURENODEADDRESS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureNodeAddress",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTURENODEADDRESS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureNodeAddress {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTURENODEADDRESS_TYPE_INFO
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


pub static TEXTURENODEADDRESS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureNodeAddress-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("TextureNodeAddress"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterExclusionVolumesAsset {
    pub _glacier_base: super::emitter_base::EmitterExclusionVolumesBaseAsset,
    pub exclusion_volumes_count: u32,
    pub exclusion_volumes: Vec<EmitterExclusionVolume>,
    pub exclusion_volume_bounding_spheres: Vec<EmitterExclusionVolumeBoundingSphereSoA>,
}

pub trait EmitterExclusionVolumesAssetTrait: super::emitter_base::EmitterExclusionVolumesBaseAssetTrait {
    fn exclusion_volumes_count(&self) -> &u32;
    fn exclusion_volumes_count_mut(&mut self) -> &mut u32;
    fn exclusion_volumes(&self) -> &Vec<EmitterExclusionVolume>;
    fn exclusion_volumes_mut(&mut self) -> &mut Vec<EmitterExclusionVolume>;
    fn exclusion_volume_bounding_spheres(&self) -> &Vec<EmitterExclusionVolumeBoundingSphereSoA>;
    fn exclusion_volume_bounding_spheres_mut(&mut self) -> &mut Vec<EmitterExclusionVolumeBoundingSphereSoA>;
}

impl EmitterExclusionVolumesAssetTrait for EmitterExclusionVolumesAsset {
    fn exclusion_volumes_count(&self) -> &u32 {
        &self.exclusion_volumes_count
    }
    fn exclusion_volumes_count_mut(&mut self) -> &mut u32 {
        &mut self.exclusion_volumes_count
    }
    fn exclusion_volumes(&self) -> &Vec<EmitterExclusionVolume> {
        &self.exclusion_volumes
    }
    fn exclusion_volumes_mut(&mut self) -> &mut Vec<EmitterExclusionVolume> {
        &mut self.exclusion_volumes
    }
    fn exclusion_volume_bounding_spheres(&self) -> &Vec<EmitterExclusionVolumeBoundingSphereSoA> {
        &self.exclusion_volume_bounding_spheres
    }
    fn exclusion_volume_bounding_spheres_mut(&mut self) -> &mut Vec<EmitterExclusionVolumeBoundingSphereSoA> {
        &mut self.exclusion_volume_bounding_spheres
    }
}

impl super::emitter_base::EmitterExclusionVolumesBaseAssetTrait for EmitterExclusionVolumesAsset {
}

impl super::core::AssetTrait for EmitterExclusionVolumesAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EmitterExclusionVolumesAsset {
}

pub static EMITTEREXCLUSIONVOLUMESASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumesAsset",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::emitter_base::EMITTEREXCLUSIONVOLUMESBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterExclusionVolumesAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ExclusionVolumesCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterExclusionVolumesAsset, exclusion_volumes_count),
            },
            FieldInfoData {
                name: "ExclusionVolumes",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterExclusionVolume-Array",
                rust_offset: offset_of!(EmitterExclusionVolumesAsset, exclusion_volumes),
            },
            FieldInfoData {
                name: "ExclusionVolumeBoundingSpheres",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterExclusionVolumeBoundingSphereSoA-Array",
                rust_offset: offset_of!(EmitterExclusionVolumesAsset, exclusion_volume_bounding_spheres),
            },
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUMESASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterExclusionVolumesAsset {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUMESASSET_TYPE_INFO
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


pub static EMITTEREXCLUSIONVOLUMESASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumesAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterExclusionVolumesAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshEmitterMaskAsset {
    pub _glacier_base: super::effect_base::MeshEmitterMaskBaseAsset,
    pub mesh_emitter_mask_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait MeshEmitterMaskAssetTrait: super::effect_base::MeshEmitterMaskBaseAssetTrait {
    fn mesh_emitter_mask_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn mesh_emitter_mask_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl MeshEmitterMaskAssetTrait for MeshEmitterMaskAsset {
    fn mesh_emitter_mask_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.mesh_emitter_mask_resource
    }
    fn mesh_emitter_mask_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.mesh_emitter_mask_resource
    }
}

impl super::effect_base::MeshEmitterMaskBaseAssetTrait for MeshEmitterMaskAsset {
}

impl super::core::AssetTrait for MeshEmitterMaskAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MeshEmitterMaskAsset {
}

pub static MESHEMITTERMASKASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskAsset",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::effect_base::MESHEMITTERMASKBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshEmitterMaskAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MeshEmitterMaskResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(MeshEmitterMaskAsset, mesh_emitter_mask_resource),
            },
        ],
    }),
    array_type: Some(MESHEMITTERMASKASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshEmitterMaskAsset {
    fn type_info(&self) -> &'static TypeInfo {
        MESHEMITTERMASKASSET_TYPE_INFO
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


pub static MESHEMITTERMASKASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MeshEmitterMaskAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshEmitterAsset {
    pub _glacier_base: super::effect_base::MeshEmitterBaseAsset,
    pub mesh_emitter_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait MeshEmitterAssetTrait: super::effect_base::MeshEmitterBaseAssetTrait {
    fn mesh_emitter_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn mesh_emitter_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl MeshEmitterAssetTrait for MeshEmitterAsset {
    fn mesh_emitter_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.mesh_emitter_resource
    }
    fn mesh_emitter_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.mesh_emitter_resource
    }
}

impl super::effect_base::MeshEmitterBaseAssetTrait for MeshEmitterAsset {
}

impl super::core::AssetTrait for MeshEmitterAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MeshEmitterAsset {
}

pub static MESHEMITTERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterAsset",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::effect_base::MESHEMITTERBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshEmitterAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MeshEmitterResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(MeshEmitterAsset, mesh_emitter_resource),
            },
        ],
    }),
    array_type: Some(MESHEMITTERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshEmitterAsset {
    fn type_info(&self) -> &'static TypeInfo {
        MESHEMITTERASSET_TYPE_INFO
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


pub static MESHEMITTERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MeshEmitterAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterAsset {
    pub _glacier_base: super::emitter_base::EmitterBaseAsset,
}

pub trait EmitterAssetTrait: super::emitter_base::EmitterBaseAssetTrait {
}

impl EmitterAssetTrait for EmitterAsset {
}

impl super::emitter_base::EmitterBaseAssetTrait for EmitterAsset {
}

impl super::core::AssetTrait for EmitterAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EmitterAsset {
}

pub static EMITTERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterAsset",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::emitter_base::EMITTERBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EMITTERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterAsset {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERASSET_TYPE_INFO
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


pub static EMITTERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateStencilMaskData {
    pub _glacier_base: ProcessorData,
    pub mask: EmitterStencilMask,
}

pub trait UpdateStencilMaskDataTrait: ProcessorDataTrait {
    fn mask(&self) -> &EmitterStencilMask;
    fn mask_mut(&mut self) -> &mut EmitterStencilMask;
}

impl UpdateStencilMaskDataTrait for UpdateStencilMaskData {
    fn mask(&self) -> &EmitterStencilMask {
        &self.mask
    }
    fn mask_mut(&mut self) -> &mut EmitterStencilMask {
        &mut self.mask
    }
}

impl ProcessorDataTrait for UpdateStencilMaskData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateStencilMaskData {
}

pub static UPDATESTENCILMASKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateStencilMaskData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateStencilMaskData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Mask",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterStencilMask",
                rust_offset: offset_of!(UpdateStencilMaskData, mask),
            },
        ],
    }),
    array_type: Some(UPDATESTENCILMASKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateStencilMaskData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATESTENCILMASKDATA_TYPE_INFO
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


pub static UPDATESTENCILMASKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateStencilMaskData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateStencilMaskData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterStencilMask {
    #[default]
    EmitterStencilMask_None = 0,
    EmitterStencilMask_Static = 1,
    EmitterStencilMask_Dynamic = 2,
}

pub static EMITTERSTENCILMASK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterStencilMask",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERSTENCILMASK_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterStencilMask {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERSTENCILMASK_TYPE_INFO
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


pub static EMITTERSTENCILMASK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterStencilMask-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterStencilMask"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateVolumeMaskData {
    pub _glacier_base: ProcessorData,
    pub mask_type: VolumeMaskType,
}

pub trait UpdateVolumeMaskDataTrait: ProcessorDataTrait {
    fn mask_type(&self) -> &VolumeMaskType;
    fn mask_type_mut(&mut self) -> &mut VolumeMaskType;
}

impl UpdateVolumeMaskDataTrait for UpdateVolumeMaskData {
    fn mask_type(&self) -> &VolumeMaskType {
        &self.mask_type
    }
    fn mask_type_mut(&mut self) -> &mut VolumeMaskType {
        &mut self.mask_type
    }
}

impl ProcessorDataTrait for UpdateVolumeMaskData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateVolumeMaskData {
}

pub static UPDATEVOLUMEMASKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVolumeMaskData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateVolumeMaskData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaskType",
                flags: MemberInfoFlags::new(0),
                field_type: "VolumeMaskType",
                rust_offset: offset_of!(UpdateVolumeMaskData, mask_type),
            },
        ],
    }),
    array_type: Some(UPDATEVOLUMEMASKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateVolumeMaskData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEVOLUMEMASKDATA_TYPE_INFO
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


pub static UPDATEVOLUMEMASKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVolumeMaskData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateVolumeMaskData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum VolumeMaskType {
    #[default]
    VolumeMaskType_None = 0,
    VolumeMaskType_Static = 1,
    VolumeMaskType_Dynamic = 2,
}

pub static VOLUMEMASKTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumeMaskType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(VOLUMEMASKTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VolumeMaskType {
    fn type_info(&self) -> &'static TypeInfo {
        VOLUMEMASKTYPE_TYPE_INFO
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


pub static VOLUMEMASKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumeMaskType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("VolumeMaskType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateVolumetricData {
    pub _glacier_base: ProcessorData,
    pub absorption: f32,
    pub high_quality_injection: bool,
}

pub trait UpdateVolumetricDataTrait: ProcessorDataTrait {
    fn absorption(&self) -> &f32;
    fn absorption_mut(&mut self) -> &mut f32;
    fn high_quality_injection(&self) -> &bool;
    fn high_quality_injection_mut(&mut self) -> &mut bool;
}

impl UpdateVolumetricDataTrait for UpdateVolumetricData {
    fn absorption(&self) -> &f32 {
        &self.absorption
    }
    fn absorption_mut(&mut self) -> &mut f32 {
        &mut self.absorption
    }
    fn high_quality_injection(&self) -> &bool {
        &self.high_quality_injection
    }
    fn high_quality_injection_mut(&mut self) -> &mut bool {
        &mut self.high_quality_injection
    }
}

impl ProcessorDataTrait for UpdateVolumetricData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateVolumetricData {
}

pub static UPDATEVOLUMETRICDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVolumetricData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateVolumetricData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateVolumetricData, absorption),
            },
            FieldInfoData {
                name: "HighQualityInjection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateVolumetricData, high_quality_injection),
            },
        ],
    }),
    array_type: Some(UPDATEVOLUMETRICDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateVolumetricData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEVOLUMETRICDATA_TYPE_INFO
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


pub static UPDATEVOLUMETRICDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVolumetricData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateVolumetricData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateMeshEmitterMaskData {
    pub _glacier_base: ProcessorData,
    pub mesh_emitter_mask: Option<Arc<Mutex<dyn MeshEmitterMaskAssetTrait>>>,
}

pub trait UpdateMeshEmitterMaskDataTrait: ProcessorDataTrait {
    fn mesh_emitter_mask(&self) -> &Option<Arc<Mutex<dyn MeshEmitterMaskAssetTrait>>>;
    fn mesh_emitter_mask_mut(&mut self) -> &mut Option<Arc<Mutex<dyn MeshEmitterMaskAssetTrait>>>;
}

impl UpdateMeshEmitterMaskDataTrait for UpdateMeshEmitterMaskData {
    fn mesh_emitter_mask(&self) -> &Option<Arc<Mutex<dyn MeshEmitterMaskAssetTrait>>> {
        &self.mesh_emitter_mask
    }
    fn mesh_emitter_mask_mut(&mut self) -> &mut Option<Arc<Mutex<dyn MeshEmitterMaskAssetTrait>>> {
        &mut self.mesh_emitter_mask
    }
}

impl ProcessorDataTrait for UpdateMeshEmitterMaskData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateMeshEmitterMaskData {
}

pub static UPDATEMESHEMITTERMASKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMeshEmitterMaskData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateMeshEmitterMaskData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MeshEmitterMask",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshEmitterMaskAsset",
                rust_offset: offset_of!(UpdateMeshEmitterMaskData, mesh_emitter_mask),
            },
        ],
    }),
    array_type: Some(UPDATEMESHEMITTERMASKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateMeshEmitterMaskData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEMESHEMITTERMASKDATA_TYPE_INFO
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


pub static UPDATEMESHEMITTERMASKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMeshEmitterMaskData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateMeshEmitterMaskData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateMeshEmitterSourceData {
    pub _glacier_base: ProcessorData,
    pub mesh_emitter: Option<Arc<Mutex<dyn MeshEmitterAssetTrait>>>,
    pub generate_position: bool,
    pub generate_normal: bool,
    pub generate_u_vs: bool,
    pub send_mesh_uvs_to_shader_graph: bool,
    pub sequential_emission: bool,
    pub particles_per_primitive: u32,
    pub random_position: f32,
}

pub trait UpdateMeshEmitterSourceDataTrait: ProcessorDataTrait {
    fn mesh_emitter(&self) -> &Option<Arc<Mutex<dyn MeshEmitterAssetTrait>>>;
    fn mesh_emitter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn MeshEmitterAssetTrait>>>;
    fn generate_position(&self) -> &bool;
    fn generate_position_mut(&mut self) -> &mut bool;
    fn generate_normal(&self) -> &bool;
    fn generate_normal_mut(&mut self) -> &mut bool;
    fn generate_u_vs(&self) -> &bool;
    fn generate_u_vs_mut(&mut self) -> &mut bool;
    fn send_mesh_uvs_to_shader_graph(&self) -> &bool;
    fn send_mesh_uvs_to_shader_graph_mut(&mut self) -> &mut bool;
    fn sequential_emission(&self) -> &bool;
    fn sequential_emission_mut(&mut self) -> &mut bool;
    fn particles_per_primitive(&self) -> &u32;
    fn particles_per_primitive_mut(&mut self) -> &mut u32;
    fn random_position(&self) -> &f32;
    fn random_position_mut(&mut self) -> &mut f32;
}

impl UpdateMeshEmitterSourceDataTrait for UpdateMeshEmitterSourceData {
    fn mesh_emitter(&self) -> &Option<Arc<Mutex<dyn MeshEmitterAssetTrait>>> {
        &self.mesh_emitter
    }
    fn mesh_emitter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn MeshEmitterAssetTrait>>> {
        &mut self.mesh_emitter
    }
    fn generate_position(&self) -> &bool {
        &self.generate_position
    }
    fn generate_position_mut(&mut self) -> &mut bool {
        &mut self.generate_position
    }
    fn generate_normal(&self) -> &bool {
        &self.generate_normal
    }
    fn generate_normal_mut(&mut self) -> &mut bool {
        &mut self.generate_normal
    }
    fn generate_u_vs(&self) -> &bool {
        &self.generate_u_vs
    }
    fn generate_u_vs_mut(&mut self) -> &mut bool {
        &mut self.generate_u_vs
    }
    fn send_mesh_uvs_to_shader_graph(&self) -> &bool {
        &self.send_mesh_uvs_to_shader_graph
    }
    fn send_mesh_uvs_to_shader_graph_mut(&mut self) -> &mut bool {
        &mut self.send_mesh_uvs_to_shader_graph
    }
    fn sequential_emission(&self) -> &bool {
        &self.sequential_emission
    }
    fn sequential_emission_mut(&mut self) -> &mut bool {
        &mut self.sequential_emission
    }
    fn particles_per_primitive(&self) -> &u32 {
        &self.particles_per_primitive
    }
    fn particles_per_primitive_mut(&mut self) -> &mut u32 {
        &mut self.particles_per_primitive
    }
    fn random_position(&self) -> &f32 {
        &self.random_position
    }
    fn random_position_mut(&mut self) -> &mut f32 {
        &mut self.random_position
    }
}

impl ProcessorDataTrait for UpdateMeshEmitterSourceData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateMeshEmitterSourceData {
}

pub static UPDATEMESHEMITTERSOURCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMeshEmitterSourceData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateMeshEmitterSourceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MeshEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshEmitterAsset",
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, mesh_emitter),
            },
            FieldInfoData {
                name: "GeneratePosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, generate_position),
            },
            FieldInfoData {
                name: "GenerateNormal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, generate_normal),
            },
            FieldInfoData {
                name: "GenerateUVs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, generate_u_vs),
            },
            FieldInfoData {
                name: "SendMeshUvsToShaderGraph",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, send_mesh_uvs_to_shader_graph),
            },
            FieldInfoData {
                name: "SequentialEmission",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, sequential_emission),
            },
            FieldInfoData {
                name: "ParticlesPerPrimitive",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, particles_per_primitive),
            },
            FieldInfoData {
                name: "RandomPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateMeshEmitterSourceData, random_position),
            },
        ],
    }),
    array_type: Some(UPDATEMESHEMITTERSOURCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateMeshEmitterSourceData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEMESHEMITTERSOURCEDATA_TYPE_INFO
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


pub static UPDATEMESHEMITTERSOURCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMeshEmitterSourceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateMeshEmitterSourceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateVertexAnimData {
    pub _glacier_base: ProcessorData,
    pub shader_fragment: Option<Arc<Mutex<dyn super::render::VertexShaderFragmentAssetTrait>>>,
    pub per_particle_randomness: f32,
    pub frequency_multiplier: f32,
    pub animation_parameters: super::core::Vec4,
    pub animation_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
}

pub trait UpdateVertexAnimDataTrait: ProcessorDataTrait {
    fn shader_fragment(&self) -> &Option<Arc<Mutex<dyn super::render::VertexShaderFragmentAssetTrait>>>;
    fn shader_fragment_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::VertexShaderFragmentAssetTrait>>>;
    fn per_particle_randomness(&self) -> &f32;
    fn per_particle_randomness_mut(&mut self) -> &mut f32;
    fn frequency_multiplier(&self) -> &f32;
    fn frequency_multiplier_mut(&mut self) -> &mut f32;
    fn animation_parameters(&self) -> &super::core::Vec4;
    fn animation_parameters_mut(&mut self) -> &mut super::core::Vec4;
    fn animation_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn animation_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
}

impl UpdateVertexAnimDataTrait for UpdateVertexAnimData {
    fn shader_fragment(&self) -> &Option<Arc<Mutex<dyn super::render::VertexShaderFragmentAssetTrait>>> {
        &self.shader_fragment
    }
    fn shader_fragment_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::VertexShaderFragmentAssetTrait>>> {
        &mut self.shader_fragment
    }
    fn per_particle_randomness(&self) -> &f32 {
        &self.per_particle_randomness
    }
    fn per_particle_randomness_mut(&mut self) -> &mut f32 {
        &mut self.per_particle_randomness
    }
    fn frequency_multiplier(&self) -> &f32 {
        &self.frequency_multiplier
    }
    fn frequency_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.frequency_multiplier
    }
    fn animation_parameters(&self) -> &super::core::Vec4 {
        &self.animation_parameters
    }
    fn animation_parameters_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.animation_parameters
    }
    fn animation_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.animation_texture
    }
    fn animation_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &mut self.animation_texture
    }
}

impl ProcessorDataTrait for UpdateVertexAnimData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateVertexAnimData {
}

pub static UPDATEVERTEXANIMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVertexAnimData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateVertexAnimData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ShaderFragment",
                flags: MemberInfoFlags::new(0),
                field_type: "VertexShaderFragmentAsset",
                rust_offset: offset_of!(UpdateVertexAnimData, shader_fragment),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateVertexAnimData, per_particle_randomness),
            },
            FieldInfoData {
                name: "FrequencyMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateVertexAnimData, frequency_multiplier),
            },
            FieldInfoData {
                name: "AnimationParameters",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UpdateVertexAnimData, animation_parameters),
            },
            FieldInfoData {
                name: "AnimationTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(UpdateVertexAnimData, animation_texture),
            },
        ],
    }),
    array_type: Some(UPDATEVERTEXANIMDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateVertexAnimData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEVERTEXANIMDATA_TYPE_INFO
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


pub static UPDATEVERTEXANIMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateVertexAnimData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateVertexAnimData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateBeamPointData {
    pub _glacier_base: ProcessorData,
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

pub trait UpdateBeamPointDataTrait: ProcessorDataTrait {
    fn num_points(&self) -> &u32;
    fn num_points_mut(&mut self) -> &mut u32;
    fn num_ctrl_points(&self) -> &u32;
    fn num_ctrl_points_mut(&mut self) -> &mut u32;
    fn taper_coefficients(&self) -> &super::core::Vec4;
    fn taper_coefficients_mut(&mut self) -> &mut super::core::Vec4;
    fn attractor(&self) -> &LocationSelection;
    fn attractor_mut(&mut self) -> &mut LocationSelection;
    fn attractor_coefficients(&self) -> &super::core::Vec4;
    fn attractor_coefficients_mut(&mut self) -> &mut super::core::Vec4;
    fn param_override(&self) -> &ParamOverrideSelection;
    fn param_override_mut(&mut self) -> &mut ParamOverrideSelection;
    fn param_coefficients(&self) -> &super::core::Vec4;
    fn param_coefficients_mut(&mut self) -> &mut super::core::Vec4;
    fn beam_interpolation(&self) -> &BeamInterpolation;
    fn beam_interpolation_mut(&mut self) -> &mut BeamInterpolation;
    fn coefficient(&self) -> &f32;
    fn coefficient_mut(&mut self) -> &mut f32;
}

impl UpdateBeamPointDataTrait for UpdateBeamPointData {
    fn num_points(&self) -> &u32 {
        &self.num_points
    }
    fn num_points_mut(&mut self) -> &mut u32 {
        &mut self.num_points
    }
    fn num_ctrl_points(&self) -> &u32 {
        &self.num_ctrl_points
    }
    fn num_ctrl_points_mut(&mut self) -> &mut u32 {
        &mut self.num_ctrl_points
    }
    fn taper_coefficients(&self) -> &super::core::Vec4 {
        &self.taper_coefficients
    }
    fn taper_coefficients_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.taper_coefficients
    }
    fn attractor(&self) -> &LocationSelection {
        &self.attractor
    }
    fn attractor_mut(&mut self) -> &mut LocationSelection {
        &mut self.attractor
    }
    fn attractor_coefficients(&self) -> &super::core::Vec4 {
        &self.attractor_coefficients
    }
    fn attractor_coefficients_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.attractor_coefficients
    }
    fn param_override(&self) -> &ParamOverrideSelection {
        &self.param_override
    }
    fn param_override_mut(&mut self) -> &mut ParamOverrideSelection {
        &mut self.param_override
    }
    fn param_coefficients(&self) -> &super::core::Vec4 {
        &self.param_coefficients
    }
    fn param_coefficients_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.param_coefficients
    }
    fn beam_interpolation(&self) -> &BeamInterpolation {
        &self.beam_interpolation
    }
    fn beam_interpolation_mut(&mut self) -> &mut BeamInterpolation {
        &mut self.beam_interpolation
    }
    fn coefficient(&self) -> &f32 {
        &self.coefficient
    }
    fn coefficient_mut(&mut self) -> &mut f32 {
        &mut self.coefficient
    }
}

impl ProcessorDataTrait for UpdateBeamPointData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateBeamPointData {
}

pub static UPDATEBEAMPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamPointData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateBeamPointData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "NumPoints",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UpdateBeamPointData, num_points),
            },
            FieldInfoData {
                name: "NumCtrlPoints",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UpdateBeamPointData, num_ctrl_points),
            },
            FieldInfoData {
                name: "TaperCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UpdateBeamPointData, taper_coefficients),
            },
            FieldInfoData {
                name: "Attractor",
                flags: MemberInfoFlags::new(0),
                field_type: "LocationSelection",
                rust_offset: offset_of!(UpdateBeamPointData, attractor),
            },
            FieldInfoData {
                name: "AttractorCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UpdateBeamPointData, attractor_coefficients),
            },
            FieldInfoData {
                name: "ParamOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "ParamOverrideSelection",
                rust_offset: offset_of!(UpdateBeamPointData, param_override),
            },
            FieldInfoData {
                name: "ParamCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UpdateBeamPointData, param_coefficients),
            },
            FieldInfoData {
                name: "BeamInterpolation",
                flags: MemberInfoFlags::new(0),
                field_type: "BeamInterpolation",
                rust_offset: offset_of!(UpdateBeamPointData, beam_interpolation),
            },
            FieldInfoData {
                name: "Coefficient",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateBeamPointData, coefficient),
            },
        ],
    }),
    array_type: Some(UPDATEBEAMPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateBeamPointData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEBEAMPOINTDATA_TYPE_INFO
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


pub static UPDATEBEAMPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamPointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateBeamPointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum BeamInterpolation {
    #[default]
    BeamInterpolation_Linear = 0,
    BeamInterpolation_Spline = 1,
    BeamInterpolation_Curve = 2,
}

pub static BEAMINTERPOLATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BeamInterpolation",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(BEAMINTERPOLATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BeamInterpolation {
    fn type_info(&self) -> &'static TypeInfo {
        BEAMINTERPOLATION_TYPE_INFO
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


pub static BEAMINTERPOLATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BeamInterpolation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("BeamInterpolation"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateBeamTargetData {
    pub _glacier_base: ProcessorData,
    pub target: LocationSelection,
}

pub trait UpdateBeamTargetDataTrait: ProcessorDataTrait {
    fn target(&self) -> &LocationSelection;
    fn target_mut(&mut self) -> &mut LocationSelection;
}

impl UpdateBeamTargetDataTrait for UpdateBeamTargetData {
    fn target(&self) -> &LocationSelection {
        &self.target
    }
    fn target_mut(&mut self) -> &mut LocationSelection {
        &mut self.target
    }
}

impl ProcessorDataTrait for UpdateBeamTargetData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateBeamTargetData {
}

pub static UPDATEBEAMTARGETDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamTargetData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateBeamTargetData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Target",
                flags: MemberInfoFlags::new(0),
                field_type: "LocationSelection",
                rust_offset: offset_of!(UpdateBeamTargetData, target),
            },
        ],
    }),
    array_type: Some(UPDATEBEAMTARGETDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateBeamTargetData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEBEAMTARGETDATA_TYPE_INFO
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


pub static UPDATEBEAMTARGETDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamTargetData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateBeamTargetData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateBeamSourceData {
    pub _glacier_base: ProcessorData,
    pub source: LocationSelection,
}

pub trait UpdateBeamSourceDataTrait: ProcessorDataTrait {
    fn source(&self) -> &LocationSelection;
    fn source_mut(&mut self) -> &mut LocationSelection;
}

impl UpdateBeamSourceDataTrait for UpdateBeamSourceData {
    fn source(&self) -> &LocationSelection {
        &self.source
    }
    fn source_mut(&mut self) -> &mut LocationSelection {
        &mut self.source
    }
}

impl ProcessorDataTrait for UpdateBeamSourceData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateBeamSourceData {
}

pub static UPDATEBEAMSOURCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamSourceData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateBeamSourceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Source",
                flags: MemberInfoFlags::new(0),
                field_type: "LocationSelection",
                rust_offset: offset_of!(UpdateBeamSourceData, source),
            },
        ],
    }),
    array_type: Some(UPDATEBEAMSOURCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateBeamSourceData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEBEAMSOURCEDATA_TYPE_INFO
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


pub static UPDATEBEAMSOURCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBeamSourceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateBeamSourceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ParamOverrideSelection {
    #[default]
    ParamOverride_None = 0,
    ParamOverride_R = 1,
    ParamOverride_G = 2,
    ParamOverride_B = 3,
    ParamOverride_A = 4,
}

pub static PARAMOVERRIDESELECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParamOverrideSelection",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(PARAMOVERRIDESELECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ParamOverrideSelection {
    fn type_info(&self) -> &'static TypeInfo {
        PARAMOVERRIDESELECTION_TYPE_INFO
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


pub static PARAMOVERRIDESELECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParamOverrideSelection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ParamOverrideSelection"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static LOCATIONSELECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationSelection",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(LOCATIONSELECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocationSelection {
    fn type_info(&self) -> &'static TypeInfo {
        LOCATIONSELECTION_TYPE_INFO
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


pub static LOCATIONSELECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationSelection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("LocationSelection"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateQuadBendingAngleData {
    pub _glacier_base: ProcessorData,
}

pub trait UpdateQuadBendingAngleDataTrait: ProcessorDataTrait {
}

impl UpdateQuadBendingAngleDataTrait for UpdateQuadBendingAngleData {
}

impl ProcessorDataTrait for UpdateQuadBendingAngleData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateQuadBendingAngleData {
}

pub static UPDATEQUADBENDINGANGLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateQuadBendingAngleData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateQuadBendingAngleData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UPDATEQUADBENDINGANGLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateQuadBendingAngleData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEQUADBENDINGANGLEDATA_TYPE_INFO
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


pub static UPDATEQUADBENDINGANGLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateQuadBendingAngleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateQuadBendingAngleData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateClipScaleData {
    pub _glacier_base: ProcessorData,
    pub lookup: Vec<i16>,
}

pub trait UpdateClipScaleDataTrait: ProcessorDataTrait {
    fn lookup(&self) -> &Vec<i16>;
    fn lookup_mut(&mut self) -> &mut Vec<i16>;
}

impl UpdateClipScaleDataTrait for UpdateClipScaleData {
    fn lookup(&self) -> &Vec<i16> {
        &self.lookup
    }
    fn lookup_mut(&mut self) -> &mut Vec<i16> {
        &mut self.lookup
    }
}

impl ProcessorDataTrait for UpdateClipScaleData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateClipScaleData {
}

pub static UPDATECLIPSCALEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateClipScaleData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateClipScaleData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Lookup",
                flags: MemberInfoFlags::new(144),
                field_type: "Int16-Array",
                rust_offset: offset_of!(UpdateClipScaleData, lookup),
            },
        ],
    }),
    array_type: Some(UPDATECLIPSCALEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateClipScaleData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATECLIPSCALEDATA_TYPE_INFO
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


pub static UPDATECLIPSCALEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateClipScaleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateClipScaleData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SnapToWaterData {
    pub _glacier_base: ProcessorData,
    pub offset: f32,
}

pub trait SnapToWaterDataTrait: ProcessorDataTrait {
    fn offset(&self) -> &f32;
    fn offset_mut(&mut self) -> &mut f32;
}

impl SnapToWaterDataTrait for SnapToWaterData {
    fn offset(&self) -> &f32 {
        &self.offset
    }
    fn offset_mut(&mut self) -> &mut f32 {
        &mut self.offset
    }
}

impl ProcessorDataTrait for SnapToWaterData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SnapToWaterData {
}

pub static SNAPTOWATERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapToWaterData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnapToWaterData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SnapToWaterData, offset),
            },
        ],
    }),
    array_type: Some(SNAPTOWATERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SnapToWaterData {
    fn type_info(&self) -> &'static TypeInfo {
        SNAPTOWATERDATA_TYPE_INFO
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


pub static SNAPTOWATERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnapToWaterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SnapToWaterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateCollisionData {
    pub _glacier_base: ProcessorData,
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

pub trait UpdateCollisionDataTrait: ProcessorDataTrait {
    fn restitution(&self) -> &f32;
    fn restitution_mut(&mut self) -> &mut f32;
    fn reflection_bias(&self) -> &f32;
    fn reflection_bias_mut(&mut self) -> &mut f32;
    fn rest_speed_threshold(&self) -> &f32;
    fn rest_speed_threshold_mut(&mut self) -> &mut f32;
    fn randomness(&self) -> &f32;
    fn randomness_mut(&mut self) -> &mut f32;
    fn kill_on_collision(&self) -> &bool;
    fn kill_on_collision_mut(&mut self) -> &mut bool;
    fn death_effect_orientation(&self) -> &EmitterCollisionEffectOrientation;
    fn death_effect_orientation_mut(&mut self) -> &mut EmitterCollisionEffectOrientation;
    fn collision_type(&self) -> &EmitterCollisionType;
    fn collision_type_mut(&mut self) -> &mut EmitterCollisionType;
    fn collision_radius_factor(&self) -> &f32;
    fn collision_radius_factor_mut(&mut self) -> &mut f32;
    fn method(&self) -> &EmitterCollisionMethod;
    fn method_mut(&mut self) -> &mut EmitterCollisionMethod;
    fn priority(&self) -> &EmitterCollisionPriority;
    fn priority_mut(&mut self) -> &mut EmitterCollisionPriority;
    fn snap_on_terrain(&self) -> &bool;
    fn snap_on_terrain_mut(&mut self) -> &mut bool;
    fn snap_offset_based_on_emitter(&self) -> &f32;
    fn snap_offset_based_on_emitter_mut(&mut self) -> &mut f32;
    fn snap_relative_offset_factor(&self) -> &f32;
    fn snap_relative_offset_factor_mut(&mut self) -> &mut f32;
    fn snap_range(&self) -> &f32;
    fn snap_range_mut(&mut self) -> &mut f32;
    fn snap_velocity(&self) -> &EmitterSnapVelocityType;
    fn snap_velocity_mut(&mut self) -> &mut EmitterSnapVelocityType;
    fn snap_type(&self) -> &EmitterTerrainSnapType;
    fn snap_type_mut(&mut self) -> &mut EmitterTerrainSnapType;
    fn repel_factor(&self) -> &f32;
    fn repel_factor_mut(&mut self) -> &mut f32;
    fn repel_height(&self) -> &f32;
    fn repel_height_mut(&mut self) -> &mut f32;
    fn material_pair(&self) -> &super::entity::MaterialDecl;
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl;
    fn throttle(&self) -> &f32;
    fn throttle_mut(&mut self) -> &mut f32;
    fn throttle_far_distance(&self) -> &f32;
    fn throttle_far_distance_mut(&mut self) -> &mut f32;
    fn throttle_envelope(&self) -> &super::core::Vec4;
    fn throttle_envelope_mut(&mut self) -> &mut super::core::Vec4;
    fn check_water(&self) -> &bool;
    fn check_water_mut(&mut self) -> &mut bool;
    fn check_terrain(&self) -> &bool;
    fn check_terrain_mut(&mut self) -> &mut bool;
    fn check_ragdoll(&self) -> &bool;
    fn check_ragdoll_mut(&mut self) -> &mut bool;
    fn check_character(&self) -> &bool;
    fn check_character_mut(&mut self) -> &mut bool;
    fn check_group(&self) -> &bool;
    fn check_group_mut(&mut self) -> &mut bool;
    fn check_phantoms(&self) -> &bool;
    fn check_phantoms_mut(&mut self) -> &mut bool;
    fn check_simple_shape(&self) -> &bool;
    fn check_simple_shape_mut(&mut self) -> &mut bool;
}

impl UpdateCollisionDataTrait for UpdateCollisionData {
    fn restitution(&self) -> &f32 {
        &self.restitution
    }
    fn restitution_mut(&mut self) -> &mut f32 {
        &mut self.restitution
    }
    fn reflection_bias(&self) -> &f32 {
        &self.reflection_bias
    }
    fn reflection_bias_mut(&mut self) -> &mut f32 {
        &mut self.reflection_bias
    }
    fn rest_speed_threshold(&self) -> &f32 {
        &self.rest_speed_threshold
    }
    fn rest_speed_threshold_mut(&mut self) -> &mut f32 {
        &mut self.rest_speed_threshold
    }
    fn randomness(&self) -> &f32 {
        &self.randomness
    }
    fn randomness_mut(&mut self) -> &mut f32 {
        &mut self.randomness
    }
    fn kill_on_collision(&self) -> &bool {
        &self.kill_on_collision
    }
    fn kill_on_collision_mut(&mut self) -> &mut bool {
        &mut self.kill_on_collision
    }
    fn death_effect_orientation(&self) -> &EmitterCollisionEffectOrientation {
        &self.death_effect_orientation
    }
    fn death_effect_orientation_mut(&mut self) -> &mut EmitterCollisionEffectOrientation {
        &mut self.death_effect_orientation
    }
    fn collision_type(&self) -> &EmitterCollisionType {
        &self.collision_type
    }
    fn collision_type_mut(&mut self) -> &mut EmitterCollisionType {
        &mut self.collision_type
    }
    fn collision_radius_factor(&self) -> &f32 {
        &self.collision_radius_factor
    }
    fn collision_radius_factor_mut(&mut self) -> &mut f32 {
        &mut self.collision_radius_factor
    }
    fn method(&self) -> &EmitterCollisionMethod {
        &self.method
    }
    fn method_mut(&mut self) -> &mut EmitterCollisionMethod {
        &mut self.method
    }
    fn priority(&self) -> &EmitterCollisionPriority {
        &self.priority
    }
    fn priority_mut(&mut self) -> &mut EmitterCollisionPriority {
        &mut self.priority
    }
    fn snap_on_terrain(&self) -> &bool {
        &self.snap_on_terrain
    }
    fn snap_on_terrain_mut(&mut self) -> &mut bool {
        &mut self.snap_on_terrain
    }
    fn snap_offset_based_on_emitter(&self) -> &f32 {
        &self.snap_offset_based_on_emitter
    }
    fn snap_offset_based_on_emitter_mut(&mut self) -> &mut f32 {
        &mut self.snap_offset_based_on_emitter
    }
    fn snap_relative_offset_factor(&self) -> &f32 {
        &self.snap_relative_offset_factor
    }
    fn snap_relative_offset_factor_mut(&mut self) -> &mut f32 {
        &mut self.snap_relative_offset_factor
    }
    fn snap_range(&self) -> &f32 {
        &self.snap_range
    }
    fn snap_range_mut(&mut self) -> &mut f32 {
        &mut self.snap_range
    }
    fn snap_velocity(&self) -> &EmitterSnapVelocityType {
        &self.snap_velocity
    }
    fn snap_velocity_mut(&mut self) -> &mut EmitterSnapVelocityType {
        &mut self.snap_velocity
    }
    fn snap_type(&self) -> &EmitterTerrainSnapType {
        &self.snap_type
    }
    fn snap_type_mut(&mut self) -> &mut EmitterTerrainSnapType {
        &mut self.snap_type
    }
    fn repel_factor(&self) -> &f32 {
        &self.repel_factor
    }
    fn repel_factor_mut(&mut self) -> &mut f32 {
        &mut self.repel_factor
    }
    fn repel_height(&self) -> &f32 {
        &self.repel_height
    }
    fn repel_height_mut(&mut self) -> &mut f32 {
        &mut self.repel_height
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        &self.material_pair
    }
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl {
        &mut self.material_pair
    }
    fn throttle(&self) -> &f32 {
        &self.throttle
    }
    fn throttle_mut(&mut self) -> &mut f32 {
        &mut self.throttle
    }
    fn throttle_far_distance(&self) -> &f32 {
        &self.throttle_far_distance
    }
    fn throttle_far_distance_mut(&mut self) -> &mut f32 {
        &mut self.throttle_far_distance
    }
    fn throttle_envelope(&self) -> &super::core::Vec4 {
        &self.throttle_envelope
    }
    fn throttle_envelope_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.throttle_envelope
    }
    fn check_water(&self) -> &bool {
        &self.check_water
    }
    fn check_water_mut(&mut self) -> &mut bool {
        &mut self.check_water
    }
    fn check_terrain(&self) -> &bool {
        &self.check_terrain
    }
    fn check_terrain_mut(&mut self) -> &mut bool {
        &mut self.check_terrain
    }
    fn check_ragdoll(&self) -> &bool {
        &self.check_ragdoll
    }
    fn check_ragdoll_mut(&mut self) -> &mut bool {
        &mut self.check_ragdoll
    }
    fn check_character(&self) -> &bool {
        &self.check_character
    }
    fn check_character_mut(&mut self) -> &mut bool {
        &mut self.check_character
    }
    fn check_group(&self) -> &bool {
        &self.check_group
    }
    fn check_group_mut(&mut self) -> &mut bool {
        &mut self.check_group
    }
    fn check_phantoms(&self) -> &bool {
        &self.check_phantoms
    }
    fn check_phantoms_mut(&mut self) -> &mut bool {
        &mut self.check_phantoms
    }
    fn check_simple_shape(&self) -> &bool {
        &self.check_simple_shape
    }
    fn check_simple_shape_mut(&mut self) -> &mut bool {
        &mut self.check_simple_shape
    }
}

impl ProcessorDataTrait for UpdateCollisionData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateCollisionData {
}

pub static UPDATECOLLISIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCollisionData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateCollisionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Restitution",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, restitution),
            },
            FieldInfoData {
                name: "ReflectionBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, reflection_bias),
            },
            FieldInfoData {
                name: "RestSpeedThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, rest_speed_threshold),
            },
            FieldInfoData {
                name: "Randomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, randomness),
            },
            FieldInfoData {
                name: "KillOnCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateCollisionData, kill_on_collision),
            },
            FieldInfoData {
                name: "DeathEffectOrientation",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterCollisionEffectOrientation",
                rust_offset: offset_of!(UpdateCollisionData, death_effect_orientation),
            },
            FieldInfoData {
                name: "CollisionType",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterCollisionType",
                rust_offset: offset_of!(UpdateCollisionData, collision_type),
            },
            FieldInfoData {
                name: "CollisionRadiusFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, collision_radius_factor),
            },
            FieldInfoData {
                name: "Method",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterCollisionMethod",
                rust_offset: offset_of!(UpdateCollisionData, method),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterCollisionPriority",
                rust_offset: offset_of!(UpdateCollisionData, priority),
            },
            FieldInfoData {
                name: "SnapOnTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateCollisionData, snap_on_terrain),
            },
            FieldInfoData {
                name: "SnapOffsetBasedOnEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, snap_offset_based_on_emitter),
            },
            FieldInfoData {
                name: "SnapRelativeOffsetFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, snap_relative_offset_factor),
            },
            FieldInfoData {
                name: "SnapRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, snap_range),
            },
            FieldInfoData {
                name: "SnapVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterSnapVelocityType",
                rust_offset: offset_of!(UpdateCollisionData, snap_velocity),
            },
            FieldInfoData {
                name: "SnapType",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterTerrainSnapType",
                rust_offset: offset_of!(UpdateCollisionData, snap_type),
            },
            FieldInfoData {
                name: "RepelFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, repel_factor),
            },
            FieldInfoData {
                name: "RepelHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, repel_height),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(UpdateCollisionData, material_pair),
            },
            FieldInfoData {
                name: "Throttle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, throttle),
            },
            FieldInfoData {
                name: "ThrottleFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCollisionData, throttle_far_distance),
            },
            FieldInfoData {
                name: "ThrottleEnvelope",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UpdateCollisionData, throttle_envelope),
            },
            FieldInfoData {
                name: "CheckWater",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateCollisionData, check_water),
            },
            FieldInfoData {
                name: "CheckTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateCollisionData, check_terrain),
            },
            FieldInfoData {
                name: "CheckRagdoll",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateCollisionData, check_ragdoll),
            },
            FieldInfoData {
                name: "CheckCharacter",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateCollisionData, check_character),
            },
            FieldInfoData {
                name: "CheckGroup",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateCollisionData, check_group),
            },
            FieldInfoData {
                name: "CheckPhantoms",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateCollisionData, check_phantoms),
            },
            FieldInfoData {
                name: "CheckSimpleShape",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateCollisionData, check_simple_shape),
            },
        ],
    }),
    array_type: Some(UPDATECOLLISIONDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateCollisionData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATECOLLISIONDATA_TYPE_INFO
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


pub static UPDATECOLLISIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCollisionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateCollisionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterCollisionEffectOrientation {
    #[default]
    EmitterCollisionEffectOrientation_BounceDirection = 0,
    EmitterCollisionEffectOrientation_Normal = 1,
    EmitterCollisionEffectOrientation_ImpactDirection = 2,
}

pub static EMITTERCOLLISIONEFFECTORIENTATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionEffectOrientation",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERCOLLISIONEFFECTORIENTATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterCollisionEffectOrientation {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERCOLLISIONEFFECTORIENTATION_TYPE_INFO
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


pub static EMITTERCOLLISIONEFFECTORIENTATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionEffectOrientation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterCollisionEffectOrientation"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterCollisionType {
    #[default]
    EmitterCollisionType_PassThrough = 0,
    EmitterCollisionType_Stick = 1,
    EmitterCollisionType_Bounce = 2,
    EmitterCollisionType_Count = 3,
}

pub static EMITTERCOLLISIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERCOLLISIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterCollisionType {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERCOLLISIONTYPE_TYPE_INFO
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


pub static EMITTERCOLLISIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterCollisionType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterCollisionPriority {
    #[default]
    EmitterCollisionPriority_Low = 0,
    EmitterCollisionPriority_Medium = 1,
    EmitterCollisionPriority_High = 2,
}

pub static EMITTERCOLLISIONPRIORITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionPriority",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERCOLLISIONPRIORITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterCollisionPriority {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERCOLLISIONPRIORITY_TYPE_INFO
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


pub static EMITTERCOLLISIONPRIORITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionPriority-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterCollisionPriority"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterSnapVelocityType {
    #[default]
    EmitterSnapVelocityType_Disabled = 0,
    EmitterSnapVelocityType_MaintainMagnitude = 1,
    EmitterSnapVelocityType_MaintainDirection = 2,
}

pub static EMITTERSNAPVELOCITYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSnapVelocityType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERSNAPVELOCITYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterSnapVelocityType {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERSNAPVELOCITYTYPE_TYPE_INFO
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


pub static EMITTERSNAPVELOCITYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSnapVelocityType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterSnapVelocityType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterTerrainSnapType {
    #[default]
    EmitterTerrainSnapType_SpawnOnly = 0,
    EmitterTerrainSnapType_RenderingOnly = 1,
    EmitterTerrainSnapType_SpawnAndRendering = 2,
    EmitterTerrainSnapType_SpawnAndRepel = 3,
}

pub static EMITTERTERRAINSNAPTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTerrainSnapType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERTERRAINSNAPTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterTerrainSnapType {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERTERRAINSNAPTYPE_TYPE_INFO
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


pub static EMITTERTERRAINSNAPTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTerrainSnapType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterTerrainSnapType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterCollisionMethod {
    #[default]
    EmitterCollisionMethod_TerrainHeightMap = 0,
    EmitterCollisionMethod_RayCast = 1,
    EmitterCollisionMethod_RayCastDetailed = 2,
}

pub static EMITTERCOLLISIONMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERCOLLISIONMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterCollisionMethod {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERCOLLISIONMETHOD_TYPE_INFO
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


pub static EMITTERCOLLISIONMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCollisionMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterCollisionMethod"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateCameraProximityData {
    pub _glacier_base: ProcessorData,
    pub size: super::core::Vec3,
    pub fade_distance: super::core::Vec3,
    pub forward_offset: f32,
}

pub trait UpdateCameraProximityDataTrait: ProcessorDataTrait {
    fn size(&self) -> &super::core::Vec3;
    fn size_mut(&mut self) -> &mut super::core::Vec3;
    fn fade_distance(&self) -> &super::core::Vec3;
    fn fade_distance_mut(&mut self) -> &mut super::core::Vec3;
    fn forward_offset(&self) -> &f32;
    fn forward_offset_mut(&mut self) -> &mut f32;
}

impl UpdateCameraProximityDataTrait for UpdateCameraProximityData {
    fn size(&self) -> &super::core::Vec3 {
        &self.size
    }
    fn size_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.size
    }
    fn fade_distance(&self) -> &super::core::Vec3 {
        &self.fade_distance
    }
    fn fade_distance_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.fade_distance
    }
    fn forward_offset(&self) -> &f32 {
        &self.forward_offset
    }
    fn forward_offset_mut(&mut self) -> &mut f32 {
        &mut self.forward_offset
    }
}

impl ProcessorDataTrait for UpdateCameraProximityData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateCameraProximityData {
}

pub static UPDATECAMERAPROXIMITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCameraProximityData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateCameraProximityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UpdateCameraProximityData, size),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UpdateCameraProximityData, fade_distance),
            },
            FieldInfoData {
                name: "ForwardOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateCameraProximityData, forward_offset),
            },
        ],
    }),
    array_type: Some(UPDATECAMERAPROXIMITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateCameraProximityData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATECAMERAPROXIMITYDATA_TYPE_INFO
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


pub static UPDATECAMERAPROXIMITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCameraProximityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateCameraProximityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateTrapezoidShapeData {
    pub _glacier_base: ProcessorData,
    pub scale: f32,
}

pub trait UpdateTrapezoidShapeDataTrait: ProcessorDataTrait {
    fn scale(&self) -> &f32;
    fn scale_mut(&mut self) -> &mut f32;
}

impl UpdateTrapezoidShapeDataTrait for UpdateTrapezoidShapeData {
    fn scale(&self) -> &f32 {
        &self.scale
    }
    fn scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }
}

impl ProcessorDataTrait for UpdateTrapezoidShapeData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateTrapezoidShapeData {
}

pub static UPDATETRAPEZOIDSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTrapezoidShapeData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateTrapezoidShapeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateTrapezoidShapeData, scale),
            },
        ],
    }),
    array_type: Some(UPDATETRAPEZOIDSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateTrapezoidShapeData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATETRAPEZOIDSHAPEDATA_TYPE_INFO
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


pub static UPDATETRAPEZOIDSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTrapezoidShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateTrapezoidShapeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateBackLightData {
    pub _glacier_base: ProcessorData,
    pub vertex_back_light: f32,
    pub gnomon_back_light: f32,
    pub pixel_contrast: f32,
    pub view_independent_contrast: f32,
}

pub trait UpdateBackLightDataTrait: ProcessorDataTrait {
    fn vertex_back_light(&self) -> &f32;
    fn vertex_back_light_mut(&mut self) -> &mut f32;
    fn gnomon_back_light(&self) -> &f32;
    fn gnomon_back_light_mut(&mut self) -> &mut f32;
    fn pixel_contrast(&self) -> &f32;
    fn pixel_contrast_mut(&mut self) -> &mut f32;
    fn view_independent_contrast(&self) -> &f32;
    fn view_independent_contrast_mut(&mut self) -> &mut f32;
}

impl UpdateBackLightDataTrait for UpdateBackLightData {
    fn vertex_back_light(&self) -> &f32 {
        &self.vertex_back_light
    }
    fn vertex_back_light_mut(&mut self) -> &mut f32 {
        &mut self.vertex_back_light
    }
    fn gnomon_back_light(&self) -> &f32 {
        &self.gnomon_back_light
    }
    fn gnomon_back_light_mut(&mut self) -> &mut f32 {
        &mut self.gnomon_back_light
    }
    fn pixel_contrast(&self) -> &f32 {
        &self.pixel_contrast
    }
    fn pixel_contrast_mut(&mut self) -> &mut f32 {
        &mut self.pixel_contrast
    }
    fn view_independent_contrast(&self) -> &f32 {
        &self.view_independent_contrast
    }
    fn view_independent_contrast_mut(&mut self) -> &mut f32 {
        &mut self.view_independent_contrast
    }
}

impl ProcessorDataTrait for UpdateBackLightData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateBackLightData {
}

pub static UPDATEBACKLIGHTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBackLightData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateBackLightData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "VertexBackLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateBackLightData, vertex_back_light),
            },
            FieldInfoData {
                name: "GnomonBackLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateBackLightData, gnomon_back_light),
            },
            FieldInfoData {
                name: "PixelContrast",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateBackLightData, pixel_contrast),
            },
            FieldInfoData {
                name: "ViewIndependentContrast",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateBackLightData, view_independent_contrast),
            },
        ],
    }),
    array_type: Some(UPDATEBACKLIGHTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateBackLightData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEBACKLIGHTDATA_TYPE_INFO
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


pub static UPDATEBACKLIGHTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateBackLightData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateBackLightData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateLightWrapAroundData {
    pub _glacier_base: ProcessorData,
}

pub trait UpdateLightWrapAroundDataTrait: ProcessorDataTrait {
}

impl UpdateLightWrapAroundDataTrait for UpdateLightWrapAroundData {
}

impl ProcessorDataTrait for UpdateLightWrapAroundData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateLightWrapAroundData {
}

pub static UPDATELIGHTWRAPAROUNDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateLightWrapAroundData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateLightWrapAroundData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UPDATELIGHTWRAPAROUNDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateLightWrapAroundData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATELIGHTWRAPAROUNDDATA_TYPE_INFO
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


pub static UPDATELIGHTWRAPAROUNDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateLightWrapAroundData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateLightWrapAroundData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateTextureColorLerpData {
    pub _glacier_base: ProcessorData,
    pub texture_color_strength: f32,
}

pub trait UpdateTextureColorLerpDataTrait: ProcessorDataTrait {
    fn texture_color_strength(&self) -> &f32;
    fn texture_color_strength_mut(&mut self) -> &mut f32;
}

impl UpdateTextureColorLerpDataTrait for UpdateTextureColorLerpData {
    fn texture_color_strength(&self) -> &f32 {
        &self.texture_color_strength
    }
    fn texture_color_strength_mut(&mut self) -> &mut f32 {
        &mut self.texture_color_strength
    }
}

impl ProcessorDataTrait for UpdateTextureColorLerpData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateTextureColorLerpData {
}

pub static UPDATETEXTURECOLORLERPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTextureColorLerpData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateTextureColorLerpData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TextureColorStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateTextureColorLerpData, texture_color_strength),
            },
        ],
    }),
    array_type: Some(UPDATETEXTURECOLORLERPDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateTextureColorLerpData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATETEXTURECOLORLERPDATA_TYPE_INFO
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


pub static UPDATETEXTURECOLORLERPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTextureColorLerpData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateTextureColorLerpData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateAlphaLevelScaleData {
    pub _glacier_base: ProcessorData,
    pub exponent: f32,
}

pub trait UpdateAlphaLevelScaleDataTrait: ProcessorDataTrait {
    fn exponent(&self) -> &f32;
    fn exponent_mut(&mut self) -> &mut f32;
}

impl UpdateAlphaLevelScaleDataTrait for UpdateAlphaLevelScaleData {
    fn exponent(&self) -> &f32 {
        &self.exponent
    }
    fn exponent_mut(&mut self) -> &mut f32 {
        &mut self.exponent
    }
}

impl ProcessorDataTrait for UpdateAlphaLevelScaleData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateAlphaLevelScaleData {
}

pub static UPDATEALPHALEVELSCALEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelScaleData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateAlphaLevelScaleData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Exponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateAlphaLevelScaleData, exponent),
            },
        ],
    }),
    array_type: Some(UPDATEALPHALEVELSCALEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateAlphaLevelScaleData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEALPHALEVELSCALEDATA_TYPE_INFO
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


pub static UPDATEALPHALEVELSCALEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelScaleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateAlphaLevelScaleData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateAlphaLevelMaxData {
    pub _glacier_base: ProcessorData,
    pub max_level: f32,
}

pub trait UpdateAlphaLevelMaxDataTrait: ProcessorDataTrait {
    fn max_level(&self) -> &f32;
    fn max_level_mut(&mut self) -> &mut f32;
}

impl UpdateAlphaLevelMaxDataTrait for UpdateAlphaLevelMaxData {
    fn max_level(&self) -> &f32 {
        &self.max_level
    }
    fn max_level_mut(&mut self) -> &mut f32 {
        &mut self.max_level
    }
}

impl ProcessorDataTrait for UpdateAlphaLevelMaxData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateAlphaLevelMaxData {
}

pub static UPDATEALPHALEVELMAXDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelMaxData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateAlphaLevelMaxData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateAlphaLevelMaxData, max_level),
            },
        ],
    }),
    array_type: Some(UPDATEALPHALEVELMAXDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateAlphaLevelMaxData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEALPHALEVELMAXDATA_TYPE_INFO
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


pub static UPDATEALPHALEVELMAXDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelMaxData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateAlphaLevelMaxData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateAlphaLevelMinData {
    pub _glacier_base: ProcessorData,
    pub min_level: f32,
}

pub trait UpdateAlphaLevelMinDataTrait: ProcessorDataTrait {
    fn min_level(&self) -> &f32;
    fn min_level_mut(&mut self) -> &mut f32;
}

impl UpdateAlphaLevelMinDataTrait for UpdateAlphaLevelMinData {
    fn min_level(&self) -> &f32 {
        &self.min_level
    }
    fn min_level_mut(&mut self) -> &mut f32 {
        &mut self.min_level
    }
}

impl ProcessorDataTrait for UpdateAlphaLevelMinData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateAlphaLevelMinData {
}

pub static UPDATEALPHALEVELMINDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelMinData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateAlphaLevelMinData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MinLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateAlphaLevelMinData, min_level),
            },
        ],
    }),
    array_type: Some(UPDATEALPHALEVELMINDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateAlphaLevelMinData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEALPHALEVELMINDATA_TYPE_INFO
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


pub static UPDATEALPHALEVELMINDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAlphaLevelMinData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateAlphaLevelMinData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateRibbonTextureData {
    pub _glacier_base: ProcessorData,
    pub texture_particle_count: i32,
    pub mirror_texture: bool,
    pub beam_like_coords: bool,
}

pub trait UpdateRibbonTextureDataTrait: ProcessorDataTrait {
    fn texture_particle_count(&self) -> &i32;
    fn texture_particle_count_mut(&mut self) -> &mut i32;
    fn mirror_texture(&self) -> &bool;
    fn mirror_texture_mut(&mut self) -> &mut bool;
    fn beam_like_coords(&self) -> &bool;
    fn beam_like_coords_mut(&mut self) -> &mut bool;
}

impl UpdateRibbonTextureDataTrait for UpdateRibbonTextureData {
    fn texture_particle_count(&self) -> &i32 {
        &self.texture_particle_count
    }
    fn texture_particle_count_mut(&mut self) -> &mut i32 {
        &mut self.texture_particle_count
    }
    fn mirror_texture(&self) -> &bool {
        &self.mirror_texture
    }
    fn mirror_texture_mut(&mut self) -> &mut bool {
        &mut self.mirror_texture
    }
    fn beam_like_coords(&self) -> &bool {
        &self.beam_like_coords
    }
    fn beam_like_coords_mut(&mut self) -> &mut bool {
        &mut self.beam_like_coords
    }
}

impl ProcessorDataTrait for UpdateRibbonTextureData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateRibbonTextureData {
}

pub static UPDATERIBBONTEXTUREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRibbonTextureData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateRibbonTextureData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TextureParticleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UpdateRibbonTextureData, texture_particle_count),
            },
            FieldInfoData {
                name: "MirrorTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateRibbonTextureData, mirror_texture),
            },
            FieldInfoData {
                name: "BeamLikeCoords",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateRibbonTextureData, beam_like_coords),
            },
        ],
    }),
    array_type: Some(UPDATERIBBONTEXTUREDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateRibbonTextureData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATERIBBONTEXTUREDATA_TYPE_INFO
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


pub static UPDATERIBBONTEXTUREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRibbonTextureData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateRibbonTextureData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateRibbonFadeData {
    pub _glacier_base: ProcessorData,
    pub fade_in_particle_count: i32,
    pub fade_out_particle_count: i32,
}

pub trait UpdateRibbonFadeDataTrait: ProcessorDataTrait {
    fn fade_in_particle_count(&self) -> &i32;
    fn fade_in_particle_count_mut(&mut self) -> &mut i32;
    fn fade_out_particle_count(&self) -> &i32;
    fn fade_out_particle_count_mut(&mut self) -> &mut i32;
}

impl UpdateRibbonFadeDataTrait for UpdateRibbonFadeData {
    fn fade_in_particle_count(&self) -> &i32 {
        &self.fade_in_particle_count
    }
    fn fade_in_particle_count_mut(&mut self) -> &mut i32 {
        &mut self.fade_in_particle_count
    }
    fn fade_out_particle_count(&self) -> &i32 {
        &self.fade_out_particle_count
    }
    fn fade_out_particle_count_mut(&mut self) -> &mut i32 {
        &mut self.fade_out_particle_count
    }
}

impl ProcessorDataTrait for UpdateRibbonFadeData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateRibbonFadeData {
}

pub static UPDATERIBBONFADEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRibbonFadeData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateRibbonFadeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FadeInParticleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UpdateRibbonFadeData, fade_in_particle_count),
            },
            FieldInfoData {
                name: "FadeOutParticleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UpdateRibbonFadeData, fade_out_particle_count),
            },
        ],
    }),
    array_type: Some(UPDATERIBBONFADEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateRibbonFadeData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATERIBBONFADEDATA_TYPE_INFO
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


pub static UPDATERIBBONFADEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRibbonFadeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateRibbonFadeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateTransparencySecondaryData {
    pub _glacier_base: ProcessorData,
}

pub trait UpdateTransparencySecondaryDataTrait: ProcessorDataTrait {
}

impl UpdateTransparencySecondaryDataTrait for UpdateTransparencySecondaryData {
}

impl ProcessorDataTrait for UpdateTransparencySecondaryData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateTransparencySecondaryData {
}

pub static UPDATETRANSPARENCYSECONDARYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTransparencySecondaryData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateTransparencySecondaryData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UPDATETRANSPARENCYSECONDARYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateTransparencySecondaryData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATETRANSPARENCYSECONDARYDATA_TYPE_INFO
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


pub static UPDATETRANSPARENCYSECONDARYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTransparencySecondaryData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateTransparencySecondaryData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateTransparencyData {
    pub _glacier_base: ProcessorData,
}

pub trait UpdateTransparencyDataTrait: ProcessorDataTrait {
}

impl UpdateTransparencyDataTrait for UpdateTransparencyData {
}

impl ProcessorDataTrait for UpdateTransparencyData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateTransparencyData {
}

pub static UPDATETRANSPARENCYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTransparencyData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateTransparencyData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UPDATETRANSPARENCYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateTransparencyData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATETRANSPARENCYDATA_TYPE_INFO
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


pub static UPDATETRANSPARENCYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTransparencyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateTransparencyData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateColorSecondaryData {
    pub _glacier_base: ProcessorData,
    pub color: super::core::Vec3,
}

pub trait UpdateColorSecondaryDataTrait: ProcessorDataTrait {
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
}

impl UpdateColorSecondaryDataTrait for UpdateColorSecondaryData {
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
}

impl ProcessorDataTrait for UpdateColorSecondaryData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateColorSecondaryData {
}

pub static UPDATECOLORSECONDARYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateColorSecondaryData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateColorSecondaryData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UpdateColorSecondaryData, color),
            },
        ],
    }),
    array_type: Some(UPDATECOLORSECONDARYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateColorSecondaryData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATECOLORSECONDARYDATA_TYPE_INFO
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


pub static UPDATECOLORSECONDARYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateColorSecondaryData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateColorSecondaryData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateColorData {
    pub _glacier_base: ProcessorData,
    pub color: super::core::Vec3,
}

pub trait UpdateColorDataTrait: ProcessorDataTrait {
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
}

impl UpdateColorDataTrait for UpdateColorData {
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
}

impl ProcessorDataTrait for UpdateColorData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateColorData {
}

pub static UPDATECOLORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateColorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateColorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(UpdateColorData, color),
            },
        ],
    }),
    array_type: Some(UPDATECOLORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateColorData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATECOLORDATA_TYPE_INFO
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


pub static UPDATECOLORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateColorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateColorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateCustomParamsData {
    pub _glacier_base: ProcessorData,
}

pub trait UpdateCustomParamsDataTrait: ProcessorDataTrait {
}

impl UpdateCustomParamsDataTrait for UpdateCustomParamsData {
}

impl ProcessorDataTrait for UpdateCustomParamsData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateCustomParamsData {
}

pub static UPDATECUSTOMPARAMSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCustomParamsData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateCustomParamsData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UPDATECUSTOMPARAMSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateCustomParamsData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATECUSTOMPARAMSDATA_TYPE_INFO
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


pub static UPDATECUSTOMPARAMSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateCustomParamsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateCustomParamsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateSizeZData {
    pub _glacier_base: ProcessorData,
}

pub trait UpdateSizeZDataTrait: ProcessorDataTrait {
}

impl UpdateSizeZDataTrait for UpdateSizeZData {
}

impl ProcessorDataTrait for UpdateSizeZData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateSizeZData {
}

pub static UPDATESIZEZDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeZData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateSizeZData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UPDATESIZEZDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateSizeZData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATESIZEZDATA_TYPE_INFO
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


pub static UPDATESIZEZDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeZData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateSizeZData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateSizeYData {
    pub _glacier_base: ProcessorData,
}

pub trait UpdateSizeYDataTrait: ProcessorDataTrait {
}

impl UpdateSizeYDataTrait for UpdateSizeYData {
}

impl ProcessorDataTrait for UpdateSizeYData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateSizeYData {
}

pub static UPDATESIZEYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeYData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateSizeYData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UPDATESIZEYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateSizeYData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATESIZEYDATA_TYPE_INFO
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


pub static UPDATESIZEYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeYData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateSizeYData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateSizeXData {
    pub _glacier_base: ProcessorData,
}

pub trait UpdateSizeXDataTrait: ProcessorDataTrait {
}

impl UpdateSizeXDataTrait for UpdateSizeXData {
}

impl ProcessorDataTrait for UpdateSizeXData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateSizeXData {
}

pub static UPDATESIZEXDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeXData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateSizeXData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UPDATESIZEXDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateSizeXData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATESIZEXDATA_TYPE_INFO
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


pub static UPDATESIZEXDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeXData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateSizeXData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateSizeData {
    pub _glacier_base: ProcessorData,
    pub pivot: super::core::Vec2,
    pub multiply_with_size_x_y_z: bool,
}

pub trait UpdateSizeDataTrait: ProcessorDataTrait {
    fn pivot(&self) -> &super::core::Vec2;
    fn pivot_mut(&mut self) -> &mut super::core::Vec2;
    fn multiply_with_size_x_y_z(&self) -> &bool;
    fn multiply_with_size_x_y_z_mut(&mut self) -> &mut bool;
}

impl UpdateSizeDataTrait for UpdateSizeData {
    fn pivot(&self) -> &super::core::Vec2 {
        &self.pivot
    }
    fn pivot_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.pivot
    }
    fn multiply_with_size_x_y_z(&self) -> &bool {
        &self.multiply_with_size_x_y_z
    }
    fn multiply_with_size_x_y_z_mut(&mut self) -> &mut bool {
        &mut self.multiply_with_size_x_y_z
    }
}

impl ProcessorDataTrait for UpdateSizeData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateSizeData {
}

pub static UPDATESIZEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateSizeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Pivot",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UpdateSizeData, pivot),
            },
            FieldInfoData {
                name: "MultiplyWithSizeXYZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateSizeData, multiply_with_size_x_y_z),
            },
        ],
    }),
    array_type: Some(UPDATESIZEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateSizeData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATESIZEDATA_TYPE_INFO
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


pub static UPDATESIZEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateSizeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateSizeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateRotationData {
    pub _glacier_base: ProcessorData,
}

pub trait UpdateRotationDataTrait: ProcessorDataTrait {
}

impl UpdateRotationDataTrait for UpdateRotationData {
}

impl ProcessorDataTrait for UpdateRotationData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateRotationData {
}

pub static UPDATEROTATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRotationData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateRotationData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UPDATEROTATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateRotationData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEROTATIONDATA_TYPE_INFO
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


pub static UPDATEROTATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateRotationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateRotationData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateTextureCoordsData {
    pub _glacier_base: ProcessorData,
    pub base_texture: Option<Arc<Mutex<dyn super::render::AtlasTextureAssetTrait>>>,
    pub normal_texture: Option<Arc<Mutex<dyn super::render::AtlasTextureAssetTrait>>>,
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

pub trait UpdateTextureCoordsDataTrait: ProcessorDataTrait {
    fn base_texture(&self) -> &Option<Arc<Mutex<dyn super::render::AtlasTextureAssetTrait>>>;
    fn base_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::AtlasTextureAssetTrait>>>;
    fn normal_texture(&self) -> &Option<Arc<Mutex<dyn super::render::AtlasTextureAssetTrait>>>;
    fn normal_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::AtlasTextureAssetTrait>>>;
    fn disable_clip_scale_optimization(&self) -> &bool;
    fn disable_clip_scale_optimization_mut(&mut self) -> &mut bool;
    fn modifier_u(&self) -> &TexCoordModifier;
    fn modifier_u_mut(&mut self) -> &mut TexCoordModifier;
    fn modifier_v(&self) -> &TexCoordModifier;
    fn modifier_v_mut(&mut self) -> &mut TexCoordModifier;
    fn scale_u(&self) -> &f32;
    fn scale_u_mut(&mut self) -> &mut f32;
    fn scale_v(&self) -> &f32;
    fn scale_v_mut(&mut self) -> &mut f32;
    fn bias_u(&self) -> &f32;
    fn bias_u_mut(&mut self) -> &mut f32;
    fn bias_v(&self) -> &f32;
    fn bias_v_mut(&mut self) -> &mut f32;
    fn direct_texture_frame_lookup(&self) -> &bool;
    fn direct_texture_frame_lookup_mut(&mut self) -> &mut bool;
    fn input_start_min(&self) -> &f32;
    fn input_start_min_mut(&mut self) -> &mut f32;
    fn input_start_max(&self) -> &f32;
    fn input_start_max_mut(&mut self) -> &mut f32;
    fn enable_frame_blending(&self) -> &bool;
    fn enable_frame_blending_mut(&mut self) -> &mut bool;
}

impl UpdateTextureCoordsDataTrait for UpdateTextureCoordsData {
    fn base_texture(&self) -> &Option<Arc<Mutex<dyn super::render::AtlasTextureAssetTrait>>> {
        &self.base_texture
    }
    fn base_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::AtlasTextureAssetTrait>>> {
        &mut self.base_texture
    }
    fn normal_texture(&self) -> &Option<Arc<Mutex<dyn super::render::AtlasTextureAssetTrait>>> {
        &self.normal_texture
    }
    fn normal_texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render::AtlasTextureAssetTrait>>> {
        &mut self.normal_texture
    }
    fn disable_clip_scale_optimization(&self) -> &bool {
        &self.disable_clip_scale_optimization
    }
    fn disable_clip_scale_optimization_mut(&mut self) -> &mut bool {
        &mut self.disable_clip_scale_optimization
    }
    fn modifier_u(&self) -> &TexCoordModifier {
        &self.modifier_u
    }
    fn modifier_u_mut(&mut self) -> &mut TexCoordModifier {
        &mut self.modifier_u
    }
    fn modifier_v(&self) -> &TexCoordModifier {
        &self.modifier_v
    }
    fn modifier_v_mut(&mut self) -> &mut TexCoordModifier {
        &mut self.modifier_v
    }
    fn scale_u(&self) -> &f32 {
        &self.scale_u
    }
    fn scale_u_mut(&mut self) -> &mut f32 {
        &mut self.scale_u
    }
    fn scale_v(&self) -> &f32 {
        &self.scale_v
    }
    fn scale_v_mut(&mut self) -> &mut f32 {
        &mut self.scale_v
    }
    fn bias_u(&self) -> &f32 {
        &self.bias_u
    }
    fn bias_u_mut(&mut self) -> &mut f32 {
        &mut self.bias_u
    }
    fn bias_v(&self) -> &f32 {
        &self.bias_v
    }
    fn bias_v_mut(&mut self) -> &mut f32 {
        &mut self.bias_v
    }
    fn direct_texture_frame_lookup(&self) -> &bool {
        &self.direct_texture_frame_lookup
    }
    fn direct_texture_frame_lookup_mut(&mut self) -> &mut bool {
        &mut self.direct_texture_frame_lookup
    }
    fn input_start_min(&self) -> &f32 {
        &self.input_start_min
    }
    fn input_start_min_mut(&mut self) -> &mut f32 {
        &mut self.input_start_min
    }
    fn input_start_max(&self) -> &f32 {
        &self.input_start_max
    }
    fn input_start_max_mut(&mut self) -> &mut f32 {
        &mut self.input_start_max
    }
    fn enable_frame_blending(&self) -> &bool {
        &self.enable_frame_blending
    }
    fn enable_frame_blending_mut(&mut self) -> &mut bool {
        &mut self.enable_frame_blending
    }
}

impl ProcessorDataTrait for UpdateTextureCoordsData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateTextureCoordsData {
}

pub static UPDATETEXTURECOORDSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTextureCoordsData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateTextureCoordsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BaseTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "AtlasTextureAsset",
                rust_offset: offset_of!(UpdateTextureCoordsData, base_texture),
            },
            FieldInfoData {
                name: "NormalTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "AtlasTextureAsset",
                rust_offset: offset_of!(UpdateTextureCoordsData, normal_texture),
            },
            FieldInfoData {
                name: "DisableClipScaleOptimization",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateTextureCoordsData, disable_clip_scale_optimization),
            },
            FieldInfoData {
                name: "ModifierU",
                flags: MemberInfoFlags::new(0),
                field_type: "TexCoordModifier",
                rust_offset: offset_of!(UpdateTextureCoordsData, modifier_u),
            },
            FieldInfoData {
                name: "ModifierV",
                flags: MemberInfoFlags::new(0),
                field_type: "TexCoordModifier",
                rust_offset: offset_of!(UpdateTextureCoordsData, modifier_v),
            },
            FieldInfoData {
                name: "ScaleU",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateTextureCoordsData, scale_u),
            },
            FieldInfoData {
                name: "ScaleV",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateTextureCoordsData, scale_v),
            },
            FieldInfoData {
                name: "BiasU",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateTextureCoordsData, bias_u),
            },
            FieldInfoData {
                name: "BiasV",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateTextureCoordsData, bias_v),
            },
            FieldInfoData {
                name: "DirectTextureFrameLookup",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateTextureCoordsData, direct_texture_frame_lookup),
            },
            FieldInfoData {
                name: "InputStartMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateTextureCoordsData, input_start_min),
            },
            FieldInfoData {
                name: "InputStartMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateTextureCoordsData, input_start_max),
            },
            FieldInfoData {
                name: "EnableFrameBlending",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateTextureCoordsData, enable_frame_blending),
            },
        ],
    }),
    array_type: Some(UPDATETEXTURECOORDSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateTextureCoordsData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATETEXTURECOORDSDATA_TYPE_INFO
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


pub static UPDATETEXTURECOORDSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateTextureCoordsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateTextureCoordsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TexCoordModifier {
    #[default]
    TCM_None = 0,
    TCM_Absolute = 1,
    TCM_WorldSpaceExtent = 2,
    TCM_PerBeamPoint = 3,
}

pub static TEXCOORDMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TexCoordModifier",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(TEXCOORDMODIFIER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TexCoordModifier {
    fn type_info(&self) -> &'static TypeInfo {
        TEXCOORDMODIFIER_TYPE_INFO
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


pub static TEXCOORDMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TexCoordModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("TexCoordModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateMimicOverridesData {
    pub _glacier_base: ProcessorData,
    pub lifetime_scale: f32,
    pub unique_random: bool,
    pub size_scale: f32,
    pub size_y_scale_multiplier: f32,
    pub uniform_scale_original: bool,
    pub override_rotation: bool,
}

pub trait UpdateMimicOverridesDataTrait: ProcessorDataTrait {
    fn lifetime_scale(&self) -> &f32;
    fn lifetime_scale_mut(&mut self) -> &mut f32;
    fn unique_random(&self) -> &bool;
    fn unique_random_mut(&mut self) -> &mut bool;
    fn size_scale(&self) -> &f32;
    fn size_scale_mut(&mut self) -> &mut f32;
    fn size_y_scale_multiplier(&self) -> &f32;
    fn size_y_scale_multiplier_mut(&mut self) -> &mut f32;
    fn uniform_scale_original(&self) -> &bool;
    fn uniform_scale_original_mut(&mut self) -> &mut bool;
    fn override_rotation(&self) -> &bool;
    fn override_rotation_mut(&mut self) -> &mut bool;
}

impl UpdateMimicOverridesDataTrait for UpdateMimicOverridesData {
    fn lifetime_scale(&self) -> &f32 {
        &self.lifetime_scale
    }
    fn lifetime_scale_mut(&mut self) -> &mut f32 {
        &mut self.lifetime_scale
    }
    fn unique_random(&self) -> &bool {
        &self.unique_random
    }
    fn unique_random_mut(&mut self) -> &mut bool {
        &mut self.unique_random
    }
    fn size_scale(&self) -> &f32 {
        &self.size_scale
    }
    fn size_scale_mut(&mut self) -> &mut f32 {
        &mut self.size_scale
    }
    fn size_y_scale_multiplier(&self) -> &f32 {
        &self.size_y_scale_multiplier
    }
    fn size_y_scale_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.size_y_scale_multiplier
    }
    fn uniform_scale_original(&self) -> &bool {
        &self.uniform_scale_original
    }
    fn uniform_scale_original_mut(&mut self) -> &mut bool {
        &mut self.uniform_scale_original
    }
    fn override_rotation(&self) -> &bool {
        &self.override_rotation
    }
    fn override_rotation_mut(&mut self) -> &mut bool {
        &mut self.override_rotation
    }
}

impl ProcessorDataTrait for UpdateMimicOverridesData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateMimicOverridesData {
}

pub static UPDATEMIMICOVERRIDESDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMimicOverridesData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateMimicOverridesData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LifetimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateMimicOverridesData, lifetime_scale),
            },
            FieldInfoData {
                name: "UniqueRandom",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateMimicOverridesData, unique_random),
            },
            FieldInfoData {
                name: "SizeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateMimicOverridesData, size_scale),
            },
            FieldInfoData {
                name: "SizeYScaleMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateMimicOverridesData, size_y_scale_multiplier),
            },
            FieldInfoData {
                name: "UniformScaleOriginal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateMimicOverridesData, uniform_scale_original),
            },
            FieldInfoData {
                name: "OverrideRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UpdateMimicOverridesData, override_rotation),
            },
        ],
    }),
    array_type: Some(UPDATEMIMICOVERRIDESDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UpdateMimicOverridesData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEMIMICOVERRIDESDATA_TYPE_INFO
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


pub static UPDATEMIMICOVERRIDESDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateMimicOverridesData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateMimicOverridesData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MimicEmitterData {
    pub _glacier_base: ProcessorData,
    pub emitter_assets: Vec<Option<Arc<Mutex<dyn EmitterDocumentTrait>>>>,
    pub kill_mimics_when_deactivated: bool,
}

pub trait MimicEmitterDataTrait: ProcessorDataTrait {
    fn emitter_assets(&self) -> &Vec<Option<Arc<Mutex<dyn EmitterDocumentTrait>>>>;
    fn emitter_assets_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn EmitterDocumentTrait>>>>;
    fn kill_mimics_when_deactivated(&self) -> &bool;
    fn kill_mimics_when_deactivated_mut(&mut self) -> &mut bool;
}

impl MimicEmitterDataTrait for MimicEmitterData {
    fn emitter_assets(&self) -> &Vec<Option<Arc<Mutex<dyn EmitterDocumentTrait>>>> {
        &self.emitter_assets
    }
    fn emitter_assets_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn EmitterDocumentTrait>>>> {
        &mut self.emitter_assets
    }
    fn kill_mimics_when_deactivated(&self) -> &bool {
        &self.kill_mimics_when_deactivated
    }
    fn kill_mimics_when_deactivated_mut(&mut self) -> &mut bool {
        &mut self.kill_mimics_when_deactivated
    }
}

impl ProcessorDataTrait for MimicEmitterData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for MimicEmitterData {
}

pub static MIMICEMITTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MimicEmitterData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MimicEmitterData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EmitterAssets",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterDocument-Array",
                rust_offset: offset_of!(MimicEmitterData, emitter_assets),
            },
            FieldInfoData {
                name: "KillMimicsWhenDeactivated",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MimicEmitterData, kill_mimics_when_deactivated),
            },
        ],
    }),
    array_type: Some(MIMICEMITTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MimicEmitterData {
    fn type_info(&self) -> &'static TypeInfo {
        MIMICEMITTERDATA_TYPE_INFO
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


pub static MIMICEMITTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MimicEmitterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MimicEmitterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterData {
    pub _glacier_base: ProcessorData,
    pub emitter_assets: Vec<Option<Arc<Mutex<dyn EmitterDocumentTrait>>>>,
    pub emitter_graph_assets: Vec<Option<Arc<Mutex<dyn EmitterGraphTrait>>>>,
}

pub trait EmitterDataTrait: ProcessorDataTrait {
    fn emitter_assets(&self) -> &Vec<Option<Arc<Mutex<dyn EmitterDocumentTrait>>>>;
    fn emitter_assets_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn EmitterDocumentTrait>>>>;
    fn emitter_graph_assets(&self) -> &Vec<Option<Arc<Mutex<dyn EmitterGraphTrait>>>>;
    fn emitter_graph_assets_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn EmitterGraphTrait>>>>;
}

impl EmitterDataTrait for EmitterData {
    fn emitter_assets(&self) -> &Vec<Option<Arc<Mutex<dyn EmitterDocumentTrait>>>> {
        &self.emitter_assets
    }
    fn emitter_assets_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn EmitterDocumentTrait>>>> {
        &mut self.emitter_assets
    }
    fn emitter_graph_assets(&self) -> &Vec<Option<Arc<Mutex<dyn EmitterGraphTrait>>>> {
        &self.emitter_graph_assets
    }
    fn emitter_graph_assets_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn EmitterGraphTrait>>>> {
        &mut self.emitter_graph_assets
    }
}

impl ProcessorDataTrait for EmitterData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for EmitterData {
}

pub static EMITTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EmitterAssets",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterDocument-Array",
                rust_offset: offset_of!(EmitterData, emitter_assets),
            },
            FieldInfoData {
                name: "EmitterGraphAssets",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterGraph-Array",
                rust_offset: offset_of!(EmitterData, emitter_graph_assets),
            },
        ],
    }),
    array_type: Some(EMITTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterData {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERDATA_TYPE_INFO
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


pub static EMITTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("EmitterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TurbulanceData {
    pub _glacier_base: ProcessorData,
    pub intensity: f32,
    pub noise_type: TurbulenceNoiseType,
    pub period_space: f32,
    pub turbulence_force_as_instant_velocity: f32,
    pub octaves: i32,
    pub octave_persistence: f32,
    pub per_particle_randomness: f32,
    pub multiplier: super::core::Vec3,
}

pub trait TurbulanceDataTrait: ProcessorDataTrait {
    fn intensity(&self) -> &f32;
    fn intensity_mut(&mut self) -> &mut f32;
    fn noise_type(&self) -> &TurbulenceNoiseType;
    fn noise_type_mut(&mut self) -> &mut TurbulenceNoiseType;
    fn period_space(&self) -> &f32;
    fn period_space_mut(&mut self) -> &mut f32;
    fn turbulence_force_as_instant_velocity(&self) -> &f32;
    fn turbulence_force_as_instant_velocity_mut(&mut self) -> &mut f32;
    fn octaves(&self) -> &i32;
    fn octaves_mut(&mut self) -> &mut i32;
    fn octave_persistence(&self) -> &f32;
    fn octave_persistence_mut(&mut self) -> &mut f32;
    fn per_particle_randomness(&self) -> &f32;
    fn per_particle_randomness_mut(&mut self) -> &mut f32;
    fn multiplier(&self) -> &super::core::Vec3;
    fn multiplier_mut(&mut self) -> &mut super::core::Vec3;
}

impl TurbulanceDataTrait for TurbulanceData {
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn intensity_mut(&mut self) -> &mut f32 {
        &mut self.intensity
    }
    fn noise_type(&self) -> &TurbulenceNoiseType {
        &self.noise_type
    }
    fn noise_type_mut(&mut self) -> &mut TurbulenceNoiseType {
        &mut self.noise_type
    }
    fn period_space(&self) -> &f32 {
        &self.period_space
    }
    fn period_space_mut(&mut self) -> &mut f32 {
        &mut self.period_space
    }
    fn turbulence_force_as_instant_velocity(&self) -> &f32 {
        &self.turbulence_force_as_instant_velocity
    }
    fn turbulence_force_as_instant_velocity_mut(&mut self) -> &mut f32 {
        &mut self.turbulence_force_as_instant_velocity
    }
    fn octaves(&self) -> &i32 {
        &self.octaves
    }
    fn octaves_mut(&mut self) -> &mut i32 {
        &mut self.octaves
    }
    fn octave_persistence(&self) -> &f32 {
        &self.octave_persistence
    }
    fn octave_persistence_mut(&mut self) -> &mut f32 {
        &mut self.octave_persistence
    }
    fn per_particle_randomness(&self) -> &f32 {
        &self.per_particle_randomness
    }
    fn per_particle_randomness_mut(&mut self) -> &mut f32 {
        &mut self.per_particle_randomness
    }
    fn multiplier(&self) -> &super::core::Vec3 {
        &self.multiplier
    }
    fn multiplier_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.multiplier
    }
}

impl ProcessorDataTrait for TurbulanceData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for TurbulanceData {
}

pub static TURBULANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurbulanceData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TurbulanceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurbulanceData, intensity),
            },
            FieldInfoData {
                name: "NoiseType",
                flags: MemberInfoFlags::new(0),
                field_type: "TurbulenceNoiseType",
                rust_offset: offset_of!(TurbulanceData, noise_type),
            },
            FieldInfoData {
                name: "PeriodSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurbulanceData, period_space),
            },
            FieldInfoData {
                name: "TurbulenceForceAsInstantVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurbulanceData, turbulence_force_as_instant_velocity),
            },
            FieldInfoData {
                name: "Octaves",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TurbulanceData, octaves),
            },
            FieldInfoData {
                name: "OctavePersistence",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurbulanceData, octave_persistence),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurbulanceData, per_particle_randomness),
            },
            FieldInfoData {
                name: "Multiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TurbulanceData, multiplier),
            },
        ],
    }),
    array_type: Some(TURBULANCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TurbulanceData {
    fn type_info(&self) -> &'static TypeInfo {
        TURBULANCEDATA_TYPE_INFO
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


pub static TURBULANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurbulanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("TurbulanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TurbulenceNoiseType {
    #[default]
    TurbulenceNoiseType_Random = 0,
    TurbulenceNoiseType_Perlin = 1,
    TurbulenceNoiseType_PerlinSimplex = 2,
    TurbulenceNoiseType_PerlinCurl = 3,
}

pub static TURBULENCENOISETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurbulenceNoiseType",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(TURBULENCENOISETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TurbulenceNoiseType {
    fn type_info(&self) -> &'static TypeInfo {
        TURBULENCENOISETYPE_TYPE_INFO
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


pub static TURBULENCENOISETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurbulenceNoiseType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("TurbulenceNoiseType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AirResistanceData {
    pub _glacier_base: ProcessorData,
    pub drag_factor: f32,
}

pub trait AirResistanceDataTrait: ProcessorDataTrait {
    fn drag_factor(&self) -> &f32;
    fn drag_factor_mut(&mut self) -> &mut f32;
}

impl AirResistanceDataTrait for AirResistanceData {
    fn drag_factor(&self) -> &f32 {
        &self.drag_factor
    }
    fn drag_factor_mut(&mut self) -> &mut f32 {
        &mut self.drag_factor
    }
}

impl ProcessorDataTrait for AirResistanceData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for AirResistanceData {
}

pub static AIRRESISTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AirResistanceData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AirResistanceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DragFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AirResistanceData, drag_factor),
            },
        ],
    }),
    array_type: Some(AIRRESISTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AirResistanceData {
    fn type_info(&self) -> &'static TypeInfo {
        AIRRESISTANCEDATA_TYPE_INFO
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


pub static AIRRESISTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AirResistanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("AirResistanceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WorldForcesData {
    pub _glacier_base: ProcessorData,
    pub forces_multiplier: f32,
    pub per_particle_randomness: f32,
}

pub trait WorldForcesDataTrait: ProcessorDataTrait {
    fn forces_multiplier(&self) -> &f32;
    fn forces_multiplier_mut(&mut self) -> &mut f32;
    fn per_particle_randomness(&self) -> &f32;
    fn per_particle_randomness_mut(&mut self) -> &mut f32;
}

impl WorldForcesDataTrait for WorldForcesData {
    fn forces_multiplier(&self) -> &f32 {
        &self.forces_multiplier
    }
    fn forces_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.forces_multiplier
    }
    fn per_particle_randomness(&self) -> &f32 {
        &self.per_particle_randomness
    }
    fn per_particle_randomness_mut(&mut self) -> &mut f32 {
        &mut self.per_particle_randomness
    }
}

impl ProcessorDataTrait for WorldForcesData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for WorldForcesData {
}

pub static WORLDFORCESDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldForcesData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WorldForcesData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ForcesMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldForcesData, forces_multiplier),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldForcesData, per_particle_randomness),
            },
        ],
    }),
    array_type: Some(WORLDFORCESDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldForcesData {
    fn type_info(&self) -> &'static TypeInfo {
        WORLDFORCESDATA_TYPE_INFO
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


pub static WORLDFORCESDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldForcesData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("WorldForcesData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WorldWindData {
    pub _glacier_base: ProcessorData,
    pub wind_multiplier: f32,
    pub per_particle_randomness: f32,
}

pub trait WorldWindDataTrait: ProcessorDataTrait {
    fn wind_multiplier(&self) -> &f32;
    fn wind_multiplier_mut(&mut self) -> &mut f32;
    fn per_particle_randomness(&self) -> &f32;
    fn per_particle_randomness_mut(&mut self) -> &mut f32;
}

impl WorldWindDataTrait for WorldWindData {
    fn wind_multiplier(&self) -> &f32 {
        &self.wind_multiplier
    }
    fn wind_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.wind_multiplier
    }
    fn per_particle_randomness(&self) -> &f32 {
        &self.per_particle_randomness
    }
    fn per_particle_randomness_mut(&mut self) -> &mut f32 {
        &mut self.per_particle_randomness
    }
}

impl ProcessorDataTrait for WorldWindData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for WorldWindData {
}

pub static WORLDWINDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldWindData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WorldWindData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "WindMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldWindData, wind_multiplier),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldWindData, per_particle_randomness),
            },
        ],
    }),
    array_type: Some(WORLDWINDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldWindData {
    fn type_info(&self) -> &'static TypeInfo {
        WORLDWINDDATA_TYPE_INFO
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


pub static WORLDWINDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldWindData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("WorldWindData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalForceData {
    pub _glacier_base: ProcessorData,
    pub local_force: super::core::Vec3,
    pub emitter_local_space_force: bool,
    pub per_particle_randomness: f32,
}

pub trait LocalForceDataTrait: ProcessorDataTrait {
    fn local_force(&self) -> &super::core::Vec3;
    fn local_force_mut(&mut self) -> &mut super::core::Vec3;
    fn emitter_local_space_force(&self) -> &bool;
    fn emitter_local_space_force_mut(&mut self) -> &mut bool;
    fn per_particle_randomness(&self) -> &f32;
    fn per_particle_randomness_mut(&mut self) -> &mut f32;
}

impl LocalForceDataTrait for LocalForceData {
    fn local_force(&self) -> &super::core::Vec3 {
        &self.local_force
    }
    fn local_force_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.local_force
    }
    fn emitter_local_space_force(&self) -> &bool {
        &self.emitter_local_space_force
    }
    fn emitter_local_space_force_mut(&mut self) -> &mut bool {
        &mut self.emitter_local_space_force
    }
    fn per_particle_randomness(&self) -> &f32 {
        &self.per_particle_randomness
    }
    fn per_particle_randomness_mut(&mut self) -> &mut f32 {
        &mut self.per_particle_randomness
    }
}

impl ProcessorDataTrait for LocalForceData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for LocalForceData {
}

pub static LOCALFORCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalForceData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalForceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LocalForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LocalForceData, local_force),
            },
            FieldInfoData {
                name: "EmitterLocalSpaceForce",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LocalForceData, emitter_local_space_force),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalForceData, per_particle_randomness),
            },
        ],
    }),
    array_type: Some(LOCALFORCEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalForceData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALFORCEDATA_TYPE_INFO
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


pub static LOCALFORCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalForceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("LocalForceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GravityData {
    pub _glacier_base: ProcessorData,
    pub gravity: f32,
    pub per_particle_randomness: f32,
}

pub trait GravityDataTrait: ProcessorDataTrait {
    fn gravity(&self) -> &f32;
    fn gravity_mut(&mut self) -> &mut f32;
    fn per_particle_randomness(&self) -> &f32;
    fn per_particle_randomness_mut(&mut self) -> &mut f32;
}

impl GravityDataTrait for GravityData {
    fn gravity(&self) -> &f32 {
        &self.gravity
    }
    fn gravity_mut(&mut self) -> &mut f32 {
        &mut self.gravity
    }
    fn per_particle_randomness(&self) -> &f32 {
        &self.per_particle_randomness
    }
    fn per_particle_randomness_mut(&mut self) -> &mut f32 {
        &mut self.per_particle_randomness
    }
}

impl ProcessorDataTrait for GravityData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for GravityData {
}

pub static GRAVITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GravityData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GravityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Gravity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GravityData, gravity),
            },
            FieldInfoData {
                name: "PerParticleRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GravityData, per_particle_randomness),
            },
        ],
    }),
    array_type: Some(GRAVITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GravityData {
    fn type_info(&self) -> &'static TypeInfo {
        GRAVITYDATA_TYPE_INFO
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


pub static GRAVITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GravityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("GravityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UpdateAgeData {
    pub _glacier_base: ProcessorData,
    pub lifetime: f32,
    pub random_lifetime_scale: f32,
    pub max_factor: f32,
    pub death_effect: Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>,
    pub throttle: f32,
    pub throttle_far_distance: f32,
    pub throttle_envelope: super::core::Vec4,
}

pub trait UpdateAgeDataTrait: ProcessorDataTrait {
    fn lifetime(&self) -> &f32;
    fn lifetime_mut(&mut self) -> &mut f32;
    fn random_lifetime_scale(&self) -> &f32;
    fn random_lifetime_scale_mut(&mut self) -> &mut f32;
    fn max_factor(&self) -> &f32;
    fn max_factor_mut(&mut self) -> &mut f32;
    fn death_effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>;
    fn death_effect_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>;
    fn throttle(&self) -> &f32;
    fn throttle_mut(&mut self) -> &mut f32;
    fn throttle_far_distance(&self) -> &f32;
    fn throttle_far_distance_mut(&mut self) -> &mut f32;
    fn throttle_envelope(&self) -> &super::core::Vec4;
    fn throttle_envelope_mut(&mut self) -> &mut super::core::Vec4;
}

impl UpdateAgeDataTrait for UpdateAgeData {
    fn lifetime(&self) -> &f32 {
        &self.lifetime
    }
    fn lifetime_mut(&mut self) -> &mut f32 {
        &mut self.lifetime
    }
    fn random_lifetime_scale(&self) -> &f32 {
        &self.random_lifetime_scale
    }
    fn random_lifetime_scale_mut(&mut self) -> &mut f32 {
        &mut self.random_lifetime_scale
    }
    fn max_factor(&self) -> &f32 {
        &self.max_factor
    }
    fn max_factor_mut(&mut self) -> &mut f32 {
        &mut self.max_factor
    }
    fn death_effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>> {
        &self.death_effect
    }
    fn death_effect_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>> {
        &mut self.death_effect
    }
    fn throttle(&self) -> &f32 {
        &self.throttle
    }
    fn throttle_mut(&mut self) -> &mut f32 {
        &mut self.throttle
    }
    fn throttle_far_distance(&self) -> &f32 {
        &self.throttle_far_distance
    }
    fn throttle_far_distance_mut(&mut self) -> &mut f32 {
        &mut self.throttle_far_distance
    }
    fn throttle_envelope(&self) -> &super::core::Vec4 {
        &self.throttle_envelope
    }
    fn throttle_envelope_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.throttle_envelope
    }
}

impl ProcessorDataTrait for UpdateAgeData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for UpdateAgeData {
}

pub static UPDATEAGEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAgeData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UpdateAgeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateAgeData, lifetime),
            },
            FieldInfoData {
                name: "RandomLifetimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateAgeData, random_lifetime_scale),
            },
            FieldInfoData {
                name: "MaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateAgeData, max_factor),
            },
            FieldInfoData {
                name: "DeathEffect",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(UpdateAgeData, death_effect),
            },
            FieldInfoData {
                name: "Throttle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateAgeData, throttle),
            },
            FieldInfoData {
                name: "ThrottleFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UpdateAgeData, throttle_far_distance),
            },
            FieldInfoData {
                name: "ThrottleEnvelope",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UpdateAgeData, throttle_envelope),
            },
        ],
    }),
    array_type: Some(UPDATEAGEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UpdateAgeData {
    fn type_info(&self) -> &'static TypeInfo {
        UPDATEAGEDATA_TYPE_INFO
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


pub static UPDATEAGEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdateAgeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("UpdateAgeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnColorRandomData {
    pub _glacier_base: ProcessorData,
    pub color0: super::core::Vec3,
    pub color1: super::core::Vec3,
}

pub trait SpawnColorRandomDataTrait: ProcessorDataTrait {
    fn color0(&self) -> &super::core::Vec3;
    fn color0_mut(&mut self) -> &mut super::core::Vec3;
    fn color1(&self) -> &super::core::Vec3;
    fn color1_mut(&mut self) -> &mut super::core::Vec3;
}

impl SpawnColorRandomDataTrait for SpawnColorRandomData {
    fn color0(&self) -> &super::core::Vec3 {
        &self.color0
    }
    fn color0_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color0
    }
    fn color1(&self) -> &super::core::Vec3 {
        &self.color1
    }
    fn color1_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color1
    }
}

impl ProcessorDataTrait for SpawnColorRandomData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnColorRandomData {
}

pub static SPAWNCOLORRANDOMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnColorRandomData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnColorRandomData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Color0",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SpawnColorRandomData, color0),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SpawnColorRandomData, color1),
            },
        ],
    }),
    array_type: Some(SPAWNCOLORRANDOMDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SpawnColorRandomData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNCOLORRANDOMDATA_TYPE_INFO
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


pub static SPAWNCOLORRANDOMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnColorRandomData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnColorRandomData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnRotationSpeedData {
    pub _glacier_base: ProcessorData,
    pub rotation_speed: f32,
}

pub trait SpawnRotationSpeedDataTrait: ProcessorDataTrait {
    fn rotation_speed(&self) -> &f32;
    fn rotation_speed_mut(&mut self) -> &mut f32;
}

impl SpawnRotationSpeedDataTrait for SpawnRotationSpeedData {
    fn rotation_speed(&self) -> &f32 {
        &self.rotation_speed
    }
    fn rotation_speed_mut(&mut self) -> &mut f32 {
        &mut self.rotation_speed
    }
}

impl ProcessorDataTrait for SpawnRotationSpeedData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnRotationSpeedData {
}

pub static SPAWNROTATIONSPEEDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRotationSpeedData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnRotationSpeedData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RotationSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnRotationSpeedData, rotation_speed),
            },
        ],
    }),
    array_type: Some(SPAWNROTATIONSPEEDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnRotationSpeedData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNROTATIONSPEEDDATA_TYPE_INFO
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


pub static SPAWNROTATIONSPEEDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRotationSpeedData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnRotationSpeedData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnOrientationData {
    pub _glacier_base: ProcessorData,
}

pub trait SpawnOrientationDataTrait: ProcessorDataTrait {
}

impl SpawnOrientationDataTrait for SpawnOrientationData {
}

impl ProcessorDataTrait for SpawnOrientationData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnOrientationData {
}

pub static SPAWNORIENTATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnOrientationData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnOrientationData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SPAWNORIENTATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnOrientationData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNORIENTATIONDATA_TYPE_INFO
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


pub static SPAWNORIENTATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnOrientationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnOrientationData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnRotationData {
    pub _glacier_base: ProcessorData,
    pub rotation: f32,
}

pub trait SpawnRotationDataTrait: ProcessorDataTrait {
    fn rotation(&self) -> &f32;
    fn rotation_mut(&mut self) -> &mut f32;
}

impl SpawnRotationDataTrait for SpawnRotationData {
    fn rotation(&self) -> &f32 {
        &self.rotation
    }
    fn rotation_mut(&mut self) -> &mut f32 {
        &mut self.rotation
    }
}

impl ProcessorDataTrait for SpawnRotationData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnRotationData {
}

pub static SPAWNROTATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRotationData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnRotationData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnRotationData, rotation),
            },
        ],
    }),
    array_type: Some(SPAWNROTATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnRotationData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNROTATIONDATA_TYPE_INFO
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


pub static SPAWNROTATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRotationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnRotationData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnAnimationFrameData {
    pub _glacier_base: ProcessorData,
    pub animation_frame: u32,
}

pub trait SpawnAnimationFrameDataTrait: ProcessorDataTrait {
    fn animation_frame(&self) -> &u32;
    fn animation_frame_mut(&mut self) -> &mut u32;
}

impl SpawnAnimationFrameDataTrait for SpawnAnimationFrameData {
    fn animation_frame(&self) -> &u32 {
        &self.animation_frame
    }
    fn animation_frame_mut(&mut self) -> &mut u32 {
        &mut self.animation_frame
    }
}

impl ProcessorDataTrait for SpawnAnimationFrameData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnAnimationFrameData {
}

pub static SPAWNANIMATIONFRAMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnAnimationFrameData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnAnimationFrameData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AnimationFrame",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SpawnAnimationFrameData, animation_frame),
            },
        ],
    }),
    array_type: Some(SPAWNANIMATIONFRAMEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnAnimationFrameData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNANIMATIONFRAMEDATA_TYPE_INFO
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


pub static SPAWNANIMATIONFRAMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnAnimationFrameData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnAnimationFrameData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnAnimationData {
    pub _glacier_base: ProcessorData,
    pub animation_speed: f32,
    pub based_on_lifetime: bool,
}

pub trait SpawnAnimationDataTrait: ProcessorDataTrait {
    fn animation_speed(&self) -> &f32;
    fn animation_speed_mut(&mut self) -> &mut f32;
    fn based_on_lifetime(&self) -> &bool;
    fn based_on_lifetime_mut(&mut self) -> &mut bool;
}

impl SpawnAnimationDataTrait for SpawnAnimationData {
    fn animation_speed(&self) -> &f32 {
        &self.animation_speed
    }
    fn animation_speed_mut(&mut self) -> &mut f32 {
        &mut self.animation_speed
    }
    fn based_on_lifetime(&self) -> &bool {
        &self.based_on_lifetime
    }
    fn based_on_lifetime_mut(&mut self) -> &mut bool {
        &mut self.based_on_lifetime
    }
}

impl ProcessorDataTrait for SpawnAnimationData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnAnimationData {
}

pub static SPAWNANIMATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnAnimationData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnAnimationData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AnimationSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnAnimationData, animation_speed),
            },
            FieldInfoData {
                name: "BasedOnLifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SpawnAnimationData, based_on_lifetime),
            },
        ],
    }),
    array_type: Some(SPAWNANIMATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnAnimationData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNANIMATIONDATA_TYPE_INFO
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


pub static SPAWNANIMATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnAnimationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnAnimationData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnPositionData {
    pub _glacier_base: ProcessorData,
    pub apply_screen_aspect_ratio: bool,
}

pub trait SpawnPositionDataTrait: ProcessorDataTrait {
    fn apply_screen_aspect_ratio(&self) -> &bool;
    fn apply_screen_aspect_ratio_mut(&mut self) -> &mut bool;
}

impl SpawnPositionDataTrait for SpawnPositionData {
    fn apply_screen_aspect_ratio(&self) -> &bool {
        &self.apply_screen_aspect_ratio
    }
    fn apply_screen_aspect_ratio_mut(&mut self) -> &mut bool {
        &mut self.apply_screen_aspect_ratio
    }
}

impl ProcessorDataTrait for SpawnPositionData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnPositionData {
}

pub static SPAWNPOSITIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnPositionData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnPositionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ApplyScreenAspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SpawnPositionData, apply_screen_aspect_ratio),
            },
        ],
    }),
    array_type: Some(SPAWNPOSITIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnPositionData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNPOSITIONDATA_TYPE_INFO
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


pub static SPAWNPOSITIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnPositionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnPositionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnSizeData {
    pub _glacier_base: ProcessorData,
    pub size: f32,
}

pub trait SpawnSizeDataTrait: ProcessorDataTrait {
    fn size(&self) -> &f32;
    fn size_mut(&mut self) -> &mut f32;
}

impl SpawnSizeDataTrait for SpawnSizeData {
    fn size(&self) -> &f32 {
        &self.size
    }
    fn size_mut(&mut self) -> &mut f32 {
        &mut self.size
    }
}

impl ProcessorDataTrait for SpawnSizeData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnSizeData {
}

pub static SPAWNSIZEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnSizeData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnSizeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnSizeData, size),
            },
        ],
    }),
    array_type: Some(SPAWNSIZEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnSizeData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNSIZEDATA_TYPE_INFO
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


pub static SPAWNSIZEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnSizeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnSizeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnSpeedData {
    pub _glacier_base: ProcessorData,
    pub speed: f32,
}

pub trait SpawnSpeedDataTrait: ProcessorDataTrait {
    fn speed(&self) -> &f32;
    fn speed_mut(&mut self) -> &mut f32;
}

impl SpawnSpeedDataTrait for SpawnSpeedData {
    fn speed(&self) -> &f32 {
        &self.speed
    }
    fn speed_mut(&mut self) -> &mut f32 {
        &mut self.speed
    }
}

impl ProcessorDataTrait for SpawnSpeedData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnSpeedData {
}

pub static SPAWNSPEEDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnSpeedData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnSpeedData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Speed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnSpeedData, speed),
            },
        ],
    }),
    array_type: Some(SPAWNSPEEDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnSpeedData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNSPEEDDATA_TYPE_INFO
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


pub static SPAWNSPEEDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnSpeedData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnSpeedData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnDirectionData {
    pub _glacier_base: ProcessorData,
    pub inherit_speed_and_direction_from_emitter: bool,
    pub direction_from_emitter_origin: f32,
    pub inherit_speed_amount: f32,
    pub inherit_speed_smoothing_factor: f32,
    pub inherit_speed_randomness: f32,
}

pub trait SpawnDirectionDataTrait: ProcessorDataTrait {
    fn inherit_speed_and_direction_from_emitter(&self) -> &bool;
    fn inherit_speed_and_direction_from_emitter_mut(&mut self) -> &mut bool;
    fn direction_from_emitter_origin(&self) -> &f32;
    fn direction_from_emitter_origin_mut(&mut self) -> &mut f32;
    fn inherit_speed_amount(&self) -> &f32;
    fn inherit_speed_amount_mut(&mut self) -> &mut f32;
    fn inherit_speed_smoothing_factor(&self) -> &f32;
    fn inherit_speed_smoothing_factor_mut(&mut self) -> &mut f32;
    fn inherit_speed_randomness(&self) -> &f32;
    fn inherit_speed_randomness_mut(&mut self) -> &mut f32;
}

impl SpawnDirectionDataTrait for SpawnDirectionData {
    fn inherit_speed_and_direction_from_emitter(&self) -> &bool {
        &self.inherit_speed_and_direction_from_emitter
    }
    fn inherit_speed_and_direction_from_emitter_mut(&mut self) -> &mut bool {
        &mut self.inherit_speed_and_direction_from_emitter
    }
    fn direction_from_emitter_origin(&self) -> &f32 {
        &self.direction_from_emitter_origin
    }
    fn direction_from_emitter_origin_mut(&mut self) -> &mut f32 {
        &mut self.direction_from_emitter_origin
    }
    fn inherit_speed_amount(&self) -> &f32 {
        &self.inherit_speed_amount
    }
    fn inherit_speed_amount_mut(&mut self) -> &mut f32 {
        &mut self.inherit_speed_amount
    }
    fn inherit_speed_smoothing_factor(&self) -> &f32 {
        &self.inherit_speed_smoothing_factor
    }
    fn inherit_speed_smoothing_factor_mut(&mut self) -> &mut f32 {
        &mut self.inherit_speed_smoothing_factor
    }
    fn inherit_speed_randomness(&self) -> &f32 {
        &self.inherit_speed_randomness
    }
    fn inherit_speed_randomness_mut(&mut self) -> &mut f32 {
        &mut self.inherit_speed_randomness
    }
}

impl ProcessorDataTrait for SpawnDirectionData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnDirectionData {
}

pub static SPAWNDIRECTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnDirectionData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnDirectionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InheritSpeedAndDirectionFromEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SpawnDirectionData, inherit_speed_and_direction_from_emitter),
            },
            FieldInfoData {
                name: "DirectionFromEmitterOrigin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnDirectionData, direction_from_emitter_origin),
            },
            FieldInfoData {
                name: "InheritSpeedAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnDirectionData, inherit_speed_amount),
            },
            FieldInfoData {
                name: "InheritSpeedSmoothingFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnDirectionData, inherit_speed_smoothing_factor),
            },
            FieldInfoData {
                name: "InheritSpeedRandomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnDirectionData, inherit_speed_randomness),
            },
        ],
    }),
    array_type: Some(SPAWNDIRECTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnDirectionData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNDIRECTIONDATA_TYPE_INFO
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


pub static SPAWNDIRECTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnDirectionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnDirectionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnPointCloudData {
    pub _glacier_base: ProcessorData,
}

pub trait SpawnPointCloudDataTrait: ProcessorDataTrait {
}

impl SpawnPointCloudDataTrait for SpawnPointCloudData {
}

impl ProcessorDataTrait for SpawnPointCloudData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnPointCloudData {
}

pub static SPAWNPOINTCLOUDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnPointCloudData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnPointCloudData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SPAWNPOINTCLOUDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnPointCloudData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNPOINTCLOUDDATA_TYPE_INFO
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


pub static SPAWNPOINTCLOUDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnPointCloudData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnPointCloudData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PreRollData {
    pub _glacier_base: ProcessorData,
    pub pre_roll: f32,
    pub updates_per_second: f32,
    pub skip_simulate_post_pre_roll: bool,
}

pub trait PreRollDataTrait: ProcessorDataTrait {
    fn pre_roll(&self) -> &f32;
    fn pre_roll_mut(&mut self) -> &mut f32;
    fn updates_per_second(&self) -> &f32;
    fn updates_per_second_mut(&mut self) -> &mut f32;
    fn skip_simulate_post_pre_roll(&self) -> &bool;
    fn skip_simulate_post_pre_roll_mut(&mut self) -> &mut bool;
}

impl PreRollDataTrait for PreRollData {
    fn pre_roll(&self) -> &f32 {
        &self.pre_roll
    }
    fn pre_roll_mut(&mut self) -> &mut f32 {
        &mut self.pre_roll
    }
    fn updates_per_second(&self) -> &f32 {
        &self.updates_per_second
    }
    fn updates_per_second_mut(&mut self) -> &mut f32 {
        &mut self.updates_per_second
    }
    fn skip_simulate_post_pre_roll(&self) -> &bool {
        &self.skip_simulate_post_pre_roll
    }
    fn skip_simulate_post_pre_roll_mut(&mut self) -> &mut bool {
        &mut self.skip_simulate_post_pre_roll
    }
}

impl ProcessorDataTrait for PreRollData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for PreRollData {
}

pub static PREROLLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreRollData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PreRollData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PreRoll",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PreRollData, pre_roll),
            },
            FieldInfoData {
                name: "UpdatesPerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PreRollData, updates_per_second),
            },
            FieldInfoData {
                name: "SkipSimulatePostPreRoll",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PreRollData, skip_simulate_post_pre_roll),
            },
        ],
    }),
    array_type: Some(PREROLLDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PreRollData {
    fn type_info(&self) -> &'static TypeInfo {
        PREROLLDATA_TYPE_INFO
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


pub static PREROLLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PreRollData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PreRollData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnRibbonRateData {
    pub _glacier_base: ProcessorData,
    pub spawn_rate: f32,
    pub distribute_over_distance: bool,
    pub angle_deviation: f32,
}

pub trait SpawnRibbonRateDataTrait: ProcessorDataTrait {
    fn spawn_rate(&self) -> &f32;
    fn spawn_rate_mut(&mut self) -> &mut f32;
    fn distribute_over_distance(&self) -> &bool;
    fn distribute_over_distance_mut(&mut self) -> &mut bool;
    fn angle_deviation(&self) -> &f32;
    fn angle_deviation_mut(&mut self) -> &mut f32;
}

impl SpawnRibbonRateDataTrait for SpawnRibbonRateData {
    fn spawn_rate(&self) -> &f32 {
        &self.spawn_rate
    }
    fn spawn_rate_mut(&mut self) -> &mut f32 {
        &mut self.spawn_rate
    }
    fn distribute_over_distance(&self) -> &bool {
        &self.distribute_over_distance
    }
    fn distribute_over_distance_mut(&mut self) -> &mut bool {
        &mut self.distribute_over_distance
    }
    fn angle_deviation(&self) -> &f32 {
        &self.angle_deviation
    }
    fn angle_deviation_mut(&mut self) -> &mut f32 {
        &mut self.angle_deviation
    }
}

impl ProcessorDataTrait for SpawnRibbonRateData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnRibbonRateData {
}

pub static SPAWNRIBBONRATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRibbonRateData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnRibbonRateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnRibbonRateData, spawn_rate),
            },
            FieldInfoData {
                name: "DistributeOverDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SpawnRibbonRateData, distribute_over_distance),
            },
            FieldInfoData {
                name: "AngleDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnRibbonRateData, angle_deviation),
            },
        ],
    }),
    array_type: Some(SPAWNRIBBONRATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnRibbonRateData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNRIBBONRATEDATA_TYPE_INFO
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


pub static SPAWNRIBBONRATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRibbonRateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnRibbonRateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpawnRateData {
    pub _glacier_base: ProcessorData,
    pub spawn_rate: f32,
    pub distribute_over_time: bool,
    pub distribute_over_distance: bool,
}

pub trait SpawnRateDataTrait: ProcessorDataTrait {
    fn spawn_rate(&self) -> &f32;
    fn spawn_rate_mut(&mut self) -> &mut f32;
    fn distribute_over_time(&self) -> &bool;
    fn distribute_over_time_mut(&mut self) -> &mut bool;
    fn distribute_over_distance(&self) -> &bool;
    fn distribute_over_distance_mut(&mut self) -> &mut bool;
}

impl SpawnRateDataTrait for SpawnRateData {
    fn spawn_rate(&self) -> &f32 {
        &self.spawn_rate
    }
    fn spawn_rate_mut(&mut self) -> &mut f32 {
        &mut self.spawn_rate
    }
    fn distribute_over_time(&self) -> &bool {
        &self.distribute_over_time
    }
    fn distribute_over_time_mut(&mut self) -> &mut bool {
        &mut self.distribute_over_time
    }
    fn distribute_over_distance(&self) -> &bool {
        &self.distribute_over_distance
    }
    fn distribute_over_distance_mut(&mut self) -> &mut bool {
        &mut self.distribute_over_distance
    }
}

impl ProcessorDataTrait for SpawnRateData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SpawnRateData {
}

pub static SPAWNRATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRateData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpawnRateData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SpawnRateData, spawn_rate),
            },
            FieldInfoData {
                name: "DistributeOverTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SpawnRateData, distribute_over_time),
            },
            FieldInfoData {
                name: "DistributeOverDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SpawnRateData, distribute_over_distance),
            },
        ],
    }),
    array_type: Some(SPAWNRATEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpawnRateData {
    fn type_info(&self) -> &'static TypeInfo {
        SPAWNRATEDATA_TYPE_INFO
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


pub static SPAWNRATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnRateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SpawnRateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CustomShaderData {
    pub _glacier_base: ProcessorData,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub animation_frame_count: f32,
    pub animation_frame_column_count: f32,
    pub emitter_draw_order: EmitterDrawOrder,
    pub particle_sorting: ParticleSorting,
}

pub trait CustomShaderDataTrait: ProcessorDataTrait {
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct;
    fn animation_frame_count(&self) -> &f32;
    fn animation_frame_count_mut(&mut self) -> &mut f32;
    fn animation_frame_column_count(&self) -> &f32;
    fn animation_frame_column_count_mut(&mut self) -> &mut f32;
    fn emitter_draw_order(&self) -> &EmitterDrawOrder;
    fn emitter_draw_order_mut(&mut self) -> &mut EmitterDrawOrder;
    fn particle_sorting(&self) -> &ParticleSorting;
    fn particle_sorting_mut(&mut self) -> &mut ParticleSorting;
}

impl CustomShaderDataTrait for CustomShaderData {
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct {
        &mut self.shader
    }
    fn animation_frame_count(&self) -> &f32 {
        &self.animation_frame_count
    }
    fn animation_frame_count_mut(&mut self) -> &mut f32 {
        &mut self.animation_frame_count
    }
    fn animation_frame_column_count(&self) -> &f32 {
        &self.animation_frame_column_count
    }
    fn animation_frame_column_count_mut(&mut self) -> &mut f32 {
        &mut self.animation_frame_column_count
    }
    fn emitter_draw_order(&self) -> &EmitterDrawOrder {
        &self.emitter_draw_order
    }
    fn emitter_draw_order_mut(&mut self) -> &mut EmitterDrawOrder {
        &mut self.emitter_draw_order
    }
    fn particle_sorting(&self) -> &ParticleSorting {
        &self.particle_sorting
    }
    fn particle_sorting_mut(&mut self) -> &mut ParticleSorting {
        &mut self.particle_sorting
    }
}

impl ProcessorDataTrait for CustomShaderData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for CustomShaderData {
}

pub static CUSTOMSHADERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomShaderData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CustomShaderData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(CustomShaderData, shader),
            },
            FieldInfoData {
                name: "AnimationFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CustomShaderData, animation_frame_count),
            },
            FieldInfoData {
                name: "AnimationFrameColumnCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CustomShaderData, animation_frame_column_count),
            },
            FieldInfoData {
                name: "EmitterDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterDrawOrder",
                rust_offset: offset_of!(CustomShaderData, emitter_draw_order),
            },
            FieldInfoData {
                name: "ParticleSorting",
                flags: MemberInfoFlags::new(0),
                field_type: "ParticleSorting",
                rust_offset: offset_of!(CustomShaderData, particle_sorting),
            },
        ],
    }),
    array_type: Some(CUSTOMSHADERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomShaderData {
    fn type_info(&self) -> &'static TypeInfo {
        CUSTOMSHADERDATA_TYPE_INFO
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


pub static CUSTOMSHADERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomShaderData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("CustomShaderData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BaseEmitterData {
    pub _glacier_base: ProcessorData,
    pub emitter_asset: Option<Arc<Mutex<dyn EmitterDocumentTrait>>>,
}

pub trait BaseEmitterDataTrait: ProcessorDataTrait {
    fn emitter_asset(&self) -> &Option<Arc<Mutex<dyn EmitterDocumentTrait>>>;
    fn emitter_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterDocumentTrait>>>;
}

impl BaseEmitterDataTrait for BaseEmitterData {
    fn emitter_asset(&self) -> &Option<Arc<Mutex<dyn EmitterDocumentTrait>>> {
        &self.emitter_asset
    }
    fn emitter_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EmitterDocumentTrait>>> {
        &mut self.emitter_asset
    }
}

impl ProcessorDataTrait for BaseEmitterData {
    fn pre(&self) -> &Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre()
    }
    fn pre_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EvaluatorDataTrait>>> {
        self._glacier_base.pre_mut()
    }
    fn next_processor(&self) -> &Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor()
    }
    fn next_processor_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProcessorDataTrait>>> {
        self._glacier_base.next_processor_mut()
    }
    fn evaluator_input(&self) -> &EmittableField {
        self._glacier_base.evaluator_input()
    }
    fn evaluator_input_mut(&mut self) -> &mut EmittableField {
        self._glacier_base.evaluator_input_mut()
    }
    fn evaluator_input_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param()
    }
    fn evaluator_input_param_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.evaluator_input_param_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for BaseEmitterData {
}

pub static BASEEMITTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseEmitterData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCESSORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BaseEmitterData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EmitterAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "EmitterDocument",
                rust_offset: offset_of!(BaseEmitterData, emitter_asset),
            },
        ],
    }),
    array_type: Some(BASEEMITTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BaseEmitterData {
    fn type_info(&self) -> &'static TypeInfo {
        BASEEMITTERDATA_TYPE_INFO
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


pub static BASEEMITTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseEmitterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("BaseEmitterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PolynomialXYZWEvaluatorData {
    pub _glacier_base: EvaluatorData,
    pub x_coefficients: super::core::Vec4,
    pub y_coefficients: super::core::Vec4,
    pub z_coefficients: super::core::Vec4,
    pub w_coefficients: super::core::Vec4,
    pub scale_value: super::core::Vec4,
    pub min_clamp: super::core::Vec4,
    pub max_clamp: super::core::Vec4,
}

pub trait PolynomialXYZWEvaluatorDataTrait: EvaluatorDataTrait {
    fn x_coefficients(&self) -> &super::core::Vec4;
    fn x_coefficients_mut(&mut self) -> &mut super::core::Vec4;
    fn y_coefficients(&self) -> &super::core::Vec4;
    fn y_coefficients_mut(&mut self) -> &mut super::core::Vec4;
    fn z_coefficients(&self) -> &super::core::Vec4;
    fn z_coefficients_mut(&mut self) -> &mut super::core::Vec4;
    fn w_coefficients(&self) -> &super::core::Vec4;
    fn w_coefficients_mut(&mut self) -> &mut super::core::Vec4;
    fn scale_value(&self) -> &super::core::Vec4;
    fn scale_value_mut(&mut self) -> &mut super::core::Vec4;
    fn min_clamp(&self) -> &super::core::Vec4;
    fn min_clamp_mut(&mut self) -> &mut super::core::Vec4;
    fn max_clamp(&self) -> &super::core::Vec4;
    fn max_clamp_mut(&mut self) -> &mut super::core::Vec4;
}

impl PolynomialXYZWEvaluatorDataTrait for PolynomialXYZWEvaluatorData {
    fn x_coefficients(&self) -> &super::core::Vec4 {
        &self.x_coefficients
    }
    fn x_coefficients_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.x_coefficients
    }
    fn y_coefficients(&self) -> &super::core::Vec4 {
        &self.y_coefficients
    }
    fn y_coefficients_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.y_coefficients
    }
    fn z_coefficients(&self) -> &super::core::Vec4 {
        &self.z_coefficients
    }
    fn z_coefficients_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.z_coefficients
    }
    fn w_coefficients(&self) -> &super::core::Vec4 {
        &self.w_coefficients
    }
    fn w_coefficients_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.w_coefficients
    }
    fn scale_value(&self) -> &super::core::Vec4 {
        &self.scale_value
    }
    fn scale_value_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.scale_value
    }
    fn min_clamp(&self) -> &super::core::Vec4 {
        &self.min_clamp
    }
    fn min_clamp_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.min_clamp
    }
    fn max_clamp(&self) -> &super::core::Vec4 {
        &self.max_clamp
    }
    fn max_clamp_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.max_clamp
    }
}

impl EvaluatorDataTrait for PolynomialXYZWEvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for PolynomialXYZWEvaluatorData {
}

pub static POLYNOMIALXYZWEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialXYZWEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PolynomialXYZWEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "XCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, x_coefficients),
            },
            FieldInfoData {
                name: "YCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, y_coefficients),
            },
            FieldInfoData {
                name: "ZCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, z_coefficients),
            },
            FieldInfoData {
                name: "WCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, w_coefficients),
            },
            FieldInfoData {
                name: "ScaleValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, scale_value),
            },
            FieldInfoData {
                name: "MinClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, min_clamp),
            },
            FieldInfoData {
                name: "MaxClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(PolynomialXYZWEvaluatorData, max_clamp),
            },
        ],
    }),
    array_type: Some(POLYNOMIALXYZWEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PolynomialXYZWEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        POLYNOMIALXYZWEVALUATORDATA_TYPE_INFO
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


pub static POLYNOMIALXYZWEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialXYZWEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialXYZWEvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MultiColorInterpData {
    pub _glacier_base: EvaluatorData,
    pub gradient: MultiColorGradient,
}

pub trait MultiColorInterpDataTrait: EvaluatorDataTrait {
    fn gradient(&self) -> &MultiColorGradient;
    fn gradient_mut(&mut self) -> &mut MultiColorGradient;
}

impl MultiColorInterpDataTrait for MultiColorInterpData {
    fn gradient(&self) -> &MultiColorGradient {
        &self.gradient
    }
    fn gradient_mut(&mut self) -> &mut MultiColorGradient {
        &mut self.gradient
    }
}

impl EvaluatorDataTrait for MultiColorInterpData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for MultiColorInterpData {
}

pub static MULTICOLORINTERPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorInterpData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiColorInterpData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Gradient",
                flags: MemberInfoFlags::new(0),
                field_type: "MultiColorGradient",
                rust_offset: offset_of!(MultiColorInterpData, gradient),
            },
        ],
    }),
    array_type: Some(MULTICOLORINTERPDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MultiColorInterpData {
    fn type_info(&self) -> &'static TypeInfo {
        MULTICOLORINTERPDATA_TYPE_INFO
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


pub static MULTICOLORINTERPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorInterpData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MultiColorInterpData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MultiColorGradient {
    pub key_points: Vec<MultiColorGradientKeyPoint>,
}

pub trait MultiColorGradientTrait: TypeObject {
    fn key_points(&self) -> &Vec<MultiColorGradientKeyPoint>;
    fn key_points_mut(&mut self) -> &mut Vec<MultiColorGradientKeyPoint>;
}

impl MultiColorGradientTrait for MultiColorGradient {
    fn key_points(&self) -> &Vec<MultiColorGradientKeyPoint> {
        &self.key_points
    }
    fn key_points_mut(&mut self) -> &mut Vec<MultiColorGradientKeyPoint> {
        &mut self.key_points
    }
}

pub static MULTICOLORGRADIENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorGradient",
    flags: MemberInfoFlags::new(73),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiColorGradient as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "KeyPoints",
                flags: MemberInfoFlags::new(144),
                field_type: "MultiColorGradientKeyPoint-Array",
                rust_offset: offset_of!(MultiColorGradient, key_points),
            },
        ],
    }),
    array_type: Some(MULTICOLORGRADIENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MultiColorGradient {
    fn type_info(&self) -> &'static TypeInfo {
        MULTICOLORGRADIENT_TYPE_INFO
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


pub static MULTICOLORGRADIENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorGradient-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MultiColorGradient"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MultiColorGradientKeyPoint {
    pub color: super::core::Vec3,
}

pub trait MultiColorGradientKeyPointTrait: TypeObject {
    fn color(&self) -> &super::core::Vec3;
    fn color_mut(&mut self) -> &mut super::core::Vec3;
}

impl MultiColorGradientKeyPointTrait for MultiColorGradientKeyPoint {
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color
    }
}

pub static MULTICOLORGRADIENTKEYPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorGradientKeyPoint",
    flags: MemberInfoFlags::new(36937),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiColorGradientKeyPoint as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(MultiColorGradientKeyPoint, color),
            },
        ],
    }),
    array_type: Some(MULTICOLORGRADIENTKEYPOINT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MultiColorGradientKeyPoint {
    fn type_info(&self) -> &'static TypeInfo {
        MULTICOLORGRADIENTKEYPOINT_TYPE_INFO
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


pub static MULTICOLORGRADIENTKEYPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiColorGradientKeyPoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("MultiColorGradientKeyPoint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PolynomialColorInterpData {
    pub _glacier_base: EvaluatorData,
    pub color0: super::core::Vec3,
    pub color1: super::core::Vec3,
    pub coefficients: super::core::Vec4,
}

pub trait PolynomialColorInterpDataTrait: EvaluatorDataTrait {
    fn color0(&self) -> &super::core::Vec3;
    fn color0_mut(&mut self) -> &mut super::core::Vec3;
    fn color1(&self) -> &super::core::Vec3;
    fn color1_mut(&mut self) -> &mut super::core::Vec3;
    fn coefficients(&self) -> &super::core::Vec4;
    fn coefficients_mut(&mut self) -> &mut super::core::Vec4;
}

impl PolynomialColorInterpDataTrait for PolynomialColorInterpData {
    fn color0(&self) -> &super::core::Vec3 {
        &self.color0
    }
    fn color0_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color0
    }
    fn color1(&self) -> &super::core::Vec3 {
        &self.color1
    }
    fn color1_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color1
    }
    fn coefficients(&self) -> &super::core::Vec4 {
        &self.coefficients
    }
    fn coefficients_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.coefficients
    }
}

impl EvaluatorDataTrait for PolynomialColorInterpData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for PolynomialColorInterpData {
}

pub static POLYNOMIALCOLORINTERPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialColorInterpData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PolynomialColorInterpData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Color0",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PolynomialColorInterpData, color0),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PolynomialColorInterpData, color1),
            },
            FieldInfoData {
                name: "Coefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(PolynomialColorInterpData, coefficients),
            },
        ],
    }),
    array_type: Some(POLYNOMIALCOLORINTERPDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PolynomialColorInterpData {
    fn type_info(&self) -> &'static TypeInfo {
        POLYNOMIALCOLORINTERPDATA_TYPE_INFO
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


pub static POLYNOMIALCOLORINTERPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialColorInterpData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialColorInterpData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConstantEvaluatorData {
    pub _glacier_base: EvaluatorData,
    pub scale: f32,
}

pub trait ConstantEvaluatorDataTrait: EvaluatorDataTrait {
    fn scale(&self) -> &f32;
    fn scale_mut(&mut self) -> &mut f32;
}

impl ConstantEvaluatorDataTrait for ConstantEvaluatorData {
    fn scale(&self) -> &f32 {
        &self.scale
    }
    fn scale_mut(&mut self) -> &mut f32 {
        &mut self.scale
    }
}

impl EvaluatorDataTrait for ConstantEvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for ConstantEvaluatorData {
}

pub static CONSTANTEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstantEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConstantEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConstantEvaluatorData, scale),
            },
        ],
    }),
    array_type: Some(CONSTANTEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConstantEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        CONSTANTEVALUATORDATA_TYPE_INFO
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


pub static CONSTANTEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConstantEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("ConstantEvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraProximityEvaluatorData {
    pub _glacier_base: EvaluatorData,
    pub size: super::core::Vec3,
    pub offset: super::core::Vec3,
    pub forward_offset: f32,
    pub inner_radius: f32,
    pub inner_radius_direction: super::core::Vec3,
}

pub trait CameraProximityEvaluatorDataTrait: EvaluatorDataTrait {
    fn size(&self) -> &super::core::Vec3;
    fn size_mut(&mut self) -> &mut super::core::Vec3;
    fn offset(&self) -> &super::core::Vec3;
    fn offset_mut(&mut self) -> &mut super::core::Vec3;
    fn forward_offset(&self) -> &f32;
    fn forward_offset_mut(&mut self) -> &mut f32;
    fn inner_radius(&self) -> &f32;
    fn inner_radius_mut(&mut self) -> &mut f32;
    fn inner_radius_direction(&self) -> &super::core::Vec3;
    fn inner_radius_direction_mut(&mut self) -> &mut super::core::Vec3;
}

impl CameraProximityEvaluatorDataTrait for CameraProximityEvaluatorData {
    fn size(&self) -> &super::core::Vec3 {
        &self.size
    }
    fn size_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.size
    }
    fn offset(&self) -> &super::core::Vec3 {
        &self.offset
    }
    fn offset_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.offset
    }
    fn forward_offset(&self) -> &f32 {
        &self.forward_offset
    }
    fn forward_offset_mut(&mut self) -> &mut f32 {
        &mut self.forward_offset
    }
    fn inner_radius(&self) -> &f32 {
        &self.inner_radius
    }
    fn inner_radius_mut(&mut self) -> &mut f32 {
        &mut self.inner_radius
    }
    fn inner_radius_direction(&self) -> &super::core::Vec3 {
        &self.inner_radius_direction
    }
    fn inner_radius_direction_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.inner_radius_direction
    }
}

impl EvaluatorDataTrait for CameraProximityEvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for CameraProximityEvaluatorData {
}

pub static CAMERAPROXIMITYEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraProximityEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraProximityEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CameraProximityEvaluatorData, size),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CameraProximityEvaluatorData, offset),
            },
            FieldInfoData {
                name: "ForwardOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraProximityEvaluatorData, forward_offset),
            },
            FieldInfoData {
                name: "InnerRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraProximityEvaluatorData, inner_radius),
            },
            FieldInfoData {
                name: "InnerRadiusDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CameraProximityEvaluatorData, inner_radius_direction),
            },
        ],
    }),
    array_type: Some(CAMERAPROXIMITYEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CameraProximityEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERAPROXIMITYEVALUATORDATA_TYPE_INFO
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


pub static CAMERAPROXIMITYEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraProximityEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("CameraProximityEvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SuperSphereEvaluatorData {
    pub _glacier_base: EvaluatorData,
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

pub trait SuperSphereEvaluatorDataTrait: EvaluatorDataTrait {
    fn scale(&self) -> &super::core::Vec3;
    fn scale_mut(&mut self) -> &mut super::core::Vec3;
    fn pivot(&self) -> &super::core::Vec3;
    fn pivot_mut(&mut self) -> &mut super::core::Vec3;
    fn inner_radius(&self) -> &f32;
    fn inner_radius_mut(&mut self) -> &mut f32;
    fn outer_radius(&self) -> &f32;
    fn outer_radius_mut(&mut self) -> &mut f32;
    fn start_zenith_angle(&self) -> &f32;
    fn start_zenith_angle_mut(&mut self) -> &mut f32;
    fn end_zenith_angle(&self) -> &f32;
    fn end_zenith_angle_mut(&mut self) -> &mut f32;
    fn inner_radius_bound(&self) -> &f32;
    fn inner_radius_bound_mut(&mut self) -> &mut f32;
    fn start_zenith_angle_bound(&self) -> &f32;
    fn start_zenith_angle_bound_mut(&mut self) -> &mut f32;
    fn end_zenith_angle_bound(&self) -> &f32;
    fn end_zenith_angle_bound_mut(&mut self) -> &mut f32;
    fn start_azimuth_angle(&self) -> &f32;
    fn start_azimuth_angle_mut(&mut self) -> &mut f32;
    fn end_azimuth_angle(&self) -> &f32;
    fn end_azimuth_angle_mut(&mut self) -> &mut f32;
    fn distribution_along_radius(&self) -> &f32;
    fn distribution_along_radius_mut(&mut self) -> &mut f32;
    fn orient_along_z(&self) -> &bool;
    fn orient_along_z_mut(&mut self) -> &mut bool;
}

impl SuperSphereEvaluatorDataTrait for SuperSphereEvaluatorData {
    fn scale(&self) -> &super::core::Vec3 {
        &self.scale
    }
    fn scale_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.scale
    }
    fn pivot(&self) -> &super::core::Vec3 {
        &self.pivot
    }
    fn pivot_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.pivot
    }
    fn inner_radius(&self) -> &f32 {
        &self.inner_radius
    }
    fn inner_radius_mut(&mut self) -> &mut f32 {
        &mut self.inner_radius
    }
    fn outer_radius(&self) -> &f32 {
        &self.outer_radius
    }
    fn outer_radius_mut(&mut self) -> &mut f32 {
        &mut self.outer_radius
    }
    fn start_zenith_angle(&self) -> &f32 {
        &self.start_zenith_angle
    }
    fn start_zenith_angle_mut(&mut self) -> &mut f32 {
        &mut self.start_zenith_angle
    }
    fn end_zenith_angle(&self) -> &f32 {
        &self.end_zenith_angle
    }
    fn end_zenith_angle_mut(&mut self) -> &mut f32 {
        &mut self.end_zenith_angle
    }
    fn inner_radius_bound(&self) -> &f32 {
        &self.inner_radius_bound
    }
    fn inner_radius_bound_mut(&mut self) -> &mut f32 {
        &mut self.inner_radius_bound
    }
    fn start_zenith_angle_bound(&self) -> &f32 {
        &self.start_zenith_angle_bound
    }
    fn start_zenith_angle_bound_mut(&mut self) -> &mut f32 {
        &mut self.start_zenith_angle_bound
    }
    fn end_zenith_angle_bound(&self) -> &f32 {
        &self.end_zenith_angle_bound
    }
    fn end_zenith_angle_bound_mut(&mut self) -> &mut f32 {
        &mut self.end_zenith_angle_bound
    }
    fn start_azimuth_angle(&self) -> &f32 {
        &self.start_azimuth_angle
    }
    fn start_azimuth_angle_mut(&mut self) -> &mut f32 {
        &mut self.start_azimuth_angle
    }
    fn end_azimuth_angle(&self) -> &f32 {
        &self.end_azimuth_angle
    }
    fn end_azimuth_angle_mut(&mut self) -> &mut f32 {
        &mut self.end_azimuth_angle
    }
    fn distribution_along_radius(&self) -> &f32 {
        &self.distribution_along_radius
    }
    fn distribution_along_radius_mut(&mut self) -> &mut f32 {
        &mut self.distribution_along_radius
    }
    fn orient_along_z(&self) -> &bool {
        &self.orient_along_z
    }
    fn orient_along_z_mut(&mut self) -> &mut bool {
        &mut self.orient_along_z
    }
}

impl EvaluatorDataTrait for SuperSphereEvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SuperSphereEvaluatorData {
}

pub static SUPERSPHEREEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SuperSphereEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SuperSphereEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SuperSphereEvaluatorData, scale),
            },
            FieldInfoData {
                name: "Pivot",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SuperSphereEvaluatorData, pivot),
            },
            FieldInfoData {
                name: "InnerRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SuperSphereEvaluatorData, inner_radius),
            },
            FieldInfoData {
                name: "OuterRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SuperSphereEvaluatorData, outer_radius),
            },
            FieldInfoData {
                name: "StartZenithAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SuperSphereEvaluatorData, start_zenith_angle),
            },
            FieldInfoData {
                name: "EndZenithAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SuperSphereEvaluatorData, end_zenith_angle),
            },
            FieldInfoData {
                name: "InnerRadiusBound",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SuperSphereEvaluatorData, inner_radius_bound),
            },
            FieldInfoData {
                name: "StartZenithAngleBound",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SuperSphereEvaluatorData, start_zenith_angle_bound),
            },
            FieldInfoData {
                name: "EndZenithAngleBound",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SuperSphereEvaluatorData, end_zenith_angle_bound),
            },
            FieldInfoData {
                name: "StartAzimuthAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SuperSphereEvaluatorData, start_azimuth_angle),
            },
            FieldInfoData {
                name: "EndAzimuthAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SuperSphereEvaluatorData, end_azimuth_angle),
            },
            FieldInfoData {
                name: "DistributionAlongRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SuperSphereEvaluatorData, distribution_along_radius),
            },
            FieldInfoData {
                name: "OrientAlongZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SuperSphereEvaluatorData, orient_along_z),
            },
        ],
    }),
    array_type: Some(SUPERSPHEREEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SuperSphereEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        SUPERSPHEREEVALUATORDATA_TYPE_INFO
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


pub static SUPERSPHEREEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SuperSphereEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SuperSphereEvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SphereEvaluatorData {
    pub _glacier_base: EvaluatorData,
    pub scale: super::core::Vec3,
    pub radius: f32,
    pub pivot: super::core::Vec3,
}

pub trait SphereEvaluatorDataTrait: EvaluatorDataTrait {
    fn scale(&self) -> &super::core::Vec3;
    fn scale_mut(&mut self) -> &mut super::core::Vec3;
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn pivot(&self) -> &super::core::Vec3;
    fn pivot_mut(&mut self) -> &mut super::core::Vec3;
}

impl SphereEvaluatorDataTrait for SphereEvaluatorData {
    fn scale(&self) -> &super::core::Vec3 {
        &self.scale
    }
    fn scale_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.scale
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn pivot(&self) -> &super::core::Vec3 {
        &self.pivot
    }
    fn pivot_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.pivot
    }
}

impl EvaluatorDataTrait for SphereEvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SphereEvaluatorData {
}

pub static SPHEREEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SphereEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SphereEvaluatorData, scale),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereEvaluatorData, radius),
            },
            FieldInfoData {
                name: "Pivot",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SphereEvaluatorData, pivot),
            },
        ],
    }),
    array_type: Some(SPHEREEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SphereEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        SPHEREEVALUATORDATA_TYPE_INFO
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


pub static SPHEREEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SphereEvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoxEvaluatorData {
    pub _glacier_base: EvaluatorData,
    pub dimensions: super::core::Vec3,
    pub pivot: super::core::Vec3,
}

pub trait BoxEvaluatorDataTrait: EvaluatorDataTrait {
    fn dimensions(&self) -> &super::core::Vec3;
    fn dimensions_mut(&mut self) -> &mut super::core::Vec3;
    fn pivot(&self) -> &super::core::Vec3;
    fn pivot_mut(&mut self) -> &mut super::core::Vec3;
}

impl BoxEvaluatorDataTrait for BoxEvaluatorData {
    fn dimensions(&self) -> &super::core::Vec3 {
        &self.dimensions
    }
    fn dimensions_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.dimensions
    }
    fn pivot(&self) -> &super::core::Vec3 {
        &self.pivot
    }
    fn pivot_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.pivot
    }
}

impl EvaluatorDataTrait for BoxEvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for BoxEvaluatorData {
}

pub static BOXEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoxEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Dimensions",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(BoxEvaluatorData, dimensions),
            },
            FieldInfoData {
                name: "Pivot",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(BoxEvaluatorData, pivot),
            },
        ],
    }),
    array_type: Some(BOXEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoxEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        BOXEVALUATORDATA_TYPE_INFO
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


pub static BOXEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("BoxEvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RandomXYZWEvaluatorData {
    pub _glacier_base: EvaluatorData,
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

pub trait RandomXYZWEvaluatorDataTrait: EvaluatorDataTrait {
    fn random_frequency(&self) -> &RandomFrequency;
    fn random_frequency_mut(&mut self) -> &mut RandomFrequency;
    fn max_x(&self) -> &f32;
    fn max_x_mut(&mut self) -> &mut f32;
    fn min_x(&self) -> &f32;
    fn min_x_mut(&mut self) -> &mut f32;
    fn max_y(&self) -> &f32;
    fn max_y_mut(&mut self) -> &mut f32;
    fn min_y(&self) -> &f32;
    fn min_y_mut(&mut self) -> &mut f32;
    fn max_z(&self) -> &f32;
    fn max_z_mut(&mut self) -> &mut f32;
    fn min_z(&self) -> &f32;
    fn min_z_mut(&mut self) -> &mut f32;
    fn max_w(&self) -> &f32;
    fn max_w_mut(&mut self) -> &mut f32;
    fn min_w(&self) -> &f32;
    fn min_w_mut(&mut self) -> &mut f32;
    fn mirror(&self) -> &bool;
    fn mirror_mut(&mut self) -> &mut bool;
}

impl RandomXYZWEvaluatorDataTrait for RandomXYZWEvaluatorData {
    fn random_frequency(&self) -> &RandomFrequency {
        &self.random_frequency
    }
    fn random_frequency_mut(&mut self) -> &mut RandomFrequency {
        &mut self.random_frequency
    }
    fn max_x(&self) -> &f32 {
        &self.max_x
    }
    fn max_x_mut(&mut self) -> &mut f32 {
        &mut self.max_x
    }
    fn min_x(&self) -> &f32 {
        &self.min_x
    }
    fn min_x_mut(&mut self) -> &mut f32 {
        &mut self.min_x
    }
    fn max_y(&self) -> &f32 {
        &self.max_y
    }
    fn max_y_mut(&mut self) -> &mut f32 {
        &mut self.max_y
    }
    fn min_y(&self) -> &f32 {
        &self.min_y
    }
    fn min_y_mut(&mut self) -> &mut f32 {
        &mut self.min_y
    }
    fn max_z(&self) -> &f32 {
        &self.max_z
    }
    fn max_z_mut(&mut self) -> &mut f32 {
        &mut self.max_z
    }
    fn min_z(&self) -> &f32 {
        &self.min_z
    }
    fn min_z_mut(&mut self) -> &mut f32 {
        &mut self.min_z
    }
    fn max_w(&self) -> &f32 {
        &self.max_w
    }
    fn max_w_mut(&mut self) -> &mut f32 {
        &mut self.max_w
    }
    fn min_w(&self) -> &f32 {
        &self.min_w
    }
    fn min_w_mut(&mut self) -> &mut f32 {
        &mut self.min_w
    }
    fn mirror(&self) -> &bool {
        &self.mirror
    }
    fn mirror_mut(&mut self) -> &mut bool {
        &mut self.mirror
    }
}

impl EvaluatorDataTrait for RandomXYZWEvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for RandomXYZWEvaluatorData {
}

pub static RANDOMXYZWEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomXYZWEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RandomXYZWEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RandomFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "RandomFrequency",
                rust_offset: offset_of!(RandomXYZWEvaluatorData, random_frequency),
            },
            FieldInfoData {
                name: "MaxX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZWEvaluatorData, max_x),
            },
            FieldInfoData {
                name: "MinX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZWEvaluatorData, min_x),
            },
            FieldInfoData {
                name: "MaxY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZWEvaluatorData, max_y),
            },
            FieldInfoData {
                name: "MinY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZWEvaluatorData, min_y),
            },
            FieldInfoData {
                name: "MaxZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZWEvaluatorData, max_z),
            },
            FieldInfoData {
                name: "MinZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZWEvaluatorData, min_z),
            },
            FieldInfoData {
                name: "MaxW",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZWEvaluatorData, max_w),
            },
            FieldInfoData {
                name: "MinW",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZWEvaluatorData, min_w),
            },
            FieldInfoData {
                name: "Mirror",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RandomXYZWEvaluatorData, mirror),
            },
        ],
    }),
    array_type: Some(RANDOMXYZWEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomXYZWEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        RANDOMXYZWEVALUATORDATA_TYPE_INFO
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


pub static RANDOMXYZWEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomXYZWEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RandomXYZWEvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RandomXYZEvaluatorData {
    pub _glacier_base: EvaluatorData,
    pub random_frequency: RandomFrequency,
    pub max_x: f32,
    pub min_x: f32,
    pub max_y: f32,
    pub min_y: f32,
    pub max_z: f32,
    pub min_z: f32,
    pub mirror: bool,
}

pub trait RandomXYZEvaluatorDataTrait: EvaluatorDataTrait {
    fn random_frequency(&self) -> &RandomFrequency;
    fn random_frequency_mut(&mut self) -> &mut RandomFrequency;
    fn max_x(&self) -> &f32;
    fn max_x_mut(&mut self) -> &mut f32;
    fn min_x(&self) -> &f32;
    fn min_x_mut(&mut self) -> &mut f32;
    fn max_y(&self) -> &f32;
    fn max_y_mut(&mut self) -> &mut f32;
    fn min_y(&self) -> &f32;
    fn min_y_mut(&mut self) -> &mut f32;
    fn max_z(&self) -> &f32;
    fn max_z_mut(&mut self) -> &mut f32;
    fn min_z(&self) -> &f32;
    fn min_z_mut(&mut self) -> &mut f32;
    fn mirror(&self) -> &bool;
    fn mirror_mut(&mut self) -> &mut bool;
}

impl RandomXYZEvaluatorDataTrait for RandomXYZEvaluatorData {
    fn random_frequency(&self) -> &RandomFrequency {
        &self.random_frequency
    }
    fn random_frequency_mut(&mut self) -> &mut RandomFrequency {
        &mut self.random_frequency
    }
    fn max_x(&self) -> &f32 {
        &self.max_x
    }
    fn max_x_mut(&mut self) -> &mut f32 {
        &mut self.max_x
    }
    fn min_x(&self) -> &f32 {
        &self.min_x
    }
    fn min_x_mut(&mut self) -> &mut f32 {
        &mut self.min_x
    }
    fn max_y(&self) -> &f32 {
        &self.max_y
    }
    fn max_y_mut(&mut self) -> &mut f32 {
        &mut self.max_y
    }
    fn min_y(&self) -> &f32 {
        &self.min_y
    }
    fn min_y_mut(&mut self) -> &mut f32 {
        &mut self.min_y
    }
    fn max_z(&self) -> &f32 {
        &self.max_z
    }
    fn max_z_mut(&mut self) -> &mut f32 {
        &mut self.max_z
    }
    fn min_z(&self) -> &f32 {
        &self.min_z
    }
    fn min_z_mut(&mut self) -> &mut f32 {
        &mut self.min_z
    }
    fn mirror(&self) -> &bool {
        &self.mirror
    }
    fn mirror_mut(&mut self) -> &mut bool {
        &mut self.mirror
    }
}

impl EvaluatorDataTrait for RandomXYZEvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for RandomXYZEvaluatorData {
}

pub static RANDOMXYZEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomXYZEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RandomXYZEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RandomFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "RandomFrequency",
                rust_offset: offset_of!(RandomXYZEvaluatorData, random_frequency),
            },
            FieldInfoData {
                name: "MaxX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZEvaluatorData, max_x),
            },
            FieldInfoData {
                name: "MinX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZEvaluatorData, min_x),
            },
            FieldInfoData {
                name: "MaxY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZEvaluatorData, max_y),
            },
            FieldInfoData {
                name: "MinY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZEvaluatorData, min_y),
            },
            FieldInfoData {
                name: "MaxZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZEvaluatorData, max_z),
            },
            FieldInfoData {
                name: "MinZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomXYZEvaluatorData, min_z),
            },
            FieldInfoData {
                name: "Mirror",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RandomXYZEvaluatorData, mirror),
            },
        ],
    }),
    array_type: Some(RANDOMXYZEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomXYZEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        RANDOMXYZEVALUATORDATA_TYPE_INFO
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


pub static RANDOMXYZEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomXYZEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RandomXYZEvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RandomEvaluatorData {
    pub _glacier_base: EvaluatorData,
    pub random_frequency: RandomFrequency,
    pub max: f32,
    pub min: f32,
    pub mirror: bool,
}

pub trait RandomEvaluatorDataTrait: EvaluatorDataTrait {
    fn random_frequency(&self) -> &RandomFrequency;
    fn random_frequency_mut(&mut self) -> &mut RandomFrequency;
    fn max(&self) -> &f32;
    fn max_mut(&mut self) -> &mut f32;
    fn min(&self) -> &f32;
    fn min_mut(&mut self) -> &mut f32;
    fn mirror(&self) -> &bool;
    fn mirror_mut(&mut self) -> &mut bool;
}

impl RandomEvaluatorDataTrait for RandomEvaluatorData {
    fn random_frequency(&self) -> &RandomFrequency {
        &self.random_frequency
    }
    fn random_frequency_mut(&mut self) -> &mut RandomFrequency {
        &mut self.random_frequency
    }
    fn max(&self) -> &f32 {
        &self.max
    }
    fn max_mut(&mut self) -> &mut f32 {
        &mut self.max
    }
    fn min(&self) -> &f32 {
        &self.min
    }
    fn min_mut(&mut self) -> &mut f32 {
        &mut self.min
    }
    fn mirror(&self) -> &bool {
        &self.mirror
    }
    fn mirror_mut(&mut self) -> &mut bool {
        &mut self.mirror
    }
}

impl EvaluatorDataTrait for RandomEvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for RandomEvaluatorData {
}

pub static RANDOMEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RandomEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RandomFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "RandomFrequency",
                rust_offset: offset_of!(RandomEvaluatorData, random_frequency),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomEvaluatorData, max),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RandomEvaluatorData, min),
            },
            FieldInfoData {
                name: "Mirror",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RandomEvaluatorData, mirror),
            },
        ],
    }),
    array_type: Some(RANDOMEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        RANDOMEVALUATORDATA_TYPE_INFO
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


pub static RANDOMEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RandomEvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RotateVectorData {
    pub _glacier_base: EvaluatorData,
    pub angle: f32,
    pub input_affects_phi: bool,
    pub rotate_within_plane: bool,
}

pub trait RotateVectorDataTrait: EvaluatorDataTrait {
    fn angle(&self) -> &f32;
    fn angle_mut(&mut self) -> &mut f32;
    fn input_affects_phi(&self) -> &bool;
    fn input_affects_phi_mut(&mut self) -> &mut bool;
    fn rotate_within_plane(&self) -> &bool;
    fn rotate_within_plane_mut(&mut self) -> &mut bool;
}

impl RotateVectorDataTrait for RotateVectorData {
    fn angle(&self) -> &f32 {
        &self.angle
    }
    fn angle_mut(&mut self) -> &mut f32 {
        &mut self.angle
    }
    fn input_affects_phi(&self) -> &bool {
        &self.input_affects_phi
    }
    fn input_affects_phi_mut(&mut self) -> &mut bool {
        &mut self.input_affects_phi
    }
    fn rotate_within_plane(&self) -> &bool {
        &self.rotate_within_plane
    }
    fn rotate_within_plane_mut(&mut self) -> &mut bool {
        &mut self.rotate_within_plane
    }
}

impl EvaluatorDataTrait for RotateVectorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for RotateVectorData {
}

pub static ROTATEVECTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotateVectorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RotateVectorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Angle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RotateVectorData, angle),
            },
            FieldInfoData {
                name: "InputAffectsPhi",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RotateVectorData, input_affects_phi),
            },
            FieldInfoData {
                name: "RotateWithinPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RotateVectorData, rotate_within_plane),
            },
        ],
    }),
    array_type: Some(ROTATEVECTORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RotateVectorData {
    fn type_info(&self) -> &'static TypeInfo {
        ROTATEVECTORDATA_TYPE_INFO
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


pub static ROTATEVECTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotateVectorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("RotateVectorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SampleTextureData {
    pub _glacier_base: EvaluatorData,
    pub gradient_data: Vec<super::core::Vec4>,
    pub color_intensity_max: super::core::Vec3,
    pub color_intensity_min: super::core::Vec3,
    pub texture_dimensions: super::core::Vec2,
    pub texture_origin_u: f32,
    pub texture_origin_v: f32,
}

pub trait SampleTextureDataTrait: EvaluatorDataTrait {
    fn gradient_data(&self) -> &Vec<super::core::Vec4>;
    fn gradient_data_mut(&mut self) -> &mut Vec<super::core::Vec4>;
    fn color_intensity_max(&self) -> &super::core::Vec3;
    fn color_intensity_max_mut(&mut self) -> &mut super::core::Vec3;
    fn color_intensity_min(&self) -> &super::core::Vec3;
    fn color_intensity_min_mut(&mut self) -> &mut super::core::Vec3;
    fn texture_dimensions(&self) -> &super::core::Vec2;
    fn texture_dimensions_mut(&mut self) -> &mut super::core::Vec2;
    fn texture_origin_u(&self) -> &f32;
    fn texture_origin_u_mut(&mut self) -> &mut f32;
    fn texture_origin_v(&self) -> &f32;
    fn texture_origin_v_mut(&mut self) -> &mut f32;
}

impl SampleTextureDataTrait for SampleTextureData {
    fn gradient_data(&self) -> &Vec<super::core::Vec4> {
        &self.gradient_data
    }
    fn gradient_data_mut(&mut self) -> &mut Vec<super::core::Vec4> {
        &mut self.gradient_data
    }
    fn color_intensity_max(&self) -> &super::core::Vec3 {
        &self.color_intensity_max
    }
    fn color_intensity_max_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color_intensity_max
    }
    fn color_intensity_min(&self) -> &super::core::Vec3 {
        &self.color_intensity_min
    }
    fn color_intensity_min_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color_intensity_min
    }
    fn texture_dimensions(&self) -> &super::core::Vec2 {
        &self.texture_dimensions
    }
    fn texture_dimensions_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.texture_dimensions
    }
    fn texture_origin_u(&self) -> &f32 {
        &self.texture_origin_u
    }
    fn texture_origin_u_mut(&mut self) -> &mut f32 {
        &mut self.texture_origin_u
    }
    fn texture_origin_v(&self) -> &f32 {
        &self.texture_origin_v
    }
    fn texture_origin_v_mut(&mut self) -> &mut f32 {
        &mut self.texture_origin_v
    }
}

impl EvaluatorDataTrait for SampleTextureData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SampleTextureData {
}

pub static SAMPLETEXTUREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SampleTextureData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SampleTextureData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GradientData",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec4-Array",
                rust_offset: offset_of!(SampleTextureData, gradient_data),
            },
            FieldInfoData {
                name: "ColorIntensityMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SampleTextureData, color_intensity_max),
            },
            FieldInfoData {
                name: "ColorIntensityMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SampleTextureData, color_intensity_min),
            },
            FieldInfoData {
                name: "TextureDimensions",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SampleTextureData, texture_dimensions),
            },
            FieldInfoData {
                name: "TextureOriginU",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SampleTextureData, texture_origin_u),
            },
            FieldInfoData {
                name: "TextureOriginV",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SampleTextureData, texture_origin_v),
            },
        ],
    }),
    array_type: Some(SAMPLETEXTUREDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SampleTextureData {
    fn type_info(&self) -> &'static TypeInfo {
        SAMPLETEXTUREDATA_TYPE_INFO
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


pub static SAMPLETEXTUREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SampleTextureData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SampleTextureData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SplineData {
    pub _glacier_base: EvaluatorData,
    pub spline_curve: super::core::SplineCurve,
}

pub trait SplineDataTrait: EvaluatorDataTrait {
    fn spline_curve(&self) -> &super::core::SplineCurve;
    fn spline_curve_mut(&mut self) -> &mut super::core::SplineCurve;
}

impl SplineDataTrait for SplineData {
    fn spline_curve(&self) -> &super::core::SplineCurve {
        &self.spline_curve
    }
    fn spline_curve_mut(&mut self) -> &mut super::core::SplineCurve {
        &mut self.spline_curve
    }
}

impl EvaluatorDataTrait for SplineData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for SplineData {
}

pub static SPLINEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SplineData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SplineCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "SplineCurve",
                rust_offset: offset_of!(SplineData, spline_curve),
            },
        ],
    }),
    array_type: Some(SPLINEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SplineData {
    fn type_info(&self) -> &'static TypeInfo {
        SPLINEDATA_TYPE_INFO
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


pub static SPLINEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("SplineData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PolynomialOperatorData {
    pub _glacier_base: EvaluatorData,
    pub first_operand: PolynomialTempData,
    pub second_operand: PolynomialTempData,
    pub operation: PolynomialOperation,
    pub min_clamp_result: f32,
    pub max_clamp_result: f32,
}

pub trait PolynomialOperatorDataTrait: EvaluatorDataTrait {
    fn first_operand(&self) -> &PolynomialTempData;
    fn first_operand_mut(&mut self) -> &mut PolynomialTempData;
    fn second_operand(&self) -> &PolynomialTempData;
    fn second_operand_mut(&mut self) -> &mut PolynomialTempData;
    fn operation(&self) -> &PolynomialOperation;
    fn operation_mut(&mut self) -> &mut PolynomialOperation;
    fn min_clamp_result(&self) -> &f32;
    fn min_clamp_result_mut(&mut self) -> &mut f32;
    fn max_clamp_result(&self) -> &f32;
    fn max_clamp_result_mut(&mut self) -> &mut f32;
}

impl PolynomialOperatorDataTrait for PolynomialOperatorData {
    fn first_operand(&self) -> &PolynomialTempData {
        &self.first_operand
    }
    fn first_operand_mut(&mut self) -> &mut PolynomialTempData {
        &mut self.first_operand
    }
    fn second_operand(&self) -> &PolynomialTempData {
        &self.second_operand
    }
    fn second_operand_mut(&mut self) -> &mut PolynomialTempData {
        &mut self.second_operand
    }
    fn operation(&self) -> &PolynomialOperation {
        &self.operation
    }
    fn operation_mut(&mut self) -> &mut PolynomialOperation {
        &mut self.operation
    }
    fn min_clamp_result(&self) -> &f32 {
        &self.min_clamp_result
    }
    fn min_clamp_result_mut(&mut self) -> &mut f32 {
        &mut self.min_clamp_result
    }
    fn max_clamp_result(&self) -> &f32 {
        &self.max_clamp_result
    }
    fn max_clamp_result_mut(&mut self) -> &mut f32 {
        &mut self.max_clamp_result
    }
}

impl EvaluatorDataTrait for PolynomialOperatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for PolynomialOperatorData {
}

pub static POLYNOMIALOPERATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialOperatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PolynomialOperatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FirstOperand",
                flags: MemberInfoFlags::new(0),
                field_type: "PolynomialTempData",
                rust_offset: offset_of!(PolynomialOperatorData, first_operand),
            },
            FieldInfoData {
                name: "SecondOperand",
                flags: MemberInfoFlags::new(0),
                field_type: "PolynomialTempData",
                rust_offset: offset_of!(PolynomialOperatorData, second_operand),
            },
            FieldInfoData {
                name: "Operation",
                flags: MemberInfoFlags::new(0),
                field_type: "PolynomialOperation",
                rust_offset: offset_of!(PolynomialOperatorData, operation),
            },
            FieldInfoData {
                name: "MinClampResult",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PolynomialOperatorData, min_clamp_result),
            },
            FieldInfoData {
                name: "MaxClampResult",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PolynomialOperatorData, max_clamp_result),
            },
        ],
    }),
    array_type: Some(POLYNOMIALOPERATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PolynomialOperatorData {
    fn type_info(&self) -> &'static TypeInfo {
        POLYNOMIALOPERATORDATA_TYPE_INFO
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


pub static POLYNOMIALOPERATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialOperatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialOperatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PolynomialOperation {
    #[default]
    Multiplication = 0,
    Addition = 1,
    Subtraction = 2,
}

pub static POLYNOMIALOPERATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialOperation",
    flags: MemberInfoFlags::new(49429),
    module: "Emitter",
    data: TypeInfoData::Enum,
    array_type: Some(POLYNOMIALOPERATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PolynomialOperation {
    fn type_info(&self) -> &'static TypeInfo {
        POLYNOMIALOPERATION_TYPE_INFO
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


pub static POLYNOMIALOPERATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialOperation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialOperation"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PolynomialTempData {
    pub coefficients: super::core::Vec4,
    pub scale_value: f32,
    pub min_clamp: f32,
    pub max_clamp: f32,
}

pub trait PolynomialTempDataTrait: TypeObject {
    fn coefficients(&self) -> &super::core::Vec4;
    fn coefficients_mut(&mut self) -> &mut super::core::Vec4;
    fn scale_value(&self) -> &f32;
    fn scale_value_mut(&mut self) -> &mut f32;
    fn min_clamp(&self) -> &f32;
    fn min_clamp_mut(&mut self) -> &mut f32;
    fn max_clamp(&self) -> &f32;
    fn max_clamp_mut(&mut self) -> &mut f32;
}

impl PolynomialTempDataTrait for PolynomialTempData {
    fn coefficients(&self) -> &super::core::Vec4 {
        &self.coefficients
    }
    fn coefficients_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.coefficients
    }
    fn scale_value(&self) -> &f32 {
        &self.scale_value
    }
    fn scale_value_mut(&mut self) -> &mut f32 {
        &mut self.scale_value
    }
    fn min_clamp(&self) -> &f32 {
        &self.min_clamp
    }
    fn min_clamp_mut(&mut self) -> &mut f32 {
        &mut self.min_clamp
    }
    fn max_clamp(&self) -> &f32 {
        &self.max_clamp
    }
    fn max_clamp_mut(&mut self) -> &mut f32 {
        &mut self.max_clamp
    }
}

pub static POLYNOMIALTEMPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialTempData",
    flags: MemberInfoFlags::new(36937),
    module: "Emitter",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PolynomialTempData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Coefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(PolynomialTempData, coefficients),
            },
            FieldInfoData {
                name: "ScaleValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PolynomialTempData, scale_value),
            },
            FieldInfoData {
                name: "MinClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PolynomialTempData, min_clamp),
            },
            FieldInfoData {
                name: "MaxClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PolynomialTempData, max_clamp),
            },
        ],
    }),
    array_type: Some(POLYNOMIALTEMPDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PolynomialTempData {
    fn type_info(&self) -> &'static TypeInfo {
        POLYNOMIALTEMPDATA_TYPE_INFO
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


pub static POLYNOMIALTEMPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialTempData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialTempData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PolynomialData {
    pub _glacier_base: EvaluatorData,
    pub coefficients: super::core::Vec4,
    pub scale_value: f32,
    pub min_clamp: f32,
    pub max_clamp: f32,
}

pub trait PolynomialDataTrait: EvaluatorDataTrait {
    fn coefficients(&self) -> &super::core::Vec4;
    fn coefficients_mut(&mut self) -> &mut super::core::Vec4;
    fn scale_value(&self) -> &f32;
    fn scale_value_mut(&mut self) -> &mut f32;
    fn min_clamp(&self) -> &f32;
    fn min_clamp_mut(&mut self) -> &mut f32;
    fn max_clamp(&self) -> &f32;
    fn max_clamp_mut(&mut self) -> &mut f32;
}

impl PolynomialDataTrait for PolynomialData {
    fn coefficients(&self) -> &super::core::Vec4 {
        &self.coefficients
    }
    fn coefficients_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.coefficients
    }
    fn scale_value(&self) -> &f32 {
        &self.scale_value
    }
    fn scale_value_mut(&mut self) -> &mut f32 {
        &mut self.scale_value
    }
    fn min_clamp(&self) -> &f32 {
        &self.min_clamp
    }
    fn min_clamp_mut(&mut self) -> &mut f32 {
        &mut self.min_clamp
    }
    fn max_clamp(&self) -> &f32 {
        &self.max_clamp
    }
    fn max_clamp_mut(&mut self) -> &mut f32 {
        &mut self.max_clamp
    }
}

impl EvaluatorDataTrait for PolynomialData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for PolynomialData {
}

pub static POLYNOMIALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PolynomialData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Coefficients",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(PolynomialData, coefficients),
            },
            FieldInfoData {
                name: "ScaleValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PolynomialData, scale_value),
            },
            FieldInfoData {
                name: "MinClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PolynomialData, min_clamp),
            },
            FieldInfoData {
                name: "MaxClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PolynomialData, max_clamp),
            },
        ],
    }),
    array_type: Some(POLYNOMIALDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PolynomialData {
    fn type_info(&self) -> &'static TypeInfo {
        POLYNOMIALDATA_TYPE_INFO
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


pub static POLYNOMIALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PolynomialData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("PolynomialData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DefaultEvaluatorData {
    pub _glacier_base: EvaluatorData,
    pub values: super::core::Vec4,
}

pub trait DefaultEvaluatorDataTrait: EvaluatorDataTrait {
    fn values(&self) -> &super::core::Vec4;
    fn values_mut(&mut self) -> &mut super::core::Vec4;
}

impl DefaultEvaluatorDataTrait for DefaultEvaluatorData {
    fn values(&self) -> &super::core::Vec4 {
        &self.values
    }
    fn values_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.values
    }
}

impl EvaluatorDataTrait for DefaultEvaluatorData {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter()
    }
    fn parameter_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        self._glacier_base.parameter_mut()
    }
    fn schematics_enable(&self) -> &bool {
        self._glacier_base.schematics_enable()
    }
    fn schematics_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.schematics_enable_mut()
    }
}

impl super::core::DataContainerTrait for DefaultEvaluatorData {
}

pub static DEFAULTEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "Emitter",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVALUATORDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DefaultEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(DefaultEvaluatorData, values),
            },
        ],
    }),
    array_type: Some(DEFAULTEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DefaultEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        DEFAULTEVALUATORDATA_TYPE_INFO
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


pub static DEFAULTEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Emitter",
    data: TypeInfoData::Array("DefaultEvaluatorData"),
    array_type: None,
    alignment: 8,
};


