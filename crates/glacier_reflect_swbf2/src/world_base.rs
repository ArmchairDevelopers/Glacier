use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_world_base_types(registry: &mut TypeRegistry) {
    registry.register_type(LOCALFOGVOLUMEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(LOCALFOGVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALFOGVOLUMESTATICSTATE_TYPE_INFO);
    registry.register_type(LOCALFOGVOLUMESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VOLUMETRICDYNAMICSTATE_TYPE_INFO);
    registry.register_type(VOLUMETRICDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VOLUMETRICSTATICSTATE_TYPE_INFO);
    registry.register_type(VOLUMETRICSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PARTICIPATINGMEDIA_TYPE_INFO);
    registry.register_type(PARTICIPATINGMEDIA_ARRAY_TYPE_INFO);
    registry.register_type(PARTICIPATINGMEDIAMATERIALDATA_TYPE_INFO);
    registry.register_type(PARTICIPATINGMEDIAMATERIALDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTICIPATINGMEDIASPECIFICATIONMODE_TYPE_INFO);
    registry.register_type(PARTICIPATINGMEDIASPECIFICATIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(CLOUDCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(CLOUDCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SUNFLAREEFFECTCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SUNFLAREEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SKYEFFECTCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SKYEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SKYCLOUDLAYER_TYPE_INFO);
    registry.register_type(SKYCLOUDLAYER_ARRAY_TYPE_INFO);
    registry.register_type(FOGEFFECTCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(FOGEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OUTDOORLIGHTEFFECTCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(OUTDOORLIGHTEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SONARPARAMSCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SONARPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(HOLOGRAMPARAMSCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(HOLOGRAMPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(HOLOGRAMPLANECOUNT_TYPE_INFO);
    registry.register_type(HOLOGRAMPLANECOUNT_ARRAY_TYPE_INFO);
    registry.register_type(HOLOGRAMRENDERMODE_TYPE_INFO);
    registry.register_type(HOLOGRAMRENDERMODE_ARRAY_TYPE_INFO);
    registry.register_type(THREATALERTHIGHLIGHTPARAMSCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(THREATALERTHIGHLIGHTPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTPARAMSCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(ANTIALIASCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(ANTIALIASCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SKYCELESTIALMULTICOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SKYCELESTIALMULTICOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SKYCELESTIALSINGLECOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SKYCELESTIALSINGLECOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SKYCELESTIALROTATIONCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SKYCELESTIALROTATIONCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SKYCELESTIALCOMPONENTCOUNTMAX_TYPE_INFO);
    registry.register_type(SKYCELESTIALCOMPONENTCOUNTMAX_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERPARAMGLOBALCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(EMITTERPARAMGLOBALCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERPARAMGLOBALCOUNTMAX_TYPE_INFO);
    registry.register_type(EMITTERPARAMGLOBALCOUNTMAX_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERPARAMCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(EMITTERPARAMCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERPARAMOVERRIDE_TYPE_INFO);
    registry.register_type(EMITTERPARAMOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(RAYTRACEREFLECTIONCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(RAYTRACEREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SCREENSPACERAYTRACECOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SCREENSPACERAYTRACECOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(MOTIONBLURCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(MOTIONBLURCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERLIGHTINGCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(CHARACTERLIGHTINGCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERLIGHTINGMODE_TYPE_INFO);
    registry.register_type(CHARACTERLIGHTINGMODE_ARRAY_TYPE_INFO);
    registry.register_type(DAMAGEEFFECTCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(DAMAGEEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SCREENEFFECTCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SCREENEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SCREENEFFECTFRAMETYPE_TYPE_INFO);
    registry.register_type(SCREENEFFECTFRAMETYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWSCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SHADOWSCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(MESHSETTINGSCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(MESHSETTINGSCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CAMERAPARAMSCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(CAMERAPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERCOLORPARAMSCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SHADERCOLORPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMSCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SHADERPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(ENLIGHTENCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SKYBOXBLENDMODE_TYPE_INFO);
    registry.register_type(SKYBOXBLENDMODE_ARRAY_TYPE_INFO);
    registry.register_type(SUBSURFACESCATTERINGCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SUBSURFACESCATTERINGCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICAOCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(DYNAMICAOCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SUNFLARECOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SUNFLARECOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(WINDCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(WINDCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICENVMAPCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(DYNAMICENVMAPCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SKYCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(SKYCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SKYTYPE_TYPE_INFO);
    registry.register_type(SKYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(ALPHAOUTPUTMODE_TYPE_INFO);
    registry.register_type(ALPHAOUTPUTMODE_ARRAY_TYPE_INFO);
    registry.register_type(FOGCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(FOGCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OUTDOORLIGHTCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(OUTDOORLIGHTCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(INDIRECTSPECULARCOMPONENTSTATE_TYPE_INFO);
    registry.register_type(INDIRECTSPECULARCOMPONENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWFILTERINGTYPE_TYPE_INFO);
    registry.register_type(SHADOWFILTERINGTYPE_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTCOMPONENTSTATICSTATE_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTCOMPONENTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTSTATICSTATE_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTHANDLE_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(FORWARDLIGHTSCATTERINGATTENUATION_TYPE_INFO);
    registry.register_type(FORWARDLIGHTSCATTERINGATTENUATION_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTSETTINGS_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURESETDYNAMICSTATE_TYPE_INFO);
    registry.register_type(TEXTURESETDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURESETSTATICSTATE_TYPE_INFO);
    registry.register_type(TEXTURESETSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(MESHDEFORMERHANDLE_TYPE_INFO);
    registry.register_type(MESHDEFORMERHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTHANDLE_TYPE_INFO);
    registry.register_type(LIGHTHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(MODELHANDLE_TYPE_INFO);
    registry.register_type(MODELHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(WASVISIBLEHANDLE_TYPE_INFO);
    registry.register_type(WASVISIBLEHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLEVOLUMETRICSDYNAMICSTATE_TYPE_INFO);
    registry.register_type(SIMPLEVOLUMETRICSDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLEVOLUMETRICSSTATICSTATE_TYPE_INFO);
    registry.register_type(SIMPLEVOLUMETRICSSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLEVOLUMETRICSDRAWPASS_TYPE_INFO);
    registry.register_type(SIMPLEVOLUMETRICSDRAWPASS_ARRAY_TYPE_INFO);
    registry.register_type(RESOURCEREFDYNAMICSTATE_TYPE_INFO);
    registry.register_type(RESOURCEREFDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RESOURCEREFSTATICSTATE_TYPE_INFO);
    registry.register_type(RESOURCEREFSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RESOURCEREFHANDLE_TYPE_INFO);
    registry.register_type(RESOURCEREFHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERPLANEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(OCCLUDERPLANEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERPLANESTATICSTATE_TYPE_INFO);
    registry.register_type(OCCLUDERPLANESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERVOLUMEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(OCCLUDERVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERVOLUMESTATICSTATE_TYPE_INFO);
    registry.register_type(OCCLUDERVOLUMESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERMESHDYNAMICSTATE_TYPE_INFO);
    registry.register_type(OCCLUDERMESHDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERMESHSTATICSTATE_TYPE_INFO);
    registry.register_type(OCCLUDERMESHSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTSTATICSTATE_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTTYPE_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EDGEMODELSBASEDATA_TYPE_INFO);
    registry.register_type(EDGEMODELSBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(EDGEMODELCONNECTIONINFO_TYPE_INFO);
    registry.register_type(EDGEMODELCONNECTIONINFO_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMEBASEASSET_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMEBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMEBASEDATA_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMEBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(MESHDYNAMICSTATE_TYPE_INFO);
    registry.register_type(MESHDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(MESHSTATICSTATE_TYPE_INFO);
    registry.register_type(MESHSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(MODELCREATEINFO_TYPE_INFO);
    registry.register_type(MODELCREATEINFO_ARRAY_TYPE_INFO);
    registry.register_type(MODELDYNAMICSTATE_TYPE_INFO);
    registry.register_type(MODELDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(MODELSTATICSTATE_TYPE_INFO);
    registry.register_type(MODELSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(MODELHIERARCHYLEVEL_TYPE_INFO);
    registry.register_type(MODELHIERARCHYLEVEL_ARRAY_TYPE_INFO);
    registry.register_type(VISIBLEAREADYNAMICSTATE_TYPE_INFO);
    registry.register_type(VISIBLEAREADYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VISIBLEAREASTATICSTATE_TYPE_INFO);
    registry.register_type(VISIBLEAREASTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONLOCATORDYNAMICSTATE_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONLOCATORDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONLOCATORSTATICSTATE_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONLOCATORSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTPROBEVOLUMEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(LIGHTPROBEVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTPROBEVOLUMESTATICSTATE_TYPE_INFO);
    registry.register_type(LIGHTPROBEVOLUMESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICENLIGHTENDYNAMICSTATE_TYPE_INFO);
    registry.register_type(DYNAMICENLIGHTENDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICENLIGHTENSTATICSTATE_TYPE_INFO);
    registry.register_type(DYNAMICENLIGHTENSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(STATICENLIGHTENDYNAMICSTATE_TYPE_INFO);
    registry.register_type(STATICENLIGHTENDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(STATICENLIGHTENSTATICSTATE_TYPE_INFO);
    registry.register_type(STATICENLIGHTENSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMODIFIERDYNAMICSTATE_TYPE_INFO);
    registry.register_type(RADIOSITYMODIFIERDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMODIFIERSTATICSTATE_TYPE_INFO);
    registry.register_type(RADIOSITYMODIFIERSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALTRIGGERDYNAMICSTATE_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALTRIGGERDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALTRIGGERSTATICSTATE_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALTRIGGERSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALDYNAMICSTATE_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALSTATICSTATE_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(GROUNDHEIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(GROUNDHEIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(GROUNDHEIGHTSTATICSTATE_TYPE_INFO);
    registry.register_type(GROUNDHEIGHTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(GROUNDHEIGHTDATA_TYPE_INFO);
    registry.register_type(GROUNDHEIGHTDATA_ARRAY_TYPE_INFO);
    registry.register_type(RENDERVOLUMEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(RENDERVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERVOLUMESTATICSTATE_TYPE_INFO);
    registry.register_type(RENDERVOLUMESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERVOLUMETRANSFORMTYPE_TYPE_INFO);
    registry.register_type(RENDERVOLUMETRANSFORMTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PBRRECTANGULARLIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(PBRRECTANGULARLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PBRRECTANGULARLIGHTSTATICSTATE_TYPE_INFO);
    registry.register_type(PBRRECTANGULARLIGHTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PBRTUBELIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(PBRTUBELIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PBRTUBELIGHTSTATICSTATE_TYPE_INFO);
    registry.register_type(PBRTUBELIGHTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PBRSPOTLIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(PBRSPOTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PBRSPOTLIGHTSTATICSTATE_TYPE_INFO);
    registry.register_type(PBRSPOTLIGHTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PBRSPHERELIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(PBRSPHERELIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PBRSPHERELIGHTSTATICSTATE_TYPE_INFO);
    registry.register_type(PBRSPHERELIGHTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PBRANALYTICLIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(PBRANALYTICLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OLDSPOTLIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(OLDSPOTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OLDSPOTLIGHTSTATICSTATE_TYPE_INFO);
    registry.register_type(OLDSPOTLIGHTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OLDPOINTLIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(OLDPOINTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OLDPOINTLIGHTSTATICSTATE_TYPE_INFO);
    registry.register_type(OLDPOINTLIGHTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OLDLIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(OLDLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTDYNAMICSTATE_TYPE_INFO);
    registry.register_type(LIGHTDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTSTATICSTATE_TYPE_INFO);
    registry.register_type(LIGHTSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(IESPROFILEASSET_TYPE_INFO);
    registry.register_type(IESPROFILEASSET_ARRAY_TYPE_INFO);
    registry.register_type(SPOTLIGHTSHAPE_TYPE_INFO);
    registry.register_type(SPOTLIGHTSHAPE_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENCOLORMODE_TYPE_INFO);
    registry.register_type(ENLIGHTENCOLORMODE_ARRAY_TYPE_INFO);
    registry.register_type(SETLIGHTTRANSFORM_LIGHTHANDLE_LINEARTRANSFORM__TYPE_INFO);
    registry.register_type(CREATELIGHT_LIGHTHANDLE_LINEARTRANSFORM_VEC3__TYPE_INFO);
    registry.register_type(LENSFLARECREATESTATE_TYPE_INFO);
    registry.register_type(LENSFLARECREATESTATE_ARRAY_TYPE_INFO);
    registry.register_type(LENSFLAREDYNAMICSTATE_TYPE_INFO);
    registry.register_type(LENSFLAREDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LENSFLARESTATICSTATE_TYPE_INFO);
    registry.register_type(LENSFLARESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LENSFLARERENDERMODE_TYPE_INFO);
    registry.register_type(LENSFLARERENDERMODE_ARRAY_TYPE_INFO);
    registry.register_type(FLAREDIRECTIONMODE_TYPE_INFO);
    registry.register_type(FLAREDIRECTIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(LENSFLAREELEMENT_TYPE_INFO);
    registry.register_type(LENSFLAREELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(REFLECTIONVOLUMEQUERYDYNAMICSTATE_TYPE_INFO);
    registry.register_type(REFLECTIONVOLUMEQUERYDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(REFLECTIONVOLUMEQUERYHANDLE_TYPE_INFO);
    registry.register_type(REFLECTIONVOLUMEQUERYHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALPLANARREFLECTIONDYNAMICSTATE_TYPE_INFO);
    registry.register_type(LOCALPLANARREFLECTIONDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALPLANARREFLECTIONSTATICSTATE_TYPE_INFO);
    registry.register_type(LOCALPLANARREFLECTIONSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DISTANTIBLDYNAMICSTATE_TYPE_INFO);
    registry.register_type(DISTANTIBLDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DISTANTIBLSTATICSTATE_TYPE_INFO);
    registry.register_type(DISTANTIBLSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(BOXIBLDYNAMICSTATE_TYPE_INFO);
    registry.register_type(BOXIBLDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(BOXIBLSTATICSTATE_TYPE_INFO);
    registry.register_type(BOXIBLSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SPHEREIBLDYNAMICSTATE_TYPE_INFO);
    registry.register_type(SPHEREIBLDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SPHEREIBLSTATICSTATE_TYPE_INFO);
    registry.register_type(SPHEREIBLSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(IBLDYNAMICSTATE_TYPE_INFO);
    registry.register_type(IBLDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(IBLSTATICSTATE_TYPE_INFO);
    registry.register_type(IBLSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(FOGEXCLUSIONVOLUMEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(FOGEXCLUSIONVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(FOGEXCLUSIONVOLUMESTATICSTATE_TYPE_INFO);
    registry.register_type(FOGEXCLUSIONVOLUMESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(FOGEXCLUSIONVOLUMESHAPE_TYPE_INFO);
    registry.register_type(FOGEXCLUSIONVOLUMESHAPE_ARRAY_TYPE_INFO);
    registry.register_type(DISTANTSHADOWCACHEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(DISTANTSHADOWCACHEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DISTANTSHADOWCACHESTATICSTATE_TYPE_INFO);
    registry.register_type(DISTANTSHADOWCACHESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(BAKEABLETEXTUREDYNAMICSTATE_TYPE_INFO);
    registry.register_type(BAKEABLETEXTUREDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(BAKEABLETEXTURESTATICSTATE_TYPE_INFO);
    registry.register_type(BAKEABLETEXTURESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(POSECONSUMER_TYPE_INFO);
    registry.register_type(MESHHANDLE_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct LocalFogVolumeDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub refresh_counter: u32,
    pub participating_media: ParticipatingMedia,
    pub ambient_lighting_scale: f32,
    pub density_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub field_flag_changed0: u8,
}

pub trait LocalFogVolumeDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn enabled(&self) -> &bool;
    fn refresh_counter(&self) -> &u32;
    fn participating_media(&self) -> &ParticipatingMedia;
    fn ambient_lighting_scale(&self) -> &f32;
    fn density_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn field_flag_changed0(&self) -> &u8;
}

impl LocalFogVolumeDynamicStateTrait for LocalFogVolumeDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn refresh_counter(&self) -> &u32 {
        &self.refresh_counter
    }
    fn participating_media(&self) -> &ParticipatingMedia {
        &self.participating_media
    }
    fn ambient_lighting_scale(&self) -> &f32 {
        &self.ambient_lighting_scale
    }
    fn density_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.density_texture
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static LOCALFOGVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalFogVolumeDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(LocalFogVolumeDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LocalFogVolumeDynamicState, enabled),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LocalFogVolumeDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "ParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: "ParticipatingMedia",
                rust_offset: offset_of!(LocalFogVolumeDynamicState, participating_media),
            },
            FieldInfoData {
                name: "AmbientLightingScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalFogVolumeDynamicState, ambient_lighting_scale),
            },
            FieldInfoData {
                name: "DensityTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(LocalFogVolumeDynamicState, density_texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(LocalFogVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LOCALFOGVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalFogVolumeDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALFOGVOLUMEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALFOGVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LocalFogVolumeDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalFogVolumeStaticState {
    pub guid: glacier_util::guid::Guid,
    pub object_layers: u16,
    pub field_flag_changed0: u8,
}

pub trait LocalFogVolumeStaticStateTrait: TypeObject {
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn object_layers(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u8;
}

impl LocalFogVolumeStaticStateTrait for LocalFogVolumeStaticState {
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn object_layers(&self) -> &u16 {
        &self.object_layers
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static LOCALFOGVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalFogVolumeStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(LocalFogVolumeStaticState, guid),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(LocalFogVolumeStaticState, object_layers),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(LocalFogVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LOCALFOGVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LocalFogVolumeStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALFOGVOLUMESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALFOGVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LocalFogVolumeStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VolumetricDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub refresh_counter: u32,
    pub participating_media: ParticipatingMedia,
    pub ambient_lighting_scale: f32,
    pub density_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
}

pub trait VolumetricDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn enabled(&self) -> &bool;
    fn refresh_counter(&self) -> &u32;
    fn participating_media(&self) -> &ParticipatingMedia;
    fn ambient_lighting_scale(&self) -> &f32;
    fn density_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
}

impl VolumetricDynamicStateTrait for VolumetricDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn refresh_counter(&self) -> &u32 {
        &self.refresh_counter
    }
    fn participating_media(&self) -> &ParticipatingMedia {
        &self.participating_media
    }
    fn ambient_lighting_scale(&self) -> &f32 {
        &self.ambient_lighting_scale
    }
    fn density_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.density_texture
    }
}

pub static VOLUMETRICDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumetricDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VolumetricDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(VolumetricDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VolumetricDynamicState, enabled),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VolumetricDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "ParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: "ParticipatingMedia",
                rust_offset: offset_of!(VolumetricDynamicState, participating_media),
            },
            FieldInfoData {
                name: "AmbientLightingScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VolumetricDynamicState, ambient_lighting_scale),
            },
            FieldInfoData {
                name: "DensityTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(VolumetricDynamicState, density_texture),
            },
        ],
    }),
    array_type: Some(VOLUMETRICDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VolumetricDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        VOLUMETRICDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VOLUMETRICDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumetricDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VolumetricDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VolumetricStaticState {
    pub guid: glacier_util::guid::Guid,
    pub object_layers: u16,
}

pub trait VolumetricStaticStateTrait: TypeObject {
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn object_layers(&self) -> &u16;
}

impl VolumetricStaticStateTrait for VolumetricStaticState {
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn object_layers(&self) -> &u16 {
        &self.object_layers
    }
}

pub static VOLUMETRICSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumetricStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VolumetricStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(VolumetricStaticState, guid),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(VolumetricStaticState, object_layers),
            },
        ],
    }),
    array_type: Some(VOLUMETRICSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VolumetricStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        VOLUMETRICSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VOLUMETRICSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumetricStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VolumetricStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ParticipatingMedia {
    pub specification_mode: ParticipatingMediaSpecificationMode,
    pub absorption: f32,
    pub scattering: super::core::Vec3,
    pub exctinction: f32,
    pub albedo: super::core::Vec3,
    pub emissive: super::core::Vec3,
    pub phase: f32,
}

pub trait ParticipatingMediaTrait: TypeObject {
    fn specification_mode(&self) -> &ParticipatingMediaSpecificationMode;
    fn absorption(&self) -> &f32;
    fn scattering(&self) -> &super::core::Vec3;
    fn exctinction(&self) -> &f32;
    fn albedo(&self) -> &super::core::Vec3;
    fn emissive(&self) -> &super::core::Vec3;
    fn phase(&self) -> &f32;
}

impl ParticipatingMediaTrait for ParticipatingMedia {
    fn specification_mode(&self) -> &ParticipatingMediaSpecificationMode {
        &self.specification_mode
    }
    fn absorption(&self) -> &f32 {
        &self.absorption
    }
    fn scattering(&self) -> &super::core::Vec3 {
        &self.scattering
    }
    fn exctinction(&self) -> &f32 {
        &self.exctinction
    }
    fn albedo(&self) -> &super::core::Vec3 {
        &self.albedo
    }
    fn emissive(&self) -> &super::core::Vec3 {
        &self.emissive
    }
    fn phase(&self) -> &f32 {
        &self.phase
    }
}

pub static PARTICIPATINGMEDIA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMedia",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ParticipatingMedia as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SpecificationMode",
                flags: MemberInfoFlags::new(0),
                field_type: "ParticipatingMediaSpecificationMode",
                rust_offset: offset_of!(ParticipatingMedia, specification_mode),
            },
            FieldInfoData {
                name: "Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParticipatingMedia, absorption),
            },
            FieldInfoData {
                name: "Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ParticipatingMedia, scattering),
            },
            FieldInfoData {
                name: "Exctinction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParticipatingMedia, exctinction),
            },
            FieldInfoData {
                name: "Albedo",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ParticipatingMedia, albedo),
            },
            FieldInfoData {
                name: "Emissive",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ParticipatingMedia, emissive),
            },
            FieldInfoData {
                name: "Phase",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParticipatingMedia, phase),
            },
        ],
    }),
    array_type: Some(PARTICIPATINGMEDIA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ParticipatingMedia {
    fn type_info(&self) -> &'static TypeInfo {
        PARTICIPATINGMEDIA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PARTICIPATINGMEDIA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMedia-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ParticipatingMedia"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ParticipatingMediaMaterialData {
    pub specification_mode: ParticipatingMediaSpecificationMode,
    pub absorption: f32,
    pub scattering: super::core::Vec3,
    pub exctinction: f32,
    pub albedo: super::core::Vec3,
    pub emissive: super::core::Vec3,
    pub phase: f32,
}

pub trait ParticipatingMediaMaterialDataTrait: TypeObject {
    fn specification_mode(&self) -> &ParticipatingMediaSpecificationMode;
    fn absorption(&self) -> &f32;
    fn scattering(&self) -> &super::core::Vec3;
    fn exctinction(&self) -> &f32;
    fn albedo(&self) -> &super::core::Vec3;
    fn emissive(&self) -> &super::core::Vec3;
    fn phase(&self) -> &f32;
}

impl ParticipatingMediaMaterialDataTrait for ParticipatingMediaMaterialData {
    fn specification_mode(&self) -> &ParticipatingMediaSpecificationMode {
        &self.specification_mode
    }
    fn absorption(&self) -> &f32 {
        &self.absorption
    }
    fn scattering(&self) -> &super::core::Vec3 {
        &self.scattering
    }
    fn exctinction(&self) -> &f32 {
        &self.exctinction
    }
    fn albedo(&self) -> &super::core::Vec3 {
        &self.albedo
    }
    fn emissive(&self) -> &super::core::Vec3 {
        &self.emissive
    }
    fn phase(&self) -> &f32 {
        &self.phase
    }
}

pub static PARTICIPATINGMEDIAMATERIALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaMaterialData",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ParticipatingMediaMaterialData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SpecificationMode",
                flags: MemberInfoFlags::new(0),
                field_type: "ParticipatingMediaSpecificationMode",
                rust_offset: offset_of!(ParticipatingMediaMaterialData, specification_mode),
            },
            FieldInfoData {
                name: "Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParticipatingMediaMaterialData, absorption),
            },
            FieldInfoData {
                name: "Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ParticipatingMediaMaterialData, scattering),
            },
            FieldInfoData {
                name: "Exctinction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParticipatingMediaMaterialData, exctinction),
            },
            FieldInfoData {
                name: "Albedo",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ParticipatingMediaMaterialData, albedo),
            },
            FieldInfoData {
                name: "Emissive",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ParticipatingMediaMaterialData, emissive),
            },
            FieldInfoData {
                name: "Phase",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ParticipatingMediaMaterialData, phase),
            },
        ],
    }),
    array_type: Some(PARTICIPATINGMEDIAMATERIALDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ParticipatingMediaMaterialData {
    fn type_info(&self) -> &'static TypeInfo {
        PARTICIPATINGMEDIAMATERIALDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PARTICIPATINGMEDIAMATERIALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaMaterialData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ParticipatingMediaMaterialData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ParticipatingMediaSpecificationMode {
    #[default]
    ParticipatingMediaSpecificationMode_AbsorptionScattering = 0,
    ParticipatingMediaSpecificationMode_ExtinctionAlbedo = 1,
}

pub static PARTICIPATINGMEDIASPECIFICATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaSpecificationMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(PARTICIPATINGMEDIASPECIFICATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ParticipatingMediaSpecificationMode {
    fn type_info(&self) -> &'static TypeInfo {
        PARTICIPATINGMEDIASPECIFICATIONMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PARTICIPATINGMEDIASPECIFICATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaSpecificationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ParticipatingMediaSpecificationMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CloudComponentState {
    pub enable: bool,
    pub base_to_top_multiplier: f32,
    pub edge_detail_multiplier: f32,
    pub cloud_density_multiplier: f32,
    pub absorption: super::core::Vec3,
    pub scattering: super::core::Vec3,
    pub phase_g0: f32,
    pub phase_g1: f32,
    pub phase_blend: f32,
    pub ambient_multiplicator: f32,
    pub ambient_desaturate: f32,
    pub aerial_perspective_scale: f32,
    pub enable_shadow: bool,
    pub scattering_order: i32,
    pub scattering_factor: f32,
    pub extinction_factor: f32,
    pub phase_factor: f32,
    pub shape_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub shape_texture_scale: f32,
    pub shape_texture_contrast: f32,
    pub detail_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub detail_texture_scale: f32,
    pub weather_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub weather_texture_scale: f32,
    pub cloud_type_density_gradient_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub planet_radius: f32,
    pub cut_off_distance: f32,
    pub cloud_layer_start_height: f32,
    pub cloud_layer_thickness: f32,
    pub wind_scale: f32,
    pub offset: super::core::Vec2,
    pub field_flag_override0: u32,
    pub field_flag_changed0: u32,
}

pub trait CloudComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn base_to_top_multiplier(&self) -> &f32;
    fn edge_detail_multiplier(&self) -> &f32;
    fn cloud_density_multiplier(&self) -> &f32;
    fn absorption(&self) -> &super::core::Vec3;
    fn scattering(&self) -> &super::core::Vec3;
    fn phase_g0(&self) -> &f32;
    fn phase_g1(&self) -> &f32;
    fn phase_blend(&self) -> &f32;
    fn ambient_multiplicator(&self) -> &f32;
    fn ambient_desaturate(&self) -> &f32;
    fn aerial_perspective_scale(&self) -> &f32;
    fn enable_shadow(&self) -> &bool;
    fn scattering_order(&self) -> &i32;
    fn scattering_factor(&self) -> &f32;
    fn extinction_factor(&self) -> &f32;
    fn phase_factor(&self) -> &f32;
    fn shape_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn shape_texture_scale(&self) -> &f32;
    fn shape_texture_contrast(&self) -> &f32;
    fn detail_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn detail_texture_scale(&self) -> &f32;
    fn weather_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn weather_texture_scale(&self) -> &f32;
    fn cloud_type_density_gradient_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn planet_radius(&self) -> &f32;
    fn cut_off_distance(&self) -> &f32;
    fn cloud_layer_start_height(&self) -> &f32;
    fn cloud_layer_thickness(&self) -> &f32;
    fn wind_scale(&self) -> &f32;
    fn offset(&self) -> &super::core::Vec2;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u32;
}

impl CloudComponentStateTrait for CloudComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn base_to_top_multiplier(&self) -> &f32 {
        &self.base_to_top_multiplier
    }
    fn edge_detail_multiplier(&self) -> &f32 {
        &self.edge_detail_multiplier
    }
    fn cloud_density_multiplier(&self) -> &f32 {
        &self.cloud_density_multiplier
    }
    fn absorption(&self) -> &super::core::Vec3 {
        &self.absorption
    }
    fn scattering(&self) -> &super::core::Vec3 {
        &self.scattering
    }
    fn phase_g0(&self) -> &f32 {
        &self.phase_g0
    }
    fn phase_g1(&self) -> &f32 {
        &self.phase_g1
    }
    fn phase_blend(&self) -> &f32 {
        &self.phase_blend
    }
    fn ambient_multiplicator(&self) -> &f32 {
        &self.ambient_multiplicator
    }
    fn ambient_desaturate(&self) -> &f32 {
        &self.ambient_desaturate
    }
    fn aerial_perspective_scale(&self) -> &f32 {
        &self.aerial_perspective_scale
    }
    fn enable_shadow(&self) -> &bool {
        &self.enable_shadow
    }
    fn scattering_order(&self) -> &i32 {
        &self.scattering_order
    }
    fn scattering_factor(&self) -> &f32 {
        &self.scattering_factor
    }
    fn extinction_factor(&self) -> &f32 {
        &self.extinction_factor
    }
    fn phase_factor(&self) -> &f32 {
        &self.phase_factor
    }
    fn shape_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.shape_texture
    }
    fn shape_texture_scale(&self) -> &f32 {
        &self.shape_texture_scale
    }
    fn shape_texture_contrast(&self) -> &f32 {
        &self.shape_texture_contrast
    }
    fn detail_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.detail_texture
    }
    fn detail_texture_scale(&self) -> &f32 {
        &self.detail_texture_scale
    }
    fn weather_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.weather_texture
    }
    fn weather_texture_scale(&self) -> &f32 {
        &self.weather_texture_scale
    }
    fn cloud_type_density_gradient_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.cloud_type_density_gradient_texture
    }
    fn planet_radius(&self) -> &f32 {
        &self.planet_radius
    }
    fn cut_off_distance(&self) -> &f32 {
        &self.cut_off_distance
    }
    fn cloud_layer_start_height(&self) -> &f32 {
        &self.cloud_layer_start_height
    }
    fn cloud_layer_thickness(&self) -> &f32 {
        &self.cloud_layer_thickness
    }
    fn wind_scale(&self) -> &f32 {
        &self.wind_scale
    }
    fn offset(&self) -> &super::core::Vec2 {
        &self.offset
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static CLOUDCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CloudComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CloudComponentState, enable),
            },
            FieldInfoData {
                name: "BaseToTopMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, base_to_top_multiplier),
            },
            FieldInfoData {
                name: "EdgeDetailMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, edge_detail_multiplier),
            },
            FieldInfoData {
                name: "CloudDensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, cloud_density_multiplier),
            },
            FieldInfoData {
                name: "Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CloudComponentState, absorption),
            },
            FieldInfoData {
                name: "Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CloudComponentState, scattering),
            },
            FieldInfoData {
                name: "PhaseG0",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, phase_g0),
            },
            FieldInfoData {
                name: "PhaseG1",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, phase_g1),
            },
            FieldInfoData {
                name: "PhaseBlend",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, phase_blend),
            },
            FieldInfoData {
                name: "AmbientMultiplicator",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, ambient_multiplicator),
            },
            FieldInfoData {
                name: "AmbientDesaturate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, ambient_desaturate),
            },
            FieldInfoData {
                name: "AerialPerspectiveScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, aerial_perspective_scale),
            },
            FieldInfoData {
                name: "EnableShadow",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CloudComponentState, enable_shadow),
            },
            FieldInfoData {
                name: "ScatteringOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CloudComponentState, scattering_order),
            },
            FieldInfoData {
                name: "ScatteringFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, scattering_factor),
            },
            FieldInfoData {
                name: "ExtinctionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, extinction_factor),
            },
            FieldInfoData {
                name: "PhaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, phase_factor),
            },
            FieldInfoData {
                name: "ShapeTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(CloudComponentState, shape_texture),
            },
            FieldInfoData {
                name: "ShapeTextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, shape_texture_scale),
            },
            FieldInfoData {
                name: "ShapeTextureContrast",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, shape_texture_contrast),
            },
            FieldInfoData {
                name: "DetailTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(CloudComponentState, detail_texture),
            },
            FieldInfoData {
                name: "DetailTextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, detail_texture_scale),
            },
            FieldInfoData {
                name: "WeatherTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(CloudComponentState, weather_texture),
            },
            FieldInfoData {
                name: "WeatherTextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, weather_texture_scale),
            },
            FieldInfoData {
                name: "CloudTypeDensityGradientTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(CloudComponentState, cloud_type_density_gradient_texture),
            },
            FieldInfoData {
                name: "PlanetRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, planet_radius),
            },
            FieldInfoData {
                name: "CutOffDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, cut_off_distance),
            },
            FieldInfoData {
                name: "CloudLayerStartHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, cloud_layer_start_height),
            },
            FieldInfoData {
                name: "CloudLayerThickness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, cloud_layer_thickness),
            },
            FieldInfoData {
                name: "WindScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CloudComponentState, wind_scale),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(CloudComponentState, offset),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CloudComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CloudComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CLOUDCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CloudComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        CLOUDCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLOUDCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("CloudComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SunFlareEffectComponentState {
    pub _glacier_base: super::core::DataContainer,
    pub enable: bool,
    pub debug_draw_occluder: bool,
    pub occluder_size: f32,
    pub element1_enable: bool,
    pub element1_shader: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub element1_ray_distance: f32,
    pub element1_size: super::core::Vec2,
    pub element1_size_occluder_curve: super::core::Vec4,
    pub element1_size_screen_pos_curve: super::core::Vec4,
    pub element1_alpha_occluder_curve: super::core::Vec4,
    pub element1_alpha_screen_pos_curve: super::core::Vec4,
    pub element2_enable: bool,
    pub element2_shader: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub element2_ray_distance: f32,
    pub element2_size: super::core::Vec2,
    pub element2_size_occluder_curve: super::core::Vec4,
    pub element2_size_screen_pos_curve: super::core::Vec4,
    pub element2_alpha_occluder_curve: super::core::Vec4,
    pub element2_alpha_screen_pos_curve: super::core::Vec4,
    pub element3_enable: bool,
    pub element3_shader: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub element3_ray_distance: f32,
    pub element3_size: super::core::Vec2,
    pub element3_size_occluder_curve: super::core::Vec4,
    pub element3_size_screen_pos_curve: super::core::Vec4,
    pub element3_alpha_occluder_curve: super::core::Vec4,
    pub element3_alpha_screen_pos_curve: super::core::Vec4,
    pub element4_enable: bool,
    pub element4_shader: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub element4_ray_distance: f32,
    pub element4_size: super::core::Vec2,
    pub element4_size_occluder_curve: super::core::Vec4,
    pub element4_size_screen_pos_curve: super::core::Vec4,
    pub element4_alpha_occluder_curve: super::core::Vec4,
    pub element4_alpha_screen_pos_curve: super::core::Vec4,
    pub element5_enable: bool,
    pub element5_shader: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub element5_ray_distance: f32,
    pub element5_size: super::core::Vec2,
    pub element5_size_occluder_curve: super::core::Vec4,
    pub element5_size_screen_pos_curve: super::core::Vec4,
    pub element5_alpha_occluder_curve: super::core::Vec4,
    pub element5_alpha_screen_pos_curve: super::core::Vec4,
}

pub trait SunFlareEffectComponentStateTrait: super::core::DataContainerTrait {
    fn enable(&self) -> &bool;
    fn debug_draw_occluder(&self) -> &bool;
    fn occluder_size(&self) -> &f32;
    fn element1_enable(&self) -> &bool;
    fn element1_shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn element1_ray_distance(&self) -> &f32;
    fn element1_size(&self) -> &super::core::Vec2;
    fn element1_size_occluder_curve(&self) -> &super::core::Vec4;
    fn element1_size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element1_alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn element1_alpha_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element2_enable(&self) -> &bool;
    fn element2_shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn element2_ray_distance(&self) -> &f32;
    fn element2_size(&self) -> &super::core::Vec2;
    fn element2_size_occluder_curve(&self) -> &super::core::Vec4;
    fn element2_size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element2_alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn element2_alpha_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element3_enable(&self) -> &bool;
    fn element3_shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn element3_ray_distance(&self) -> &f32;
    fn element3_size(&self) -> &super::core::Vec2;
    fn element3_size_occluder_curve(&self) -> &super::core::Vec4;
    fn element3_size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element3_alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn element3_alpha_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element4_enable(&self) -> &bool;
    fn element4_shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn element4_ray_distance(&self) -> &f32;
    fn element4_size(&self) -> &super::core::Vec2;
    fn element4_size_occluder_curve(&self) -> &super::core::Vec4;
    fn element4_size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element4_alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn element4_alpha_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element5_enable(&self) -> &bool;
    fn element5_shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn element5_ray_distance(&self) -> &f32;
    fn element5_size(&self) -> &super::core::Vec2;
    fn element5_size_occluder_curve(&self) -> &super::core::Vec4;
    fn element5_size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element5_alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn element5_alpha_screen_pos_curve(&self) -> &super::core::Vec4;
}

impl SunFlareEffectComponentStateTrait for SunFlareEffectComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn debug_draw_occluder(&self) -> &bool {
        &self.debug_draw_occluder
    }
    fn occluder_size(&self) -> &f32 {
        &self.occluder_size
    }
    fn element1_enable(&self) -> &bool {
        &self.element1_enable
    }
    fn element1_shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.element1_shader
    }
    fn element1_ray_distance(&self) -> &f32 {
        &self.element1_ray_distance
    }
    fn element1_size(&self) -> &super::core::Vec2 {
        &self.element1_size
    }
    fn element1_size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element1_size_occluder_curve
    }
    fn element1_size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element1_size_screen_pos_curve
    }
    fn element1_alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element1_alpha_occluder_curve
    }
    fn element1_alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element1_alpha_screen_pos_curve
    }
    fn element2_enable(&self) -> &bool {
        &self.element2_enable
    }
    fn element2_shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.element2_shader
    }
    fn element2_ray_distance(&self) -> &f32 {
        &self.element2_ray_distance
    }
    fn element2_size(&self) -> &super::core::Vec2 {
        &self.element2_size
    }
    fn element2_size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element2_size_occluder_curve
    }
    fn element2_size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element2_size_screen_pos_curve
    }
    fn element2_alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element2_alpha_occluder_curve
    }
    fn element2_alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element2_alpha_screen_pos_curve
    }
    fn element3_enable(&self) -> &bool {
        &self.element3_enable
    }
    fn element3_shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.element3_shader
    }
    fn element3_ray_distance(&self) -> &f32 {
        &self.element3_ray_distance
    }
    fn element3_size(&self) -> &super::core::Vec2 {
        &self.element3_size
    }
    fn element3_size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element3_size_occluder_curve
    }
    fn element3_size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element3_size_screen_pos_curve
    }
    fn element3_alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element3_alpha_occluder_curve
    }
    fn element3_alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element3_alpha_screen_pos_curve
    }
    fn element4_enable(&self) -> &bool {
        &self.element4_enable
    }
    fn element4_shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.element4_shader
    }
    fn element4_ray_distance(&self) -> &f32 {
        &self.element4_ray_distance
    }
    fn element4_size(&self) -> &super::core::Vec2 {
        &self.element4_size
    }
    fn element4_size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element4_size_occluder_curve
    }
    fn element4_size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element4_size_screen_pos_curve
    }
    fn element4_alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element4_alpha_occluder_curve
    }
    fn element4_alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element4_alpha_screen_pos_curve
    }
    fn element5_enable(&self) -> &bool {
        &self.element5_enable
    }
    fn element5_shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.element5_shader
    }
    fn element5_ray_distance(&self) -> &f32 {
        &self.element5_ray_distance
    }
    fn element5_size(&self) -> &super::core::Vec2 {
        &self.element5_size
    }
    fn element5_size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element5_size_occluder_curve
    }
    fn element5_size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element5_size_screen_pos_curve
    }
    fn element5_alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element5_alpha_occluder_curve
    }
    fn element5_alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element5_alpha_screen_pos_curve
    }
}

impl super::core::DataContainerTrait for SunFlareEffectComponentState {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SUNFLAREEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareEffectComponentState",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SunFlareEffectComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareEffectComponentState, enable),
            },
            FieldInfoData {
                name: "DebugDrawOccluder",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareEffectComponentState, debug_draw_occluder),
            },
            FieldInfoData {
                name: "OccluderSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareEffectComponentState, occluder_size),
            },
            FieldInfoData {
                name: "Element1Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_enable),
            },
            FieldInfoData {
                name: "Element1Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_shader),
            },
            FieldInfoData {
                name: "Element1RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_ray_distance),
            },
            FieldInfoData {
                name: "Element1Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_size),
            },
            FieldInfoData {
                name: "Element1SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element1SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element1AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element1AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element2Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_enable),
            },
            FieldInfoData {
                name: "Element2Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_shader),
            },
            FieldInfoData {
                name: "Element2RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_ray_distance),
            },
            FieldInfoData {
                name: "Element2Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_size),
            },
            FieldInfoData {
                name: "Element2SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element2SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element2AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element2AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element3Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_enable),
            },
            FieldInfoData {
                name: "Element3Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_shader),
            },
            FieldInfoData {
                name: "Element3RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_ray_distance),
            },
            FieldInfoData {
                name: "Element3Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_size),
            },
            FieldInfoData {
                name: "Element3SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element3SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element3AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element3AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element4Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_enable),
            },
            FieldInfoData {
                name: "Element4Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_shader),
            },
            FieldInfoData {
                name: "Element4RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_ray_distance),
            },
            FieldInfoData {
                name: "Element4Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_size),
            },
            FieldInfoData {
                name: "Element4SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element4SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element4AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element4AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element5Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_enable),
            },
            FieldInfoData {
                name: "Element5Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_shader),
            },
            FieldInfoData {
                name: "Element5RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_ray_distance),
            },
            FieldInfoData {
                name: "Element5Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_size),
            },
            FieldInfoData {
                name: "Element5SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element5SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element5AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element5AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_alpha_screen_pos_curve),
            },
        ],
    }),
    array_type: Some(SUNFLAREEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SunFlareEffectComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SUNFLAREEFFECTCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SUNFLAREEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SunFlareEffectComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SkyEffectComponentState {
    pub _glacier_base: super::core::DataContainer,
    pub enable: bool,
    pub sun_size: f32,
    pub sun_scale: f32,
    pub sky_gradient_scale: f32,
    pub sky_gradient_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub panoramic_u_v_min_x: f32,
    pub panoramic_u_v_max_x: f32,
    pub panoramic_u_v_min_y: f32,
    pub panoramic_u_v_max_y: f32,
    pub panoramic_tile_factor: f32,
    pub panoramic_rotation: f32,
    pub panoramic_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub panoramic_alpha_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub cloud_layer_sun_color: super::core::Vec3,
    pub cloud_layer_mask_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub cloud_layer1: SkyCloudLayer,
    pub cloud_layer2: SkyCloudLayer,
    pub static_envmap_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub wind_direction: f32,
}

pub trait SkyEffectComponentStateTrait: super::core::DataContainerTrait {
    fn enable(&self) -> &bool;
    fn sun_size(&self) -> &f32;
    fn sun_scale(&self) -> &f32;
    fn sky_gradient_scale(&self) -> &f32;
    fn sky_gradient_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn panoramic_u_v_min_x(&self) -> &f32;
    fn panoramic_u_v_max_x(&self) -> &f32;
    fn panoramic_u_v_min_y(&self) -> &f32;
    fn panoramic_u_v_max_y(&self) -> &f32;
    fn panoramic_tile_factor(&self) -> &f32;
    fn panoramic_rotation(&self) -> &f32;
    fn panoramic_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn panoramic_alpha_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn cloud_layer_sun_color(&self) -> &super::core::Vec3;
    fn cloud_layer_mask_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn cloud_layer1(&self) -> &SkyCloudLayer;
    fn cloud_layer2(&self) -> &SkyCloudLayer;
    fn static_envmap_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn wind_direction(&self) -> &f32;
}

impl SkyEffectComponentStateTrait for SkyEffectComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn sun_size(&self) -> &f32 {
        &self.sun_size
    }
    fn sun_scale(&self) -> &f32 {
        &self.sun_scale
    }
    fn sky_gradient_scale(&self) -> &f32 {
        &self.sky_gradient_scale
    }
    fn sky_gradient_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.sky_gradient_texture
    }
    fn panoramic_u_v_min_x(&self) -> &f32 {
        &self.panoramic_u_v_min_x
    }
    fn panoramic_u_v_max_x(&self) -> &f32 {
        &self.panoramic_u_v_max_x
    }
    fn panoramic_u_v_min_y(&self) -> &f32 {
        &self.panoramic_u_v_min_y
    }
    fn panoramic_u_v_max_y(&self) -> &f32 {
        &self.panoramic_u_v_max_y
    }
    fn panoramic_tile_factor(&self) -> &f32 {
        &self.panoramic_tile_factor
    }
    fn panoramic_rotation(&self) -> &f32 {
        &self.panoramic_rotation
    }
    fn panoramic_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.panoramic_texture
    }
    fn panoramic_alpha_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.panoramic_alpha_texture
    }
    fn cloud_layer_sun_color(&self) -> &super::core::Vec3 {
        &self.cloud_layer_sun_color
    }
    fn cloud_layer_mask_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.cloud_layer_mask_texture
    }
    fn cloud_layer1(&self) -> &SkyCloudLayer {
        &self.cloud_layer1
    }
    fn cloud_layer2(&self) -> &SkyCloudLayer {
        &self.cloud_layer2
    }
    fn static_envmap_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.static_envmap_texture
    }
    fn wind_direction(&self) -> &f32 {
        &self.wind_direction
    }
}

impl super::core::DataContainerTrait for SkyEffectComponentState {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SKYEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyEffectComponentState",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SkyEffectComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyEffectComponentState, enable),
            },
            FieldInfoData {
                name: "SunSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyEffectComponentState, sun_size),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyEffectComponentState, sun_scale),
            },
            FieldInfoData {
                name: "SkyGradientScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyEffectComponentState, sky_gradient_scale),
            },
            FieldInfoData {
                name: "SkyGradientTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyEffectComponentState, sky_gradient_texture),
            },
            FieldInfoData {
                name: "PanoramicUVMinX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_u_v_min_x),
            },
            FieldInfoData {
                name: "PanoramicUVMaxX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_u_v_max_x),
            },
            FieldInfoData {
                name: "PanoramicUVMinY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_u_v_min_y),
            },
            FieldInfoData {
                name: "PanoramicUVMaxY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_u_v_max_y),
            },
            FieldInfoData {
                name: "PanoramicTileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_tile_factor),
            },
            FieldInfoData {
                name: "PanoramicRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_rotation),
            },
            FieldInfoData {
                name: "PanoramicTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_texture),
            },
            FieldInfoData {
                name: "PanoramicAlphaTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_alpha_texture),
            },
            FieldInfoData {
                name: "CloudLayerSunColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyEffectComponentState, cloud_layer_sun_color),
            },
            FieldInfoData {
                name: "CloudLayerMaskTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyEffectComponentState, cloud_layer_mask_texture),
            },
            FieldInfoData {
                name: "CloudLayer1",
                flags: MemberInfoFlags::new(0),
                field_type: "SkyCloudLayer",
                rust_offset: offset_of!(SkyEffectComponentState, cloud_layer1),
            },
            FieldInfoData {
                name: "CloudLayer2",
                flags: MemberInfoFlags::new(0),
                field_type: "SkyCloudLayer",
                rust_offset: offset_of!(SkyEffectComponentState, cloud_layer2),
            },
            FieldInfoData {
                name: "StaticEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyEffectComponentState, static_envmap_texture),
            },
            FieldInfoData {
                name: "WindDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyEffectComponentState, wind_direction),
            },
        ],
    }),
    array_type: Some(SKYEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyEffectComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SKYEFFECTCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SKYEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyEffectComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SkyCloudLayer {
    pub altitude: f32,
    pub tile_factor: f32,
    pub rotation: f32,
    pub speed: f32,
    pub sun_light_intensity: f32,
    pub sun_light_power: f32,
    pub ambient_light_intensity: f32,
    pub color: super::core::Vec3,
    pub alpha_mul: f32,
    pub texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
}

pub trait SkyCloudLayerTrait: TypeObject {
    fn altitude(&self) -> &f32;
    fn tile_factor(&self) -> &f32;
    fn rotation(&self) -> &f32;
    fn speed(&self) -> &f32;
    fn sun_light_intensity(&self) -> &f32;
    fn sun_light_power(&self) -> &f32;
    fn ambient_light_intensity(&self) -> &f32;
    fn color(&self) -> &super::core::Vec3;
    fn alpha_mul(&self) -> &f32;
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
}

impl SkyCloudLayerTrait for SkyCloudLayer {
    fn altitude(&self) -> &f32 {
        &self.altitude
    }
    fn tile_factor(&self) -> &f32 {
        &self.tile_factor
    }
    fn rotation(&self) -> &f32 {
        &self.rotation
    }
    fn speed(&self) -> &f32 {
        &self.speed
    }
    fn sun_light_intensity(&self) -> &f32 {
        &self.sun_light_intensity
    }
    fn sun_light_power(&self) -> &f32 {
        &self.sun_light_power
    }
    fn ambient_light_intensity(&self) -> &f32 {
        &self.ambient_light_intensity
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn alpha_mul(&self) -> &f32 {
        &self.alpha_mul
    }
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture
    }
}

pub static SKYCLOUDLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCloudLayer",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SkyCloudLayer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Altitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCloudLayer, altitude),
            },
            FieldInfoData {
                name: "TileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCloudLayer, tile_factor),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCloudLayer, rotation),
            },
            FieldInfoData {
                name: "Speed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCloudLayer, speed),
            },
            FieldInfoData {
                name: "SunLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCloudLayer, sun_light_intensity),
            },
            FieldInfoData {
                name: "SunLightPower",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCloudLayer, sun_light_power),
            },
            FieldInfoData {
                name: "AmbientLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCloudLayer, ambient_light_intensity),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyCloudLayer, color),
            },
            FieldInfoData {
                name: "AlphaMul",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCloudLayer, alpha_mul),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyCloudLayer, texture),
            },
        ],
    }),
    array_type: Some(SKYCLOUDLAYER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCloudLayer {
    fn type_info(&self) -> &'static TypeInfo {
        SKYCLOUDLAYER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SKYCLOUDLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCloudLayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyCloudLayer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FogEffectComponentState {
    pub _glacier_base: super::core::DataContainer,
    pub enable: bool,
    pub start: f32,
    pub end: f32,
    pub curve: super::core::Vec4,
    pub fog_color: super::core::Vec3,
    pub fog_color_start: f32,
    pub fog_color_end: f32,
    pub fog_color_curve: super::core::Vec4,
    pub transparency_fade_start: f32,
    pub transparency_fade_end: f32,
    pub transparency_fade_clamp: f32,
    pub forward_light_scattering_color: super::core::Vec3,
    pub forward_light_scattering_enabled: bool,
    pub forward_light_scattering_phase_g: f32,
    pub forward_light_scattering_strength: f32,
    pub forward_light_scattering_presence: f32,
    pub forward_light_scattering_max_blur_length: f32,
    pub forward_light_scattering_extinction: f32,
    pub forward_light_scattering_smoothness: f32,
    pub height_fog_enable: bool,
    pub height_fog_follow_camera: f32,
    pub height_fog_altitude: f32,
    pub height_fog_depth: f32,
    pub height_fog_visibility_range: f32,
}

pub trait FogEffectComponentStateTrait: super::core::DataContainerTrait {
    fn enable(&self) -> &bool;
    fn start(&self) -> &f32;
    fn end(&self) -> &f32;
    fn curve(&self) -> &super::core::Vec4;
    fn fog_color(&self) -> &super::core::Vec3;
    fn fog_color_start(&self) -> &f32;
    fn fog_color_end(&self) -> &f32;
    fn fog_color_curve(&self) -> &super::core::Vec4;
    fn transparency_fade_start(&self) -> &f32;
    fn transparency_fade_end(&self) -> &f32;
    fn transparency_fade_clamp(&self) -> &f32;
    fn forward_light_scattering_color(&self) -> &super::core::Vec3;
    fn forward_light_scattering_enabled(&self) -> &bool;
    fn forward_light_scattering_phase_g(&self) -> &f32;
    fn forward_light_scattering_strength(&self) -> &f32;
    fn forward_light_scattering_presence(&self) -> &f32;
    fn forward_light_scattering_max_blur_length(&self) -> &f32;
    fn forward_light_scattering_extinction(&self) -> &f32;
    fn forward_light_scattering_smoothness(&self) -> &f32;
    fn height_fog_enable(&self) -> &bool;
    fn height_fog_follow_camera(&self) -> &f32;
    fn height_fog_altitude(&self) -> &f32;
    fn height_fog_depth(&self) -> &f32;
    fn height_fog_visibility_range(&self) -> &f32;
}

impl FogEffectComponentStateTrait for FogEffectComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn start(&self) -> &f32 {
        &self.start
    }
    fn end(&self) -> &f32 {
        &self.end
    }
    fn curve(&self) -> &super::core::Vec4 {
        &self.curve
    }
    fn fog_color(&self) -> &super::core::Vec3 {
        &self.fog_color
    }
    fn fog_color_start(&self) -> &f32 {
        &self.fog_color_start
    }
    fn fog_color_end(&self) -> &f32 {
        &self.fog_color_end
    }
    fn fog_color_curve(&self) -> &super::core::Vec4 {
        &self.fog_color_curve
    }
    fn transparency_fade_start(&self) -> &f32 {
        &self.transparency_fade_start
    }
    fn transparency_fade_end(&self) -> &f32 {
        &self.transparency_fade_end
    }
    fn transparency_fade_clamp(&self) -> &f32 {
        &self.transparency_fade_clamp
    }
    fn forward_light_scattering_color(&self) -> &super::core::Vec3 {
        &self.forward_light_scattering_color
    }
    fn forward_light_scattering_enabled(&self) -> &bool {
        &self.forward_light_scattering_enabled
    }
    fn forward_light_scattering_phase_g(&self) -> &f32 {
        &self.forward_light_scattering_phase_g
    }
    fn forward_light_scattering_strength(&self) -> &f32 {
        &self.forward_light_scattering_strength
    }
    fn forward_light_scattering_presence(&self) -> &f32 {
        &self.forward_light_scattering_presence
    }
    fn forward_light_scattering_max_blur_length(&self) -> &f32 {
        &self.forward_light_scattering_max_blur_length
    }
    fn forward_light_scattering_extinction(&self) -> &f32 {
        &self.forward_light_scattering_extinction
    }
    fn forward_light_scattering_smoothness(&self) -> &f32 {
        &self.forward_light_scattering_smoothness
    }
    fn height_fog_enable(&self) -> &bool {
        &self.height_fog_enable
    }
    fn height_fog_follow_camera(&self) -> &f32 {
        &self.height_fog_follow_camera
    }
    fn height_fog_altitude(&self) -> &f32 {
        &self.height_fog_altitude
    }
    fn height_fog_depth(&self) -> &f32 {
        &self.height_fog_depth
    }
    fn height_fog_visibility_range(&self) -> &f32 {
        &self.height_fog_visibility_range
    }
}

impl super::core::DataContainerTrait for FogEffectComponentState {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FOGEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogEffectComponentState",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FogEffectComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogEffectComponentState, enable),
            },
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, start),
            },
            FieldInfoData {
                name: "End",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, end),
            },
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(FogEffectComponentState, curve),
            },
            FieldInfoData {
                name: "FogColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FogEffectComponentState, fog_color),
            },
            FieldInfoData {
                name: "FogColorStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, fog_color_start),
            },
            FieldInfoData {
                name: "FogColorEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, fog_color_end),
            },
            FieldInfoData {
                name: "FogColorCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(FogEffectComponentState, fog_color_curve),
            },
            FieldInfoData {
                name: "TransparencyFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, transparency_fade_start),
            },
            FieldInfoData {
                name: "TransparencyFadeEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, transparency_fade_end),
            },
            FieldInfoData {
                name: "TransparencyFadeClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, transparency_fade_clamp),
            },
            FieldInfoData {
                name: "ForwardLightScatteringColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_color),
            },
            FieldInfoData {
                name: "ForwardLightScatteringEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_enabled),
            },
            FieldInfoData {
                name: "ForwardLightScatteringPhaseG",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_phase_g),
            },
            FieldInfoData {
                name: "ForwardLightScatteringStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_strength),
            },
            FieldInfoData {
                name: "ForwardLightScatteringPresence",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_presence),
            },
            FieldInfoData {
                name: "ForwardLightScatteringMaxBlurLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_max_blur_length),
            },
            FieldInfoData {
                name: "ForwardLightScatteringExtinction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_extinction),
            },
            FieldInfoData {
                name: "ForwardLightScatteringSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_smoothness),
            },
            FieldInfoData {
                name: "HeightFogEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogEffectComponentState, height_fog_enable),
            },
            FieldInfoData {
                name: "HeightFogFollowCamera",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, height_fog_follow_camera),
            },
            FieldInfoData {
                name: "HeightFogAltitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, height_fog_altitude),
            },
            FieldInfoData {
                name: "HeightFogDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, height_fog_depth),
            },
            FieldInfoData {
                name: "HeightFogVisibilityRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogEffectComponentState, height_fog_visibility_range),
            },
        ],
    }),
    array_type: Some(FOGEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FogEffectComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        FOGEFFECTCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FOGEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FogEffectComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OutdoorLightEffectComponentState {
    pub _glacier_base: super::core::DataContainer,
    pub enable: bool,
    pub sun_rotation_x: f32,
    pub sun_rotation_y: f32,
    pub sun_color: super::core::Vec3,
    pub sky_color: super::core::Vec3,
    pub ground_color: super::core::Vec3,
    pub sky_light_angle_factor: f32,
    pub sun_shadow_height_scale: f32,
    pub cloud_shadow_enable: bool,
    pub cloud_shadow_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub cloud_shadow_speed: super::core::Vec2,
    pub cloud_shadow_size: f32,
    pub cloud_shadow_coverage: f32,
    pub cloud_shadow_exponent: f32,
}

pub trait OutdoorLightEffectComponentStateTrait: super::core::DataContainerTrait {
    fn enable(&self) -> &bool;
    fn sun_rotation_x(&self) -> &f32;
    fn sun_rotation_y(&self) -> &f32;
    fn sun_color(&self) -> &super::core::Vec3;
    fn sky_color(&self) -> &super::core::Vec3;
    fn ground_color(&self) -> &super::core::Vec3;
    fn sky_light_angle_factor(&self) -> &f32;
    fn sun_shadow_height_scale(&self) -> &f32;
    fn cloud_shadow_enable(&self) -> &bool;
    fn cloud_shadow_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn cloud_shadow_speed(&self) -> &super::core::Vec2;
    fn cloud_shadow_size(&self) -> &f32;
    fn cloud_shadow_coverage(&self) -> &f32;
    fn cloud_shadow_exponent(&self) -> &f32;
}

impl OutdoorLightEffectComponentStateTrait for OutdoorLightEffectComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn sun_rotation_x(&self) -> &f32 {
        &self.sun_rotation_x
    }
    fn sun_rotation_y(&self) -> &f32 {
        &self.sun_rotation_y
    }
    fn sun_color(&self) -> &super::core::Vec3 {
        &self.sun_color
    }
    fn sky_color(&self) -> &super::core::Vec3 {
        &self.sky_color
    }
    fn ground_color(&self) -> &super::core::Vec3 {
        &self.ground_color
    }
    fn sky_light_angle_factor(&self) -> &f32 {
        &self.sky_light_angle_factor
    }
    fn sun_shadow_height_scale(&self) -> &f32 {
        &self.sun_shadow_height_scale
    }
    fn cloud_shadow_enable(&self) -> &bool {
        &self.cloud_shadow_enable
    }
    fn cloud_shadow_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.cloud_shadow_texture
    }
    fn cloud_shadow_speed(&self) -> &super::core::Vec2 {
        &self.cloud_shadow_speed
    }
    fn cloud_shadow_size(&self) -> &f32 {
        &self.cloud_shadow_size
    }
    fn cloud_shadow_coverage(&self) -> &f32 {
        &self.cloud_shadow_coverage
    }
    fn cloud_shadow_exponent(&self) -> &f32 {
        &self.cloud_shadow_exponent
    }
}

impl super::core::DataContainerTrait for OutdoorLightEffectComponentState {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static OUTDOORLIGHTEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightEffectComponentState",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OutdoorLightEffectComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, enable),
            },
            FieldInfoData {
                name: "SunRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sun_rotation_x),
            },
            FieldInfoData {
                name: "SunRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sun_rotation_y),
            },
            FieldInfoData {
                name: "SunColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sun_color),
            },
            FieldInfoData {
                name: "SkyColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sky_color),
            },
            FieldInfoData {
                name: "GroundColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, ground_color),
            },
            FieldInfoData {
                name: "SkyLightAngleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sky_light_angle_factor),
            },
            FieldInfoData {
                name: "SunShadowHeightScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sun_shadow_height_scale),
            },
            FieldInfoData {
                name: "CloudShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_enable),
            },
            FieldInfoData {
                name: "CloudShadowTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_texture),
            },
            FieldInfoData {
                name: "CloudShadowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_speed),
            },
            FieldInfoData {
                name: "CloudShadowSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_size),
            },
            FieldInfoData {
                name: "CloudShadowCoverage",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_coverage),
            },
            FieldInfoData {
                name: "CloudShadowExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_exponent),
            },
        ],
    }),
    array_type: Some(OUTDOORLIGHTEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OutdoorLightEffectComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        OUTDOORLIGHTEFFECTCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OUTDOORLIGHTEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OutdoorLightEffectComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SonarParamsComponentState {
    pub enable: bool,
    pub color1: super::core::Vec3,
    pub color2: super::core::Vec3,
    pub color3: super::core::Vec3,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait SonarParamsComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn color1(&self) -> &super::core::Vec3;
    fn color2(&self) -> &super::core::Vec3;
    fn color3(&self) -> &super::core::Vec3;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl SonarParamsComponentStateTrait for SonarParamsComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn color1(&self) -> &super::core::Vec3 {
        &self.color1
    }
    fn color2(&self) -> &super::core::Vec3 {
        &self.color2
    }
    fn color3(&self) -> &super::core::Vec3 {
        &self.color3
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static SONARPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SonarParamsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SonarParamsComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SonarParamsComponentState, enable),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SonarParamsComponentState, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SonarParamsComponentState, color2),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SonarParamsComponentState, color3),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SonarParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SonarParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SONARPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SonarParamsComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SONARPARAMSCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SONARPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SonarParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SonarParamsComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct HologramParamsComponentState {
    pub enable: bool,
    pub render_mode: HologramRenderMode,
    pub key_illuminance: super::core::Vec3,
    pub key_light_dir: super::core::Vec3,
    pub resolution_scale: f32,
    pub color1: super::core::Vec3,
    pub color2: super::core::Vec3,
    pub color3: super::core::Vec3,
    pub color4: super::core::Vec3,
    pub color5: super::core::Vec3,
    pub float1: f32,
    pub float2: f32,
    pub float3: f32,
    pub float4: f32,
    pub float5: f32,
    pub float6: f32,
    pub float7: f32,
    pub float8: f32,
    pub float9: f32,
    pub brightness2: f32,
    pub brightness3: f32,
    pub brightness4: f32,
    pub brightness5: f32,
    pub streaks_enabled: bool,
    pub time_offset: f32,
    pub opacity1: f32,
    pub distortion_scale1: f32,
    pub source_pos1: super::core::Vec3,
    pub target_pos1: super::core::Vec3,
    pub source_radius1: f32,
    pub target_radius1: f32,
    pub opacity2: f32,
    pub distortion_scale2: f32,
    pub source_pos2: super::core::Vec3,
    pub target_pos2: super::core::Vec3,
    pub source_radius2: f32,
    pub target_radius2: f32,
    pub opacity3: f32,
    pub distortion_scale3: f32,
    pub source_pos3: super::core::Vec3,
    pub target_pos3: super::core::Vec3,
    pub source_radius3: f32,
    pub target_radius3: f32,
    pub opacity4: f32,
    pub distortion_scale4: f32,
    pub source_pos4: super::core::Vec3,
    pub target_pos4: super::core::Vec3,
    pub source_radius4: f32,
    pub target_radius4: f32,
    pub opacity5: f32,
    pub distortion_scale5: f32,
    pub source_pos5: super::core::Vec3,
    pub target_pos5: super::core::Vec3,
    pub source_radius5: f32,
    pub target_radius5: f32,
    pub field_flag_override0: u32,
    pub field_flag_override1: u32,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u32,
}

pub trait HologramParamsComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn render_mode(&self) -> &HologramRenderMode;
    fn key_illuminance(&self) -> &super::core::Vec3;
    fn key_light_dir(&self) -> &super::core::Vec3;
    fn resolution_scale(&self) -> &f32;
    fn color1(&self) -> &super::core::Vec3;
    fn color2(&self) -> &super::core::Vec3;
    fn color3(&self) -> &super::core::Vec3;
    fn color4(&self) -> &super::core::Vec3;
    fn color5(&self) -> &super::core::Vec3;
    fn float1(&self) -> &f32;
    fn float2(&self) -> &f32;
    fn float3(&self) -> &f32;
    fn float4(&self) -> &f32;
    fn float5(&self) -> &f32;
    fn float6(&self) -> &f32;
    fn float7(&self) -> &f32;
    fn float8(&self) -> &f32;
    fn float9(&self) -> &f32;
    fn brightness2(&self) -> &f32;
    fn brightness3(&self) -> &f32;
    fn brightness4(&self) -> &f32;
    fn brightness5(&self) -> &f32;
    fn streaks_enabled(&self) -> &bool;
    fn time_offset(&self) -> &f32;
    fn opacity1(&self) -> &f32;
    fn distortion_scale1(&self) -> &f32;
    fn source_pos1(&self) -> &super::core::Vec3;
    fn target_pos1(&self) -> &super::core::Vec3;
    fn source_radius1(&self) -> &f32;
    fn target_radius1(&self) -> &f32;
    fn opacity2(&self) -> &f32;
    fn distortion_scale2(&self) -> &f32;
    fn source_pos2(&self) -> &super::core::Vec3;
    fn target_pos2(&self) -> &super::core::Vec3;
    fn source_radius2(&self) -> &f32;
    fn target_radius2(&self) -> &f32;
    fn opacity3(&self) -> &f32;
    fn distortion_scale3(&self) -> &f32;
    fn source_pos3(&self) -> &super::core::Vec3;
    fn target_pos3(&self) -> &super::core::Vec3;
    fn source_radius3(&self) -> &f32;
    fn target_radius3(&self) -> &f32;
    fn opacity4(&self) -> &f32;
    fn distortion_scale4(&self) -> &f32;
    fn source_pos4(&self) -> &super::core::Vec3;
    fn target_pos4(&self) -> &super::core::Vec3;
    fn source_radius4(&self) -> &f32;
    fn target_radius4(&self) -> &f32;
    fn opacity5(&self) -> &f32;
    fn distortion_scale5(&self) -> &f32;
    fn source_pos5(&self) -> &super::core::Vec3;
    fn target_pos5(&self) -> &super::core::Vec3;
    fn source_radius5(&self) -> &f32;
    fn target_radius5(&self) -> &f32;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_override1(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u32;
}

impl HologramParamsComponentStateTrait for HologramParamsComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn render_mode(&self) -> &HologramRenderMode {
        &self.render_mode
    }
    fn key_illuminance(&self) -> &super::core::Vec3 {
        &self.key_illuminance
    }
    fn key_light_dir(&self) -> &super::core::Vec3 {
        &self.key_light_dir
    }
    fn resolution_scale(&self) -> &f32 {
        &self.resolution_scale
    }
    fn color1(&self) -> &super::core::Vec3 {
        &self.color1
    }
    fn color2(&self) -> &super::core::Vec3 {
        &self.color2
    }
    fn color3(&self) -> &super::core::Vec3 {
        &self.color3
    }
    fn color4(&self) -> &super::core::Vec3 {
        &self.color4
    }
    fn color5(&self) -> &super::core::Vec3 {
        &self.color5
    }
    fn float1(&self) -> &f32 {
        &self.float1
    }
    fn float2(&self) -> &f32 {
        &self.float2
    }
    fn float3(&self) -> &f32 {
        &self.float3
    }
    fn float4(&self) -> &f32 {
        &self.float4
    }
    fn float5(&self) -> &f32 {
        &self.float5
    }
    fn float6(&self) -> &f32 {
        &self.float6
    }
    fn float7(&self) -> &f32 {
        &self.float7
    }
    fn float8(&self) -> &f32 {
        &self.float8
    }
    fn float9(&self) -> &f32 {
        &self.float9
    }
    fn brightness2(&self) -> &f32 {
        &self.brightness2
    }
    fn brightness3(&self) -> &f32 {
        &self.brightness3
    }
    fn brightness4(&self) -> &f32 {
        &self.brightness4
    }
    fn brightness5(&self) -> &f32 {
        &self.brightness5
    }
    fn streaks_enabled(&self) -> &bool {
        &self.streaks_enabled
    }
    fn time_offset(&self) -> &f32 {
        &self.time_offset
    }
    fn opacity1(&self) -> &f32 {
        &self.opacity1
    }
    fn distortion_scale1(&self) -> &f32 {
        &self.distortion_scale1
    }
    fn source_pos1(&self) -> &super::core::Vec3 {
        &self.source_pos1
    }
    fn target_pos1(&self) -> &super::core::Vec3 {
        &self.target_pos1
    }
    fn source_radius1(&self) -> &f32 {
        &self.source_radius1
    }
    fn target_radius1(&self) -> &f32 {
        &self.target_radius1
    }
    fn opacity2(&self) -> &f32 {
        &self.opacity2
    }
    fn distortion_scale2(&self) -> &f32 {
        &self.distortion_scale2
    }
    fn source_pos2(&self) -> &super::core::Vec3 {
        &self.source_pos2
    }
    fn target_pos2(&self) -> &super::core::Vec3 {
        &self.target_pos2
    }
    fn source_radius2(&self) -> &f32 {
        &self.source_radius2
    }
    fn target_radius2(&self) -> &f32 {
        &self.target_radius2
    }
    fn opacity3(&self) -> &f32 {
        &self.opacity3
    }
    fn distortion_scale3(&self) -> &f32 {
        &self.distortion_scale3
    }
    fn source_pos3(&self) -> &super::core::Vec3 {
        &self.source_pos3
    }
    fn target_pos3(&self) -> &super::core::Vec3 {
        &self.target_pos3
    }
    fn source_radius3(&self) -> &f32 {
        &self.source_radius3
    }
    fn target_radius3(&self) -> &f32 {
        &self.target_radius3
    }
    fn opacity4(&self) -> &f32 {
        &self.opacity4
    }
    fn distortion_scale4(&self) -> &f32 {
        &self.distortion_scale4
    }
    fn source_pos4(&self) -> &super::core::Vec3 {
        &self.source_pos4
    }
    fn target_pos4(&self) -> &super::core::Vec3 {
        &self.target_pos4
    }
    fn source_radius4(&self) -> &f32 {
        &self.source_radius4
    }
    fn target_radius4(&self) -> &f32 {
        &self.target_radius4
    }
    fn opacity5(&self) -> &f32 {
        &self.opacity5
    }
    fn distortion_scale5(&self) -> &f32 {
        &self.distortion_scale5
    }
    fn source_pos5(&self) -> &super::core::Vec3 {
        &self.source_pos5
    }
    fn target_pos5(&self) -> &super::core::Vec3 {
        &self.target_pos5
    }
    fn source_radius5(&self) -> &f32 {
        &self.source_radius5
    }
    fn target_radius5(&self) -> &f32 {
        &self.target_radius5
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_override1(&self) -> &u32 {
        &self.field_flag_override1
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u32 {
        &self.field_flag_changed1
    }
}

pub static HOLOGRAMPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramParamsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HologramParamsComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HologramParamsComponentState, enable),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: "HologramRenderMode",
                rust_offset: offset_of!(HologramParamsComponentState, render_mode),
            },
            FieldInfoData {
                name: "KeyIlluminance",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, key_illuminance),
            },
            FieldInfoData {
                name: "KeyLightDir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, key_light_dir),
            },
            FieldInfoData {
                name: "ResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, resolution_scale),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, color2),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, color3),
            },
            FieldInfoData {
                name: "Color4",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, color4),
            },
            FieldInfoData {
                name: "Color5",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, color5),
            },
            FieldInfoData {
                name: "Float1",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, float1),
            },
            FieldInfoData {
                name: "Float2",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, float2),
            },
            FieldInfoData {
                name: "Float3",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, float3),
            },
            FieldInfoData {
                name: "Float4",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, float4),
            },
            FieldInfoData {
                name: "Float5",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, float5),
            },
            FieldInfoData {
                name: "Float6",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, float6),
            },
            FieldInfoData {
                name: "Float7",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, float7),
            },
            FieldInfoData {
                name: "Float8",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, float8),
            },
            FieldInfoData {
                name: "Float9",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, float9),
            },
            FieldInfoData {
                name: "Brightness2",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, brightness2),
            },
            FieldInfoData {
                name: "Brightness3",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, brightness3),
            },
            FieldInfoData {
                name: "Brightness4",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, brightness4),
            },
            FieldInfoData {
                name: "Brightness5",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, brightness5),
            },
            FieldInfoData {
                name: "StreaksEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HologramParamsComponentState, streaks_enabled),
            },
            FieldInfoData {
                name: "TimeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, time_offset),
            },
            FieldInfoData {
                name: "Opacity1",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, opacity1),
            },
            FieldInfoData {
                name: "DistortionScale1",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, distortion_scale1),
            },
            FieldInfoData {
                name: "SourcePos1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, source_pos1),
            },
            FieldInfoData {
                name: "TargetPos1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, target_pos1),
            },
            FieldInfoData {
                name: "SourceRadius1",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, source_radius1),
            },
            FieldInfoData {
                name: "TargetRadius1",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, target_radius1),
            },
            FieldInfoData {
                name: "Opacity2",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, opacity2),
            },
            FieldInfoData {
                name: "DistortionScale2",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, distortion_scale2),
            },
            FieldInfoData {
                name: "SourcePos2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, source_pos2),
            },
            FieldInfoData {
                name: "TargetPos2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, target_pos2),
            },
            FieldInfoData {
                name: "SourceRadius2",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, source_radius2),
            },
            FieldInfoData {
                name: "TargetRadius2",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, target_radius2),
            },
            FieldInfoData {
                name: "Opacity3",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, opacity3),
            },
            FieldInfoData {
                name: "DistortionScale3",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, distortion_scale3),
            },
            FieldInfoData {
                name: "SourcePos3",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, source_pos3),
            },
            FieldInfoData {
                name: "TargetPos3",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, target_pos3),
            },
            FieldInfoData {
                name: "SourceRadius3",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, source_radius3),
            },
            FieldInfoData {
                name: "TargetRadius3",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, target_radius3),
            },
            FieldInfoData {
                name: "Opacity4",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, opacity4),
            },
            FieldInfoData {
                name: "DistortionScale4",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, distortion_scale4),
            },
            FieldInfoData {
                name: "SourcePos4",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, source_pos4),
            },
            FieldInfoData {
                name: "TargetPos4",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, target_pos4),
            },
            FieldInfoData {
                name: "SourceRadius4",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, source_radius4),
            },
            FieldInfoData {
                name: "TargetRadius4",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, target_radius4),
            },
            FieldInfoData {
                name: "Opacity5",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, opacity5),
            },
            FieldInfoData {
                name: "DistortionScale5",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, distortion_scale5),
            },
            FieldInfoData {
                name: "SourcePos5",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, source_pos5),
            },
            FieldInfoData {
                name: "TargetPos5",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HologramParamsComponentState, target_pos5),
            },
            FieldInfoData {
                name: "SourceRadius5",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, source_radius5),
            },
            FieldInfoData {
                name: "TargetRadius5",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HologramParamsComponentState, target_radius5),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(HologramParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(HologramParamsComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(HologramParamsComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(HologramParamsComponentState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(HOLOGRAMPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HologramParamsComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        HOLOGRAMPARAMSCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HOLOGRAMPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("HologramParamsComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum HologramPlaneCount {
    #[default]
    HologramPlaneCount_Max = 5,
}

pub static HOLOGRAMPLANECOUNT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramPlaneCount",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(HOLOGRAMPLANECOUNT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for HologramPlaneCount {
    fn type_info(&self) -> &'static TypeInfo {
        HOLOGRAMPLANECOUNT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HOLOGRAMPLANECOUNT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramPlaneCount-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("HologramPlaneCount"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum HologramRenderMode {
    #[default]
    HologramRenderMode_ForwardSimple = 0,
    HologramRenderMode_Forward = 1,
}

pub static HOLOGRAMRENDERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramRenderMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(HOLOGRAMRENDERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for HologramRenderMode {
    fn type_info(&self) -> &'static TypeInfo {
        HOLOGRAMRENDERMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HOLOGRAMRENDERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramRenderMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("HologramRenderMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ThreatAlertHighlightParamsComponentState {
    pub enable: bool,
    pub color1: super::core::Vec3,
    pub color2: super::core::Vec3,
    pub color3: super::core::Vec3,
    pub color4: super::core::Vec3,
    pub color5: super::core::Vec3,
    pub use_outline: bool,
    pub outline_opacity: f32,
    pub use_scan_lines: bool,
    pub scan_line_offset: i32,
    pub scan_line_opacity: f32,
    pub scan_line_thickness: i32,
    pub scan_line_spacing: i32,
    pub use_horizontal_scan_lines: bool,
    pub use_alt_lines: bool,
    pub alt_line_offset: i32,
    pub alt_line_opacity: f32,
    pub alt_line_thickness: i32,
    pub alt_line_spacing: i32,
    pub use_horizontal_alt_lines: bool,
    pub field_flag_override0: u32,
    pub field_flag_changed0: u32,
}

pub trait ThreatAlertHighlightParamsComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn color1(&self) -> &super::core::Vec3;
    fn color2(&self) -> &super::core::Vec3;
    fn color3(&self) -> &super::core::Vec3;
    fn color4(&self) -> &super::core::Vec3;
    fn color5(&self) -> &super::core::Vec3;
    fn use_outline(&self) -> &bool;
    fn outline_opacity(&self) -> &f32;
    fn use_scan_lines(&self) -> &bool;
    fn scan_line_offset(&self) -> &i32;
    fn scan_line_opacity(&self) -> &f32;
    fn scan_line_thickness(&self) -> &i32;
    fn scan_line_spacing(&self) -> &i32;
    fn use_horizontal_scan_lines(&self) -> &bool;
    fn use_alt_lines(&self) -> &bool;
    fn alt_line_offset(&self) -> &i32;
    fn alt_line_opacity(&self) -> &f32;
    fn alt_line_thickness(&self) -> &i32;
    fn alt_line_spacing(&self) -> &i32;
    fn use_horizontal_alt_lines(&self) -> &bool;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u32;
}

impl ThreatAlertHighlightParamsComponentStateTrait for ThreatAlertHighlightParamsComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn color1(&self) -> &super::core::Vec3 {
        &self.color1
    }
    fn color2(&self) -> &super::core::Vec3 {
        &self.color2
    }
    fn color3(&self) -> &super::core::Vec3 {
        &self.color3
    }
    fn color4(&self) -> &super::core::Vec3 {
        &self.color4
    }
    fn color5(&self) -> &super::core::Vec3 {
        &self.color5
    }
    fn use_outline(&self) -> &bool {
        &self.use_outline
    }
    fn outline_opacity(&self) -> &f32 {
        &self.outline_opacity
    }
    fn use_scan_lines(&self) -> &bool {
        &self.use_scan_lines
    }
    fn scan_line_offset(&self) -> &i32 {
        &self.scan_line_offset
    }
    fn scan_line_opacity(&self) -> &f32 {
        &self.scan_line_opacity
    }
    fn scan_line_thickness(&self) -> &i32 {
        &self.scan_line_thickness
    }
    fn scan_line_spacing(&self) -> &i32 {
        &self.scan_line_spacing
    }
    fn use_horizontal_scan_lines(&self) -> &bool {
        &self.use_horizontal_scan_lines
    }
    fn use_alt_lines(&self) -> &bool {
        &self.use_alt_lines
    }
    fn alt_line_offset(&self) -> &i32 {
        &self.alt_line_offset
    }
    fn alt_line_opacity(&self) -> &f32 {
        &self.alt_line_opacity
    }
    fn alt_line_thickness(&self) -> &i32 {
        &self.alt_line_thickness
    }
    fn alt_line_spacing(&self) -> &i32 {
        &self.alt_line_spacing
    }
    fn use_horizontal_alt_lines(&self) -> &bool {
        &self.use_horizontal_alt_lines
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static THREATALERTHIGHLIGHTPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreatAlertHighlightParamsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ThreatAlertHighlightParamsComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, enable),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, color2),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, color3),
            },
            FieldInfoData {
                name: "Color4",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, color4),
            },
            FieldInfoData {
                name: "Color5",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, color5),
            },
            FieldInfoData {
                name: "UseOutline",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, use_outline),
            },
            FieldInfoData {
                name: "OutlineOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, outline_opacity),
            },
            FieldInfoData {
                name: "UseScanLines",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, use_scan_lines),
            },
            FieldInfoData {
                name: "ScanLineOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, scan_line_offset),
            },
            FieldInfoData {
                name: "ScanLineOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, scan_line_opacity),
            },
            FieldInfoData {
                name: "ScanLineThickness",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, scan_line_thickness),
            },
            FieldInfoData {
                name: "ScanLineSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, scan_line_spacing),
            },
            FieldInfoData {
                name: "UseHorizontalScanLines",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, use_horizontal_scan_lines),
            },
            FieldInfoData {
                name: "UseAltLines",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, use_alt_lines),
            },
            FieldInfoData {
                name: "AltLineOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, alt_line_offset),
            },
            FieldInfoData {
                name: "AltLineOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, alt_line_opacity),
            },
            FieldInfoData {
                name: "AltLineThickness",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, alt_line_thickness),
            },
            FieldInfoData {
                name: "AltLineSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, alt_line_spacing),
            },
            FieldInfoData {
                name: "UseHorizontalAltLines",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, use_horizontal_alt_lines),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(THREATALERTHIGHLIGHTPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ThreatAlertHighlightParamsComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        THREATALERTHIGHLIGHTPARAMSCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static THREATALERTHIGHLIGHTPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreatAlertHighlightParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ThreatAlertHighlightParamsComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ObjectHighlightParamsComponentState {
    pub enable: bool,
    pub brightness: f32,
    pub color1: super::core::Vec3,
    pub color1_alpha: f32,
    pub color2: super::core::Vec3,
    pub color2_alpha: f32,
    pub color3: super::core::Vec3,
    pub color3_alpha: f32,
    pub color4: super::core::Vec3,
    pub color4_alpha: f32,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait ObjectHighlightParamsComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn brightness(&self) -> &f32;
    fn color1(&self) -> &super::core::Vec3;
    fn color1_alpha(&self) -> &f32;
    fn color2(&self) -> &super::core::Vec3;
    fn color2_alpha(&self) -> &f32;
    fn color3(&self) -> &super::core::Vec3;
    fn color3_alpha(&self) -> &f32;
    fn color4(&self) -> &super::core::Vec3;
    fn color4_alpha(&self) -> &f32;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u16;
}

impl ObjectHighlightParamsComponentStateTrait for ObjectHighlightParamsComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn brightness(&self) -> &f32 {
        &self.brightness
    }
    fn color1(&self) -> &super::core::Vec3 {
        &self.color1
    }
    fn color1_alpha(&self) -> &f32 {
        &self.color1_alpha
    }
    fn color2(&self) -> &super::core::Vec3 {
        &self.color2
    }
    fn color2_alpha(&self) -> &f32 {
        &self.color2_alpha
    }
    fn color3(&self) -> &super::core::Vec3 {
        &self.color3
    }
    fn color3_alpha(&self) -> &f32 {
        &self.color3_alpha
    }
    fn color4(&self) -> &super::core::Vec3 {
        &self.color4
    }
    fn color4_alpha(&self) -> &f32 {
        &self.color4_alpha
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static OBJECTHIGHLIGHTPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightParamsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObjectHighlightParamsComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, enable),
            },
            FieldInfoData {
                name: "Brightness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, brightness),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color1),
            },
            FieldInfoData {
                name: "Color1Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color1_alpha),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color2),
            },
            FieldInfoData {
                name: "Color2Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color2_alpha),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color3),
            },
            FieldInfoData {
                name: "Color3Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color3_alpha),
            },
            FieldInfoData {
                name: "Color4",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color4),
            },
            FieldInfoData {
                name: "Color4Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color4_alpha),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OBJECTHIGHLIGHTPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ObjectHighlightParamsComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        OBJECTHIGHLIGHTPARAMSCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OBJECTHIGHLIGHTPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ObjectHighlightParamsComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntiAliasComponentState {
    pub enable: bool,
    pub disocclusion_rejection_factor: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait AntiAliasComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn disocclusion_rejection_factor(&self) -> &f32;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl AntiAliasComponentStateTrait for AntiAliasComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn disocclusion_rejection_factor(&self) -> &f32 {
        &self.disocclusion_rejection_factor
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static ANTIALIASCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiAliasComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntiAliasComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntiAliasComponentState, enable),
            },
            FieldInfoData {
                name: "DisocclusionRejectionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntiAliasComponentState, disocclusion_rejection_factor),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(AntiAliasComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(AntiAliasComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(ANTIALIASCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AntiAliasComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        ANTIALIASCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTIALIASCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiAliasComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("AntiAliasComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SkyCelestialMultiComponentState {
    pub enable: bool,
    pub enabled_on_quality_levels: super::core::QualityScalableBool,
    pub draw_order: u32,
    pub planar_reflection: super::core::QualityScalableBool,
    pub render_in_sky_env_map: bool,
    pub write_alpha: bool,
    pub quad_count: super::core::QualityScalableInt,
    pub texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub u_v_start: super::core::Vec2,
    pub u_v_end: super::core::Vec2,
    pub u_v_grid: super::core::Vec2,
    pub tint: super::core::Vec3,
    pub sky_envmap_tint_scale: f32,
    pub random_seed: i32,
    pub min_scale: f32,
    pub max_scale: f32,
    pub scale_multiplier: f32,
    pub zenith_stop: f32,
    pub nadir_stop: f32,
    pub field_flag_override0: u32,
    pub field_flag_changed0: u32,
}

pub trait SkyCelestialMultiComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enabled_on_quality_levels(&self) -> &super::core::QualityScalableBool;
    fn draw_order(&self) -> &u32;
    fn planar_reflection(&self) -> &super::core::QualityScalableBool;
    fn render_in_sky_env_map(&self) -> &bool;
    fn write_alpha(&self) -> &bool;
    fn quad_count(&self) -> &super::core::QualityScalableInt;
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn u_v_start(&self) -> &super::core::Vec2;
    fn u_v_end(&self) -> &super::core::Vec2;
    fn u_v_grid(&self) -> &super::core::Vec2;
    fn tint(&self) -> &super::core::Vec3;
    fn sky_envmap_tint_scale(&self) -> &f32;
    fn random_seed(&self) -> &i32;
    fn min_scale(&self) -> &f32;
    fn max_scale(&self) -> &f32;
    fn scale_multiplier(&self) -> &f32;
    fn zenith_stop(&self) -> &f32;
    fn nadir_stop(&self) -> &f32;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u32;
}

impl SkyCelestialMultiComponentStateTrait for SkyCelestialMultiComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enabled_on_quality_levels(&self) -> &super::core::QualityScalableBool {
        &self.enabled_on_quality_levels
    }
    fn draw_order(&self) -> &u32 {
        &self.draw_order
    }
    fn planar_reflection(&self) -> &super::core::QualityScalableBool {
        &self.planar_reflection
    }
    fn render_in_sky_env_map(&self) -> &bool {
        &self.render_in_sky_env_map
    }
    fn write_alpha(&self) -> &bool {
        &self.write_alpha
    }
    fn quad_count(&self) -> &super::core::QualityScalableInt {
        &self.quad_count
    }
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture
    }
    fn u_v_start(&self) -> &super::core::Vec2 {
        &self.u_v_start
    }
    fn u_v_end(&self) -> &super::core::Vec2 {
        &self.u_v_end
    }
    fn u_v_grid(&self) -> &super::core::Vec2 {
        &self.u_v_grid
    }
    fn tint(&self) -> &super::core::Vec3 {
        &self.tint
    }
    fn sky_envmap_tint_scale(&self) -> &f32 {
        &self.sky_envmap_tint_scale
    }
    fn random_seed(&self) -> &i32 {
        &self.random_seed
    }
    fn min_scale(&self) -> &f32 {
        &self.min_scale
    }
    fn max_scale(&self) -> &f32 {
        &self.max_scale
    }
    fn scale_multiplier(&self) -> &f32 {
        &self.scale_multiplier
    }
    fn zenith_stop(&self) -> &f32 {
        &self.zenith_stop
    }
    fn nadir_stop(&self) -> &f32 {
        &self.nadir_stop
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static SKYCELESTIALMULTICOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialMultiComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SkyCelestialMultiComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, enable),
            },
            FieldInfoData {
                name: "EnabledOnQualityLevels",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableBool",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, enabled_on_quality_levels),
            },
            FieldInfoData {
                name: "DrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, draw_order),
            },
            FieldInfoData {
                name: "PlanarReflection",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableBool",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, planar_reflection),
            },
            FieldInfoData {
                name: "RenderInSkyEnvMap",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, render_in_sky_env_map),
            },
            FieldInfoData {
                name: "WriteAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, write_alpha),
            },
            FieldInfoData {
                name: "QuadCount",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, quad_count),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, texture),
            },
            FieldInfoData {
                name: "UVStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, u_v_start),
            },
            FieldInfoData {
                name: "UVEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, u_v_end),
            },
            FieldInfoData {
                name: "UVGrid",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, u_v_grid),
            },
            FieldInfoData {
                name: "Tint",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, tint),
            },
            FieldInfoData {
                name: "SkyEnvmapTintScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, sky_envmap_tint_scale),
            },
            FieldInfoData {
                name: "RandomSeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, random_seed),
            },
            FieldInfoData {
                name: "MinScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, min_scale),
            },
            FieldInfoData {
                name: "MaxScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, max_scale),
            },
            FieldInfoData {
                name: "ScaleMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, scale_multiplier),
            },
            FieldInfoData {
                name: "ZenithStop",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, zenith_stop),
            },
            FieldInfoData {
                name: "NadirStop",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, nadir_stop),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyCelestialMultiComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SKYCELESTIALMULTICOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCelestialMultiComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SKYCELESTIALMULTICOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SKYCELESTIALMULTICOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialMultiComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyCelestialMultiComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SkyCelestialSingleComponentState {
    pub enable: bool,
    pub enabled_on_quality_levels: super::core::QualityScalableBool,
    pub draw_order: u32,
    pub planar_reflection: super::core::QualityScalableBool,
    pub render_in_sky_env_map: bool,
    pub write_alpha: bool,
    pub texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub u_v_start: super::core::Vec2,
    pub u_v_end: super::core::Vec2,
    pub tint: super::core::Vec3,
    pub sky_envmap_tint_scale: f32,
    pub longitude: f32,
    pub latitude: f32,
    pub rotation: f32,
    pub scale: super::core::Vec2,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait SkyCelestialSingleComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enabled_on_quality_levels(&self) -> &super::core::QualityScalableBool;
    fn draw_order(&self) -> &u32;
    fn planar_reflection(&self) -> &super::core::QualityScalableBool;
    fn render_in_sky_env_map(&self) -> &bool;
    fn write_alpha(&self) -> &bool;
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn u_v_start(&self) -> &super::core::Vec2;
    fn u_v_end(&self) -> &super::core::Vec2;
    fn tint(&self) -> &super::core::Vec3;
    fn sky_envmap_tint_scale(&self) -> &f32;
    fn longitude(&self) -> &f32;
    fn latitude(&self) -> &f32;
    fn rotation(&self) -> &f32;
    fn scale(&self) -> &super::core::Vec2;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u16;
}

impl SkyCelestialSingleComponentStateTrait for SkyCelestialSingleComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enabled_on_quality_levels(&self) -> &super::core::QualityScalableBool {
        &self.enabled_on_quality_levels
    }
    fn draw_order(&self) -> &u32 {
        &self.draw_order
    }
    fn planar_reflection(&self) -> &super::core::QualityScalableBool {
        &self.planar_reflection
    }
    fn render_in_sky_env_map(&self) -> &bool {
        &self.render_in_sky_env_map
    }
    fn write_alpha(&self) -> &bool {
        &self.write_alpha
    }
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture
    }
    fn u_v_start(&self) -> &super::core::Vec2 {
        &self.u_v_start
    }
    fn u_v_end(&self) -> &super::core::Vec2 {
        &self.u_v_end
    }
    fn tint(&self) -> &super::core::Vec3 {
        &self.tint
    }
    fn sky_envmap_tint_scale(&self) -> &f32 {
        &self.sky_envmap_tint_scale
    }
    fn longitude(&self) -> &f32 {
        &self.longitude
    }
    fn latitude(&self) -> &f32 {
        &self.latitude
    }
    fn rotation(&self) -> &f32 {
        &self.rotation
    }
    fn scale(&self) -> &super::core::Vec2 {
        &self.scale
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static SKYCELESTIALSINGLECOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialSingleComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SkyCelestialSingleComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, enable),
            },
            FieldInfoData {
                name: "EnabledOnQualityLevels",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableBool",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, enabled_on_quality_levels),
            },
            FieldInfoData {
                name: "DrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, draw_order),
            },
            FieldInfoData {
                name: "PlanarReflection",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableBool",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, planar_reflection),
            },
            FieldInfoData {
                name: "RenderInSkyEnvMap",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, render_in_sky_env_map),
            },
            FieldInfoData {
                name: "WriteAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, write_alpha),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, texture),
            },
            FieldInfoData {
                name: "UVStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, u_v_start),
            },
            FieldInfoData {
                name: "UVEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, u_v_end),
            },
            FieldInfoData {
                name: "Tint",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, tint),
            },
            FieldInfoData {
                name: "SkyEnvmapTintScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, sky_envmap_tint_scale),
            },
            FieldInfoData {
                name: "Longitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, longitude),
            },
            FieldInfoData {
                name: "Latitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, latitude),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, rotation),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SkyCelestialSingleComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SKYCELESTIALSINGLECOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCelestialSingleComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SKYCELESTIALSINGLECOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SKYCELESTIALSINGLECOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialSingleComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyCelestialSingleComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SkyCelestialRotationComponentState {
    pub enable: bool,
    pub rotation: f32,
    pub rotation_axis: super::core::Vec3,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait SkyCelestialRotationComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn rotation(&self) -> &f32;
    fn rotation_axis(&self) -> &super::core::Vec3;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl SkyCelestialRotationComponentStateTrait for SkyCelestialRotationComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn rotation(&self) -> &f32 {
        &self.rotation
    }
    fn rotation_axis(&self) -> &super::core::Vec3 {
        &self.rotation_axis
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static SKYCELESTIALROTATIONCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialRotationComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SkyCelestialRotationComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyCelestialRotationComponentState, enable),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyCelestialRotationComponentState, rotation),
            },
            FieldInfoData {
                name: "RotationAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyCelestialRotationComponentState, rotation_axis),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SkyCelestialRotationComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SkyCelestialRotationComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SKYCELESTIALROTATIONCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCelestialRotationComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SKYCELESTIALROTATIONCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SKYCELESTIALROTATIONCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialRotationComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyCelestialRotationComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SkyCelestialComponentCountMax {
    #[default]
    SkyCelestialComponentCountMax_Single = 4,
    SkyCelestialComponentCountMax_Total = 8,
}

pub static SKYCELESTIALCOMPONENTCOUNTMAX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialComponentCountMax",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SKYCELESTIALCOMPONENTCOUNTMAX_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SkyCelestialComponentCountMax {
    fn type_info(&self) -> &'static TypeInfo {
        SKYCELESTIALCOMPONENTCOUNTMAX_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SKYCELESTIALCOMPONENTCOUNTMAX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialComponentCountMax-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyCelestialComponentCountMax"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterParamGlobalComponentState {
    pub parameter: Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>,
    pub value: super::core::Vec4,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait EmitterParamGlobalComponentStateTrait: TypeObject {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>;
    fn value(&self) -> &super::core::Vec4;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl EmitterParamGlobalComponentStateTrait for EmitterParamGlobalComponentState {
    fn parameter(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        &self.parameter
    }
    fn value(&self) -> &super::core::Vec4 {
        &self.value
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static EMITTERPARAMGLOBALCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterParamGlobalComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Parameter",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParameter",
                rust_offset: offset_of!(EmitterParamGlobalComponentState, parameter),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(EmitterParamGlobalComponentState, value),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EmitterParamGlobalComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EmitterParamGlobalComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(EMITTERPARAMGLOBALCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterParamGlobalComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERPARAMGLOBALCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EMITTERPARAMGLOBALCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EmitterParamGlobalComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterParamGlobalCountMax {
    #[default]
    EmitterParamGlobal_Count = 10,
}

pub static EMITTERPARAMGLOBALCOUNTMAX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalCountMax",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERPARAMGLOBALCOUNTMAX_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterParamGlobalCountMax {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERPARAMGLOBALCOUNTMAX_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EMITTERPARAMGLOBALCOUNTMAX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalCountMax-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EmitterParamGlobalCountMax"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterParamComponentState {
    pub value: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait EmitterParamComponentStateTrait: TypeObject {
    fn value(&self) -> &f32;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl EmitterParamComponentStateTrait for EmitterParamComponentState {
    fn value(&self) -> &f32 {
        &self.value
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static EMITTERPARAMCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterParamComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterParamComponentState, value),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EmitterParamComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(EmitterParamComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(EMITTERPARAMCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EmitterParamComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERPARAMCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EMITTERPARAMCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EmitterParamComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterParamOverride {
    #[default]
    EmitterParamOverride_EmitterParameter1 = 0,
    EmitterParamOverride_EmitterParameter2 = 1,
    EmitterParamOverride_EmitterParameter3 = 2,
    EmitterParamOverride_Count = 3,
}

pub static EMITTERPARAMOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamOverride",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERPARAMOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterParamOverride {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERPARAMOVERRIDE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EMITTERPARAMOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EmitterParamOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RaytraceReflectionComponentState {
    pub enable: bool,
    pub min_smoothness: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait RaytraceReflectionComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn min_smoothness(&self) -> &f32;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl RaytraceReflectionComponentStateTrait for RaytraceReflectionComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn min_smoothness(&self) -> &f32 {
        &self.min_smoothness
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static RAYTRACEREFLECTIONCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaytraceReflectionComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RaytraceReflectionComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RaytraceReflectionComponentState, enable),
            },
            FieldInfoData {
                name: "MinSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RaytraceReflectionComponentState, min_smoothness),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RaytraceReflectionComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RaytraceReflectionComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RAYTRACEREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RaytraceReflectionComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        RAYTRACEREFLECTIONCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RAYTRACEREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaytraceReflectionComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RaytraceReflectionComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ScreenSpaceRaytraceComponentState {
    pub raytrace_enable: bool,
    pub camera_fade_start: f32,
    pub camera_fade_length: f32,
    pub radial_fade_start: f32,
    pub radial_fade_length: f32,
    pub distance_fade_start: f32,
    pub distance_fade_length: f32,
    pub screen_fade_start: f32,
    pub screen_fade_length: f32,
    pub border_fade_start: f32,
    pub border_fade_length: f32,
    pub mirror_fade_start: f32,
    pub mirror_fade_length: f32,
    pub thickness_fade_start: f32,
    pub thickness_fade_length: f32,
    pub roughness_fade_start: f32,
    pub roughness_fade_length: f32,
    pub normal_fade_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub min_samples: u32,
    pub max_samples: u32,
    pub temporal_samples: u32,
    pub temporal_period: u32,
    pub min_roughness: f32,
    pub max_roughness: f32,
    pub resolve_samples: u32,
    pub noise_threshold: f32,
    pub clamp_threshold: f32,
    pub importance_sampling_bias: f32,
    pub filter_bias: f32,
    pub filter_angular_bias: f32,
    pub temporal_filter_responsiveness: f32,
    pub field_flag_override0: u32,
    pub field_flag_changed0: u32,
}

pub trait ScreenSpaceRaytraceComponentStateTrait: TypeObject {
    fn raytrace_enable(&self) -> &bool;
    fn camera_fade_start(&self) -> &f32;
    fn camera_fade_length(&self) -> &f32;
    fn radial_fade_start(&self) -> &f32;
    fn radial_fade_length(&self) -> &f32;
    fn distance_fade_start(&self) -> &f32;
    fn distance_fade_length(&self) -> &f32;
    fn screen_fade_start(&self) -> &f32;
    fn screen_fade_length(&self) -> &f32;
    fn border_fade_start(&self) -> &f32;
    fn border_fade_length(&self) -> &f32;
    fn mirror_fade_start(&self) -> &f32;
    fn mirror_fade_length(&self) -> &f32;
    fn thickness_fade_start(&self) -> &f32;
    fn thickness_fade_length(&self) -> &f32;
    fn roughness_fade_start(&self) -> &f32;
    fn roughness_fade_length(&self) -> &f32;
    fn normal_fade_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn min_samples(&self) -> &u32;
    fn max_samples(&self) -> &u32;
    fn temporal_samples(&self) -> &u32;
    fn temporal_period(&self) -> &u32;
    fn min_roughness(&self) -> &f32;
    fn max_roughness(&self) -> &f32;
    fn resolve_samples(&self) -> &u32;
    fn noise_threshold(&self) -> &f32;
    fn clamp_threshold(&self) -> &f32;
    fn importance_sampling_bias(&self) -> &f32;
    fn filter_bias(&self) -> &f32;
    fn filter_angular_bias(&self) -> &f32;
    fn temporal_filter_responsiveness(&self) -> &f32;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u32;
}

impl ScreenSpaceRaytraceComponentStateTrait for ScreenSpaceRaytraceComponentState {
    fn raytrace_enable(&self) -> &bool {
        &self.raytrace_enable
    }
    fn camera_fade_start(&self) -> &f32 {
        &self.camera_fade_start
    }
    fn camera_fade_length(&self) -> &f32 {
        &self.camera_fade_length
    }
    fn radial_fade_start(&self) -> &f32 {
        &self.radial_fade_start
    }
    fn radial_fade_length(&self) -> &f32 {
        &self.radial_fade_length
    }
    fn distance_fade_start(&self) -> &f32 {
        &self.distance_fade_start
    }
    fn distance_fade_length(&self) -> &f32 {
        &self.distance_fade_length
    }
    fn screen_fade_start(&self) -> &f32 {
        &self.screen_fade_start
    }
    fn screen_fade_length(&self) -> &f32 {
        &self.screen_fade_length
    }
    fn border_fade_start(&self) -> &f32 {
        &self.border_fade_start
    }
    fn border_fade_length(&self) -> &f32 {
        &self.border_fade_length
    }
    fn mirror_fade_start(&self) -> &f32 {
        &self.mirror_fade_start
    }
    fn mirror_fade_length(&self) -> &f32 {
        &self.mirror_fade_length
    }
    fn thickness_fade_start(&self) -> &f32 {
        &self.thickness_fade_start
    }
    fn thickness_fade_length(&self) -> &f32 {
        &self.thickness_fade_length
    }
    fn roughness_fade_start(&self) -> &f32 {
        &self.roughness_fade_start
    }
    fn roughness_fade_length(&self) -> &f32 {
        &self.roughness_fade_length
    }
    fn normal_fade_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.normal_fade_texture
    }
    fn min_samples(&self) -> &u32 {
        &self.min_samples
    }
    fn max_samples(&self) -> &u32 {
        &self.max_samples
    }
    fn temporal_samples(&self) -> &u32 {
        &self.temporal_samples
    }
    fn temporal_period(&self) -> &u32 {
        &self.temporal_period
    }
    fn min_roughness(&self) -> &f32 {
        &self.min_roughness
    }
    fn max_roughness(&self) -> &f32 {
        &self.max_roughness
    }
    fn resolve_samples(&self) -> &u32 {
        &self.resolve_samples
    }
    fn noise_threshold(&self) -> &f32 {
        &self.noise_threshold
    }
    fn clamp_threshold(&self) -> &f32 {
        &self.clamp_threshold
    }
    fn importance_sampling_bias(&self) -> &f32 {
        &self.importance_sampling_bias
    }
    fn filter_bias(&self) -> &f32 {
        &self.filter_bias
    }
    fn filter_angular_bias(&self) -> &f32 {
        &self.filter_angular_bias
    }
    fn temporal_filter_responsiveness(&self) -> &f32 {
        &self.temporal_filter_responsiveness
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static SCREENSPACERAYTRACECOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenSpaceRaytraceComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ScreenSpaceRaytraceComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RaytraceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, raytrace_enable),
            },
            FieldInfoData {
                name: "CameraFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, camera_fade_start),
            },
            FieldInfoData {
                name: "CameraFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, camera_fade_length),
            },
            FieldInfoData {
                name: "RadialFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, radial_fade_start),
            },
            FieldInfoData {
                name: "RadialFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, radial_fade_length),
            },
            FieldInfoData {
                name: "DistanceFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, distance_fade_start),
            },
            FieldInfoData {
                name: "DistanceFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, distance_fade_length),
            },
            FieldInfoData {
                name: "ScreenFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, screen_fade_start),
            },
            FieldInfoData {
                name: "ScreenFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, screen_fade_length),
            },
            FieldInfoData {
                name: "BorderFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, border_fade_start),
            },
            FieldInfoData {
                name: "BorderFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, border_fade_length),
            },
            FieldInfoData {
                name: "MirrorFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, mirror_fade_start),
            },
            FieldInfoData {
                name: "MirrorFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, mirror_fade_length),
            },
            FieldInfoData {
                name: "ThicknessFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, thickness_fade_start),
            },
            FieldInfoData {
                name: "ThicknessFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, thickness_fade_length),
            },
            FieldInfoData {
                name: "RoughnessFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, roughness_fade_start),
            },
            FieldInfoData {
                name: "RoughnessFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, roughness_fade_length),
            },
            FieldInfoData {
                name: "NormalFadeTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, normal_fade_texture),
            },
            FieldInfoData {
                name: "MinSamples",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, min_samples),
            },
            FieldInfoData {
                name: "MaxSamples",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, max_samples),
            },
            FieldInfoData {
                name: "TemporalSamples",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, temporal_samples),
            },
            FieldInfoData {
                name: "TemporalPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, temporal_period),
            },
            FieldInfoData {
                name: "MinRoughness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, min_roughness),
            },
            FieldInfoData {
                name: "MaxRoughness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, max_roughness),
            },
            FieldInfoData {
                name: "ResolveSamples",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, resolve_samples),
            },
            FieldInfoData {
                name: "NoiseThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, noise_threshold),
            },
            FieldInfoData {
                name: "ClampThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, clamp_threshold),
            },
            FieldInfoData {
                name: "ImportanceSamplingBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, importance_sampling_bias),
            },
            FieldInfoData {
                name: "FilterBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, filter_bias),
            },
            FieldInfoData {
                name: "FilterAngularBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, filter_angular_bias),
            },
            FieldInfoData {
                name: "TemporalFilterResponsiveness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, temporal_filter_responsiveness),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SCREENSPACERAYTRACECOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScreenSpaceRaytraceComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SCREENSPACERAYTRACECOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCREENSPACERAYTRACECOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenSpaceRaytraceComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ScreenSpaceRaytraceComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MotionBlurComponentState {
    pub motion_blur_enable: bool,
    pub motion_blur_scale: f32,
    pub motion_blur_centered: bool,
    pub depth_check_max_distance: f32,
    pub radial_blur_enable: bool,
    pub radial_blur_center: super::core::Vec2,
    pub radial_blur_offset: f32,
    pub circular_offset_factor: f32,
    pub radial_blur_scale: f32,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait MotionBlurComponentStateTrait: TypeObject {
    fn motion_blur_enable(&self) -> &bool;
    fn motion_blur_scale(&self) -> &f32;
    fn motion_blur_centered(&self) -> &bool;
    fn depth_check_max_distance(&self) -> &f32;
    fn radial_blur_enable(&self) -> &bool;
    fn radial_blur_center(&self) -> &super::core::Vec2;
    fn radial_blur_offset(&self) -> &f32;
    fn circular_offset_factor(&self) -> &f32;
    fn radial_blur_scale(&self) -> &f32;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u16;
}

impl MotionBlurComponentStateTrait for MotionBlurComponentState {
    fn motion_blur_enable(&self) -> &bool {
        &self.motion_blur_enable
    }
    fn motion_blur_scale(&self) -> &f32 {
        &self.motion_blur_scale
    }
    fn motion_blur_centered(&self) -> &bool {
        &self.motion_blur_centered
    }
    fn depth_check_max_distance(&self) -> &f32 {
        &self.depth_check_max_distance
    }
    fn radial_blur_enable(&self) -> &bool {
        &self.radial_blur_enable
    }
    fn radial_blur_center(&self) -> &super::core::Vec2 {
        &self.radial_blur_center
    }
    fn radial_blur_offset(&self) -> &f32 {
        &self.radial_blur_offset
    }
    fn circular_offset_factor(&self) -> &f32 {
        &self.circular_offset_factor
    }
    fn radial_blur_scale(&self) -> &f32 {
        &self.radial_blur_scale
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static MOTIONBLURCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionBlurComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MotionBlurComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MotionBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MotionBlurComponentState, motion_blur_enable),
            },
            FieldInfoData {
                name: "MotionBlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotionBlurComponentState, motion_blur_scale),
            },
            FieldInfoData {
                name: "MotionBlurCentered",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MotionBlurComponentState, motion_blur_centered),
            },
            FieldInfoData {
                name: "DepthCheckMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotionBlurComponentState, depth_check_max_distance),
            },
            FieldInfoData {
                name: "RadialBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MotionBlurComponentState, radial_blur_enable),
            },
            FieldInfoData {
                name: "RadialBlurCenter",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(MotionBlurComponentState, radial_blur_center),
            },
            FieldInfoData {
                name: "RadialBlurOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotionBlurComponentState, radial_blur_offset),
            },
            FieldInfoData {
                name: "CircularOffsetFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotionBlurComponentState, circular_offset_factor),
            },
            FieldInfoData {
                name: "RadialBlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MotionBlurComponentState, radial_blur_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(MotionBlurComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(MotionBlurComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MOTIONBLURCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MotionBlurComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        MOTIONBLURCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOTIONBLURCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionBlurComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("MotionBlurComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CharacterLightingComponentState {
    pub character_light_enable: bool,
    pub first_person_enable: bool,
    pub lock_to_camera_direction: bool,
    pub camera_up_rotation: f32,
    pub character_lighting_mode: CharacterLightingMode,
    pub blend_factor: f32,
    pub top_light: super::core::Vec3,
    pub bottom_light: super::core::Vec3,
    pub top_light_dir_x: f32,
    pub top_light_dir_y: f32,
    pub start_fade_distance: f32,
    pub end_fade_distance: f32,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait CharacterLightingComponentStateTrait: TypeObject {
    fn character_light_enable(&self) -> &bool;
    fn first_person_enable(&self) -> &bool;
    fn lock_to_camera_direction(&self) -> &bool;
    fn camera_up_rotation(&self) -> &f32;
    fn character_lighting_mode(&self) -> &CharacterLightingMode;
    fn blend_factor(&self) -> &f32;
    fn top_light(&self) -> &super::core::Vec3;
    fn bottom_light(&self) -> &super::core::Vec3;
    fn top_light_dir_x(&self) -> &f32;
    fn top_light_dir_y(&self) -> &f32;
    fn start_fade_distance(&self) -> &f32;
    fn end_fade_distance(&self) -> &f32;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u16;
}

impl CharacterLightingComponentStateTrait for CharacterLightingComponentState {
    fn character_light_enable(&self) -> &bool {
        &self.character_light_enable
    }
    fn first_person_enable(&self) -> &bool {
        &self.first_person_enable
    }
    fn lock_to_camera_direction(&self) -> &bool {
        &self.lock_to_camera_direction
    }
    fn camera_up_rotation(&self) -> &f32 {
        &self.camera_up_rotation
    }
    fn character_lighting_mode(&self) -> &CharacterLightingMode {
        &self.character_lighting_mode
    }
    fn blend_factor(&self) -> &f32 {
        &self.blend_factor
    }
    fn top_light(&self) -> &super::core::Vec3 {
        &self.top_light
    }
    fn bottom_light(&self) -> &super::core::Vec3 {
        &self.bottom_light
    }
    fn top_light_dir_x(&self) -> &f32 {
        &self.top_light_dir_x
    }
    fn top_light_dir_y(&self) -> &f32 {
        &self.top_light_dir_y
    }
    fn start_fade_distance(&self) -> &f32 {
        &self.start_fade_distance
    }
    fn end_fade_distance(&self) -> &f32 {
        &self.end_fade_distance
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static CHARACTERLIGHTINGCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterLightingComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CharacterLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterLightingComponentState, character_light_enable),
            },
            FieldInfoData {
                name: "FirstPersonEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterLightingComponentState, first_person_enable),
            },
            FieldInfoData {
                name: "LockToCameraDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CharacterLightingComponentState, lock_to_camera_direction),
            },
            FieldInfoData {
                name: "CameraUpRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterLightingComponentState, camera_up_rotation),
            },
            FieldInfoData {
                name: "CharacterLightingMode",
                flags: MemberInfoFlags::new(0),
                field_type: "CharacterLightingMode",
                rust_offset: offset_of!(CharacterLightingComponentState, character_lighting_mode),
            },
            FieldInfoData {
                name: "BlendFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterLightingComponentState, blend_factor),
            },
            FieldInfoData {
                name: "TopLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CharacterLightingComponentState, top_light),
            },
            FieldInfoData {
                name: "BottomLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(CharacterLightingComponentState, bottom_light),
            },
            FieldInfoData {
                name: "TopLightDirX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterLightingComponentState, top_light_dir_x),
            },
            FieldInfoData {
                name: "TopLightDirY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterLightingComponentState, top_light_dir_y),
            },
            FieldInfoData {
                name: "StartFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterLightingComponentState, start_fade_distance),
            },
            FieldInfoData {
                name: "EndFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CharacterLightingComponentState, end_fade_distance),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(CharacterLightingComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(CharacterLightingComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CHARACTERLIGHTINGCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterLightingComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERLIGHTINGCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERLIGHTINGCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("CharacterLightingComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CharacterLightingMode {
    #[default]
    CharacterLightingMode_Add = 0,
    CharacterLightingMode_Blend = 1,
    CharacterLightingMode_Multiply = 2,
}

pub static CHARACTERLIGHTINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(CHARACTERLIGHTINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CharacterLightingMode {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERLIGHTINGMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHARACTERLIGHTINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("CharacterLightingMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DamageEffectComponentState {
    pub shader: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub frame_width: f32,
    pub outer_frame_opacity: f32,
    pub inner_frame_opacity: f32,
    pub fallof_time: f32,
    pub min_damage_percentage_threshold: f32,
    pub max_opacity_damage_percentage: f32,
    pub start_critical_effect_health_threshold: f32,
    pub end_critical_effect_health_threshold: f32,
    pub debug_damage: bool,
    pub top_damage: super::core::Vec4,
    pub left_damage: super::core::Vec4,
    pub bottom_damage: super::core::Vec4,
    pub right_damage: super::core::Vec4,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait DamageEffectComponentStateTrait: TypeObject {
    fn shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn frame_width(&self) -> &f32;
    fn outer_frame_opacity(&self) -> &f32;
    fn inner_frame_opacity(&self) -> &f32;
    fn fallof_time(&self) -> &f32;
    fn min_damage_percentage_threshold(&self) -> &f32;
    fn max_opacity_damage_percentage(&self) -> &f32;
    fn start_critical_effect_health_threshold(&self) -> &f32;
    fn end_critical_effect_health_threshold(&self) -> &f32;
    fn debug_damage(&self) -> &bool;
    fn top_damage(&self) -> &super::core::Vec4;
    fn left_damage(&self) -> &super::core::Vec4;
    fn bottom_damage(&self) -> &super::core::Vec4;
    fn right_damage(&self) -> &super::core::Vec4;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u16;
}

impl DamageEffectComponentStateTrait for DamageEffectComponentState {
    fn shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader
    }
    fn frame_width(&self) -> &f32 {
        &self.frame_width
    }
    fn outer_frame_opacity(&self) -> &f32 {
        &self.outer_frame_opacity
    }
    fn inner_frame_opacity(&self) -> &f32 {
        &self.inner_frame_opacity
    }
    fn fallof_time(&self) -> &f32 {
        &self.fallof_time
    }
    fn min_damage_percentage_threshold(&self) -> &f32 {
        &self.min_damage_percentage_threshold
    }
    fn max_opacity_damage_percentage(&self) -> &f32 {
        &self.max_opacity_damage_percentage
    }
    fn start_critical_effect_health_threshold(&self) -> &f32 {
        &self.start_critical_effect_health_threshold
    }
    fn end_critical_effect_health_threshold(&self) -> &f32 {
        &self.end_critical_effect_health_threshold
    }
    fn debug_damage(&self) -> &bool {
        &self.debug_damage
    }
    fn top_damage(&self) -> &super::core::Vec4 {
        &self.top_damage
    }
    fn left_damage(&self) -> &super::core::Vec4 {
        &self.left_damage
    }
    fn bottom_damage(&self) -> &super::core::Vec4 {
        &self.bottom_damage
    }
    fn right_damage(&self) -> &super::core::Vec4 {
        &self.right_damage
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static DAMAGEEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageEffectComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DamageEffectComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(DamageEffectComponentState, shader),
            },
            FieldInfoData {
                name: "FrameWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DamageEffectComponentState, frame_width),
            },
            FieldInfoData {
                name: "OuterFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DamageEffectComponentState, outer_frame_opacity),
            },
            FieldInfoData {
                name: "InnerFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DamageEffectComponentState, inner_frame_opacity),
            },
            FieldInfoData {
                name: "FallofTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DamageEffectComponentState, fallof_time),
            },
            FieldInfoData {
                name: "MinDamagePercentageThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DamageEffectComponentState, min_damage_percentage_threshold),
            },
            FieldInfoData {
                name: "MaxOpacityDamagePercentage",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DamageEffectComponentState, max_opacity_damage_percentage),
            },
            FieldInfoData {
                name: "StartCriticalEffectHealthThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DamageEffectComponentState, start_critical_effect_health_threshold),
            },
            FieldInfoData {
                name: "EndCriticalEffectHealthThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DamageEffectComponentState, end_critical_effect_health_threshold),
            },
            FieldInfoData {
                name: "DebugDamage",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DamageEffectComponentState, debug_damage),
            },
            FieldInfoData {
                name: "TopDamage",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(DamageEffectComponentState, top_damage),
            },
            FieldInfoData {
                name: "LeftDamage",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(DamageEffectComponentState, left_damage),
            },
            FieldInfoData {
                name: "BottomDamage",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(DamageEffectComponentState, bottom_damage),
            },
            FieldInfoData {
                name: "RightDamage",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(DamageEffectComponentState, right_damage),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(DamageEffectComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(DamageEffectComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DAMAGEEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DamageEffectComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        DAMAGEEFFECTCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DAMAGEEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DamageEffectComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ScreenEffectComponentState {
    pub frame_type: ScreenEffectFrameType,
    pub shader: Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>,
    pub frame_width: f32,
    pub outer_frame_opacity: f32,
    pub inner_frame_opacity: f32,
    pub angle: f32,
    pub screen_effect_params: super::core::Vec4,
    pub active_position: super::core::Vec3,
    pub state_id: u32,
    pub visibilty: f32,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait ScreenEffectComponentStateTrait: TypeObject {
    fn frame_type(&self) -> &ScreenEffectFrameType;
    fn shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>>;
    fn frame_width(&self) -> &f32;
    fn outer_frame_opacity(&self) -> &f32;
    fn inner_frame_opacity(&self) -> &f32;
    fn angle(&self) -> &f32;
    fn screen_effect_params(&self) -> &super::core::Vec4;
    fn active_position(&self) -> &super::core::Vec3;
    fn state_id(&self) -> &u32;
    fn visibilty(&self) -> &f32;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u16;
}

impl ScreenEffectComponentStateTrait for ScreenEffectComponentState {
    fn frame_type(&self) -> &ScreenEffectFrameType {
        &self.frame_type
    }
    fn shader(&self) -> &Option<Arc<Mutex<dyn super::render_base::SurfaceShaderBaseAssetTrait>>> {
        &self.shader
    }
    fn frame_width(&self) -> &f32 {
        &self.frame_width
    }
    fn outer_frame_opacity(&self) -> &f32 {
        &self.outer_frame_opacity
    }
    fn inner_frame_opacity(&self) -> &f32 {
        &self.inner_frame_opacity
    }
    fn angle(&self) -> &f32 {
        &self.angle
    }
    fn screen_effect_params(&self) -> &super::core::Vec4 {
        &self.screen_effect_params
    }
    fn active_position(&self) -> &super::core::Vec3 {
        &self.active_position
    }
    fn state_id(&self) -> &u32 {
        &self.state_id
    }
    fn visibilty(&self) -> &f32 {
        &self.visibilty
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static SCREENEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ScreenEffectComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FrameType",
                flags: MemberInfoFlags::new(0),
                field_type: "ScreenEffectFrameType",
                rust_offset: offset_of!(ScreenEffectComponentState, frame_type),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderBaseAsset",
                rust_offset: offset_of!(ScreenEffectComponentState, shader),
            },
            FieldInfoData {
                name: "FrameWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenEffectComponentState, frame_width),
            },
            FieldInfoData {
                name: "OuterFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenEffectComponentState, outer_frame_opacity),
            },
            FieldInfoData {
                name: "InnerFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenEffectComponentState, inner_frame_opacity),
            },
            FieldInfoData {
                name: "Angle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenEffectComponentState, angle),
            },
            FieldInfoData {
                name: "ScreenEffectParams",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ScreenEffectComponentState, screen_effect_params),
            },
            FieldInfoData {
                name: "ActivePosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ScreenEffectComponentState, active_position),
            },
            FieldInfoData {
                name: "StateId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ScreenEffectComponentState, state_id),
            },
            FieldInfoData {
                name: "Visibilty",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ScreenEffectComponentState, visibilty),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ScreenEffectComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ScreenEffectComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SCREENEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ScreenEffectComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SCREENEFFECTCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCREENEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ScreenEffectComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ScreenEffectFrameType {
    #[default]
    ScreenEffectFrameType_FullFrame = 0,
    ScreenEffectFrameType_SingleFramePart = 1,
    ScreenEffectFrameType_SingleSquareFramePart = 2,
}

pub static SCREENEFFECTFRAMETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectFrameType",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SCREENEFFECTFRAMETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ScreenEffectFrameType {
    fn type_info(&self) -> &'static TypeInfo {
        SCREENEFFECTFRAMETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCREENEFFECTFRAMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectFrameType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ScreenEffectFrameType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShadowsComponentState {
    pub def_sun_shadowmap_view_distance: f32,
    pub sun_shadowmap_view_distance: super::core::QualityScalableFloat,
    pub sun_shadowmap_extrusion_length: f32,
    pub sun_shadowmap_slice_scheme_weight: f32,
    pub sun_shadowmap_first_slice_scale: f32,
    pub sun_shadowmap_first_slice_extrusion_length: f32,
    pub smooth_transition_to_distant_shadows: bool,
    pub sun_shadowmap_slice_count_offset: super::core::QualityScalableInt,
    pub sun_shadowmap_slice_count_min: super::core::QualityScalableInt,
    pub sun_shadowmap_slice_count_max: super::core::QualityScalableInt,
    pub sun_shadowmap_slice_resolution_scale: super::core::QualityScalableFloat,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait ShadowsComponentStateTrait: TypeObject {
    fn def_sun_shadowmap_view_distance(&self) -> &f32;
    fn sun_shadowmap_view_distance(&self) -> &super::core::QualityScalableFloat;
    fn sun_shadowmap_extrusion_length(&self) -> &f32;
    fn sun_shadowmap_slice_scheme_weight(&self) -> &f32;
    fn sun_shadowmap_first_slice_scale(&self) -> &f32;
    fn sun_shadowmap_first_slice_extrusion_length(&self) -> &f32;
    fn smooth_transition_to_distant_shadows(&self) -> &bool;
    fn sun_shadowmap_slice_count_offset(&self) -> &super::core::QualityScalableInt;
    fn sun_shadowmap_slice_count_min(&self) -> &super::core::QualityScalableInt;
    fn sun_shadowmap_slice_count_max(&self) -> &super::core::QualityScalableInt;
    fn sun_shadowmap_slice_resolution_scale(&self) -> &super::core::QualityScalableFloat;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u16;
}

impl ShadowsComponentStateTrait for ShadowsComponentState {
    fn def_sun_shadowmap_view_distance(&self) -> &f32 {
        &self.def_sun_shadowmap_view_distance
    }
    fn sun_shadowmap_view_distance(&self) -> &super::core::QualityScalableFloat {
        &self.sun_shadowmap_view_distance
    }
    fn sun_shadowmap_extrusion_length(&self) -> &f32 {
        &self.sun_shadowmap_extrusion_length
    }
    fn sun_shadowmap_slice_scheme_weight(&self) -> &f32 {
        &self.sun_shadowmap_slice_scheme_weight
    }
    fn sun_shadowmap_first_slice_scale(&self) -> &f32 {
        &self.sun_shadowmap_first_slice_scale
    }
    fn sun_shadowmap_first_slice_extrusion_length(&self) -> &f32 {
        &self.sun_shadowmap_first_slice_extrusion_length
    }
    fn smooth_transition_to_distant_shadows(&self) -> &bool {
        &self.smooth_transition_to_distant_shadows
    }
    fn sun_shadowmap_slice_count_offset(&self) -> &super::core::QualityScalableInt {
        &self.sun_shadowmap_slice_count_offset
    }
    fn sun_shadowmap_slice_count_min(&self) -> &super::core::QualityScalableInt {
        &self.sun_shadowmap_slice_count_min
    }
    fn sun_shadowmap_slice_count_max(&self) -> &super::core::QualityScalableInt {
        &self.sun_shadowmap_slice_count_max
    }
    fn sun_shadowmap_slice_resolution_scale(&self) -> &super::core::QualityScalableFloat {
        &self.sun_shadowmap_slice_resolution_scale
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static SHADOWSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowsComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DefSunShadowmapViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ShadowsComponentState, def_sun_shadowmap_view_distance),
            },
            FieldInfoData {
                name: "SunShadowmapViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_view_distance),
            },
            FieldInfoData {
                name: "SunShadowmapExtrusionLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_extrusion_length),
            },
            FieldInfoData {
                name: "SunShadowmapSliceSchemeWeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_slice_scheme_weight),
            },
            FieldInfoData {
                name: "SunShadowmapFirstSliceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_first_slice_scale),
            },
            FieldInfoData {
                name: "SunShadowmapFirstSliceExtrusionLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_first_slice_extrusion_length),
            },
            FieldInfoData {
                name: "SmoothTransitionToDistantShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShadowsComponentState, smooth_transition_to_distant_shadows),
            },
            FieldInfoData {
                name: "SunShadowmapSliceCountOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_slice_count_offset),
            },
            FieldInfoData {
                name: "SunShadowmapSliceCountMin",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_slice_count_min),
            },
            FieldInfoData {
                name: "SunShadowmapSliceCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_slice_count_max),
            },
            FieldInfoData {
                name: "SunShadowmapSliceResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_slice_resolution_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ShadowsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ShadowsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SHADOWSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ShadowsComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWSCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SHADOWSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ShadowsComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshSettingsComponentState {
    pub lod_scale: f32,
    pub force_lod: i32,
    pub cull_screen_area_scale: f32,
    pub shadow_distance_scale: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait MeshSettingsComponentStateTrait: TypeObject {
    fn lod_scale(&self) -> &f32;
    fn force_lod(&self) -> &i32;
    fn cull_screen_area_scale(&self) -> &f32;
    fn shadow_distance_scale(&self) -> &f32;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl MeshSettingsComponentStateTrait for MeshSettingsComponentState {
    fn lod_scale(&self) -> &f32 {
        &self.lod_scale
    }
    fn force_lod(&self) -> &i32 {
        &self.force_lod
    }
    fn cull_screen_area_scale(&self) -> &f32 {
        &self.cull_screen_area_scale
    }
    fn shadow_distance_scale(&self) -> &f32 {
        &self.shadow_distance_scale
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static MESHSETTINGSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSettingsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshSettingsComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LodScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshSettingsComponentState, lod_scale),
            },
            FieldInfoData {
                name: "ForceLod",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MeshSettingsComponentState, force_lod),
            },
            FieldInfoData {
                name: "CullScreenAreaScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshSettingsComponentState, cull_screen_area_scale),
            },
            FieldInfoData {
                name: "ShadowDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshSettingsComponentState, shadow_distance_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MeshSettingsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MeshSettingsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MESHSETTINGSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MeshSettingsComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        MESHSETTINGSCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHSETTINGSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSettingsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("MeshSettingsComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CameraParamsComponentState {
    pub view_distance: f32,
    pub near_plane: f32,
    pub vegetation_max_wiggle_distance: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait CameraParamsComponentStateTrait: TypeObject {
    fn view_distance(&self) -> &f32;
    fn near_plane(&self) -> &f32;
    fn vegetation_max_wiggle_distance(&self) -> &f32;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl CameraParamsComponentStateTrait for CameraParamsComponentState {
    fn view_distance(&self) -> &f32 {
        &self.view_distance
    }
    fn near_plane(&self) -> &f32 {
        &self.near_plane
    }
    fn vegetation_max_wiggle_distance(&self) -> &f32 {
        &self.vegetation_max_wiggle_distance
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static CAMERAPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraParamsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CameraParamsComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraParamsComponentState, view_distance),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraParamsComponentState, near_plane),
            },
            FieldInfoData {
                name: "VegetationMaxWiggleDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CameraParamsComponentState, vegetation_max_wiggle_distance),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(CameraParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(CameraParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CAMERAPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CameraParamsComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        CAMERAPARAMSCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAMERAPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("CameraParamsComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderColorParamsComponentState {
    pub vec4_value: super::core::Vec4,
    pub parameter_name: String,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait ShaderColorParamsComponentStateTrait: TypeObject {
    fn vec4_value(&self) -> &super::core::Vec4;
    fn parameter_name(&self) -> &String;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl ShaderColorParamsComponentStateTrait for ShaderColorParamsComponentState {
    fn vec4_value(&self) -> &super::core::Vec4 {
        &self.vec4_value
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static SHADERCOLORPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderColorParamsComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderColorParamsComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Vec4Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ShaderColorParamsComponentState, vec4_value),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ShaderColorParamsComponentState, parameter_name),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ShaderColorParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ShaderColorParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SHADERCOLORPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderColorParamsComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERCOLORPARAMSCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SHADERCOLORPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderColorParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ShaderColorParamsComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderParamsComponentState {
    pub vec4_value: super::core::Vec4,
    pub bool_value: bool,
    pub texture_value: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub value_type: super::render_base::ExternalValueConstantType,
    pub conditional_value: u32,
    pub conditional_name: String,
    pub parameter_name: String,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait ShaderParamsComponentStateTrait: TypeObject {
    fn vec4_value(&self) -> &super::core::Vec4;
    fn bool_value(&self) -> &bool;
    fn texture_value(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn value_type(&self) -> &super::render_base::ExternalValueConstantType;
    fn conditional_value(&self) -> &u32;
    fn conditional_name(&self) -> &String;
    fn parameter_name(&self) -> &String;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl ShaderParamsComponentStateTrait for ShaderParamsComponentState {
    fn vec4_value(&self) -> &super::core::Vec4 {
        &self.vec4_value
    }
    fn bool_value(&self) -> &bool {
        &self.bool_value
    }
    fn texture_value(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture_value
    }
    fn value_type(&self) -> &super::render_base::ExternalValueConstantType {
        &self.value_type
    }
    fn conditional_value(&self) -> &u32 {
        &self.conditional_value
    }
    fn conditional_name(&self) -> &String {
        &self.conditional_name
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static SHADERPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParamsComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParamsComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Vec4Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ShaderParamsComponentState, vec4_value),
            },
            FieldInfoData {
                name: "BoolValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShaderParamsComponentState, bool_value),
            },
            FieldInfoData {
                name: "TextureValue",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(ShaderParamsComponentState, texture_value),
            },
            FieldInfoData {
                name: "ValueType",
                flags: MemberInfoFlags::new(0),
                field_type: "ExternalValueConstantType",
                rust_offset: offset_of!(ShaderParamsComponentState, value_type),
            },
            FieldInfoData {
                name: "ConditionalValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ShaderParamsComponentState, conditional_value),
            },
            FieldInfoData {
                name: "ConditionalName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ShaderParamsComponentState, conditional_name),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ShaderParamsComponentState, parameter_name),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ShaderParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ShaderParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SHADERPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderParamsComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMSCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SHADERPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ShaderParamsComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenComponentState {
    pub bounce_scale: f32,
    pub sun_scale: f32,
    pub sun_direct_lightmap_enable: bool,
    pub terrain_color: super::core::Vec3,
    pub cull_distance: f32,
    pub cull_radius: f32,
    pub sky_box_enable: bool,
    pub sky_box_cut_bottom: bool,
    pub sky_box_blend_mode: SkyBoxBlendMode,
    pub sky_box_blend: f32,
    pub sky_box_sky_color: super::core::Vec3,
    pub sky_box_ground_color: super::core::Vec3,
    pub sky_box_sun_light_color: super::core::Vec3,
    pub sky_box_sun_light_color_size: f32,
    pub sky_box_back_light_color: super::core::Vec3,
    pub sky_box_back_light_color_size: f32,
    pub sky_box_back_light_rotation_x: f32,
    pub sky_box_back_light_rotation_y: f32,
    pub opaque_alpha_test_simple_scale: super::core::Vec3,
    pub field_flag_override0: u32,
    pub field_flag_changed0: u32,
}

pub trait EnlightenComponentStateTrait: TypeObject {
    fn bounce_scale(&self) -> &f32;
    fn sun_scale(&self) -> &f32;
    fn sun_direct_lightmap_enable(&self) -> &bool;
    fn terrain_color(&self) -> &super::core::Vec3;
    fn cull_distance(&self) -> &f32;
    fn cull_radius(&self) -> &f32;
    fn sky_box_enable(&self) -> &bool;
    fn sky_box_cut_bottom(&self) -> &bool;
    fn sky_box_blend_mode(&self) -> &SkyBoxBlendMode;
    fn sky_box_blend(&self) -> &f32;
    fn sky_box_sky_color(&self) -> &super::core::Vec3;
    fn sky_box_ground_color(&self) -> &super::core::Vec3;
    fn sky_box_sun_light_color(&self) -> &super::core::Vec3;
    fn sky_box_sun_light_color_size(&self) -> &f32;
    fn sky_box_back_light_color(&self) -> &super::core::Vec3;
    fn sky_box_back_light_color_size(&self) -> &f32;
    fn sky_box_back_light_rotation_x(&self) -> &f32;
    fn sky_box_back_light_rotation_y(&self) -> &f32;
    fn opaque_alpha_test_simple_scale(&self) -> &super::core::Vec3;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u32;
}

impl EnlightenComponentStateTrait for EnlightenComponentState {
    fn bounce_scale(&self) -> &f32 {
        &self.bounce_scale
    }
    fn sun_scale(&self) -> &f32 {
        &self.sun_scale
    }
    fn sun_direct_lightmap_enable(&self) -> &bool {
        &self.sun_direct_lightmap_enable
    }
    fn terrain_color(&self) -> &super::core::Vec3 {
        &self.terrain_color
    }
    fn cull_distance(&self) -> &f32 {
        &self.cull_distance
    }
    fn cull_radius(&self) -> &f32 {
        &self.cull_radius
    }
    fn sky_box_enable(&self) -> &bool {
        &self.sky_box_enable
    }
    fn sky_box_cut_bottom(&self) -> &bool {
        &self.sky_box_cut_bottom
    }
    fn sky_box_blend_mode(&self) -> &SkyBoxBlendMode {
        &self.sky_box_blend_mode
    }
    fn sky_box_blend(&self) -> &f32 {
        &self.sky_box_blend
    }
    fn sky_box_sky_color(&self) -> &super::core::Vec3 {
        &self.sky_box_sky_color
    }
    fn sky_box_ground_color(&self) -> &super::core::Vec3 {
        &self.sky_box_ground_color
    }
    fn sky_box_sun_light_color(&self) -> &super::core::Vec3 {
        &self.sky_box_sun_light_color
    }
    fn sky_box_sun_light_color_size(&self) -> &f32 {
        &self.sky_box_sun_light_color_size
    }
    fn sky_box_back_light_color(&self) -> &super::core::Vec3 {
        &self.sky_box_back_light_color
    }
    fn sky_box_back_light_color_size(&self) -> &f32 {
        &self.sky_box_back_light_color_size
    }
    fn sky_box_back_light_rotation_x(&self) -> &f32 {
        &self.sky_box_back_light_rotation_x
    }
    fn sky_box_back_light_rotation_y(&self) -> &f32 {
        &self.sky_box_back_light_rotation_y
    }
    fn opaque_alpha_test_simple_scale(&self) -> &super::core::Vec3 {
        &self.opaque_alpha_test_simple_scale
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static ENLIGHTENCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BounceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenComponentState, bounce_scale),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenComponentState, sun_scale),
            },
            FieldInfoData {
                name: "SunDirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenComponentState, sun_direct_lightmap_enable),
            },
            FieldInfoData {
                name: "TerrainColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenComponentState, terrain_color),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenComponentState, cull_distance),
            },
            FieldInfoData {
                name: "CullRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenComponentState, cull_radius),
            },
            FieldInfoData {
                name: "SkyBoxEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_enable),
            },
            FieldInfoData {
                name: "SkyBoxCutBottom",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_cut_bottom),
            },
            FieldInfoData {
                name: "SkyBoxBlendMode",
                flags: MemberInfoFlags::new(0),
                field_type: "SkyBoxBlendMode",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_blend_mode),
            },
            FieldInfoData {
                name: "SkyBoxBlend",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_blend),
            },
            FieldInfoData {
                name: "SkyBoxSkyColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_sky_color),
            },
            FieldInfoData {
                name: "SkyBoxGroundColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_ground_color),
            },
            FieldInfoData {
                name: "SkyBoxSunLightColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_sun_light_color),
            },
            FieldInfoData {
                name: "SkyBoxSunLightColorSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_sun_light_color_size),
            },
            FieldInfoData {
                name: "SkyBoxBackLightColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_back_light_color),
            },
            FieldInfoData {
                name: "SkyBoxBackLightColorSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_back_light_color_size),
            },
            FieldInfoData {
                name: "SkyBoxBackLightRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_back_light_rotation_x),
            },
            FieldInfoData {
                name: "SkyBoxBackLightRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenComponentState, sky_box_back_light_rotation_y),
            },
            FieldInfoData {
                name: "OpaqueAlphaTestSimpleScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenComponentState, opaque_alpha_test_simple_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EnlightenComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EnlightenComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(ENLIGHTENCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnlightenComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENLIGHTENCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EnlightenComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SkyBoxBlendMode {
    #[default]
    SkyBoxBlendMode_Lerp = 0,
    SkyBoxBlendMode_Add = 1,
    SkyBoxBlendMode_Multiply = 2,
}

pub static SKYBOXBLENDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyBoxBlendMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SKYBOXBLENDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SkyBoxBlendMode {
    fn type_info(&self) -> &'static TypeInfo {
        SKYBOXBLENDMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SKYBOXBLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyBoxBlendMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyBoxBlendMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SubSurfaceScatteringComponentState {
    pub simple_sss_color: super::core::Vec3,
    pub simple_sss_rolloff_key_light: f32,
    pub simple_sss_rolloff_local_light: f32,
    pub local_light_translucency_enable: bool,
    pub profile0: super::render_base::SubSurfaceProfile,
    pub profile1: super::render_base::SubSurfaceProfile,
    pub profile2: super::render_base::SubSurfaceProfile,
    pub profile3: super::render_base::SubSurfaceProfile,
    pub profile4: super::render_base::SubSurfaceProfile,
    pub profile5: super::render_base::SubSurfaceProfile,
    pub profile6: super::render_base::SubSurfaceProfile,
    pub profile_o_a_t_s: super::render_base::SubSurfaceProfile,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait SubSurfaceScatteringComponentStateTrait: TypeObject {
    fn simple_sss_color(&self) -> &super::core::Vec3;
    fn simple_sss_rolloff_key_light(&self) -> &f32;
    fn simple_sss_rolloff_local_light(&self) -> &f32;
    fn local_light_translucency_enable(&self) -> &bool;
    fn profile0(&self) -> &super::render_base::SubSurfaceProfile;
    fn profile1(&self) -> &super::render_base::SubSurfaceProfile;
    fn profile2(&self) -> &super::render_base::SubSurfaceProfile;
    fn profile3(&self) -> &super::render_base::SubSurfaceProfile;
    fn profile4(&self) -> &super::render_base::SubSurfaceProfile;
    fn profile5(&self) -> &super::render_base::SubSurfaceProfile;
    fn profile6(&self) -> &super::render_base::SubSurfaceProfile;
    fn profile_o_a_t_s(&self) -> &super::render_base::SubSurfaceProfile;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u16;
}

impl SubSurfaceScatteringComponentStateTrait for SubSurfaceScatteringComponentState {
    fn simple_sss_color(&self) -> &super::core::Vec3 {
        &self.simple_sss_color
    }
    fn simple_sss_rolloff_key_light(&self) -> &f32 {
        &self.simple_sss_rolloff_key_light
    }
    fn simple_sss_rolloff_local_light(&self) -> &f32 {
        &self.simple_sss_rolloff_local_light
    }
    fn local_light_translucency_enable(&self) -> &bool {
        &self.local_light_translucency_enable
    }
    fn profile0(&self) -> &super::render_base::SubSurfaceProfile {
        &self.profile0
    }
    fn profile1(&self) -> &super::render_base::SubSurfaceProfile {
        &self.profile1
    }
    fn profile2(&self) -> &super::render_base::SubSurfaceProfile {
        &self.profile2
    }
    fn profile3(&self) -> &super::render_base::SubSurfaceProfile {
        &self.profile3
    }
    fn profile4(&self) -> &super::render_base::SubSurfaceProfile {
        &self.profile4
    }
    fn profile5(&self) -> &super::render_base::SubSurfaceProfile {
        &self.profile5
    }
    fn profile6(&self) -> &super::render_base::SubSurfaceProfile {
        &self.profile6
    }
    fn profile_o_a_t_s(&self) -> &super::render_base::SubSurfaceProfile {
        &self.profile_o_a_t_s
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static SUBSURFACESCATTERINGCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceScatteringComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubSurfaceScatteringComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SimpleSssColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, simple_sss_color),
            },
            FieldInfoData {
                name: "SimpleSssRolloffKeyLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, simple_sss_rolloff_key_light),
            },
            FieldInfoData {
                name: "SimpleSssRolloffLocalLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, simple_sss_rolloff_local_light),
            },
            FieldInfoData {
                name: "LocalLightTranslucencyEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, local_light_translucency_enable),
            },
            FieldInfoData {
                name: "Profile0",
                flags: MemberInfoFlags::new(0),
                field_type: "SubSurfaceProfile",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile0),
            },
            FieldInfoData {
                name: "Profile1",
                flags: MemberInfoFlags::new(0),
                field_type: "SubSurfaceProfile",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile1),
            },
            FieldInfoData {
                name: "Profile2",
                flags: MemberInfoFlags::new(0),
                field_type: "SubSurfaceProfile",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile2),
            },
            FieldInfoData {
                name: "Profile3",
                flags: MemberInfoFlags::new(0),
                field_type: "SubSurfaceProfile",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile3),
            },
            FieldInfoData {
                name: "Profile4",
                flags: MemberInfoFlags::new(0),
                field_type: "SubSurfaceProfile",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile4),
            },
            FieldInfoData {
                name: "Profile5",
                flags: MemberInfoFlags::new(0),
                field_type: "SubSurfaceProfile",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile5),
            },
            FieldInfoData {
                name: "Profile6",
                flags: MemberInfoFlags::new(0),
                field_type: "SubSurfaceProfile",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile6),
            },
            FieldInfoData {
                name: "ProfileOATS",
                flags: MemberInfoFlags::new(0),
                field_type: "SubSurfaceProfile",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile_o_a_t_s),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SUBSURFACESCATTERINGCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SubSurfaceScatteringComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SUBSURFACESCATTERINGCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SUBSURFACESCATTERINGCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceScatteringComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SubSurfaceScatteringComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DynamicAOComponentState {
    pub enable: bool,
    pub affect_outdoor_light: bool,
    pub affect_local_light: bool,
    pub dynamic_a_o_factor: f32,
    pub ssao_fade: f32,
    pub ssao_radius: f32,
    pub ssao_max_distance_inner: f32,
    pub ssao_max_distance_outer: f32,
    pub hbao_radius: f32,
    pub hbao_angle_bias: f32,
    pub hbao_attenuation: f32,
    pub hbao_contrast: f32,
    pub hbao_max_footprint_radius: f32,
    pub hbao_power_exponent: f32,
    pub hbao_blur_radius: f32,
    pub hbao_blur_sharpness: f32,
    pub temporal_filter_enable: bool,
    pub aao_dynamic_weight: bool,
    pub aao_bias: f32,
    pub aao_intensity: f32,
    pub aao_contrast: f32,
    pub aao_range_reduction: f32,
    pub aao_screen_radius: f32,
    pub aao_near_occlusion_max: f32,
    pub aao_near_falloff_threshold: f32,
    pub aao_clip_distance: f32,
    pub aao_clip_fade_distance: f32,
    pub aao_blur_depth_threshold: f32,
    pub aao_blur_const_falloff: f32,
    pub field_flag_override0: u32,
    pub field_flag_changed0: u32,
}

pub trait DynamicAOComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn affect_outdoor_light(&self) -> &bool;
    fn affect_local_light(&self) -> &bool;
    fn dynamic_a_o_factor(&self) -> &f32;
    fn ssao_fade(&self) -> &f32;
    fn ssao_radius(&self) -> &f32;
    fn ssao_max_distance_inner(&self) -> &f32;
    fn ssao_max_distance_outer(&self) -> &f32;
    fn hbao_radius(&self) -> &f32;
    fn hbao_angle_bias(&self) -> &f32;
    fn hbao_attenuation(&self) -> &f32;
    fn hbao_contrast(&self) -> &f32;
    fn hbao_max_footprint_radius(&self) -> &f32;
    fn hbao_power_exponent(&self) -> &f32;
    fn hbao_blur_radius(&self) -> &f32;
    fn hbao_blur_sharpness(&self) -> &f32;
    fn temporal_filter_enable(&self) -> &bool;
    fn aao_dynamic_weight(&self) -> &bool;
    fn aao_bias(&self) -> &f32;
    fn aao_intensity(&self) -> &f32;
    fn aao_contrast(&self) -> &f32;
    fn aao_range_reduction(&self) -> &f32;
    fn aao_screen_radius(&self) -> &f32;
    fn aao_near_occlusion_max(&self) -> &f32;
    fn aao_near_falloff_threshold(&self) -> &f32;
    fn aao_clip_distance(&self) -> &f32;
    fn aao_clip_fade_distance(&self) -> &f32;
    fn aao_blur_depth_threshold(&self) -> &f32;
    fn aao_blur_const_falloff(&self) -> &f32;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u32;
}

impl DynamicAOComponentStateTrait for DynamicAOComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn affect_outdoor_light(&self) -> &bool {
        &self.affect_outdoor_light
    }
    fn affect_local_light(&self) -> &bool {
        &self.affect_local_light
    }
    fn dynamic_a_o_factor(&self) -> &f32 {
        &self.dynamic_a_o_factor
    }
    fn ssao_fade(&self) -> &f32 {
        &self.ssao_fade
    }
    fn ssao_radius(&self) -> &f32 {
        &self.ssao_radius
    }
    fn ssao_max_distance_inner(&self) -> &f32 {
        &self.ssao_max_distance_inner
    }
    fn ssao_max_distance_outer(&self) -> &f32 {
        &self.ssao_max_distance_outer
    }
    fn hbao_radius(&self) -> &f32 {
        &self.hbao_radius
    }
    fn hbao_angle_bias(&self) -> &f32 {
        &self.hbao_angle_bias
    }
    fn hbao_attenuation(&self) -> &f32 {
        &self.hbao_attenuation
    }
    fn hbao_contrast(&self) -> &f32 {
        &self.hbao_contrast
    }
    fn hbao_max_footprint_radius(&self) -> &f32 {
        &self.hbao_max_footprint_radius
    }
    fn hbao_power_exponent(&self) -> &f32 {
        &self.hbao_power_exponent
    }
    fn hbao_blur_radius(&self) -> &f32 {
        &self.hbao_blur_radius
    }
    fn hbao_blur_sharpness(&self) -> &f32 {
        &self.hbao_blur_sharpness
    }
    fn temporal_filter_enable(&self) -> &bool {
        &self.temporal_filter_enable
    }
    fn aao_dynamic_weight(&self) -> &bool {
        &self.aao_dynamic_weight
    }
    fn aao_bias(&self) -> &f32 {
        &self.aao_bias
    }
    fn aao_intensity(&self) -> &f32 {
        &self.aao_intensity
    }
    fn aao_contrast(&self) -> &f32 {
        &self.aao_contrast
    }
    fn aao_range_reduction(&self) -> &f32 {
        &self.aao_range_reduction
    }
    fn aao_screen_radius(&self) -> &f32 {
        &self.aao_screen_radius
    }
    fn aao_near_occlusion_max(&self) -> &f32 {
        &self.aao_near_occlusion_max
    }
    fn aao_near_falloff_threshold(&self) -> &f32 {
        &self.aao_near_falloff_threshold
    }
    fn aao_clip_distance(&self) -> &f32 {
        &self.aao_clip_distance
    }
    fn aao_clip_fade_distance(&self) -> &f32 {
        &self.aao_clip_fade_distance
    }
    fn aao_blur_depth_threshold(&self) -> &f32 {
        &self.aao_blur_depth_threshold
    }
    fn aao_blur_const_falloff(&self) -> &f32 {
        &self.aao_blur_const_falloff
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static DYNAMICAOCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicAOComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DynamicAOComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DynamicAOComponentState, enable),
            },
            FieldInfoData {
                name: "AffectOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DynamicAOComponentState, affect_outdoor_light),
            },
            FieldInfoData {
                name: "AffectLocalLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DynamicAOComponentState, affect_local_light),
            },
            FieldInfoData {
                name: "DynamicAOFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, dynamic_a_o_factor),
            },
            FieldInfoData {
                name: "SsaoFade",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, ssao_fade),
            },
            FieldInfoData {
                name: "SsaoRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, ssao_radius),
            },
            FieldInfoData {
                name: "SsaoMaxDistanceInner",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, ssao_max_distance_inner),
            },
            FieldInfoData {
                name: "SsaoMaxDistanceOuter",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, ssao_max_distance_outer),
            },
            FieldInfoData {
                name: "HbaoRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, hbao_radius),
            },
            FieldInfoData {
                name: "HbaoAngleBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, hbao_angle_bias),
            },
            FieldInfoData {
                name: "HbaoAttenuation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, hbao_attenuation),
            },
            FieldInfoData {
                name: "HbaoContrast",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, hbao_contrast),
            },
            FieldInfoData {
                name: "HbaoMaxFootprintRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, hbao_max_footprint_radius),
            },
            FieldInfoData {
                name: "HbaoPowerExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, hbao_power_exponent),
            },
            FieldInfoData {
                name: "HbaoBlurRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, hbao_blur_radius),
            },
            FieldInfoData {
                name: "HbaoBlurSharpness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, hbao_blur_sharpness),
            },
            FieldInfoData {
                name: "TemporalFilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DynamicAOComponentState, temporal_filter_enable),
            },
            FieldInfoData {
                name: "AaoDynamicWeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DynamicAOComponentState, aao_dynamic_weight),
            },
            FieldInfoData {
                name: "AaoBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_bias),
            },
            FieldInfoData {
                name: "AaoIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_intensity),
            },
            FieldInfoData {
                name: "AaoContrast",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_contrast),
            },
            FieldInfoData {
                name: "AaoRangeReduction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_range_reduction),
            },
            FieldInfoData {
                name: "AaoScreenRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_screen_radius),
            },
            FieldInfoData {
                name: "AaoNearOcclusionMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_near_occlusion_max),
            },
            FieldInfoData {
                name: "AaoNearFalloffThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_near_falloff_threshold),
            },
            FieldInfoData {
                name: "AaoClipDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_clip_distance),
            },
            FieldInfoData {
                name: "AaoClipFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_clip_fade_distance),
            },
            FieldInfoData {
                name: "AaoBlurDepthThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_blur_depth_threshold),
            },
            FieldInfoData {
                name: "AaoBlurConstFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DynamicAOComponentState, aao_blur_const_falloff),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DynamicAOComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DynamicAOComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DYNAMICAOCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DynamicAOComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        DYNAMICAOCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DYNAMICAOCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicAOComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DynamicAOComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SunFlareComponentState {
    pub enable: bool,
    pub debug_draw_occluder: bool,
    pub occluder_size: f32,
    pub screen_clip: bool,
    pub render_mode: LensFlareRenderMode,
    pub use_sun_position: bool,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub element1_enable: bool,
    pub element1_shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub element1_ray_distance: f32,
    pub element1_size: super::core::Vec2,
    pub element1_size_occluder_curve: super::core::Vec4,
    pub element1_size_screen_pos_curve: super::core::Vec4,
    pub element1_alpha_occluder_curve: super::core::Vec4,
    pub element1_alpha_screen_pos_curve: super::core::Vec4,
    pub element1_rotation_local: f32,
    pub element1_rotation_aligned_to_ray: bool,
    pub element1_rotation_dist_curve: super::core::Vec4,
    pub element1_rotation_dist_multiplier: f32,
    pub element2_enable: bool,
    pub element2_shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub element2_ray_distance: f32,
    pub element2_size: super::core::Vec2,
    pub element2_size_occluder_curve: super::core::Vec4,
    pub element2_size_screen_pos_curve: super::core::Vec4,
    pub element2_alpha_occluder_curve: super::core::Vec4,
    pub element2_alpha_screen_pos_curve: super::core::Vec4,
    pub element2_rotation_local: f32,
    pub element2_rotation_aligned_to_ray: bool,
    pub element2_rotation_dist_curve: super::core::Vec4,
    pub element2_rotation_dist_multiplier: f32,
    pub element3_enable: bool,
    pub element3_shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub element3_ray_distance: f32,
    pub element3_size: super::core::Vec2,
    pub element3_size_occluder_curve: super::core::Vec4,
    pub element3_size_screen_pos_curve: super::core::Vec4,
    pub element3_alpha_occluder_curve: super::core::Vec4,
    pub element3_alpha_screen_pos_curve: super::core::Vec4,
    pub element3_rotation_local: f32,
    pub element3_rotation_aligned_to_ray: bool,
    pub element3_rotation_dist_curve: super::core::Vec4,
    pub element3_rotation_dist_multiplier: f32,
    pub element4_enable: bool,
    pub element4_shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub element4_ray_distance: f32,
    pub element4_size: super::core::Vec2,
    pub element4_size_occluder_curve: super::core::Vec4,
    pub element4_size_screen_pos_curve: super::core::Vec4,
    pub element4_alpha_occluder_curve: super::core::Vec4,
    pub element4_alpha_screen_pos_curve: super::core::Vec4,
    pub element4_rotation_local: f32,
    pub element4_rotation_aligned_to_ray: bool,
    pub element4_rotation_dist_curve: super::core::Vec4,
    pub element4_rotation_dist_multiplier: f32,
    pub element5_enable: bool,
    pub element5_shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub element5_ray_distance: f32,
    pub element5_size: super::core::Vec2,
    pub element5_size_occluder_curve: super::core::Vec4,
    pub element5_size_screen_pos_curve: super::core::Vec4,
    pub element5_alpha_occluder_curve: super::core::Vec4,
    pub element5_alpha_screen_pos_curve: super::core::Vec4,
    pub element5_rotation_local: f32,
    pub element5_rotation_aligned_to_ray: bool,
    pub element5_rotation_dist_curve: super::core::Vec4,
    pub element5_rotation_dist_multiplier: f32,
    pub field_flag_override0: u32,
    pub field_flag_override1: u32,
    pub field_flag_override2: u8,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u32,
    pub field_flag_changed2: u8,
}

pub trait SunFlareComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn debug_draw_occluder(&self) -> &bool;
    fn occluder_size(&self) -> &f32;
    fn screen_clip(&self) -> &bool;
    fn render_mode(&self) -> &LensFlareRenderMode;
    fn use_sun_position(&self) -> &bool;
    fn rotation_x(&self) -> &f32;
    fn rotation_y(&self) -> &f32;
    fn element1_enable(&self) -> &bool;
    fn element1_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn element1_ray_distance(&self) -> &f32;
    fn element1_size(&self) -> &super::core::Vec2;
    fn element1_size_occluder_curve(&self) -> &super::core::Vec4;
    fn element1_size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element1_alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn element1_alpha_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element1_rotation_local(&self) -> &f32;
    fn element1_rotation_aligned_to_ray(&self) -> &bool;
    fn element1_rotation_dist_curve(&self) -> &super::core::Vec4;
    fn element1_rotation_dist_multiplier(&self) -> &f32;
    fn element2_enable(&self) -> &bool;
    fn element2_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn element2_ray_distance(&self) -> &f32;
    fn element2_size(&self) -> &super::core::Vec2;
    fn element2_size_occluder_curve(&self) -> &super::core::Vec4;
    fn element2_size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element2_alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn element2_alpha_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element2_rotation_local(&self) -> &f32;
    fn element2_rotation_aligned_to_ray(&self) -> &bool;
    fn element2_rotation_dist_curve(&self) -> &super::core::Vec4;
    fn element2_rotation_dist_multiplier(&self) -> &f32;
    fn element3_enable(&self) -> &bool;
    fn element3_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn element3_ray_distance(&self) -> &f32;
    fn element3_size(&self) -> &super::core::Vec2;
    fn element3_size_occluder_curve(&self) -> &super::core::Vec4;
    fn element3_size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element3_alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn element3_alpha_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element3_rotation_local(&self) -> &f32;
    fn element3_rotation_aligned_to_ray(&self) -> &bool;
    fn element3_rotation_dist_curve(&self) -> &super::core::Vec4;
    fn element3_rotation_dist_multiplier(&self) -> &f32;
    fn element4_enable(&self) -> &bool;
    fn element4_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn element4_ray_distance(&self) -> &f32;
    fn element4_size(&self) -> &super::core::Vec2;
    fn element4_size_occluder_curve(&self) -> &super::core::Vec4;
    fn element4_size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element4_alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn element4_alpha_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element4_rotation_local(&self) -> &f32;
    fn element4_rotation_aligned_to_ray(&self) -> &bool;
    fn element4_rotation_dist_curve(&self) -> &super::core::Vec4;
    fn element4_rotation_dist_multiplier(&self) -> &f32;
    fn element5_enable(&self) -> &bool;
    fn element5_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn element5_ray_distance(&self) -> &f32;
    fn element5_size(&self) -> &super::core::Vec2;
    fn element5_size_occluder_curve(&self) -> &super::core::Vec4;
    fn element5_size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element5_alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn element5_alpha_screen_pos_curve(&self) -> &super::core::Vec4;
    fn element5_rotation_local(&self) -> &f32;
    fn element5_rotation_aligned_to_ray(&self) -> &bool;
    fn element5_rotation_dist_curve(&self) -> &super::core::Vec4;
    fn element5_rotation_dist_multiplier(&self) -> &f32;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_override1(&self) -> &u32;
    fn field_flag_override2(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u32;
    fn field_flag_changed2(&self) -> &u8;
}

impl SunFlareComponentStateTrait for SunFlareComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn debug_draw_occluder(&self) -> &bool {
        &self.debug_draw_occluder
    }
    fn occluder_size(&self) -> &f32 {
        &self.occluder_size
    }
    fn screen_clip(&self) -> &bool {
        &self.screen_clip
    }
    fn render_mode(&self) -> &LensFlareRenderMode {
        &self.render_mode
    }
    fn use_sun_position(&self) -> &bool {
        &self.use_sun_position
    }
    fn rotation_x(&self) -> &f32 {
        &self.rotation_x
    }
    fn rotation_y(&self) -> &f32 {
        &self.rotation_y
    }
    fn element1_enable(&self) -> &bool {
        &self.element1_enable
    }
    fn element1_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.element1_shader
    }
    fn element1_ray_distance(&self) -> &f32 {
        &self.element1_ray_distance
    }
    fn element1_size(&self) -> &super::core::Vec2 {
        &self.element1_size
    }
    fn element1_size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element1_size_occluder_curve
    }
    fn element1_size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element1_size_screen_pos_curve
    }
    fn element1_alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element1_alpha_occluder_curve
    }
    fn element1_alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element1_alpha_screen_pos_curve
    }
    fn element1_rotation_local(&self) -> &f32 {
        &self.element1_rotation_local
    }
    fn element1_rotation_aligned_to_ray(&self) -> &bool {
        &self.element1_rotation_aligned_to_ray
    }
    fn element1_rotation_dist_curve(&self) -> &super::core::Vec4 {
        &self.element1_rotation_dist_curve
    }
    fn element1_rotation_dist_multiplier(&self) -> &f32 {
        &self.element1_rotation_dist_multiplier
    }
    fn element2_enable(&self) -> &bool {
        &self.element2_enable
    }
    fn element2_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.element2_shader
    }
    fn element2_ray_distance(&self) -> &f32 {
        &self.element2_ray_distance
    }
    fn element2_size(&self) -> &super::core::Vec2 {
        &self.element2_size
    }
    fn element2_size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element2_size_occluder_curve
    }
    fn element2_size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element2_size_screen_pos_curve
    }
    fn element2_alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element2_alpha_occluder_curve
    }
    fn element2_alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element2_alpha_screen_pos_curve
    }
    fn element2_rotation_local(&self) -> &f32 {
        &self.element2_rotation_local
    }
    fn element2_rotation_aligned_to_ray(&self) -> &bool {
        &self.element2_rotation_aligned_to_ray
    }
    fn element2_rotation_dist_curve(&self) -> &super::core::Vec4 {
        &self.element2_rotation_dist_curve
    }
    fn element2_rotation_dist_multiplier(&self) -> &f32 {
        &self.element2_rotation_dist_multiplier
    }
    fn element3_enable(&self) -> &bool {
        &self.element3_enable
    }
    fn element3_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.element3_shader
    }
    fn element3_ray_distance(&self) -> &f32 {
        &self.element3_ray_distance
    }
    fn element3_size(&self) -> &super::core::Vec2 {
        &self.element3_size
    }
    fn element3_size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element3_size_occluder_curve
    }
    fn element3_size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element3_size_screen_pos_curve
    }
    fn element3_alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element3_alpha_occluder_curve
    }
    fn element3_alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element3_alpha_screen_pos_curve
    }
    fn element3_rotation_local(&self) -> &f32 {
        &self.element3_rotation_local
    }
    fn element3_rotation_aligned_to_ray(&self) -> &bool {
        &self.element3_rotation_aligned_to_ray
    }
    fn element3_rotation_dist_curve(&self) -> &super::core::Vec4 {
        &self.element3_rotation_dist_curve
    }
    fn element3_rotation_dist_multiplier(&self) -> &f32 {
        &self.element3_rotation_dist_multiplier
    }
    fn element4_enable(&self) -> &bool {
        &self.element4_enable
    }
    fn element4_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.element4_shader
    }
    fn element4_ray_distance(&self) -> &f32 {
        &self.element4_ray_distance
    }
    fn element4_size(&self) -> &super::core::Vec2 {
        &self.element4_size
    }
    fn element4_size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element4_size_occluder_curve
    }
    fn element4_size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element4_size_screen_pos_curve
    }
    fn element4_alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element4_alpha_occluder_curve
    }
    fn element4_alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element4_alpha_screen_pos_curve
    }
    fn element4_rotation_local(&self) -> &f32 {
        &self.element4_rotation_local
    }
    fn element4_rotation_aligned_to_ray(&self) -> &bool {
        &self.element4_rotation_aligned_to_ray
    }
    fn element4_rotation_dist_curve(&self) -> &super::core::Vec4 {
        &self.element4_rotation_dist_curve
    }
    fn element4_rotation_dist_multiplier(&self) -> &f32 {
        &self.element4_rotation_dist_multiplier
    }
    fn element5_enable(&self) -> &bool {
        &self.element5_enable
    }
    fn element5_shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.element5_shader
    }
    fn element5_ray_distance(&self) -> &f32 {
        &self.element5_ray_distance
    }
    fn element5_size(&self) -> &super::core::Vec2 {
        &self.element5_size
    }
    fn element5_size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element5_size_occluder_curve
    }
    fn element5_size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element5_size_screen_pos_curve
    }
    fn element5_alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.element5_alpha_occluder_curve
    }
    fn element5_alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.element5_alpha_screen_pos_curve
    }
    fn element5_rotation_local(&self) -> &f32 {
        &self.element5_rotation_local
    }
    fn element5_rotation_aligned_to_ray(&self) -> &bool {
        &self.element5_rotation_aligned_to_ray
    }
    fn element5_rotation_dist_curve(&self) -> &super::core::Vec4 {
        &self.element5_rotation_dist_curve
    }
    fn element5_rotation_dist_multiplier(&self) -> &f32 {
        &self.element5_rotation_dist_multiplier
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_override1(&self) -> &u32 {
        &self.field_flag_override1
    }
    fn field_flag_override2(&self) -> &u8 {
        &self.field_flag_override2
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u32 {
        &self.field_flag_changed1
    }
    fn field_flag_changed2(&self) -> &u8 {
        &self.field_flag_changed2
    }
}

pub static SUNFLARECOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SunFlareComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, enable),
            },
            FieldInfoData {
                name: "DebugDrawOccluder",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, debug_draw_occluder),
            },
            FieldInfoData {
                name: "OccluderSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, occluder_size),
            },
            FieldInfoData {
                name: "ScreenClip",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, screen_clip),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: "LensFlareRenderMode",
                rust_offset: offset_of!(SunFlareComponentState, render_mode),
            },
            FieldInfoData {
                name: "UseSunPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, use_sun_position),
            },
            FieldInfoData {
                name: "RotationX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, rotation_x),
            },
            FieldInfoData {
                name: "RotationY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, rotation_y),
            },
            FieldInfoData {
                name: "Element1Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, element1_enable),
            },
            FieldInfoData {
                name: "Element1Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(SunFlareComponentState, element1_shader),
            },
            FieldInfoData {
                name: "Element1RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element1_ray_distance),
            },
            FieldInfoData {
                name: "Element1Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SunFlareComponentState, element1_size),
            },
            FieldInfoData {
                name: "Element1SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element1_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element1SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element1_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element1AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element1_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element1AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element1_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element1RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element1_rotation_local),
            },
            FieldInfoData {
                name: "Element1RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, element1_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element1RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element1_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element1RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element1_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element2Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, element2_enable),
            },
            FieldInfoData {
                name: "Element2Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(SunFlareComponentState, element2_shader),
            },
            FieldInfoData {
                name: "Element2RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element2_ray_distance),
            },
            FieldInfoData {
                name: "Element2Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SunFlareComponentState, element2_size),
            },
            FieldInfoData {
                name: "Element2SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element2_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element2SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element2_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element2AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element2_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element2AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element2_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element2RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element2_rotation_local),
            },
            FieldInfoData {
                name: "Element2RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, element2_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element2RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element2_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element2RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element2_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element3Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, element3_enable),
            },
            FieldInfoData {
                name: "Element3Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(SunFlareComponentState, element3_shader),
            },
            FieldInfoData {
                name: "Element3RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element3_ray_distance),
            },
            FieldInfoData {
                name: "Element3Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SunFlareComponentState, element3_size),
            },
            FieldInfoData {
                name: "Element3SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element3_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element3SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element3_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element3AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element3_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element3AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element3_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element3RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element3_rotation_local),
            },
            FieldInfoData {
                name: "Element3RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, element3_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element3RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element3_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element3RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element3_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element4Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, element4_enable),
            },
            FieldInfoData {
                name: "Element4Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(SunFlareComponentState, element4_shader),
            },
            FieldInfoData {
                name: "Element4RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element4_ray_distance),
            },
            FieldInfoData {
                name: "Element4Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SunFlareComponentState, element4_size),
            },
            FieldInfoData {
                name: "Element4SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element4_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element4SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element4_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element4AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element4_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element4AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element4_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element4RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element4_rotation_local),
            },
            FieldInfoData {
                name: "Element4RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, element4_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element4RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element4_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element4RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element4_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element5Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, element5_enable),
            },
            FieldInfoData {
                name: "Element5Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(SunFlareComponentState, element5_shader),
            },
            FieldInfoData {
                name: "Element5RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element5_ray_distance),
            },
            FieldInfoData {
                name: "Element5Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(SunFlareComponentState, element5_size),
            },
            FieldInfoData {
                name: "Element5SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element5_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element5SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element5_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element5AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element5_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element5AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element5_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element5RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element5_rotation_local),
            },
            FieldInfoData {
                name: "Element5RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SunFlareComponentState, element5_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element5RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SunFlareComponentState, element5_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element5RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SunFlareComponentState, element5_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SunFlareComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SunFlareComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagOverride2",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SunFlareComponentState, field_flag_override2),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SunFlareComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SunFlareComponentState, field_flag_changed1),
            },
            FieldInfoData {
                name: "FieldFlagChanged2",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SunFlareComponentState, field_flag_changed2),
            },
        ],
    }),
    array_type: Some(SUNFLARECOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SunFlareComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SUNFLARECOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SUNFLARECOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SunFlareComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WindComponentState {
    pub wind_direction: f32,
    pub wind_strength: f32,
    pub wind_variation_multiplier: f32,
    pub wind_variation_rate_multiplier: f32,
    pub wind_micro_variation_multiplier: f32,
    pub turbulence_multiplier: f32,
    pub turbulence_scale: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait WindComponentStateTrait: TypeObject {
    fn wind_direction(&self) -> &f32;
    fn wind_strength(&self) -> &f32;
    fn wind_variation_multiplier(&self) -> &f32;
    fn wind_variation_rate_multiplier(&self) -> &f32;
    fn wind_micro_variation_multiplier(&self) -> &f32;
    fn turbulence_multiplier(&self) -> &f32;
    fn turbulence_scale(&self) -> &f32;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl WindComponentStateTrait for WindComponentState {
    fn wind_direction(&self) -> &f32 {
        &self.wind_direction
    }
    fn wind_strength(&self) -> &f32 {
        &self.wind_strength
    }
    fn wind_variation_multiplier(&self) -> &f32 {
        &self.wind_variation_multiplier
    }
    fn wind_variation_rate_multiplier(&self) -> &f32 {
        &self.wind_variation_rate_multiplier
    }
    fn wind_micro_variation_multiplier(&self) -> &f32 {
        &self.wind_micro_variation_multiplier
    }
    fn turbulence_multiplier(&self) -> &f32 {
        &self.turbulence_multiplier
    }
    fn turbulence_scale(&self) -> &f32 {
        &self.turbulence_scale
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static WINDCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WindComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "WindDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WindComponentState, wind_direction),
            },
            FieldInfoData {
                name: "WindStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WindComponentState, wind_strength),
            },
            FieldInfoData {
                name: "WindVariationMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WindComponentState, wind_variation_multiplier),
            },
            FieldInfoData {
                name: "WindVariationRateMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WindComponentState, wind_variation_rate_multiplier),
            },
            FieldInfoData {
                name: "WindMicroVariationMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WindComponentState, wind_micro_variation_multiplier),
            },
            FieldInfoData {
                name: "TurbulenceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WindComponentState, turbulence_multiplier),
            },
            FieldInfoData {
                name: "TurbulenceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WindComponentState, turbulence_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(WindComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(WindComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WINDCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WindComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        WINDCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WINDCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("WindComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DynamicEnvmapComponentState {
    pub enable: bool,
    pub terrain_reflections_enable: bool,
    pub key_color_envmap: super::core::Vec3,
    pub sky_color_envmap: super::core::Vec3,
    pub ground_color_envmap: super::core::Vec3,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait DynamicEnvmapComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn terrain_reflections_enable(&self) -> &bool;
    fn key_color_envmap(&self) -> &super::core::Vec3;
    fn sky_color_envmap(&self) -> &super::core::Vec3;
    fn ground_color_envmap(&self) -> &super::core::Vec3;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl DynamicEnvmapComponentStateTrait for DynamicEnvmapComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn terrain_reflections_enable(&self) -> &bool {
        &self.terrain_reflections_enable
    }
    fn key_color_envmap(&self) -> &super::core::Vec3 {
        &self.key_color_envmap
    }
    fn sky_color_envmap(&self) -> &super::core::Vec3 {
        &self.sky_color_envmap
    }
    fn ground_color_envmap(&self) -> &super::core::Vec3 {
        &self.ground_color_envmap
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static DYNAMICENVMAPCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnvmapComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DynamicEnvmapComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DynamicEnvmapComponentState, enable),
            },
            FieldInfoData {
                name: "TerrainReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DynamicEnvmapComponentState, terrain_reflections_enable),
            },
            FieldInfoData {
                name: "KeyColorEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DynamicEnvmapComponentState, key_color_envmap),
            },
            FieldInfoData {
                name: "SkyColorEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DynamicEnvmapComponentState, sky_color_envmap),
            },
            FieldInfoData {
                name: "GroundColorEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DynamicEnvmapComponentState, ground_color_envmap),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DynamicEnvmapComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DynamicEnvmapComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DYNAMICENVMAPCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DynamicEnvmapComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        DYNAMICENVMAPCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DYNAMICENVMAPCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnvmapComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DynamicEnvmapComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlanarReflectionComponentState {
    pub enable: bool,
    pub terrain_reflections_enable: bool,
    pub sky_render_enable: bool,
    pub ground_height: f32,
    pub view_distance: f32,
    pub vertical_blur_filter: super::render_base::BlurFilter,
    pub vertical_deviation: f32,
    pub horizontal_blur_filter: super::render_base::BlurFilter,
    pub horizontal_deviation: f32,
    pub clipping_offset: f32,
    pub overide_outdoor_light_colors: bool,
    pub key_color_reflection: super::core::Vec3,
    pub sky_color_reflection: super::core::Vec3,
    pub ground_color_reflection: super::core::Vec3,
    pub field_flag_override0: u16,
    pub field_flag_changed0: u16,
}

pub trait PlanarReflectionComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn terrain_reflections_enable(&self) -> &bool;
    fn sky_render_enable(&self) -> &bool;
    fn ground_height(&self) -> &f32;
    fn view_distance(&self) -> &f32;
    fn vertical_blur_filter(&self) -> &super::render_base::BlurFilter;
    fn vertical_deviation(&self) -> &f32;
    fn horizontal_blur_filter(&self) -> &super::render_base::BlurFilter;
    fn horizontal_deviation(&self) -> &f32;
    fn clipping_offset(&self) -> &f32;
    fn overide_outdoor_light_colors(&self) -> &bool;
    fn key_color_reflection(&self) -> &super::core::Vec3;
    fn sky_color_reflection(&self) -> &super::core::Vec3;
    fn ground_color_reflection(&self) -> &super::core::Vec3;
    fn field_flag_override0(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u16;
}

impl PlanarReflectionComponentStateTrait for PlanarReflectionComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn terrain_reflections_enable(&self) -> &bool {
        &self.terrain_reflections_enable
    }
    fn sky_render_enable(&self) -> &bool {
        &self.sky_render_enable
    }
    fn ground_height(&self) -> &f32 {
        &self.ground_height
    }
    fn view_distance(&self) -> &f32 {
        &self.view_distance
    }
    fn vertical_blur_filter(&self) -> &super::render_base::BlurFilter {
        &self.vertical_blur_filter
    }
    fn vertical_deviation(&self) -> &f32 {
        &self.vertical_deviation
    }
    fn horizontal_blur_filter(&self) -> &super::render_base::BlurFilter {
        &self.horizontal_blur_filter
    }
    fn horizontal_deviation(&self) -> &f32 {
        &self.horizontal_deviation
    }
    fn clipping_offset(&self) -> &f32 {
        &self.clipping_offset
    }
    fn overide_outdoor_light_colors(&self) -> &bool {
        &self.overide_outdoor_light_colors
    }
    fn key_color_reflection(&self) -> &super::core::Vec3 {
        &self.key_color_reflection
    }
    fn sky_color_reflection(&self) -> &super::core::Vec3 {
        &self.sky_color_reflection
    }
    fn ground_color_reflection(&self) -> &super::core::Vec3 {
        &self.ground_color_reflection
    }
    fn field_flag_override0(&self) -> &u16 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static PLANARREFLECTIONCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlanarReflectionComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlanarReflectionComponentState, enable),
            },
            FieldInfoData {
                name: "TerrainReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlanarReflectionComponentState, terrain_reflections_enable),
            },
            FieldInfoData {
                name: "SkyRenderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlanarReflectionComponentState, sky_render_enable),
            },
            FieldInfoData {
                name: "GroundHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlanarReflectionComponentState, ground_height),
            },
            FieldInfoData {
                name: "ViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlanarReflectionComponentState, view_distance),
            },
            FieldInfoData {
                name: "VerticalBlurFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "BlurFilter",
                rust_offset: offset_of!(PlanarReflectionComponentState, vertical_blur_filter),
            },
            FieldInfoData {
                name: "VerticalDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlanarReflectionComponentState, vertical_deviation),
            },
            FieldInfoData {
                name: "HorizontalBlurFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "BlurFilter",
                rust_offset: offset_of!(PlanarReflectionComponentState, horizontal_blur_filter),
            },
            FieldInfoData {
                name: "HorizontalDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlanarReflectionComponentState, horizontal_deviation),
            },
            FieldInfoData {
                name: "ClippingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlanarReflectionComponentState, clipping_offset),
            },
            FieldInfoData {
                name: "OverideOutdoorLightColors",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlanarReflectionComponentState, overide_outdoor_light_colors),
            },
            FieldInfoData {
                name: "KeyColorReflection",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PlanarReflectionComponentState, key_color_reflection),
            },
            FieldInfoData {
                name: "SkyColorReflection",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PlanarReflectionComponentState, sky_color_reflection),
            },
            FieldInfoData {
                name: "GroundColorReflection",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PlanarReflectionComponentState, ground_color_reflection),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(PlanarReflectionComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(PlanarReflectionComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PLANARREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PlanarReflectionComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        PLANARREFLECTIONCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLANARREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PlanarReflectionComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SkyComponentState {
    pub enable: bool,
    pub draw_sky_geo: bool,
    pub sky_type: SkyType,
    pub luminance_scale: f32,
    pub sky_gradient_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub alpha_output: AlphaOutputMode,
    pub use_sky_visibility_as_a_o: bool,
    pub hdri_rotation: f32,
    pub hdri_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub sun_size: f32,
    pub sun_scale: f32,
    pub panoramic_u_v_min_x: f32,
    pub panoramic_u_v_max_x: f32,
    pub panoramic_u_v_min_y: f32,
    pub panoramic_u_v_max_y: f32,
    pub panoramic_tile_factor: f32,
    pub panoramic_rotation: f32,
    pub panoramic_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub panoramic_alpha_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub sky_gradient_follows_panoramic_u_vs: bool,
    pub flow_enable: bool,
    pub flow_period: f32,
    pub flow_distance: f32,
    pub flow_direction: f32,
    pub flow_height_mask_scale: f32,
    pub flow_height_mask_bias: f32,
    pub flow_mask_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub cloud_layer_sun_color: super::core::Vec3,
    pub cloud_layer_mask_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub cloud_layer1_altitude: f32,
    pub cloud_layer1_tile_factor: f32,
    pub cloud_layer1_rotation: f32,
    pub cloud_layer1_speed: f32,
    pub cloud_layer1_sun_light_intensity: f32,
    pub cloud_layer1_sun_light_power: f32,
    pub cloud_layer1_ambient_light_intensity: f32,
    pub cloud_layer1_color: super::core::Vec3,
    pub cloud_layer1_alpha_mul: f32,
    pub cloud_layer1_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub cloud_layer1_absorption: f32,
    pub cloud_layer1_scattering: f32,
    pub cloud_layer1_phase: f32,
    pub cloud_layer1_thickness: f32,
    pub cloud_layer2_altitude: f32,
    pub cloud_layer2_tile_factor: f32,
    pub cloud_layer2_rotation: f32,
    pub cloud_layer2_speed: f32,
    pub cloud_layer2_sun_light_intensity: f32,
    pub cloud_layer2_sun_light_power: f32,
    pub cloud_layer2_ambient_light_intensity: f32,
    pub cloud_layer2_color: super::core::Vec3,
    pub cloud_layer2_alpha_mul: f32,
    pub cloud_layer2_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub cloud_layer2_absorption: f32,
    pub cloud_layer2_scattering: f32,
    pub cloud_layer2_phase: f32,
    pub cloud_layer2_thickness: f32,
    pub static_envmap_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub static_envmap_scale: f32,
    pub sky_envmap8_bit_tex_scale: f32,
    pub custom_envmap_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub custom_envmap_scale: f32,
    pub custom_envmap_ambient: f32,
    pub sky_visibility_exponent: f32,
    pub interior_envmap_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub interior_envmap_exp: super::core::Vec4,
    pub interior_envmap_scale: super::core::Vec4,
    pub interior_envmap_bias: super::core::Vec4,
    pub interior_envmap_sky_visibility_fade_start: f32,
    pub interior_envmap_sky_visibility_fade_length: f32,
    pub earth_radius: f32,
    pub atmosphere_radius: f32,
    pub mie_scattering_coefficient: f32,
    pub mie_g: f32,
    pub mie_extinction_coefficient_relation: f32,
    pub scale_height_mie: f32,
    pub rayleigh_scattering_coefficient: super::core::Vec3,
    pub rayleigh_scattering_coefficient_scale: f32,
    pub rayleigh_extinction_coefficient_relation: f32,
    pub scale_height_rayleigh: f32,
    pub use_ozone: bool,
    pub ozone_percentage: f32,
    pub use_aerial_perspective: bool,
    pub aerial_perspective_scale: f32,
    pub aerial_perspective_intensity: f32,
    pub aerial_perspective_dithering: f32,
    pub light1_color: super::core::Vec3,
    pub light1_intensity: f32,
    pub light1_follow_outdoor_light: bool,
    pub light1_takes_color_from_outdoor_light: bool,
    pub light1_rot_x: f32,
    pub light1_rot_y: f32,
    pub light2_color: super::core::Vec3,
    pub light2_intensity: f32,
    pub use_light_source2: bool,
    pub light2_rot_x: f32,
    pub light2_rot_y: f32,
    pub use_noise: bool,
    pub fog_start_distance: f32,
    pub rayleigh_polarization: super::core::Vec3,
    pub mie_polarization: super::core::Vec3,
    pub outdoor_light_scale: super::core::Vec3,
    pub draw_sun_disc: bool,
    pub forward_scattering_depth_visibility: super::core::Vec4,
    pub forward_scattering_start_depth: f32,
    pub forward_scattering_end_depth: f32,
    pub forward_scattering_takes_color_from_outdoor_light: f32,
    pub forward_scattering_outdoor_light_tint: super::core::Vec3,
    pub height_fog_color_add: super::core::Vec3,
    pub height_fog_color_mult: super::core::Vec3,
    pub min_height_fog_transmittance: f32,
    pub gradient_texture_blend_factor: f32,
    pub panoramic_blend_rotation: f32,
    pub gradient_blend_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub panoramic_blend_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub panoramic_alpha_blend_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub static_envmap_blend_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub flow_mask_blend_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub field_flag_override0: u32,
    pub field_flag_override1: u32,
    pub field_flag_override2: u32,
    pub field_flag_override3: u32,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u32,
    pub field_flag_changed2: u32,
    pub field_flag_changed3: u32,
}

pub trait SkyComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn draw_sky_geo(&self) -> &bool;
    fn sky_type(&self) -> &SkyType;
    fn luminance_scale(&self) -> &f32;
    fn sky_gradient_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn alpha_output(&self) -> &AlphaOutputMode;
    fn use_sky_visibility_as_a_o(&self) -> &bool;
    fn hdri_rotation(&self) -> &f32;
    fn hdri_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn sun_size(&self) -> &f32;
    fn sun_scale(&self) -> &f32;
    fn panoramic_u_v_min_x(&self) -> &f32;
    fn panoramic_u_v_max_x(&self) -> &f32;
    fn panoramic_u_v_min_y(&self) -> &f32;
    fn panoramic_u_v_max_y(&self) -> &f32;
    fn panoramic_tile_factor(&self) -> &f32;
    fn panoramic_rotation(&self) -> &f32;
    fn panoramic_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn panoramic_alpha_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn sky_gradient_follows_panoramic_u_vs(&self) -> &bool;
    fn flow_enable(&self) -> &bool;
    fn flow_period(&self) -> &f32;
    fn flow_distance(&self) -> &f32;
    fn flow_direction(&self) -> &f32;
    fn flow_height_mask_scale(&self) -> &f32;
    fn flow_height_mask_bias(&self) -> &f32;
    fn flow_mask_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn cloud_layer_sun_color(&self) -> &super::core::Vec3;
    fn cloud_layer_mask_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn cloud_layer1_altitude(&self) -> &f32;
    fn cloud_layer1_tile_factor(&self) -> &f32;
    fn cloud_layer1_rotation(&self) -> &f32;
    fn cloud_layer1_speed(&self) -> &f32;
    fn cloud_layer1_sun_light_intensity(&self) -> &f32;
    fn cloud_layer1_sun_light_power(&self) -> &f32;
    fn cloud_layer1_ambient_light_intensity(&self) -> &f32;
    fn cloud_layer1_color(&self) -> &super::core::Vec3;
    fn cloud_layer1_alpha_mul(&self) -> &f32;
    fn cloud_layer1_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn cloud_layer1_absorption(&self) -> &f32;
    fn cloud_layer1_scattering(&self) -> &f32;
    fn cloud_layer1_phase(&self) -> &f32;
    fn cloud_layer1_thickness(&self) -> &f32;
    fn cloud_layer2_altitude(&self) -> &f32;
    fn cloud_layer2_tile_factor(&self) -> &f32;
    fn cloud_layer2_rotation(&self) -> &f32;
    fn cloud_layer2_speed(&self) -> &f32;
    fn cloud_layer2_sun_light_intensity(&self) -> &f32;
    fn cloud_layer2_sun_light_power(&self) -> &f32;
    fn cloud_layer2_ambient_light_intensity(&self) -> &f32;
    fn cloud_layer2_color(&self) -> &super::core::Vec3;
    fn cloud_layer2_alpha_mul(&self) -> &f32;
    fn cloud_layer2_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn cloud_layer2_absorption(&self) -> &f32;
    fn cloud_layer2_scattering(&self) -> &f32;
    fn cloud_layer2_phase(&self) -> &f32;
    fn cloud_layer2_thickness(&self) -> &f32;
    fn static_envmap_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn static_envmap_scale(&self) -> &f32;
    fn sky_envmap8_bit_tex_scale(&self) -> &f32;
    fn custom_envmap_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn custom_envmap_scale(&self) -> &f32;
    fn custom_envmap_ambient(&self) -> &f32;
    fn sky_visibility_exponent(&self) -> &f32;
    fn interior_envmap_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn interior_envmap_exp(&self) -> &super::core::Vec4;
    fn interior_envmap_scale(&self) -> &super::core::Vec4;
    fn interior_envmap_bias(&self) -> &super::core::Vec4;
    fn interior_envmap_sky_visibility_fade_start(&self) -> &f32;
    fn interior_envmap_sky_visibility_fade_length(&self) -> &f32;
    fn earth_radius(&self) -> &f32;
    fn atmosphere_radius(&self) -> &f32;
    fn mie_scattering_coefficient(&self) -> &f32;
    fn mie_g(&self) -> &f32;
    fn mie_extinction_coefficient_relation(&self) -> &f32;
    fn scale_height_mie(&self) -> &f32;
    fn rayleigh_scattering_coefficient(&self) -> &super::core::Vec3;
    fn rayleigh_scattering_coefficient_scale(&self) -> &f32;
    fn rayleigh_extinction_coefficient_relation(&self) -> &f32;
    fn scale_height_rayleigh(&self) -> &f32;
    fn use_ozone(&self) -> &bool;
    fn ozone_percentage(&self) -> &f32;
    fn use_aerial_perspective(&self) -> &bool;
    fn aerial_perspective_scale(&self) -> &f32;
    fn aerial_perspective_intensity(&self) -> &f32;
    fn aerial_perspective_dithering(&self) -> &f32;
    fn light1_color(&self) -> &super::core::Vec3;
    fn light1_intensity(&self) -> &f32;
    fn light1_follow_outdoor_light(&self) -> &bool;
    fn light1_takes_color_from_outdoor_light(&self) -> &bool;
    fn light1_rot_x(&self) -> &f32;
    fn light1_rot_y(&self) -> &f32;
    fn light2_color(&self) -> &super::core::Vec3;
    fn light2_intensity(&self) -> &f32;
    fn use_light_source2(&self) -> &bool;
    fn light2_rot_x(&self) -> &f32;
    fn light2_rot_y(&self) -> &f32;
    fn use_noise(&self) -> &bool;
    fn fog_start_distance(&self) -> &f32;
    fn rayleigh_polarization(&self) -> &super::core::Vec3;
    fn mie_polarization(&self) -> &super::core::Vec3;
    fn outdoor_light_scale(&self) -> &super::core::Vec3;
    fn draw_sun_disc(&self) -> &bool;
    fn forward_scattering_depth_visibility(&self) -> &super::core::Vec4;
    fn forward_scattering_start_depth(&self) -> &f32;
    fn forward_scattering_end_depth(&self) -> &f32;
    fn forward_scattering_takes_color_from_outdoor_light(&self) -> &f32;
    fn forward_scattering_outdoor_light_tint(&self) -> &super::core::Vec3;
    fn height_fog_color_add(&self) -> &super::core::Vec3;
    fn height_fog_color_mult(&self) -> &super::core::Vec3;
    fn min_height_fog_transmittance(&self) -> &f32;
    fn gradient_texture_blend_factor(&self) -> &f32;
    fn panoramic_blend_rotation(&self) -> &f32;
    fn gradient_blend_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn panoramic_blend_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn panoramic_alpha_blend_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn static_envmap_blend_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn flow_mask_blend_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_override1(&self) -> &u32;
    fn field_flag_override2(&self) -> &u32;
    fn field_flag_override3(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u32;
    fn field_flag_changed2(&self) -> &u32;
    fn field_flag_changed3(&self) -> &u32;
}

impl SkyComponentStateTrait for SkyComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn draw_sky_geo(&self) -> &bool {
        &self.draw_sky_geo
    }
    fn sky_type(&self) -> &SkyType {
        &self.sky_type
    }
    fn luminance_scale(&self) -> &f32 {
        &self.luminance_scale
    }
    fn sky_gradient_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.sky_gradient_texture
    }
    fn alpha_output(&self) -> &AlphaOutputMode {
        &self.alpha_output
    }
    fn use_sky_visibility_as_a_o(&self) -> &bool {
        &self.use_sky_visibility_as_a_o
    }
    fn hdri_rotation(&self) -> &f32 {
        &self.hdri_rotation
    }
    fn hdri_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.hdri_texture
    }
    fn sun_size(&self) -> &f32 {
        &self.sun_size
    }
    fn sun_scale(&self) -> &f32 {
        &self.sun_scale
    }
    fn panoramic_u_v_min_x(&self) -> &f32 {
        &self.panoramic_u_v_min_x
    }
    fn panoramic_u_v_max_x(&self) -> &f32 {
        &self.panoramic_u_v_max_x
    }
    fn panoramic_u_v_min_y(&self) -> &f32 {
        &self.panoramic_u_v_min_y
    }
    fn panoramic_u_v_max_y(&self) -> &f32 {
        &self.panoramic_u_v_max_y
    }
    fn panoramic_tile_factor(&self) -> &f32 {
        &self.panoramic_tile_factor
    }
    fn panoramic_rotation(&self) -> &f32 {
        &self.panoramic_rotation
    }
    fn panoramic_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.panoramic_texture
    }
    fn panoramic_alpha_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.panoramic_alpha_texture
    }
    fn sky_gradient_follows_panoramic_u_vs(&self) -> &bool {
        &self.sky_gradient_follows_panoramic_u_vs
    }
    fn flow_enable(&self) -> &bool {
        &self.flow_enable
    }
    fn flow_period(&self) -> &f32 {
        &self.flow_period
    }
    fn flow_distance(&self) -> &f32 {
        &self.flow_distance
    }
    fn flow_direction(&self) -> &f32 {
        &self.flow_direction
    }
    fn flow_height_mask_scale(&self) -> &f32 {
        &self.flow_height_mask_scale
    }
    fn flow_height_mask_bias(&self) -> &f32 {
        &self.flow_height_mask_bias
    }
    fn flow_mask_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.flow_mask_texture
    }
    fn cloud_layer_sun_color(&self) -> &super::core::Vec3 {
        &self.cloud_layer_sun_color
    }
    fn cloud_layer_mask_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.cloud_layer_mask_texture
    }
    fn cloud_layer1_altitude(&self) -> &f32 {
        &self.cloud_layer1_altitude
    }
    fn cloud_layer1_tile_factor(&self) -> &f32 {
        &self.cloud_layer1_tile_factor
    }
    fn cloud_layer1_rotation(&self) -> &f32 {
        &self.cloud_layer1_rotation
    }
    fn cloud_layer1_speed(&self) -> &f32 {
        &self.cloud_layer1_speed
    }
    fn cloud_layer1_sun_light_intensity(&self) -> &f32 {
        &self.cloud_layer1_sun_light_intensity
    }
    fn cloud_layer1_sun_light_power(&self) -> &f32 {
        &self.cloud_layer1_sun_light_power
    }
    fn cloud_layer1_ambient_light_intensity(&self) -> &f32 {
        &self.cloud_layer1_ambient_light_intensity
    }
    fn cloud_layer1_color(&self) -> &super::core::Vec3 {
        &self.cloud_layer1_color
    }
    fn cloud_layer1_alpha_mul(&self) -> &f32 {
        &self.cloud_layer1_alpha_mul
    }
    fn cloud_layer1_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.cloud_layer1_texture
    }
    fn cloud_layer1_absorption(&self) -> &f32 {
        &self.cloud_layer1_absorption
    }
    fn cloud_layer1_scattering(&self) -> &f32 {
        &self.cloud_layer1_scattering
    }
    fn cloud_layer1_phase(&self) -> &f32 {
        &self.cloud_layer1_phase
    }
    fn cloud_layer1_thickness(&self) -> &f32 {
        &self.cloud_layer1_thickness
    }
    fn cloud_layer2_altitude(&self) -> &f32 {
        &self.cloud_layer2_altitude
    }
    fn cloud_layer2_tile_factor(&self) -> &f32 {
        &self.cloud_layer2_tile_factor
    }
    fn cloud_layer2_rotation(&self) -> &f32 {
        &self.cloud_layer2_rotation
    }
    fn cloud_layer2_speed(&self) -> &f32 {
        &self.cloud_layer2_speed
    }
    fn cloud_layer2_sun_light_intensity(&self) -> &f32 {
        &self.cloud_layer2_sun_light_intensity
    }
    fn cloud_layer2_sun_light_power(&self) -> &f32 {
        &self.cloud_layer2_sun_light_power
    }
    fn cloud_layer2_ambient_light_intensity(&self) -> &f32 {
        &self.cloud_layer2_ambient_light_intensity
    }
    fn cloud_layer2_color(&self) -> &super::core::Vec3 {
        &self.cloud_layer2_color
    }
    fn cloud_layer2_alpha_mul(&self) -> &f32 {
        &self.cloud_layer2_alpha_mul
    }
    fn cloud_layer2_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.cloud_layer2_texture
    }
    fn cloud_layer2_absorption(&self) -> &f32 {
        &self.cloud_layer2_absorption
    }
    fn cloud_layer2_scattering(&self) -> &f32 {
        &self.cloud_layer2_scattering
    }
    fn cloud_layer2_phase(&self) -> &f32 {
        &self.cloud_layer2_phase
    }
    fn cloud_layer2_thickness(&self) -> &f32 {
        &self.cloud_layer2_thickness
    }
    fn static_envmap_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.static_envmap_texture
    }
    fn static_envmap_scale(&self) -> &f32 {
        &self.static_envmap_scale
    }
    fn sky_envmap8_bit_tex_scale(&self) -> &f32 {
        &self.sky_envmap8_bit_tex_scale
    }
    fn custom_envmap_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.custom_envmap_texture
    }
    fn custom_envmap_scale(&self) -> &f32 {
        &self.custom_envmap_scale
    }
    fn custom_envmap_ambient(&self) -> &f32 {
        &self.custom_envmap_ambient
    }
    fn sky_visibility_exponent(&self) -> &f32 {
        &self.sky_visibility_exponent
    }
    fn interior_envmap_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.interior_envmap_texture
    }
    fn interior_envmap_exp(&self) -> &super::core::Vec4 {
        &self.interior_envmap_exp
    }
    fn interior_envmap_scale(&self) -> &super::core::Vec4 {
        &self.interior_envmap_scale
    }
    fn interior_envmap_bias(&self) -> &super::core::Vec4 {
        &self.interior_envmap_bias
    }
    fn interior_envmap_sky_visibility_fade_start(&self) -> &f32 {
        &self.interior_envmap_sky_visibility_fade_start
    }
    fn interior_envmap_sky_visibility_fade_length(&self) -> &f32 {
        &self.interior_envmap_sky_visibility_fade_length
    }
    fn earth_radius(&self) -> &f32 {
        &self.earth_radius
    }
    fn atmosphere_radius(&self) -> &f32 {
        &self.atmosphere_radius
    }
    fn mie_scattering_coefficient(&self) -> &f32 {
        &self.mie_scattering_coefficient
    }
    fn mie_g(&self) -> &f32 {
        &self.mie_g
    }
    fn mie_extinction_coefficient_relation(&self) -> &f32 {
        &self.mie_extinction_coefficient_relation
    }
    fn scale_height_mie(&self) -> &f32 {
        &self.scale_height_mie
    }
    fn rayleigh_scattering_coefficient(&self) -> &super::core::Vec3 {
        &self.rayleigh_scattering_coefficient
    }
    fn rayleigh_scattering_coefficient_scale(&self) -> &f32 {
        &self.rayleigh_scattering_coefficient_scale
    }
    fn rayleigh_extinction_coefficient_relation(&self) -> &f32 {
        &self.rayleigh_extinction_coefficient_relation
    }
    fn scale_height_rayleigh(&self) -> &f32 {
        &self.scale_height_rayleigh
    }
    fn use_ozone(&self) -> &bool {
        &self.use_ozone
    }
    fn ozone_percentage(&self) -> &f32 {
        &self.ozone_percentage
    }
    fn use_aerial_perspective(&self) -> &bool {
        &self.use_aerial_perspective
    }
    fn aerial_perspective_scale(&self) -> &f32 {
        &self.aerial_perspective_scale
    }
    fn aerial_perspective_intensity(&self) -> &f32 {
        &self.aerial_perspective_intensity
    }
    fn aerial_perspective_dithering(&self) -> &f32 {
        &self.aerial_perspective_dithering
    }
    fn light1_color(&self) -> &super::core::Vec3 {
        &self.light1_color
    }
    fn light1_intensity(&self) -> &f32 {
        &self.light1_intensity
    }
    fn light1_follow_outdoor_light(&self) -> &bool {
        &self.light1_follow_outdoor_light
    }
    fn light1_takes_color_from_outdoor_light(&self) -> &bool {
        &self.light1_takes_color_from_outdoor_light
    }
    fn light1_rot_x(&self) -> &f32 {
        &self.light1_rot_x
    }
    fn light1_rot_y(&self) -> &f32 {
        &self.light1_rot_y
    }
    fn light2_color(&self) -> &super::core::Vec3 {
        &self.light2_color
    }
    fn light2_intensity(&self) -> &f32 {
        &self.light2_intensity
    }
    fn use_light_source2(&self) -> &bool {
        &self.use_light_source2
    }
    fn light2_rot_x(&self) -> &f32 {
        &self.light2_rot_x
    }
    fn light2_rot_y(&self) -> &f32 {
        &self.light2_rot_y
    }
    fn use_noise(&self) -> &bool {
        &self.use_noise
    }
    fn fog_start_distance(&self) -> &f32 {
        &self.fog_start_distance
    }
    fn rayleigh_polarization(&self) -> &super::core::Vec3 {
        &self.rayleigh_polarization
    }
    fn mie_polarization(&self) -> &super::core::Vec3 {
        &self.mie_polarization
    }
    fn outdoor_light_scale(&self) -> &super::core::Vec3 {
        &self.outdoor_light_scale
    }
    fn draw_sun_disc(&self) -> &bool {
        &self.draw_sun_disc
    }
    fn forward_scattering_depth_visibility(&self) -> &super::core::Vec4 {
        &self.forward_scattering_depth_visibility
    }
    fn forward_scattering_start_depth(&self) -> &f32 {
        &self.forward_scattering_start_depth
    }
    fn forward_scattering_end_depth(&self) -> &f32 {
        &self.forward_scattering_end_depth
    }
    fn forward_scattering_takes_color_from_outdoor_light(&self) -> &f32 {
        &self.forward_scattering_takes_color_from_outdoor_light
    }
    fn forward_scattering_outdoor_light_tint(&self) -> &super::core::Vec3 {
        &self.forward_scattering_outdoor_light_tint
    }
    fn height_fog_color_add(&self) -> &super::core::Vec3 {
        &self.height_fog_color_add
    }
    fn height_fog_color_mult(&self) -> &super::core::Vec3 {
        &self.height_fog_color_mult
    }
    fn min_height_fog_transmittance(&self) -> &f32 {
        &self.min_height_fog_transmittance
    }
    fn gradient_texture_blend_factor(&self) -> &f32 {
        &self.gradient_texture_blend_factor
    }
    fn panoramic_blend_rotation(&self) -> &f32 {
        &self.panoramic_blend_rotation
    }
    fn gradient_blend_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.gradient_blend_texture
    }
    fn panoramic_blend_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.panoramic_blend_texture
    }
    fn panoramic_alpha_blend_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.panoramic_alpha_blend_texture
    }
    fn static_envmap_blend_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.static_envmap_blend_texture
    }
    fn flow_mask_blend_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.flow_mask_blend_texture
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_override1(&self) -> &u32 {
        &self.field_flag_override1
    }
    fn field_flag_override2(&self) -> &u32 {
        &self.field_flag_override2
    }
    fn field_flag_override3(&self) -> &u32 {
        &self.field_flag_override3
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u32 {
        &self.field_flag_changed1
    }
    fn field_flag_changed2(&self) -> &u32 {
        &self.field_flag_changed2
    }
    fn field_flag_changed3(&self) -> &u32 {
        &self.field_flag_changed3
    }
}

pub static SKYCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SkyComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, enable),
            },
            FieldInfoData {
                name: "DrawSkyGeo",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, draw_sky_geo),
            },
            FieldInfoData {
                name: "SkyType",
                flags: MemberInfoFlags::new(0),
                field_type: "SkyType",
                rust_offset: offset_of!(SkyComponentState, sky_type),
            },
            FieldInfoData {
                name: "LuminanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, luminance_scale),
            },
            FieldInfoData {
                name: "SkyGradientTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, sky_gradient_texture),
            },
            FieldInfoData {
                name: "AlphaOutput",
                flags: MemberInfoFlags::new(0),
                field_type: "AlphaOutputMode",
                rust_offset: offset_of!(SkyComponentState, alpha_output),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "HdriRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, hdri_rotation),
            },
            FieldInfoData {
                name: "HdriTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, hdri_texture),
            },
            FieldInfoData {
                name: "SunSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, sun_size),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, sun_scale),
            },
            FieldInfoData {
                name: "PanoramicUVMinX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, panoramic_u_v_min_x),
            },
            FieldInfoData {
                name: "PanoramicUVMaxX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, panoramic_u_v_max_x),
            },
            FieldInfoData {
                name: "PanoramicUVMinY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, panoramic_u_v_min_y),
            },
            FieldInfoData {
                name: "PanoramicUVMaxY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, panoramic_u_v_max_y),
            },
            FieldInfoData {
                name: "PanoramicTileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, panoramic_tile_factor),
            },
            FieldInfoData {
                name: "PanoramicRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, panoramic_rotation),
            },
            FieldInfoData {
                name: "PanoramicTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, panoramic_texture),
            },
            FieldInfoData {
                name: "PanoramicAlphaTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, panoramic_alpha_texture),
            },
            FieldInfoData {
                name: "SkyGradientFollowsPanoramicUVs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, sky_gradient_follows_panoramic_u_vs),
            },
            FieldInfoData {
                name: "FlowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, flow_enable),
            },
            FieldInfoData {
                name: "FlowPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, flow_period),
            },
            FieldInfoData {
                name: "FlowDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, flow_distance),
            },
            FieldInfoData {
                name: "FlowDirection",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, flow_direction),
            },
            FieldInfoData {
                name: "FlowHeightMaskScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, flow_height_mask_scale),
            },
            FieldInfoData {
                name: "FlowHeightMaskBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, flow_height_mask_bias),
            },
            FieldInfoData {
                name: "FlowMaskTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, flow_mask_texture),
            },
            FieldInfoData {
                name: "CloudLayerSunColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, cloud_layer_sun_color),
            },
            FieldInfoData {
                name: "CloudLayerMaskTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, cloud_layer_mask_texture),
            },
            FieldInfoData {
                name: "CloudLayer1Altitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_altitude),
            },
            FieldInfoData {
                name: "CloudLayer1TileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_tile_factor),
            },
            FieldInfoData {
                name: "CloudLayer1Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_rotation),
            },
            FieldInfoData {
                name: "CloudLayer1Speed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_speed),
            },
            FieldInfoData {
                name: "CloudLayer1SunLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_sun_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer1SunLightPower",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_sun_light_power),
            },
            FieldInfoData {
                name: "CloudLayer1AmbientLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_ambient_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer1Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_color),
            },
            FieldInfoData {
                name: "CloudLayer1AlphaMul",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_alpha_mul),
            },
            FieldInfoData {
                name: "CloudLayer1Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_texture),
            },
            FieldInfoData {
                name: "CloudLayer1Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_absorption),
            },
            FieldInfoData {
                name: "CloudLayer1Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_scattering),
            },
            FieldInfoData {
                name: "CloudLayer1Phase",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_phase),
            },
            FieldInfoData {
                name: "CloudLayer1Thickness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_thickness),
            },
            FieldInfoData {
                name: "CloudLayer2Altitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_altitude),
            },
            FieldInfoData {
                name: "CloudLayer2TileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_tile_factor),
            },
            FieldInfoData {
                name: "CloudLayer2Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_rotation),
            },
            FieldInfoData {
                name: "CloudLayer2Speed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_speed),
            },
            FieldInfoData {
                name: "CloudLayer2SunLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_sun_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer2SunLightPower",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_sun_light_power),
            },
            FieldInfoData {
                name: "CloudLayer2AmbientLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_ambient_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer2Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_color),
            },
            FieldInfoData {
                name: "CloudLayer2AlphaMul",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_alpha_mul),
            },
            FieldInfoData {
                name: "CloudLayer2Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_texture),
            },
            FieldInfoData {
                name: "CloudLayer2Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_absorption),
            },
            FieldInfoData {
                name: "CloudLayer2Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_scattering),
            },
            FieldInfoData {
                name: "CloudLayer2Phase",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_phase),
            },
            FieldInfoData {
                name: "CloudLayer2Thickness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_thickness),
            },
            FieldInfoData {
                name: "StaticEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, static_envmap_texture),
            },
            FieldInfoData {
                name: "StaticEnvmapScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, static_envmap_scale),
            },
            FieldInfoData {
                name: "SkyEnvmap8BitTexScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, sky_envmap8_bit_tex_scale),
            },
            FieldInfoData {
                name: "CustomEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, custom_envmap_texture),
            },
            FieldInfoData {
                name: "CustomEnvmapScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, custom_envmap_scale),
            },
            FieldInfoData {
                name: "CustomEnvmapAmbient",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, custom_envmap_ambient),
            },
            FieldInfoData {
                name: "SkyVisibilityExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, sky_visibility_exponent),
            },
            FieldInfoData {
                name: "InteriorEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, interior_envmap_texture),
            },
            FieldInfoData {
                name: "InteriorEnvmapExp",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SkyComponentState, interior_envmap_exp),
            },
            FieldInfoData {
                name: "InteriorEnvmapScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SkyComponentState, interior_envmap_scale),
            },
            FieldInfoData {
                name: "InteriorEnvmapBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SkyComponentState, interior_envmap_bias),
            },
            FieldInfoData {
                name: "InteriorEnvmapSkyVisibilityFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, interior_envmap_sky_visibility_fade_start),
            },
            FieldInfoData {
                name: "InteriorEnvmapSkyVisibilityFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, interior_envmap_sky_visibility_fade_length),
            },
            FieldInfoData {
                name: "EarthRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, earth_radius),
            },
            FieldInfoData {
                name: "AtmosphereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, atmosphere_radius),
            },
            FieldInfoData {
                name: "MieScatteringCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, mie_scattering_coefficient),
            },
            FieldInfoData {
                name: "MieG",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, mie_g),
            },
            FieldInfoData {
                name: "MieExtinctionCoefficientRelation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, mie_extinction_coefficient_relation),
            },
            FieldInfoData {
                name: "ScaleHeightMie",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, scale_height_mie),
            },
            FieldInfoData {
                name: "RayleighScatteringCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, rayleigh_scattering_coefficient),
            },
            FieldInfoData {
                name: "RayleighScatteringCoefficientScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, rayleigh_scattering_coefficient_scale),
            },
            FieldInfoData {
                name: "RayleighExtinctionCoefficientRelation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, rayleigh_extinction_coefficient_relation),
            },
            FieldInfoData {
                name: "ScaleHeightRayleigh",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, scale_height_rayleigh),
            },
            FieldInfoData {
                name: "UseOzone",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, use_ozone),
            },
            FieldInfoData {
                name: "OzonePercentage",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, ozone_percentage),
            },
            FieldInfoData {
                name: "UseAerialPerspective",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, use_aerial_perspective),
            },
            FieldInfoData {
                name: "AerialPerspectiveScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, aerial_perspective_scale),
            },
            FieldInfoData {
                name: "AerialPerspectiveIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, aerial_perspective_intensity),
            },
            FieldInfoData {
                name: "AerialPerspectiveDithering",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, aerial_perspective_dithering),
            },
            FieldInfoData {
                name: "Light1Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, light1_color),
            },
            FieldInfoData {
                name: "Light1Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, light1_intensity),
            },
            FieldInfoData {
                name: "Light1FollowOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, light1_follow_outdoor_light),
            },
            FieldInfoData {
                name: "Light1TakesColorFromOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, light1_takes_color_from_outdoor_light),
            },
            FieldInfoData {
                name: "Light1RotX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, light1_rot_x),
            },
            FieldInfoData {
                name: "Light1RotY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, light1_rot_y),
            },
            FieldInfoData {
                name: "Light2Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, light2_color),
            },
            FieldInfoData {
                name: "Light2Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, light2_intensity),
            },
            FieldInfoData {
                name: "UseLightSource2",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, use_light_source2),
            },
            FieldInfoData {
                name: "Light2RotX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, light2_rot_x),
            },
            FieldInfoData {
                name: "Light2RotY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, light2_rot_y),
            },
            FieldInfoData {
                name: "UseNoise",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, use_noise),
            },
            FieldInfoData {
                name: "FogStartDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, fog_start_distance),
            },
            FieldInfoData {
                name: "RayleighPolarization",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, rayleigh_polarization),
            },
            FieldInfoData {
                name: "MiePolarization",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, mie_polarization),
            },
            FieldInfoData {
                name: "OutdoorLightScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, outdoor_light_scale),
            },
            FieldInfoData {
                name: "DrawSunDisc",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SkyComponentState, draw_sun_disc),
            },
            FieldInfoData {
                name: "ForwardScatteringDepthVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SkyComponentState, forward_scattering_depth_visibility),
            },
            FieldInfoData {
                name: "ForwardScatteringStartDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, forward_scattering_start_depth),
            },
            FieldInfoData {
                name: "ForwardScatteringEndDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, forward_scattering_end_depth),
            },
            FieldInfoData {
                name: "ForwardScatteringTakesColorFromOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, forward_scattering_takes_color_from_outdoor_light),
            },
            FieldInfoData {
                name: "ForwardScatteringOutdoorLightTint",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, forward_scattering_outdoor_light_tint),
            },
            FieldInfoData {
                name: "HeightFogColorAdd",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, height_fog_color_add),
            },
            FieldInfoData {
                name: "HeightFogColorMult",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SkyComponentState, height_fog_color_mult),
            },
            FieldInfoData {
                name: "MinHeightFogTransmittance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, min_height_fog_transmittance),
            },
            FieldInfoData {
                name: "GradientTextureBlendFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, gradient_texture_blend_factor),
            },
            FieldInfoData {
                name: "PanoramicBlendRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SkyComponentState, panoramic_blend_rotation),
            },
            FieldInfoData {
                name: "GradientBlendTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, gradient_blend_texture),
            },
            FieldInfoData {
                name: "PanoramicBlendTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, panoramic_blend_texture),
            },
            FieldInfoData {
                name: "PanoramicAlphaBlendTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, panoramic_alpha_blend_texture),
            },
            FieldInfoData {
                name: "StaticEnvmapBlendTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, static_envmap_blend_texture),
            },
            FieldInfoData {
                name: "FlowMaskBlendTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SkyComponentState, flow_mask_blend_texture),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagOverride2",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyComponentState, field_flag_override2),
            },
            FieldInfoData {
                name: "FieldFlagOverride3",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyComponentState, field_flag_override3),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyComponentState, field_flag_changed1),
            },
            FieldInfoData {
                name: "FieldFlagChanged2",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyComponentState, field_flag_changed2),
            },
            FieldInfoData {
                name: "FieldFlagChanged3",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SkyComponentState, field_flag_changed3),
            },
        ],
    }),
    array_type: Some(SKYCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        SKYCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SKYCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SkyType {
    #[default]
    SkyType_Procedural = 0,
    SkyType_Hdri = 1,
    SkyType_Physical = 2,
}

pub static SKYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyType",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SKYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SkyType {
    fn type_info(&self) -> &'static TypeInfo {
        SKYTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SKYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AlphaOutputMode {
    #[default]
    AlphaOutputMode_Disabled = 0,
    AlphaOutputMode_Clear = 1,
    AlphaOutputMode_CloudLayerOnly = 2,
    AlphaOutputMode_MaskOnly = 3,
    AlphaOutputMode_CloudLayerAndMask = 4,
    AlphaOutputMode_PanoramicOnly = 5,
    AlphaOutputMode_CloudLayerAndPanoramic = 6,
    AlphaOutputMode_MaskAndPanoramic = 7,
    AlphaOutputMode_CloudLayerAndMaskAndPanoramic = 8,
}

pub static ALPHAOUTPUTMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AlphaOutputMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(ALPHAOUTPUTMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AlphaOutputMode {
    fn type_info(&self) -> &'static TypeInfo {
        ALPHAOUTPUTMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ALPHAOUTPUTMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AlphaOutputMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("AlphaOutputMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FogComponentState {
    pub enable: bool,
    pub fog_distance_multiplier: f32,
    pub fog_gradient_enable: bool,
    pub start: f32,
    pub end: f32,
    pub curve: super::core::Vec4,
    pub fog_gradient_height_fade_enable: bool,
    pub fade_start: f32,
    pub fade_end: f32,
    pub fog_color_enable: bool,
    pub fog_color: super::core::Vec3,
    pub fog_color_start: f32,
    pub fog_color_end: f32,
    pub fog_color_curve: super::core::Vec4,
    pub transparency_fade_start: f32,
    pub transparency_fade_end: f32,
    pub transparency_fade_clamp: f32,
    pub transparency_fade_curve: super::core::Vec4,
    pub forward_light_scattering_enabled: bool,
    pub forward_light_scattering_use_sun_position: bool,
    pub forward_light_scattering_rotation_x: f32,
    pub forward_light_scattering_rotation_y: f32,
    pub forward_light_scattering_phase_g: f32,
    pub forward_light_scattering_strength: f32,
    pub forward_light_scattering_color: super::core::Vec3,
    pub forward_light_scattering_presence: f32,
    pub forward_light_scattering_max_blur_length: f32,
    pub forward_light_scattering_extinction: f32,
    pub forward_light_scattering_smoothness: f32,
    pub forward_light_scattering_attenuation_type: ForwardLightScatteringAttenuation,
    pub height_fog_enable: bool,
    pub height_fog_follow_camera: f32,
    pub height_fog_altitude: f32,
    pub height_fog_depth: f32,
    pub height_fog_visibility_range: f32,
    pub participating_media_enable: bool,
    pub depth_fog_participating_media: ParticipatingMedia,
    pub height_fog_participating_media: ParticipatingMedia,
    pub fog_volume_strength: f32,
    pub field_flag_override0: u32,
    pub field_flag_override1: u8,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u16,
}

pub trait FogComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn fog_distance_multiplier(&self) -> &f32;
    fn fog_gradient_enable(&self) -> &bool;
    fn start(&self) -> &f32;
    fn end(&self) -> &f32;
    fn curve(&self) -> &super::core::Vec4;
    fn fog_gradient_height_fade_enable(&self) -> &bool;
    fn fade_start(&self) -> &f32;
    fn fade_end(&self) -> &f32;
    fn fog_color_enable(&self) -> &bool;
    fn fog_color(&self) -> &super::core::Vec3;
    fn fog_color_start(&self) -> &f32;
    fn fog_color_end(&self) -> &f32;
    fn fog_color_curve(&self) -> &super::core::Vec4;
    fn transparency_fade_start(&self) -> &f32;
    fn transparency_fade_end(&self) -> &f32;
    fn transparency_fade_clamp(&self) -> &f32;
    fn transparency_fade_curve(&self) -> &super::core::Vec4;
    fn forward_light_scattering_enabled(&self) -> &bool;
    fn forward_light_scattering_use_sun_position(&self) -> &bool;
    fn forward_light_scattering_rotation_x(&self) -> &f32;
    fn forward_light_scattering_rotation_y(&self) -> &f32;
    fn forward_light_scattering_phase_g(&self) -> &f32;
    fn forward_light_scattering_strength(&self) -> &f32;
    fn forward_light_scattering_color(&self) -> &super::core::Vec3;
    fn forward_light_scattering_presence(&self) -> &f32;
    fn forward_light_scattering_max_blur_length(&self) -> &f32;
    fn forward_light_scattering_extinction(&self) -> &f32;
    fn forward_light_scattering_smoothness(&self) -> &f32;
    fn forward_light_scattering_attenuation_type(&self) -> &ForwardLightScatteringAttenuation;
    fn height_fog_enable(&self) -> &bool;
    fn height_fog_follow_camera(&self) -> &f32;
    fn height_fog_altitude(&self) -> &f32;
    fn height_fog_depth(&self) -> &f32;
    fn height_fog_visibility_range(&self) -> &f32;
    fn participating_media_enable(&self) -> &bool;
    fn depth_fog_participating_media(&self) -> &ParticipatingMedia;
    fn height_fog_participating_media(&self) -> &ParticipatingMedia;
    fn fog_volume_strength(&self) -> &f32;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_override1(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u16;
}

impl FogComponentStateTrait for FogComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn fog_distance_multiplier(&self) -> &f32 {
        &self.fog_distance_multiplier
    }
    fn fog_gradient_enable(&self) -> &bool {
        &self.fog_gradient_enable
    }
    fn start(&self) -> &f32 {
        &self.start
    }
    fn end(&self) -> &f32 {
        &self.end
    }
    fn curve(&self) -> &super::core::Vec4 {
        &self.curve
    }
    fn fog_gradient_height_fade_enable(&self) -> &bool {
        &self.fog_gradient_height_fade_enable
    }
    fn fade_start(&self) -> &f32 {
        &self.fade_start
    }
    fn fade_end(&self) -> &f32 {
        &self.fade_end
    }
    fn fog_color_enable(&self) -> &bool {
        &self.fog_color_enable
    }
    fn fog_color(&self) -> &super::core::Vec3 {
        &self.fog_color
    }
    fn fog_color_start(&self) -> &f32 {
        &self.fog_color_start
    }
    fn fog_color_end(&self) -> &f32 {
        &self.fog_color_end
    }
    fn fog_color_curve(&self) -> &super::core::Vec4 {
        &self.fog_color_curve
    }
    fn transparency_fade_start(&self) -> &f32 {
        &self.transparency_fade_start
    }
    fn transparency_fade_end(&self) -> &f32 {
        &self.transparency_fade_end
    }
    fn transparency_fade_clamp(&self) -> &f32 {
        &self.transparency_fade_clamp
    }
    fn transparency_fade_curve(&self) -> &super::core::Vec4 {
        &self.transparency_fade_curve
    }
    fn forward_light_scattering_enabled(&self) -> &bool {
        &self.forward_light_scattering_enabled
    }
    fn forward_light_scattering_use_sun_position(&self) -> &bool {
        &self.forward_light_scattering_use_sun_position
    }
    fn forward_light_scattering_rotation_x(&self) -> &f32 {
        &self.forward_light_scattering_rotation_x
    }
    fn forward_light_scattering_rotation_y(&self) -> &f32 {
        &self.forward_light_scattering_rotation_y
    }
    fn forward_light_scattering_phase_g(&self) -> &f32 {
        &self.forward_light_scattering_phase_g
    }
    fn forward_light_scattering_strength(&self) -> &f32 {
        &self.forward_light_scattering_strength
    }
    fn forward_light_scattering_color(&self) -> &super::core::Vec3 {
        &self.forward_light_scattering_color
    }
    fn forward_light_scattering_presence(&self) -> &f32 {
        &self.forward_light_scattering_presence
    }
    fn forward_light_scattering_max_blur_length(&self) -> &f32 {
        &self.forward_light_scattering_max_blur_length
    }
    fn forward_light_scattering_extinction(&self) -> &f32 {
        &self.forward_light_scattering_extinction
    }
    fn forward_light_scattering_smoothness(&self) -> &f32 {
        &self.forward_light_scattering_smoothness
    }
    fn forward_light_scattering_attenuation_type(&self) -> &ForwardLightScatteringAttenuation {
        &self.forward_light_scattering_attenuation_type
    }
    fn height_fog_enable(&self) -> &bool {
        &self.height_fog_enable
    }
    fn height_fog_follow_camera(&self) -> &f32 {
        &self.height_fog_follow_camera
    }
    fn height_fog_altitude(&self) -> &f32 {
        &self.height_fog_altitude
    }
    fn height_fog_depth(&self) -> &f32 {
        &self.height_fog_depth
    }
    fn height_fog_visibility_range(&self) -> &f32 {
        &self.height_fog_visibility_range
    }
    fn participating_media_enable(&self) -> &bool {
        &self.participating_media_enable
    }
    fn depth_fog_participating_media(&self) -> &ParticipatingMedia {
        &self.depth_fog_participating_media
    }
    fn height_fog_participating_media(&self) -> &ParticipatingMedia {
        &self.height_fog_participating_media
    }
    fn fog_volume_strength(&self) -> &f32 {
        &self.fog_volume_strength
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_override1(&self) -> &u8 {
        &self.field_flag_override1
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u16 {
        &self.field_flag_changed1
    }
}

pub static FOGCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FogComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogComponentState, enable),
            },
            FieldInfoData {
                name: "FogDistanceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, fog_distance_multiplier),
            },
            FieldInfoData {
                name: "FogGradientEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogComponentState, fog_gradient_enable),
            },
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, start),
            },
            FieldInfoData {
                name: "End",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, end),
            },
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(FogComponentState, curve),
            },
            FieldInfoData {
                name: "FogGradientHeightFadeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogComponentState, fog_gradient_height_fade_enable),
            },
            FieldInfoData {
                name: "FadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, fade_start),
            },
            FieldInfoData {
                name: "FadeEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, fade_end),
            },
            FieldInfoData {
                name: "FogColorEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogComponentState, fog_color_enable),
            },
            FieldInfoData {
                name: "FogColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FogComponentState, fog_color),
            },
            FieldInfoData {
                name: "FogColorStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, fog_color_start),
            },
            FieldInfoData {
                name: "FogColorEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, fog_color_end),
            },
            FieldInfoData {
                name: "FogColorCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(FogComponentState, fog_color_curve),
            },
            FieldInfoData {
                name: "TransparencyFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, transparency_fade_start),
            },
            FieldInfoData {
                name: "TransparencyFadeEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, transparency_fade_end),
            },
            FieldInfoData {
                name: "TransparencyFadeClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, transparency_fade_clamp),
            },
            FieldInfoData {
                name: "TransparencyFadeCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(FogComponentState, transparency_fade_curve),
            },
            FieldInfoData {
                name: "ForwardLightScatteringEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_enabled),
            },
            FieldInfoData {
                name: "ForwardLightScatteringUseSunPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_use_sun_position),
            },
            FieldInfoData {
                name: "ForwardLightScatteringRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_rotation_x),
            },
            FieldInfoData {
                name: "ForwardLightScatteringRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_rotation_y),
            },
            FieldInfoData {
                name: "ForwardLightScatteringPhaseG",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_phase_g),
            },
            FieldInfoData {
                name: "ForwardLightScatteringStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_strength),
            },
            FieldInfoData {
                name: "ForwardLightScatteringColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_color),
            },
            FieldInfoData {
                name: "ForwardLightScatteringPresence",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_presence),
            },
            FieldInfoData {
                name: "ForwardLightScatteringMaxBlurLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_max_blur_length),
            },
            FieldInfoData {
                name: "ForwardLightScatteringExtinction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_extinction),
            },
            FieldInfoData {
                name: "ForwardLightScatteringSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_smoothness),
            },
            FieldInfoData {
                name: "ForwardLightScatteringAttenuationType",
                flags: MemberInfoFlags::new(0),
                field_type: "ForwardLightScatteringAttenuation",
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_attenuation_type),
            },
            FieldInfoData {
                name: "HeightFogEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogComponentState, height_fog_enable),
            },
            FieldInfoData {
                name: "HeightFogFollowCamera",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, height_fog_follow_camera),
            },
            FieldInfoData {
                name: "HeightFogAltitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, height_fog_altitude),
            },
            FieldInfoData {
                name: "HeightFogDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, height_fog_depth),
            },
            FieldInfoData {
                name: "HeightFogVisibilityRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, height_fog_visibility_range),
            },
            FieldInfoData {
                name: "ParticipatingMediaEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogComponentState, participating_media_enable),
            },
            FieldInfoData {
                name: "DepthFogParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: "ParticipatingMedia",
                rust_offset: offset_of!(FogComponentState, depth_fog_participating_media),
            },
            FieldInfoData {
                name: "HeightFogParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: "ParticipatingMedia",
                rust_offset: offset_of!(FogComponentState, height_fog_participating_media),
            },
            FieldInfoData {
                name: "FogVolumeStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogComponentState, fog_volume_strength),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FogComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(FogComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FogComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(FogComponentState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(FOGCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FogComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        FOGCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FOGCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FogComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OutdoorLightComponentState {
    pub enable: bool,
    pub sun_rotation_x: f32,
    pub sun_rotation_y: f32,
    pub shadow_sun_rotation_enable: bool,
    pub shadow_sun_rotation_x: f32,
    pub shadow_sun_rotation_y: f32,
    pub sun_color: super::core::Vec3,
    pub sun_intensity: f32,
    pub final_sun_luminance: super::core::Vec3,
    pub final_sun_illuminance: super::core::Vec3,
    pub outer_space_sun_luminance: super::core::Vec3,
    pub outer_space_sun_illuminance1: super::core::Vec3,
    pub outer_space_sun_illuminance2: super::core::Vec3,
    pub sun_angular_radius: f32,
    pub sky_color: super::core::Vec3,
    pub ground_color: super::core::Vec3,
    pub sky_light_angle_factor: f32,
    pub sun_specular_scale: f32,
    pub sky_envmap_shadow_scale: f32,
    pub cascade_shadow_enable: bool,
    pub sun_shadow_height_scale: f32,
    pub sun_shadow_filter_type: ShadowFilteringType,
    pub sun_shadow_forward_quality: super::render_base::ShaderShadowmapQuality,
    pub sun_pcss_filter_adaptive: bool,
    pub sun_pcss_initial_sample_count: i32,
    pub sun_pcss_maximum_sample_count: i32,
    pub sun_pcss_filter_error_threshold_pct: f32,
    pub sun_penumbra_size: f32,
    pub sun_pcss_shadow_filter_scale: f32,
    pub cloud_shadow_enable: bool,
    pub cloud_shadow_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub cloud_shadow_speed: super::core::Vec2,
    pub cloud_shadow_size: f32,
    pub cloud_shadow_coverage: f32,
    pub cloud_shadow_exponent: f32,
    pub cloud_shadow_is_top_down: bool,
    pub cloud_shadow_start_fade: f32,
    pub cloud_shadows_fade_distance: f32,
    pub cloud_shadow_height_fade_enable: bool,
    pub cloud_shadow_start_height_fade: f32,
    pub cloud_shadows_height_fade_distance: f32,
    pub cloud_x_z_translation: super::core::Vec2,
    pub cloud_shadow_addressing_mode: super::render_base::TextureAddress,
    pub cloud_radiosity_enable: bool,
    pub secondary_cloud_shadow_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub secondary_cloud_shadow_speed: super::core::Vec2,
    pub secondary_cloud_shadow_size: f32,
    pub secondary_cloud_shadow_coverage: f32,
    pub secondary_cloud_shadow_exponent: f32,
    pub secondary_cloud_shadow_is_top_down: bool,
    pub secondary_cloud_x_z_translation: super::core::Vec2,
    pub secondary_cloud_shadow_addressing_mode: super::render_base::TextureAddress,
    pub cast_terrain_shadows_enable: bool,
    pub translucency_ambient: f32,
    pub translucency_scale: f32,
    pub translucency_power: f32,
    pub translucency_distortion: f32,
    pub particle_sun_shadow_factor: f32,
    pub particle_sun_shadow_smoothing: f32,
    pub field_flag_override0: u32,
    pub field_flag_override1: u32,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u32,
}

pub trait OutdoorLightComponentStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn sun_rotation_x(&self) -> &f32;
    fn sun_rotation_y(&self) -> &f32;
    fn shadow_sun_rotation_enable(&self) -> &bool;
    fn shadow_sun_rotation_x(&self) -> &f32;
    fn shadow_sun_rotation_y(&self) -> &f32;
    fn sun_color(&self) -> &super::core::Vec3;
    fn sun_intensity(&self) -> &f32;
    fn final_sun_luminance(&self) -> &super::core::Vec3;
    fn final_sun_illuminance(&self) -> &super::core::Vec3;
    fn outer_space_sun_luminance(&self) -> &super::core::Vec3;
    fn outer_space_sun_illuminance1(&self) -> &super::core::Vec3;
    fn outer_space_sun_illuminance2(&self) -> &super::core::Vec3;
    fn sun_angular_radius(&self) -> &f32;
    fn sky_color(&self) -> &super::core::Vec3;
    fn ground_color(&self) -> &super::core::Vec3;
    fn sky_light_angle_factor(&self) -> &f32;
    fn sun_specular_scale(&self) -> &f32;
    fn sky_envmap_shadow_scale(&self) -> &f32;
    fn cascade_shadow_enable(&self) -> &bool;
    fn sun_shadow_height_scale(&self) -> &f32;
    fn sun_shadow_filter_type(&self) -> &ShadowFilteringType;
    fn sun_shadow_forward_quality(&self) -> &super::render_base::ShaderShadowmapQuality;
    fn sun_pcss_filter_adaptive(&self) -> &bool;
    fn sun_pcss_initial_sample_count(&self) -> &i32;
    fn sun_pcss_maximum_sample_count(&self) -> &i32;
    fn sun_pcss_filter_error_threshold_pct(&self) -> &f32;
    fn sun_penumbra_size(&self) -> &f32;
    fn sun_pcss_shadow_filter_scale(&self) -> &f32;
    fn cloud_shadow_enable(&self) -> &bool;
    fn cloud_shadow_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn cloud_shadow_speed(&self) -> &super::core::Vec2;
    fn cloud_shadow_size(&self) -> &f32;
    fn cloud_shadow_coverage(&self) -> &f32;
    fn cloud_shadow_exponent(&self) -> &f32;
    fn cloud_shadow_is_top_down(&self) -> &bool;
    fn cloud_shadow_start_fade(&self) -> &f32;
    fn cloud_shadows_fade_distance(&self) -> &f32;
    fn cloud_shadow_height_fade_enable(&self) -> &bool;
    fn cloud_shadow_start_height_fade(&self) -> &f32;
    fn cloud_shadows_height_fade_distance(&self) -> &f32;
    fn cloud_x_z_translation(&self) -> &super::core::Vec2;
    fn cloud_shadow_addressing_mode(&self) -> &super::render_base::TextureAddress;
    fn cloud_radiosity_enable(&self) -> &bool;
    fn secondary_cloud_shadow_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn secondary_cloud_shadow_speed(&self) -> &super::core::Vec2;
    fn secondary_cloud_shadow_size(&self) -> &f32;
    fn secondary_cloud_shadow_coverage(&self) -> &f32;
    fn secondary_cloud_shadow_exponent(&self) -> &f32;
    fn secondary_cloud_shadow_is_top_down(&self) -> &bool;
    fn secondary_cloud_x_z_translation(&self) -> &super::core::Vec2;
    fn secondary_cloud_shadow_addressing_mode(&self) -> &super::render_base::TextureAddress;
    fn cast_terrain_shadows_enable(&self) -> &bool;
    fn translucency_ambient(&self) -> &f32;
    fn translucency_scale(&self) -> &f32;
    fn translucency_power(&self) -> &f32;
    fn translucency_distortion(&self) -> &f32;
    fn particle_sun_shadow_factor(&self) -> &f32;
    fn particle_sun_shadow_smoothing(&self) -> &f32;
    fn field_flag_override0(&self) -> &u32;
    fn field_flag_override1(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u32;
}

impl OutdoorLightComponentStateTrait for OutdoorLightComponentState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn sun_rotation_x(&self) -> &f32 {
        &self.sun_rotation_x
    }
    fn sun_rotation_y(&self) -> &f32 {
        &self.sun_rotation_y
    }
    fn shadow_sun_rotation_enable(&self) -> &bool {
        &self.shadow_sun_rotation_enable
    }
    fn shadow_sun_rotation_x(&self) -> &f32 {
        &self.shadow_sun_rotation_x
    }
    fn shadow_sun_rotation_y(&self) -> &f32 {
        &self.shadow_sun_rotation_y
    }
    fn sun_color(&self) -> &super::core::Vec3 {
        &self.sun_color
    }
    fn sun_intensity(&self) -> &f32 {
        &self.sun_intensity
    }
    fn final_sun_luminance(&self) -> &super::core::Vec3 {
        &self.final_sun_luminance
    }
    fn final_sun_illuminance(&self) -> &super::core::Vec3 {
        &self.final_sun_illuminance
    }
    fn outer_space_sun_luminance(&self) -> &super::core::Vec3 {
        &self.outer_space_sun_luminance
    }
    fn outer_space_sun_illuminance1(&self) -> &super::core::Vec3 {
        &self.outer_space_sun_illuminance1
    }
    fn outer_space_sun_illuminance2(&self) -> &super::core::Vec3 {
        &self.outer_space_sun_illuminance2
    }
    fn sun_angular_radius(&self) -> &f32 {
        &self.sun_angular_radius
    }
    fn sky_color(&self) -> &super::core::Vec3 {
        &self.sky_color
    }
    fn ground_color(&self) -> &super::core::Vec3 {
        &self.ground_color
    }
    fn sky_light_angle_factor(&self) -> &f32 {
        &self.sky_light_angle_factor
    }
    fn sun_specular_scale(&self) -> &f32 {
        &self.sun_specular_scale
    }
    fn sky_envmap_shadow_scale(&self) -> &f32 {
        &self.sky_envmap_shadow_scale
    }
    fn cascade_shadow_enable(&self) -> &bool {
        &self.cascade_shadow_enable
    }
    fn sun_shadow_height_scale(&self) -> &f32 {
        &self.sun_shadow_height_scale
    }
    fn sun_shadow_filter_type(&self) -> &ShadowFilteringType {
        &self.sun_shadow_filter_type
    }
    fn sun_shadow_forward_quality(&self) -> &super::render_base::ShaderShadowmapQuality {
        &self.sun_shadow_forward_quality
    }
    fn sun_pcss_filter_adaptive(&self) -> &bool {
        &self.sun_pcss_filter_adaptive
    }
    fn sun_pcss_initial_sample_count(&self) -> &i32 {
        &self.sun_pcss_initial_sample_count
    }
    fn sun_pcss_maximum_sample_count(&self) -> &i32 {
        &self.sun_pcss_maximum_sample_count
    }
    fn sun_pcss_filter_error_threshold_pct(&self) -> &f32 {
        &self.sun_pcss_filter_error_threshold_pct
    }
    fn sun_penumbra_size(&self) -> &f32 {
        &self.sun_penumbra_size
    }
    fn sun_pcss_shadow_filter_scale(&self) -> &f32 {
        &self.sun_pcss_shadow_filter_scale
    }
    fn cloud_shadow_enable(&self) -> &bool {
        &self.cloud_shadow_enable
    }
    fn cloud_shadow_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.cloud_shadow_texture
    }
    fn cloud_shadow_speed(&self) -> &super::core::Vec2 {
        &self.cloud_shadow_speed
    }
    fn cloud_shadow_size(&self) -> &f32 {
        &self.cloud_shadow_size
    }
    fn cloud_shadow_coverage(&self) -> &f32 {
        &self.cloud_shadow_coverage
    }
    fn cloud_shadow_exponent(&self) -> &f32 {
        &self.cloud_shadow_exponent
    }
    fn cloud_shadow_is_top_down(&self) -> &bool {
        &self.cloud_shadow_is_top_down
    }
    fn cloud_shadow_start_fade(&self) -> &f32 {
        &self.cloud_shadow_start_fade
    }
    fn cloud_shadows_fade_distance(&self) -> &f32 {
        &self.cloud_shadows_fade_distance
    }
    fn cloud_shadow_height_fade_enable(&self) -> &bool {
        &self.cloud_shadow_height_fade_enable
    }
    fn cloud_shadow_start_height_fade(&self) -> &f32 {
        &self.cloud_shadow_start_height_fade
    }
    fn cloud_shadows_height_fade_distance(&self) -> &f32 {
        &self.cloud_shadows_height_fade_distance
    }
    fn cloud_x_z_translation(&self) -> &super::core::Vec2 {
        &self.cloud_x_z_translation
    }
    fn cloud_shadow_addressing_mode(&self) -> &super::render_base::TextureAddress {
        &self.cloud_shadow_addressing_mode
    }
    fn cloud_radiosity_enable(&self) -> &bool {
        &self.cloud_radiosity_enable
    }
    fn secondary_cloud_shadow_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.secondary_cloud_shadow_texture
    }
    fn secondary_cloud_shadow_speed(&self) -> &super::core::Vec2 {
        &self.secondary_cloud_shadow_speed
    }
    fn secondary_cloud_shadow_size(&self) -> &f32 {
        &self.secondary_cloud_shadow_size
    }
    fn secondary_cloud_shadow_coverage(&self) -> &f32 {
        &self.secondary_cloud_shadow_coverage
    }
    fn secondary_cloud_shadow_exponent(&self) -> &f32 {
        &self.secondary_cloud_shadow_exponent
    }
    fn secondary_cloud_shadow_is_top_down(&self) -> &bool {
        &self.secondary_cloud_shadow_is_top_down
    }
    fn secondary_cloud_x_z_translation(&self) -> &super::core::Vec2 {
        &self.secondary_cloud_x_z_translation
    }
    fn secondary_cloud_shadow_addressing_mode(&self) -> &super::render_base::TextureAddress {
        &self.secondary_cloud_shadow_addressing_mode
    }
    fn cast_terrain_shadows_enable(&self) -> &bool {
        &self.cast_terrain_shadows_enable
    }
    fn translucency_ambient(&self) -> &f32 {
        &self.translucency_ambient
    }
    fn translucency_scale(&self) -> &f32 {
        &self.translucency_scale
    }
    fn translucency_power(&self) -> &f32 {
        &self.translucency_power
    }
    fn translucency_distortion(&self) -> &f32 {
        &self.translucency_distortion
    }
    fn particle_sun_shadow_factor(&self) -> &f32 {
        &self.particle_sun_shadow_factor
    }
    fn particle_sun_shadow_smoothing(&self) -> &f32 {
        &self.particle_sun_shadow_smoothing
    }
    fn field_flag_override0(&self) -> &u32 {
        &self.field_flag_override0
    }
    fn field_flag_override1(&self) -> &u32 {
        &self.field_flag_override1
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u32 {
        &self.field_flag_changed1
    }
}

pub static OUTDOORLIGHTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OutdoorLightComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightComponentState, enable),
            },
            FieldInfoData {
                name: "SunRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_rotation_x),
            },
            FieldInfoData {
                name: "SunRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_rotation_y),
            },
            FieldInfoData {
                name: "ShadowSunRotationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightComponentState, shadow_sun_rotation_enable),
            },
            FieldInfoData {
                name: "ShadowSunRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, shadow_sun_rotation_x),
            },
            FieldInfoData {
                name: "ShadowSunRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, shadow_sun_rotation_y),
            },
            FieldInfoData {
                name: "SunColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_color),
            },
            FieldInfoData {
                name: "SunIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_intensity),
            },
            FieldInfoData {
                name: "FinalSunLuminance",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightComponentState, final_sun_luminance),
            },
            FieldInfoData {
                name: "FinalSunIlluminance",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightComponentState, final_sun_illuminance),
            },
            FieldInfoData {
                name: "OuterSpaceSunLuminance",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightComponentState, outer_space_sun_luminance),
            },
            FieldInfoData {
                name: "OuterSpaceSunIlluminance1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightComponentState, outer_space_sun_illuminance1),
            },
            FieldInfoData {
                name: "OuterSpaceSunIlluminance2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightComponentState, outer_space_sun_illuminance2),
            },
            FieldInfoData {
                name: "SunAngularRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_angular_radius),
            },
            FieldInfoData {
                name: "SkyColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightComponentState, sky_color),
            },
            FieldInfoData {
                name: "GroundColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OutdoorLightComponentState, ground_color),
            },
            FieldInfoData {
                name: "SkyLightAngleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sky_light_angle_factor),
            },
            FieldInfoData {
                name: "SunSpecularScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_specular_scale),
            },
            FieldInfoData {
                name: "SkyEnvmapShadowScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sky_envmap_shadow_scale),
            },
            FieldInfoData {
                name: "CascadeShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightComponentState, cascade_shadow_enable),
            },
            FieldInfoData {
                name: "SunShadowHeightScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_shadow_height_scale),
            },
            FieldInfoData {
                name: "SunShadowFilterType",
                flags: MemberInfoFlags::new(0),
                field_type: "ShadowFilteringType",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_shadow_filter_type),
            },
            FieldInfoData {
                name: "SunShadowForwardQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderShadowmapQuality",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_shadow_forward_quality),
            },
            FieldInfoData {
                name: "SunPcssFilterAdaptive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_pcss_filter_adaptive),
            },
            FieldInfoData {
                name: "SunPcssInitialSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_pcss_initial_sample_count),
            },
            FieldInfoData {
                name: "SunPcssMaximumSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_pcss_maximum_sample_count),
            },
            FieldInfoData {
                name: "SunPcssFilterErrorThresholdPct",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_pcss_filter_error_threshold_pct),
            },
            FieldInfoData {
                name: "SunPenumbraSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_penumbra_size),
            },
            FieldInfoData {
                name: "SunPcssShadowFilterScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, sun_pcss_shadow_filter_scale),
            },
            FieldInfoData {
                name: "CloudShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_enable),
            },
            FieldInfoData {
                name: "CloudShadowTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_texture),
            },
            FieldInfoData {
                name: "CloudShadowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_speed),
            },
            FieldInfoData {
                name: "CloudShadowSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_size),
            },
            FieldInfoData {
                name: "CloudShadowCoverage",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_coverage),
            },
            FieldInfoData {
                name: "CloudShadowExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_exponent),
            },
            FieldInfoData {
                name: "CloudShadowIsTopDown",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_is_top_down),
            },
            FieldInfoData {
                name: "CloudShadowStartFade",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_start_fade),
            },
            FieldInfoData {
                name: "CloudShadowsFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadows_fade_distance),
            },
            FieldInfoData {
                name: "CloudShadowHeightFadeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_height_fade_enable),
            },
            FieldInfoData {
                name: "CloudShadowStartHeightFade",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_start_height_fade),
            },
            FieldInfoData {
                name: "CloudShadowsHeightFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadows_height_fade_distance),
            },
            FieldInfoData {
                name: "CloudXZTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_x_z_translation),
            },
            FieldInfoData {
                name: "CloudShadowAddressingMode",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAddress",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_addressing_mode),
            },
            FieldInfoData {
                name: "CloudRadiosityEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_radiosity_enable),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_texture),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_speed),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_size),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowCoverage",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_coverage),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowExponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_exponent),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowIsTopDown",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_is_top_down),
            },
            FieldInfoData {
                name: "SecondaryCloudXZTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_x_z_translation),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowAddressingMode",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAddress",
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_addressing_mode),
            },
            FieldInfoData {
                name: "CastTerrainShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OutdoorLightComponentState, cast_terrain_shadows_enable),
            },
            FieldInfoData {
                name: "TranslucencyAmbient",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, translucency_ambient),
            },
            FieldInfoData {
                name: "TranslucencyScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, translucency_scale),
            },
            FieldInfoData {
                name: "TranslucencyPower",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, translucency_power),
            },
            FieldInfoData {
                name: "TranslucencyDistortion",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, translucency_distortion),
            },
            FieldInfoData {
                name: "ParticleSunShadowFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, particle_sun_shadow_factor),
            },
            FieldInfoData {
                name: "ParticleSunShadowSmoothing",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OutdoorLightComponentState, particle_sun_shadow_smoothing),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OutdoorLightComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OutdoorLightComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OutdoorLightComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OutdoorLightComponentState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(OUTDOORLIGHTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OutdoorLightComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        OUTDOORLIGHTCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OUTDOORLIGHTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OutdoorLightComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IndirectSpecularComponentState {
    pub enabled: bool,
    pub intensity: f32,
    pub reflectance_scale: f32,
    pub probes_intensity: f32,
    pub probes_reflectance_scale: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait IndirectSpecularComponentStateTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn intensity(&self) -> &f32;
    fn reflectance_scale(&self) -> &f32;
    fn probes_intensity(&self) -> &f32;
    fn probes_reflectance_scale(&self) -> &f32;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl IndirectSpecularComponentStateTrait for IndirectSpecularComponentState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn reflectance_scale(&self) -> &f32 {
        &self.reflectance_scale
    }
    fn probes_intensity(&self) -> &f32 {
        &self.probes_intensity
    }
    fn probes_reflectance_scale(&self) -> &f32 {
        &self.probes_reflectance_scale
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static INDIRECTSPECULARCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndirectSpecularComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IndirectSpecularComponentState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IndirectSpecularComponentState, enabled),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IndirectSpecularComponentState, intensity),
            },
            FieldInfoData {
                name: "ReflectanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IndirectSpecularComponentState, reflectance_scale),
            },
            FieldInfoData {
                name: "ProbesIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IndirectSpecularComponentState, probes_intensity),
            },
            FieldInfoData {
                name: "ProbesReflectanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IndirectSpecularComponentState, probes_reflectance_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(IndirectSpecularComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(IndirectSpecularComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(INDIRECTSPECULARCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IndirectSpecularComponentState {
    fn type_info(&self) -> &'static TypeInfo {
        INDIRECTSPECULARCOMPONENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INDIRECTSPECULARCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndirectSpecularComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("IndirectSpecularComponentState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ShadowFilteringType {
    #[default]
    ShadowFilteringType_Pcf = 0,
    ShadowFilteringType_ContactHardening = 1,
    ShadowFilteringType_Pcss = 2,
}

pub static SHADOWFILTERINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowFilteringType",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADOWFILTERINGTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShadowFilteringType {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWFILTERINGTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SHADOWFILTERINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowFilteringType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ShadowFilteringType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualEnvironmentComponentStaticState {
    pub visual_environment: VisualEnvironmentHandle,
    pub index: u32,
    pub field_flag_changed0: u8,
}

pub trait VisualEnvironmentComponentStaticStateTrait: TypeObject {
    fn visual_environment(&self) -> &VisualEnvironmentHandle;
    fn index(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u8;
}

impl VisualEnvironmentComponentStaticStateTrait for VisualEnvironmentComponentStaticState {
    fn visual_environment(&self) -> &VisualEnvironmentHandle {
        &self.visual_environment
    }
    fn index(&self) -> &u32 {
        &self.index
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static VISUALENVIRONMENTCOMPONENTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentComponentStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualEnvironmentComponentStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "VisualEnvironment",
                flags: MemberInfoFlags::new(0),
                field_type: "VisualEnvironmentHandle",
                rust_offset: offset_of!(VisualEnvironmentComponentStaticState, visual_environment),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualEnvironmentComponentStaticState, index),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VisualEnvironmentComponentStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTCOMPONENTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VisualEnvironmentComponentStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALENVIRONMENTCOMPONENTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALENVIRONMENTCOMPONENTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentComponentStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisualEnvironmentComponentStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualEnvironmentDynamicState {
    pub visibility: f32,
    pub priority: i32,
    pub blend_mode: u8,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub override_visibility: bool,
    pub field_flag_changed0: u8,
}

pub trait VisualEnvironmentDynamicStateTrait: TypeObject {
    fn visibility(&self) -> &f32;
    fn priority(&self) -> &i32;
    fn blend_mode(&self) -> &u8;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn override_visibility(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl VisualEnvironmentDynamicStateTrait for VisualEnvironmentDynamicState {
    fn visibility(&self) -> &f32 {
        &self.visibility
    }
    fn priority(&self) -> &i32 {
        &self.priority
    }
    fn blend_mode(&self) -> &u8 {
        &self.blend_mode
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
    fn override_visibility(&self) -> &bool {
        &self.override_visibility
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static VISUALENVIRONMENTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualEnvironmentDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Visibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualEnvironmentDynamicState, visibility),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VisualEnvironmentDynamicState, priority),
            },
            FieldInfoData {
                name: "BlendMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VisualEnvironmentDynamicState, blend_mode),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(VisualEnvironmentDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(VisualEnvironmentDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "OverrideVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualEnvironmentDynamicState, override_visibility),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VisualEnvironmentDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VisualEnvironmentDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALENVIRONMENTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALENVIRONMENTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisualEnvironmentDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualEnvironmentStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub debug_name: String,
    pub field_flag_changed0: u8,
}

pub trait VisualEnvironmentStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn debug_name(&self) -> &String;
    fn field_flag_changed0(&self) -> &u8;
}

impl VisualEnvironmentStaticStateTrait for VisualEnvironmentStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn debug_name(&self) -> &String {
        &self.debug_name
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static VISUALENVIRONMENTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualEnvironmentStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(VisualEnvironmentStaticState, transform_space),
            },
            FieldInfoData {
                name: "DebugName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VisualEnvironmentStaticState, debug_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VisualEnvironmentStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualEnvironmentStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALENVIRONMENTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALENVIRONMENTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisualEnvironmentStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualEnvironmentHandle {
}

pub trait VisualEnvironmentHandleTrait: TypeObject {
}

impl VisualEnvironmentHandleTrait for VisualEnvironmentHandle {
}

pub static VISUALENVIRONMENTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualEnvironmentHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VISUALENVIRONMENTHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for VisualEnvironmentHandle {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALENVIRONMENTHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALENVIRONMENTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisualEnvironmentHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ForwardLightScatteringAttenuation {
    #[default]
    ForwardLightScatteringAttenuation_None = 0,
    ForwardLightScatteringAttenuation_Corner = 1,
}

pub static FORWARDLIGHTSCATTERINGATTENUATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForwardLightScatteringAttenuation",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(FORWARDLIGHTSCATTERINGATTENUATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ForwardLightScatteringAttenuation {
    fn type_info(&self) -> &'static TypeInfo {
        FORWARDLIGHTSCATTERINGATTENUATION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FORWARDLIGHTSCATTERINGATTENUATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForwardLightScatteringAttenuation-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ForwardLightScatteringAttenuation"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualEnvironmentSettings {
    pub _glacier_base: super::core::DataContainer,
    pub sun_rotation_x: f32,
    pub sun_rotation_y: f32,
    pub sky_rotation_phi: f32,
    pub draw_stats: i32,
    pub draw_only_visible_stats: bool,
    pub hdr_grading_enable: bool,
}

pub trait VisualEnvironmentSettingsTrait: super::core::DataContainerTrait {
    fn sun_rotation_x(&self) -> &f32;
    fn sun_rotation_y(&self) -> &f32;
    fn sky_rotation_phi(&self) -> &f32;
    fn draw_stats(&self) -> &i32;
    fn draw_only_visible_stats(&self) -> &bool;
    fn hdr_grading_enable(&self) -> &bool;
}

impl VisualEnvironmentSettingsTrait for VisualEnvironmentSettings {
    fn sun_rotation_x(&self) -> &f32 {
        &self.sun_rotation_x
    }
    fn sun_rotation_y(&self) -> &f32 {
        &self.sun_rotation_y
    }
    fn sky_rotation_phi(&self) -> &f32 {
        &self.sky_rotation_phi
    }
    fn draw_stats(&self) -> &i32 {
        &self.draw_stats
    }
    fn draw_only_visible_stats(&self) -> &bool {
        &self.draw_only_visible_stats
    }
    fn hdr_grading_enable(&self) -> &bool {
        &self.hdr_grading_enable
    }
}

impl super::core::DataContainerTrait for VisualEnvironmentSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VISUALENVIRONMENTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentSettings",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualEnvironmentSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SunRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualEnvironmentSettings, sun_rotation_x),
            },
            FieldInfoData {
                name: "SunRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualEnvironmentSettings, sun_rotation_y),
            },
            FieldInfoData {
                name: "SkyRotationPhi",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualEnvironmentSettings, sky_rotation_phi),
            },
            FieldInfoData {
                name: "DrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VisualEnvironmentSettings, draw_stats),
            },
            FieldInfoData {
                name: "DrawOnlyVisibleStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualEnvironmentSettings, draw_only_visible_stats),
            },
            FieldInfoData {
                name: "HdrGradingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualEnvironmentSettings, hdr_grading_enable),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualEnvironmentSettings {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALENVIRONMENTSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALENVIRONMENTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisualEnvironmentSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TextureSetDynamicState {
}

pub trait TextureSetDynamicStateTrait: TypeObject {
}

impl TextureSetDynamicStateTrait for TextureSetDynamicState {
}

pub static TEXTURESETDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSetDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TextureSetDynamicState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TEXTURESETDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureSetDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTURESETDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TEXTURESETDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSetDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("TextureSetDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TextureSetStaticState {
    pub dds: Vec<u8>,
    pub texture: super::render_base::TextureResourceHandle,
    pub field_flag_changed0: u8,
}

pub trait TextureSetStaticStateTrait: TypeObject {
    fn dds(&self) -> &Vec<u8>;
    fn texture(&self) -> &super::render_base::TextureResourceHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl TextureSetStaticStateTrait for TextureSetStaticState {
    fn dds(&self) -> &Vec<u8> {
        &self.dds
    }
    fn texture(&self) -> &super::render_base::TextureResourceHandle {
        &self.texture
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static TEXTURESETSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSetStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TextureSetStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Dds",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint8-Array",
                rust_offset: offset_of!(TextureSetStaticState, dds),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureResourceHandle",
                rust_offset: offset_of!(TextureSetStaticState, texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TextureSetStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TEXTURESETSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureSetStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTURESETSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TEXTURESETSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSetStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("TextureSetStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshDeformerHandle {
}

pub trait MeshDeformerHandleTrait: TypeObject {
}

impl MeshDeformerHandleTrait for MeshDeformerHandle {
}

pub static MESHDEFORMERHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDeformerHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshDeformerHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHDEFORMERHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MeshDeformerHandle {
    fn type_info(&self) -> &'static TypeInfo {
        MESHDEFORMERHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHDEFORMERHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDeformerHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("MeshDeformerHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightHandle {
}

pub trait LightHandleTrait: TypeObject {
}

impl LightHandleTrait for LightHandle {
}

pub static LIGHTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LIGHTHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LightHandle {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LightHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ModelHandle {
}

pub trait ModelHandleTrait: TypeObject {
}

impl ModelHandleTrait for ModelHandle {
}

pub static MODELHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ModelHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MODELHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ModelHandle {
    fn type_info(&self) -> &'static TypeInfo {
        MODELHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MODELHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ModelHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WasVisibleHandle {
}

pub trait WasVisibleHandleTrait: TypeObject {
}

impl WasVisibleHandleTrait for WasVisibleHandle {
}

pub static WASVISIBLEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WasVisibleHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WasVisibleHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WASVISIBLEHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for WasVisibleHandle {
    fn type_info(&self) -> &'static TypeInfo {
        WASVISIBLEHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WASVISIBLEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WasVisibleHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("WasVisibleHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SimpleVolumetricsDynamicState {
    pub transform: super::core::LinearTransform,
    pub exponent: f32,
    pub emission: super::core::Vec3,
    pub emission_scale: f32,
    pub field_flag_changed0: u8,
}

pub trait SimpleVolumetricsDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn exponent(&self) -> &f32;
    fn emission(&self) -> &super::core::Vec3;
    fn emission_scale(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u8;
}

impl SimpleVolumetricsDynamicStateTrait for SimpleVolumetricsDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn exponent(&self) -> &f32 {
        &self.exponent
    }
    fn emission(&self) -> &super::core::Vec3 {
        &self.emission
    }
    fn emission_scale(&self) -> &f32 {
        &self.emission_scale
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static SIMPLEVOLUMETRICSDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SimpleVolumetricsDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(SimpleVolumetricsDynamicState, transform),
            },
            FieldInfoData {
                name: "Exponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SimpleVolumetricsDynamicState, exponent),
            },
            FieldInfoData {
                name: "Emission",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SimpleVolumetricsDynamicState, emission),
            },
            FieldInfoData {
                name: "EmissionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SimpleVolumetricsDynamicState, emission_scale),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SimpleVolumetricsDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SIMPLEVOLUMETRICSDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SimpleVolumetricsDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        SIMPLEVOLUMETRICSDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SIMPLEVOLUMETRICSDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SimpleVolumetricsDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SimpleVolumetricsStaticState {
    pub fade_out_end_radius: f32,
    pub fade_out_start_radius: f32,
    pub far_fade_start_distance: f32,
    pub far_fade_end_distance: f32,
    pub use_clipping_plane: bool,
    pub clipping_plane_offset: f32,
    pub draw_pass: SimpleVolumetricsDrawPass,
    pub scale_to_exposure: bool,
    pub field_flag_changed0: u8,
}

pub trait SimpleVolumetricsStaticStateTrait: TypeObject {
    fn fade_out_end_radius(&self) -> &f32;
    fn fade_out_start_radius(&self) -> &f32;
    fn far_fade_start_distance(&self) -> &f32;
    fn far_fade_end_distance(&self) -> &f32;
    fn use_clipping_plane(&self) -> &bool;
    fn clipping_plane_offset(&self) -> &f32;
    fn draw_pass(&self) -> &SimpleVolumetricsDrawPass;
    fn scale_to_exposure(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl SimpleVolumetricsStaticStateTrait for SimpleVolumetricsStaticState {
    fn fade_out_end_radius(&self) -> &f32 {
        &self.fade_out_end_radius
    }
    fn fade_out_start_radius(&self) -> &f32 {
        &self.fade_out_start_radius
    }
    fn far_fade_start_distance(&self) -> &f32 {
        &self.far_fade_start_distance
    }
    fn far_fade_end_distance(&self) -> &f32 {
        &self.far_fade_end_distance
    }
    fn use_clipping_plane(&self) -> &bool {
        &self.use_clipping_plane
    }
    fn clipping_plane_offset(&self) -> &f32 {
        &self.clipping_plane_offset
    }
    fn draw_pass(&self) -> &SimpleVolumetricsDrawPass {
        &self.draw_pass
    }
    fn scale_to_exposure(&self) -> &bool {
        &self.scale_to_exposure
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static SIMPLEVOLUMETRICSSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SimpleVolumetricsStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FadeOutEndRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SimpleVolumetricsStaticState, fade_out_end_radius),
            },
            FieldInfoData {
                name: "FadeOutStartRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SimpleVolumetricsStaticState, fade_out_start_radius),
            },
            FieldInfoData {
                name: "FarFadeStartDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SimpleVolumetricsStaticState, far_fade_start_distance),
            },
            FieldInfoData {
                name: "FarFadeEndDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SimpleVolumetricsStaticState, far_fade_end_distance),
            },
            FieldInfoData {
                name: "UseClippingPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SimpleVolumetricsStaticState, use_clipping_plane),
            },
            FieldInfoData {
                name: "ClippingPlaneOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SimpleVolumetricsStaticState, clipping_plane_offset),
            },
            FieldInfoData {
                name: "DrawPass",
                flags: MemberInfoFlags::new(0),
                field_type: "SimpleVolumetricsDrawPass",
                rust_offset: offset_of!(SimpleVolumetricsStaticState, draw_pass),
            },
            FieldInfoData {
                name: "ScaleToExposure",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SimpleVolumetricsStaticState, scale_to_exposure),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SimpleVolumetricsStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SIMPLEVOLUMETRICSSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SimpleVolumetricsStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        SIMPLEVOLUMETRICSSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SIMPLEVOLUMETRICSSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SimpleVolumetricsStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SimpleVolumetricsDrawPass {
    #[default]
    SimpleVolumetricsDrawPass_BeforeTransparent = 0,
    SimpleVolumetricsDrawPass_AfterTransparent = 1,
    SimpleVolumetricsDrawPass_Count = 2,
}

pub static SIMPLEVOLUMETRICSDRAWPASS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsDrawPass",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SIMPLEVOLUMETRICSDRAWPASS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SimpleVolumetricsDrawPass {
    fn type_info(&self) -> &'static TypeInfo {
        SIMPLEVOLUMETRICSDRAWPASS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SIMPLEVOLUMETRICSDRAWPASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsDrawPass-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SimpleVolumetricsDrawPass"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ResourceRefDynamicState {
}

pub trait ResourceRefDynamicStateTrait: TypeObject {
}

impl ResourceRefDynamicStateTrait for ResourceRefDynamicState {
}

pub static RESOURCEREFDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ResourceRefDynamicState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RESOURCEREFDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ResourceRefDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        RESOURCEREFDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RESOURCEREFDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ResourceRefDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ResourceRefStaticState {
    pub object: Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>,
    pub field_id: u32,
    pub field_flag_changed0: u8,
}

pub trait ResourceRefStaticStateTrait: TypeObject {
    fn object(&self) -> &Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>;
    fn field_id(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u8;
}

impl ResourceRefStaticStateTrait for ResourceRefStaticState {
    fn object(&self) -> &Option<Arc<Mutex<dyn super::core::DataContainerTrait>>> {
        &self.object
    }
    fn field_id(&self) -> &u32 {
        &self.field_id
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static RESOURCEREFSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ResourceRefStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Object",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(ResourceRefStaticState, object),
            },
            FieldInfoData {
                name: "FieldId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ResourceRefStaticState, field_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ResourceRefStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RESOURCEREFSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ResourceRefStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        RESOURCEREFSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RESOURCEREFSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ResourceRefStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ResourceRefHandle {
}

pub trait ResourceRefHandleTrait: TypeObject {
}

impl ResourceRefHandleTrait for ResourceRefHandle {
}

pub static RESOURCEREFHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ResourceRefHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RESOURCEREFHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for ResourceRefHandle {
    fn type_info(&self) -> &'static TypeInfo {
        RESOURCEREFHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RESOURCEREFHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ResourceRefHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OccluderPlaneDynamicState {
    pub transform: super::core::LinearTransform,
    pub visible: bool,
    pub field_flag_changed0: u8,
}

pub trait OccluderPlaneDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn visible(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl OccluderPlaneDynamicStateTrait for OccluderPlaneDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static OCCLUDERPLANEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OccluderPlaneDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(OccluderPlaneDynamicState, transform),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OccluderPlaneDynamicState, visible),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OccluderPlaneDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERPLANEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OccluderPlaneDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        OCCLUDERPLANEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OCCLUDERPLANEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderPlaneDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OccluderPlaneStaticState {
    pub occluder_high_priority: bool,
    pub occluder_is_conservative: bool,
    pub doubled_sided: bool,
    pub coverage_value: f32,
    pub field_flag_changed0: u8,
}

pub trait OccluderPlaneStaticStateTrait: TypeObject {
    fn occluder_high_priority(&self) -> &bool;
    fn occluder_is_conservative(&self) -> &bool;
    fn doubled_sided(&self) -> &bool;
    fn coverage_value(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u8;
}

impl OccluderPlaneStaticStateTrait for OccluderPlaneStaticState {
    fn occluder_high_priority(&self) -> &bool {
        &self.occluder_high_priority
    }
    fn occluder_is_conservative(&self) -> &bool {
        &self.occluder_is_conservative
    }
    fn doubled_sided(&self) -> &bool {
        &self.doubled_sided
    }
    fn coverage_value(&self) -> &f32 {
        &self.coverage_value
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static OCCLUDERPLANESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OccluderPlaneStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OccluderHighPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OccluderPlaneStaticState, occluder_high_priority),
            },
            FieldInfoData {
                name: "OccluderIsConservative",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OccluderPlaneStaticState, occluder_is_conservative),
            },
            FieldInfoData {
                name: "DoubledSided",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OccluderPlaneStaticState, doubled_sided),
            },
            FieldInfoData {
                name: "CoverageValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OccluderPlaneStaticState, coverage_value),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OccluderPlaneStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERPLANESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for OccluderPlaneStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        OCCLUDERPLANESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OCCLUDERPLANESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderPlaneStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OccluderVolumeDynamicState {
    pub transform: super::core::LinearTransform,
    pub visible: bool,
    pub field_flag_changed0: u8,
}

pub trait OccluderVolumeDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn visible(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl OccluderVolumeDynamicStateTrait for OccluderVolumeDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static OCCLUDERVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OccluderVolumeDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(OccluderVolumeDynamicState, transform),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OccluderVolumeDynamicState, visible),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OccluderVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OccluderVolumeDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        OCCLUDERVOLUMEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OCCLUDERVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderVolumeDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OccluderVolumeStaticState {
    pub occluder_high_priority: bool,
    pub occluder_is_conservative: bool,
    pub coverage_value: f32,
    pub field_flag_changed0: u8,
}

pub trait OccluderVolumeStaticStateTrait: TypeObject {
    fn occluder_high_priority(&self) -> &bool;
    fn occluder_is_conservative(&self) -> &bool;
    fn coverage_value(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u8;
}

impl OccluderVolumeStaticStateTrait for OccluderVolumeStaticState {
    fn occluder_high_priority(&self) -> &bool {
        &self.occluder_high_priority
    }
    fn occluder_is_conservative(&self) -> &bool {
        &self.occluder_is_conservative
    }
    fn coverage_value(&self) -> &f32 {
        &self.coverage_value
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static OCCLUDERVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OccluderVolumeStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OccluderHighPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OccluderVolumeStaticState, occluder_high_priority),
            },
            FieldInfoData {
                name: "OccluderIsConservative",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OccluderVolumeStaticState, occluder_is_conservative),
            },
            FieldInfoData {
                name: "CoverageValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OccluderVolumeStaticState, coverage_value),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OccluderVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for OccluderVolumeStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        OCCLUDERVOLUMESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OCCLUDERVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderVolumeStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OccluderMeshDynamicState {
    pub transform: super::core::LinearTransform,
    pub visible: bool,
    pub field_flag_changed0: u8,
}

pub trait OccluderMeshDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn visible(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl OccluderMeshDynamicStateTrait for OccluderMeshDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static OCCLUDERMESHDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OccluderMeshDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(OccluderMeshDynamicState, transform),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OccluderMeshDynamicState, visible),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OccluderMeshDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERMESHDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OccluderMeshDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        OCCLUDERMESHDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OCCLUDERMESHDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderMeshDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OccluderMeshStaticState {
    pub mesh: Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>,
    pub field_flag_changed0: u8,
}

pub trait OccluderMeshStaticStateTrait: TypeObject {
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>;
    fn field_flag_changed0(&self) -> &u8;
}

impl OccluderMeshStaticStateTrait for OccluderMeshStaticState {
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>> {
        &self.mesh
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static OCCLUDERMESHSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OccluderMeshStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(OccluderMeshStaticState, mesh),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OccluderMeshStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERMESHSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OccluderMeshStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        OCCLUDERMESHSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OCCLUDERMESHSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderMeshStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ObjectHighlightDynamicState {
    pub object_highlight_type: ObjectHighlightType,
    pub index: i32,
    pub enable: bool,
    pub draw_at_foreground: bool,
    pub is_mask: bool,
    pub field_flag_changed0: u8,
}

pub trait ObjectHighlightDynamicStateTrait: TypeObject {
    fn object_highlight_type(&self) -> &ObjectHighlightType;
    fn index(&self) -> &i32;
    fn enable(&self) -> &bool;
    fn draw_at_foreground(&self) -> &bool;
    fn is_mask(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl ObjectHighlightDynamicStateTrait for ObjectHighlightDynamicState {
    fn object_highlight_type(&self) -> &ObjectHighlightType {
        &self.object_highlight_type
    }
    fn index(&self) -> &i32 {
        &self.index
    }
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn draw_at_foreground(&self) -> &bool {
        &self.draw_at_foreground
    }
    fn is_mask(&self) -> &bool {
        &self.is_mask
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static OBJECTHIGHLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObjectHighlightDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ObjectHighlightType",
                flags: MemberInfoFlags::new(0),
                field_type: "ObjectHighlightType",
                rust_offset: offset_of!(ObjectHighlightDynamicState, object_highlight_type),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ObjectHighlightDynamicState, index),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ObjectHighlightDynamicState, enable),
            },
            FieldInfoData {
                name: "DrawAtForeground",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ObjectHighlightDynamicState, draw_at_foreground),
            },
            FieldInfoData {
                name: "IsMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ObjectHighlightDynamicState, is_mask),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ObjectHighlightDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OBJECTHIGHLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ObjectHighlightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        OBJECTHIGHLIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OBJECTHIGHLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ObjectHighlightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ObjectHighlightStaticState {
    pub render_view_handle: super::render_base::RenderViewHandle,
    pub model_handle: ModelHandle,
    pub field_flag_changed0: u8,
}

pub trait ObjectHighlightStaticStateTrait: TypeObject {
    fn render_view_handle(&self) -> &super::render_base::RenderViewHandle;
    fn model_handle(&self) -> &ModelHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl ObjectHighlightStaticStateTrait for ObjectHighlightStaticState {
    fn render_view_handle(&self) -> &super::render_base::RenderViewHandle {
        &self.render_view_handle
    }
    fn model_handle(&self) -> &ModelHandle {
        &self.model_handle
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static OBJECTHIGHLIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObjectHighlightStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RenderViewHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderViewHandle",
                rust_offset: offset_of!(ObjectHighlightStaticState, render_view_handle),
            },
            FieldInfoData {
                name: "ModelHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "ModelHandle",
                rust_offset: offset_of!(ObjectHighlightStaticState, model_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ObjectHighlightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OBJECTHIGHLIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ObjectHighlightStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        OBJECTHIGHLIGHTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OBJECTHIGHLIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ObjectHighlightStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ObjectHighlightType {
    #[default]
    ObjectHighlightType_Default = 0,
    ObjectHighlightType_ThreatOverlay = 1,
    ObjectHighlightType_Sonar = 2,
    ObjectHighlightType_EdgeDetect = 3,
}

pub static OBJECTHIGHLIGHTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightType",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(OBJECTHIGHLIGHTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ObjectHighlightType {
    fn type_info(&self) -> &'static TypeInfo {
        OBJECTHIGHLIGHTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OBJECTHIGHLIGHTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ObjectHighlightType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EdgeModelsBaseData {
    pub _glacier_base: super::core::DataContainer,
    pub instance_transforms: Vec<super::core::LinearTransform>,
    pub mesh_instance_ranges: Vec<u16>,
    pub connection_instance_lookup_table: Vec<u16>,
    pub connections: Vec<EdgeModelConnectionInfo>,
    pub part_connection_ranges: Vec<u16>,
}

pub trait EdgeModelsBaseDataTrait: super::core::DataContainerTrait {
    fn instance_transforms(&self) -> &Vec<super::core::LinearTransform>;
    fn mesh_instance_ranges(&self) -> &Vec<u16>;
    fn connection_instance_lookup_table(&self) -> &Vec<u16>;
    fn connections(&self) -> &Vec<EdgeModelConnectionInfo>;
    fn part_connection_ranges(&self) -> &Vec<u16>;
}

impl EdgeModelsBaseDataTrait for EdgeModelsBaseData {
    fn instance_transforms(&self) -> &Vec<super::core::LinearTransform> {
        &self.instance_transforms
    }
    fn mesh_instance_ranges(&self) -> &Vec<u16> {
        &self.mesh_instance_ranges
    }
    fn connection_instance_lookup_table(&self) -> &Vec<u16> {
        &self.connection_instance_lookup_table
    }
    fn connections(&self) -> &Vec<EdgeModelConnectionInfo> {
        &self.connections
    }
    fn part_connection_ranges(&self) -> &Vec<u16> {
        &self.part_connection_ranges
    }
}

impl super::core::DataContainerTrait for EdgeModelsBaseData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EDGEMODELSBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelsBaseData",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EdgeModelsBaseData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InstanceTransforms",
                flags: MemberInfoFlags::new(144),
                field_type: "LinearTransform-Array",
                rust_offset: offset_of!(EdgeModelsBaseData, instance_transforms),
            },
            FieldInfoData {
                name: "MeshInstanceRanges",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint16-Array",
                rust_offset: offset_of!(EdgeModelsBaseData, mesh_instance_ranges),
            },
            FieldInfoData {
                name: "ConnectionInstanceLookupTable",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint16-Array",
                rust_offset: offset_of!(EdgeModelsBaseData, connection_instance_lookup_table),
            },
            FieldInfoData {
                name: "Connections",
                flags: MemberInfoFlags::new(144),
                field_type: "EdgeModelConnectionInfo-Array",
                rust_offset: offset_of!(EdgeModelsBaseData, connections),
            },
            FieldInfoData {
                name: "PartConnectionRanges",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint16-Array",
                rust_offset: offset_of!(EdgeModelsBaseData, part_connection_ranges),
            },
        ],
    }),
    array_type: Some(EDGEMODELSBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EdgeModelsBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        EDGEMODELSBASEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EDGEMODELSBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelsBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EdgeModelsBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EdgeModelConnectionInfo {
    pub connection_instance_range: u16,
    pub neighbour_part_index: u16,
}

pub trait EdgeModelConnectionInfoTrait: TypeObject {
    fn connection_instance_range(&self) -> &u16;
    fn neighbour_part_index(&self) -> &u16;
}

impl EdgeModelConnectionInfoTrait for EdgeModelConnectionInfo {
    fn connection_instance_range(&self) -> &u16 {
        &self.connection_instance_range
    }
    fn neighbour_part_index(&self) -> &u16 {
        &self.neighbour_part_index
    }
}

pub static EDGEMODELCONNECTIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelConnectionInfo",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EdgeModelConnectionInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ConnectionInstanceRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(EdgeModelConnectionInfo, connection_instance_range),
            },
            FieldInfoData {
                name: "NeighbourPartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(EdgeModelConnectionInfo, neighbour_part_index),
            },
        ],
    }),
    array_type: Some(EDGEMODELCONNECTIONINFO_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for EdgeModelConnectionInfo {
    fn type_info(&self) -> &'static TypeInfo {
        EDGEMODELCONNECTIONINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EDGEMODELCONNECTIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelConnectionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EdgeModelConnectionInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionVolumeBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait DestructionVolumeBaseAssetTrait: super::core::AssetTrait {
}

impl DestructionVolumeBaseAssetTrait for DestructionVolumeBaseAsset {
}

impl super::core::AssetTrait for DestructionVolumeBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for DestructionVolumeBaseAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DESTRUCTIONVOLUMEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionVolumeBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionVolumeBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONVOLUMEBASEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONVOLUMEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DestructionVolumeBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionVolumeBaseData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait DestructionVolumeBaseDataTrait: super::core::DataContainerTrait {
}

impl DestructionVolumeBaseDataTrait for DestructionVolumeBaseData {
}

impl super::core::DataContainerTrait for DestructionVolumeBaseData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DESTRUCTIONVOLUMEBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeBaseData",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionVolumeBaseData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMEBASEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionVolumeBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONVOLUMEBASEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONVOLUMEBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DestructionVolumeBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshDynamicState {
    pub object_variation_name_hash: u32,
    pub asset_dependancies: Vec<Option<Arc<Mutex<dyn super::core::AssetTrait>>>>,
    pub shader_parameter_blocks: Vec<super::render_base::ShaderParameterBlockHandle>,
    pub is_streaming: bool,
    pub streaming_priority: f32,
    pub render_flags: u32,
    pub parent_render_flags_mask: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub light_map_weight: f32,
    pub destruction_volume: Option<Arc<Mutex<dyn DestructionVolumeBaseDataTrait>>>,
    pub edge_models: Option<Arc<Mutex<dyn EdgeModelsBaseDataTrait>>>,
    pub stencil_bits: u8,
    pub render_object_stencil_mask: u8,
    pub is_visible: bool,
    pub vertex_shader_fragment_override: super::render_base::VertexShaderFragmentHandle,
    pub field_flag_changed0: u16,
}

pub trait MeshDynamicStateTrait: TypeObject {
    fn object_variation_name_hash(&self) -> &u32;
    fn asset_dependancies(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::AssetTrait>>>>;
    fn shader_parameter_blocks(&self) -> &Vec<super::render_base::ShaderParameterBlockHandle>;
    fn is_streaming(&self) -> &bool;
    fn streaming_priority(&self) -> &f32;
    fn render_flags(&self) -> &u32;
    fn parent_render_flags_mask(&self) -> &u32;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn light_map_weight(&self) -> &f32;
    fn destruction_volume(&self) -> &Option<Arc<Mutex<dyn DestructionVolumeBaseDataTrait>>>;
    fn edge_models(&self) -> &Option<Arc<Mutex<dyn EdgeModelsBaseDataTrait>>>;
    fn stencil_bits(&self) -> &u8;
    fn render_object_stencil_mask(&self) -> &u8;
    fn is_visible(&self) -> &bool;
    fn vertex_shader_fragment_override(&self) -> &super::render_base::VertexShaderFragmentHandle;
    fn field_flag_changed0(&self) -> &u16;
}

impl MeshDynamicStateTrait for MeshDynamicState {
    fn object_variation_name_hash(&self) -> &u32 {
        &self.object_variation_name_hash
    }
    fn asset_dependancies(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::AssetTrait>>>> {
        &self.asset_dependancies
    }
    fn shader_parameter_blocks(&self) -> &Vec<super::render_base::ShaderParameterBlockHandle> {
        &self.shader_parameter_blocks
    }
    fn is_streaming(&self) -> &bool {
        &self.is_streaming
    }
    fn streaming_priority(&self) -> &f32 {
        &self.streaming_priority
    }
    fn render_flags(&self) -> &u32 {
        &self.render_flags
    }
    fn parent_render_flags_mask(&self) -> &u32 {
        &self.parent_render_flags_mask
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
    fn light_map_weight(&self) -> &f32 {
        &self.light_map_weight
    }
    fn destruction_volume(&self) -> &Option<Arc<Mutex<dyn DestructionVolumeBaseDataTrait>>> {
        &self.destruction_volume
    }
    fn edge_models(&self) -> &Option<Arc<Mutex<dyn EdgeModelsBaseDataTrait>>> {
        &self.edge_models
    }
    fn stencil_bits(&self) -> &u8 {
        &self.stencil_bits
    }
    fn render_object_stencil_mask(&self) -> &u8 {
        &self.render_object_stencil_mask
    }
    fn is_visible(&self) -> &bool {
        &self.is_visible
    }
    fn vertex_shader_fragment_override(&self) -> &super::render_base::VertexShaderFragmentHandle {
        &self.vertex_shader_fragment_override
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static MESHDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ObjectVariationNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MeshDynamicState, object_variation_name_hash),
            },
            FieldInfoData {
                name: "AssetDependancies",
                flags: MemberInfoFlags::new(144),
                field_type: "Asset-Array",
                rust_offset: offset_of!(MeshDynamicState, asset_dependancies),
            },
            FieldInfoData {
                name: "ShaderParameterBlocks",
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderParameterBlockHandle-Array",
                rust_offset: offset_of!(MeshDynamicState, shader_parameter_blocks),
            },
            FieldInfoData {
                name: "IsStreaming",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshDynamicState, is_streaming),
            },
            FieldInfoData {
                name: "StreamingPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshDynamicState, streaming_priority),
            },
            FieldInfoData {
                name: "RenderFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MeshDynamicState, render_flags),
            },
            FieldInfoData {
                name: "ParentRenderFlagsMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MeshDynamicState, parent_render_flags_mask),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(MeshDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(MeshDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "LightMapWeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshDynamicState, light_map_weight),
            },
            FieldInfoData {
                name: "DestructionVolume",
                flags: MemberInfoFlags::new(0),
                field_type: "DestructionVolumeBaseData",
                rust_offset: offset_of!(MeshDynamicState, destruction_volume),
            },
            FieldInfoData {
                name: "EdgeModels",
                flags: MemberInfoFlags::new(0),
                field_type: "EdgeModelsBaseData",
                rust_offset: offset_of!(MeshDynamicState, edge_models),
            },
            FieldInfoData {
                name: "StencilBits",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MeshDynamicState, stencil_bits),
            },
            FieldInfoData {
                name: "RenderObjectStencilMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MeshDynamicState, render_object_stencil_mask),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshDynamicState, is_visible),
            },
            FieldInfoData {
                name: "VertexShaderFragmentOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "VertexShaderFragmentHandle",
                rust_offset: offset_of!(MeshDynamicState, vertex_shader_fragment_override),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(MeshDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MESHDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        MESHDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("MeshDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshStaticState {
    pub mesh: Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>,
    pub mesh_deformer: MeshDeformerHandle,
    pub vertex_shader_fragment: super::render_base::VertexShaderFragmentHandle,
    pub procedural_animation_max_distance: f32,
    pub shadow_lod_offset: u8,
    pub is_terrain_shader_nodes_enable: bool,
    pub use_for_bounding_box_calculations: bool,
    pub use_parts_for_bounding_box_calculations: bool,
    pub decals_enable: bool,
    pub shader_draw_order: i32,
    pub shader_draw_order_sub_order: i32,
    pub field_flag_changed0: u16,
}

pub trait MeshStaticStateTrait: TypeObject {
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>;
    fn mesh_deformer(&self) -> &MeshDeformerHandle;
    fn vertex_shader_fragment(&self) -> &super::render_base::VertexShaderFragmentHandle;
    fn procedural_animation_max_distance(&self) -> &f32;
    fn shadow_lod_offset(&self) -> &u8;
    fn is_terrain_shader_nodes_enable(&self) -> &bool;
    fn use_for_bounding_box_calculations(&self) -> &bool;
    fn use_parts_for_bounding_box_calculations(&self) -> &bool;
    fn decals_enable(&self) -> &bool;
    fn shader_draw_order(&self) -> &i32;
    fn shader_draw_order_sub_order(&self) -> &i32;
    fn field_flag_changed0(&self) -> &u16;
}

impl MeshStaticStateTrait for MeshStaticState {
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>> {
        &self.mesh
    }
    fn mesh_deformer(&self) -> &MeshDeformerHandle {
        &self.mesh_deformer
    }
    fn vertex_shader_fragment(&self) -> &super::render_base::VertexShaderFragmentHandle {
        &self.vertex_shader_fragment
    }
    fn procedural_animation_max_distance(&self) -> &f32 {
        &self.procedural_animation_max_distance
    }
    fn shadow_lod_offset(&self) -> &u8 {
        &self.shadow_lod_offset
    }
    fn is_terrain_shader_nodes_enable(&self) -> &bool {
        &self.is_terrain_shader_nodes_enable
    }
    fn use_for_bounding_box_calculations(&self) -> &bool {
        &self.use_for_bounding_box_calculations
    }
    fn use_parts_for_bounding_box_calculations(&self) -> &bool {
        &self.use_parts_for_bounding_box_calculations
    }
    fn decals_enable(&self) -> &bool {
        &self.decals_enable
    }
    fn shader_draw_order(&self) -> &i32 {
        &self.shader_draw_order
    }
    fn shader_draw_order_sub_order(&self) -> &i32 {
        &self.shader_draw_order_sub_order
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static MESHSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(MeshStaticState, mesh),
            },
            FieldInfoData {
                name: "MeshDeformer",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshDeformerHandle",
                rust_offset: offset_of!(MeshStaticState, mesh_deformer),
            },
            FieldInfoData {
                name: "VertexShaderFragment",
                flags: MemberInfoFlags::new(0),
                field_type: "VertexShaderFragmentHandle",
                rust_offset: offset_of!(MeshStaticState, vertex_shader_fragment),
            },
            FieldInfoData {
                name: "ProceduralAnimationMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshStaticState, procedural_animation_max_distance),
            },
            FieldInfoData {
                name: "ShadowLodOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MeshStaticState, shadow_lod_offset),
            },
            FieldInfoData {
                name: "IsTerrainShaderNodesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshStaticState, is_terrain_shader_nodes_enable),
            },
            FieldInfoData {
                name: "UseForBoundingBoxCalculations",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshStaticState, use_for_bounding_box_calculations),
            },
            FieldInfoData {
                name: "UsePartsForBoundingBoxCalculations",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshStaticState, use_parts_for_bounding_box_calculations),
            },
            FieldInfoData {
                name: "DecalsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshStaticState, decals_enable),
            },
            FieldInfoData {
                name: "ShaderDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MeshStaticState, shader_draw_order),
            },
            FieldInfoData {
                name: "ShaderDrawOrderSubOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MeshStaticState, shader_draw_order_sub_order),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(MeshStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MESHSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        MESHSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("MeshStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ModelCreateInfo {
    pub transform: super::core::LinearTransform,
    pub bone_or_part_transforms: Vec<super::core::FbVec>,
    pub should_apply_world_transform: bool,
    pub should_apply_reference_translation: bool,
    pub reference_translation: super::core::Vec3,
    pub is_visible: bool,
    pub is_world_space: bool,
    pub parent_transform_space: super::state_stream::TransformSpaceHandle,
}

pub trait ModelCreateInfoTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn bone_or_part_transforms(&self) -> &Vec<super::core::FbVec>;
    fn should_apply_world_transform(&self) -> &bool;
    fn should_apply_reference_translation(&self) -> &bool;
    fn reference_translation(&self) -> &super::core::Vec3;
    fn is_visible(&self) -> &bool;
    fn is_world_space(&self) -> &bool;
    fn parent_transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
}

impl ModelCreateInfoTrait for ModelCreateInfo {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn bone_or_part_transforms(&self) -> &Vec<super::core::FbVec> {
        &self.bone_or_part_transforms
    }
    fn should_apply_world_transform(&self) -> &bool {
        &self.should_apply_world_transform
    }
    fn should_apply_reference_translation(&self) -> &bool {
        &self.should_apply_reference_translation
    }
    fn reference_translation(&self) -> &super::core::Vec3 {
        &self.reference_translation
    }
    fn is_visible(&self) -> &bool {
        &self.is_visible
    }
    fn is_world_space(&self) -> &bool {
        &self.is_world_space
    }
    fn parent_transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.parent_transform_space
    }
}

pub static MODELCREATEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelCreateInfo",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ModelCreateInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ModelCreateInfo, transform),
            },
            FieldInfoData {
                name: "BoneOrPartTransforms",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec-Array",
                rust_offset: offset_of!(ModelCreateInfo, bone_or_part_transforms),
            },
            FieldInfoData {
                name: "ShouldApplyWorldTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ModelCreateInfo, should_apply_world_transform),
            },
            FieldInfoData {
                name: "ShouldApplyReferenceTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ModelCreateInfo, should_apply_reference_translation),
            },
            FieldInfoData {
                name: "ReferenceTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ModelCreateInfo, reference_translation),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ModelCreateInfo, is_visible),
            },
            FieldInfoData {
                name: "IsWorldSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ModelCreateInfo, is_world_space),
            },
            FieldInfoData {
                name: "ParentTransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(ModelCreateInfo, parent_transform_space),
            },
        ],
    }),
    array_type: Some(MODELCREATEINFO_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ModelCreateInfo {
    fn type_info(&self) -> &'static TypeInfo {
        MODELCREATEINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MODELCREATEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelCreateInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ModelCreateInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ModelDynamicState {
    pub bone_or_part_visible_flags: Vec<u8>,
    pub children: Vec<ModelHandle>,
    pub shader_parameter_blocks: Vec<super::render_base::ShaderParameterBlockHandle>,
    pub fallback_branch: ModelHandle,
    pub object_variation_name_hash: u32,
    pub render_flags: u32,
    pub parent_render_flags_mask: u32,
    pub streaming_priority: f32,
    pub is_meshes_visible: bool,
    pub is_occlusion_test_enable: bool,
    pub was_visible_callback: WasVisibleHandle,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub is_first_person: bool,
    pub render_fov_deg: f32,
    pub custom_light_probe_type: super::core::LocalPlayerId,
    pub cast_shadow_override: super::core::BoolOverride,
    pub stencil_bits: u8,
    pub render_object_stencil_mask: u8,
    pub fallback_disable_min_required_lod_index: i8,
    pub light_probe_sample_offset: super::core::Vec3,
    pub field_flag_changed0: u32,
}

pub trait ModelDynamicStateTrait: TypeObject {
    fn bone_or_part_visible_flags(&self) -> &Vec<u8>;
    fn children(&self) -> &Vec<ModelHandle>;
    fn shader_parameter_blocks(&self) -> &Vec<super::render_base::ShaderParameterBlockHandle>;
    fn fallback_branch(&self) -> &ModelHandle;
    fn object_variation_name_hash(&self) -> &u32;
    fn render_flags(&self) -> &u32;
    fn parent_render_flags_mask(&self) -> &u32;
    fn streaming_priority(&self) -> &f32;
    fn is_meshes_visible(&self) -> &bool;
    fn is_occlusion_test_enable(&self) -> &bool;
    fn was_visible_callback(&self) -> &WasVisibleHandle;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn is_first_person(&self) -> &bool;
    fn render_fov_deg(&self) -> &f32;
    fn custom_light_probe_type(&self) -> &super::core::LocalPlayerId;
    fn cast_shadow_override(&self) -> &super::core::BoolOverride;
    fn stencil_bits(&self) -> &u8;
    fn render_object_stencil_mask(&self) -> &u8;
    fn fallback_disable_min_required_lod_index(&self) -> &i8;
    fn light_probe_sample_offset(&self) -> &super::core::Vec3;
    fn field_flag_changed0(&self) -> &u32;
}

impl ModelDynamicStateTrait for ModelDynamicState {
    fn bone_or_part_visible_flags(&self) -> &Vec<u8> {
        &self.bone_or_part_visible_flags
    }
    fn children(&self) -> &Vec<ModelHandle> {
        &self.children
    }
    fn shader_parameter_blocks(&self) -> &Vec<super::render_base::ShaderParameterBlockHandle> {
        &self.shader_parameter_blocks
    }
    fn fallback_branch(&self) -> &ModelHandle {
        &self.fallback_branch
    }
    fn object_variation_name_hash(&self) -> &u32 {
        &self.object_variation_name_hash
    }
    fn render_flags(&self) -> &u32 {
        &self.render_flags
    }
    fn parent_render_flags_mask(&self) -> &u32 {
        &self.parent_render_flags_mask
    }
    fn streaming_priority(&self) -> &f32 {
        &self.streaming_priority
    }
    fn is_meshes_visible(&self) -> &bool {
        &self.is_meshes_visible
    }
    fn is_occlusion_test_enable(&self) -> &bool {
        &self.is_occlusion_test_enable
    }
    fn was_visible_callback(&self) -> &WasVisibleHandle {
        &self.was_visible_callback
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn render_fov_deg(&self) -> &f32 {
        &self.render_fov_deg
    }
    fn custom_light_probe_type(&self) -> &super::core::LocalPlayerId {
        &self.custom_light_probe_type
    }
    fn cast_shadow_override(&self) -> &super::core::BoolOverride {
        &self.cast_shadow_override
    }
    fn stencil_bits(&self) -> &u8 {
        &self.stencil_bits
    }
    fn render_object_stencil_mask(&self) -> &u8 {
        &self.render_object_stencil_mask
    }
    fn fallback_disable_min_required_lod_index(&self) -> &i8 {
        &self.fallback_disable_min_required_lod_index
    }
    fn light_probe_sample_offset(&self) -> &super::core::Vec3 {
        &self.light_probe_sample_offset
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static MODELDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ModelDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BoneOrPartVisibleFlags",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint8-Array",
                rust_offset: offset_of!(ModelDynamicState, bone_or_part_visible_flags),
            },
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: "ModelHandle-Array",
                rust_offset: offset_of!(ModelDynamicState, children),
            },
            FieldInfoData {
                name: "ShaderParameterBlocks",
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderParameterBlockHandle-Array",
                rust_offset: offset_of!(ModelDynamicState, shader_parameter_blocks),
            },
            FieldInfoData {
                name: "FallbackBranch",
                flags: MemberInfoFlags::new(0),
                field_type: "ModelHandle",
                rust_offset: offset_of!(ModelDynamicState, fallback_branch),
            },
            FieldInfoData {
                name: "ObjectVariationNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ModelDynamicState, object_variation_name_hash),
            },
            FieldInfoData {
                name: "RenderFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ModelDynamicState, render_flags),
            },
            FieldInfoData {
                name: "ParentRenderFlagsMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ModelDynamicState, parent_render_flags_mask),
            },
            FieldInfoData {
                name: "StreamingPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ModelDynamicState, streaming_priority),
            },
            FieldInfoData {
                name: "IsMeshesVisible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ModelDynamicState, is_meshes_visible),
            },
            FieldInfoData {
                name: "IsOcclusionTestEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ModelDynamicState, is_occlusion_test_enable),
            },
            FieldInfoData {
                name: "WasVisibleCallback",
                flags: MemberInfoFlags::new(0),
                field_type: "WasVisibleHandle",
                rust_offset: offset_of!(ModelDynamicState, was_visible_callback),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(ModelDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(ModelDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ModelDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "RenderFovDeg",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ModelDynamicState, render_fov_deg),
            },
            FieldInfoData {
                name: "CustomLightProbeType",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(ModelDynamicState, custom_light_probe_type),
            },
            FieldInfoData {
                name: "CastShadowOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(ModelDynamicState, cast_shadow_override),
            },
            FieldInfoData {
                name: "StencilBits",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ModelDynamicState, stencil_bits),
            },
            FieldInfoData {
                name: "RenderObjectStencilMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ModelDynamicState, render_object_stencil_mask),
            },
            FieldInfoData {
                name: "FallbackDisableMinRequiredLodIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int8",
                rust_offset: offset_of!(ModelDynamicState, fallback_disable_min_required_lod_index),
            },
            FieldInfoData {
                name: "LightProbeSampleOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ModelDynamicState, light_probe_sample_offset),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ModelDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MODELDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ModelDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        MODELDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MODELDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ModelDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ModelStaticState {
    pub mesh_type: super::render_base::MeshType,
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub bone_or_part_count: u32,
    pub movable_parts: bool,
    pub radiosity_type_override: super::core::RadiosityTypeOverride,
    pub hierarchy_level: ModelHierarchyLevel,
    pub rendering_overrides: super::core::RenderingOverrides,
    pub use_top_of_bounding_box_for_light_probe: bool,
    pub include_bounding_box_when_not_visible: bool,
    pub lightmap_hash: u32,
    pub field_flag_changed0: u16,
}

pub trait ModelStaticStateTrait: TypeObject {
    fn mesh_type(&self) -> &super::render_base::MeshType;
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn bone_or_part_count(&self) -> &u32;
    fn movable_parts(&self) -> &bool;
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride;
    fn hierarchy_level(&self) -> &ModelHierarchyLevel;
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides;
    fn use_top_of_bounding_box_for_light_probe(&self) -> &bool;
    fn include_bounding_box_when_not_visible(&self) -> &bool;
    fn lightmap_hash(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u16;
}

impl ModelStaticStateTrait for ModelStaticState {
    fn mesh_type(&self) -> &super::render_base::MeshType {
        &self.mesh_type
    }
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn bone_or_part_count(&self) -> &u32 {
        &self.bone_or_part_count
    }
    fn movable_parts(&self) -> &bool {
        &self.movable_parts
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        &self.radiosity_type_override
    }
    fn hierarchy_level(&self) -> &ModelHierarchyLevel {
        &self.hierarchy_level
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        &self.rendering_overrides
    }
    fn use_top_of_bounding_box_for_light_probe(&self) -> &bool {
        &self.use_top_of_bounding_box_for_light_probe
    }
    fn include_bounding_box_when_not_visible(&self) -> &bool {
        &self.include_bounding_box_when_not_visible
    }
    fn lightmap_hash(&self) -> &u32 {
        &self.lightmap_hash
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static MODELSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ModelStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MeshType",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshType",
                rust_offset: offset_of!(ModelStaticState, mesh_type),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(ModelStaticState, transform_space),
            },
            FieldInfoData {
                name: "BoneOrPartCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ModelStaticState, bone_or_part_count),
            },
            FieldInfoData {
                name: "MovableParts",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ModelStaticState, movable_parts),
            },
            FieldInfoData {
                name: "RadiosityTypeOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "RadiosityTypeOverride",
                rust_offset: offset_of!(ModelStaticState, radiosity_type_override),
            },
            FieldInfoData {
                name: "HierarchyLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "ModelHierarchyLevel",
                rust_offset: offset_of!(ModelStaticState, hierarchy_level),
            },
            FieldInfoData {
                name: "RenderingOverrides",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderingOverrides",
                rust_offset: offset_of!(ModelStaticState, rendering_overrides),
            },
            FieldInfoData {
                name: "UseTopOfBoundingBoxForLightProbe",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ModelStaticState, use_top_of_bounding_box_for_light_probe),
            },
            FieldInfoData {
                name: "IncludeBoundingBoxWhenNotVisible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ModelStaticState, include_bounding_box_when_not_visible),
            },
            FieldInfoData {
                name: "LightmapHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ModelStaticState, lightmap_hash),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ModelStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MODELSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ModelStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        MODELSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MODELSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ModelStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ModelHierarchyLevel {
    #[default]
    ModelHierarchyLevel_Root = 0,
    ModelHierarchyLevel_RootWithFallback = 1,
    ModelHierarchyLevel_Child = 2,
}

pub static MODELHIERARCHYLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelHierarchyLevel",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(MODELHIERARCHYLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ModelHierarchyLevel {
    fn type_info(&self) -> &'static TypeInfo {
        MODELHIERARCHYLEVEL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MODELHIERARCHYLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelHierarchyLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ModelHierarchyLevel"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisibleAreaDynamicState {
    pub transform: super::core::LinearTransform,
    pub field_flag_changed0: u8,
}

pub trait VisibleAreaDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn field_flag_changed0(&self) -> &u8;
}

impl VisibleAreaDynamicStateTrait for VisibleAreaDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static VISIBLEAREADYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisibleAreaDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(VisibleAreaDynamicState, transform),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VisibleAreaDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISIBLEAREADYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VisibleAreaDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        VISIBLEAREADYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISIBLEAREADYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisibleAreaDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisibleAreaStaticState {
    pub visual_cull_screen_area: f32,
    pub was_visible_handle: WasVisibleHandle,
    pub field_flag_changed0: u8,
}

pub trait VisibleAreaStaticStateTrait: TypeObject {
    fn visual_cull_screen_area(&self) -> &f32;
    fn was_visible_handle(&self) -> &WasVisibleHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl VisibleAreaStaticStateTrait for VisibleAreaStaticState {
    fn visual_cull_screen_area(&self) -> &f32 {
        &self.visual_cull_screen_area
    }
    fn was_visible_handle(&self) -> &WasVisibleHandle {
        &self.was_visible_handle
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static VISIBLEAREASTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisibleAreaStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "VisualCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisibleAreaStaticState, visual_cull_screen_area),
            },
            FieldInfoData {
                name: "WasVisibleHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "WasVisibleHandle",
                rust_offset: offset_of!(VisibleAreaStaticState, was_visible_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(VisibleAreaStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISIBLEAREASTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VisibleAreaStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        VISIBLEAREASTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISIBLEAREASTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisibleAreaStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlanarReflectionLocatorDynamicState {
    pub transform: super::core::LinearTransform,
    pub enable: bool,
    pub field_flag_changed0: u8,
}

pub trait PlanarReflectionLocatorDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn enable(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl PlanarReflectionLocatorDynamicStateTrait for PlanarReflectionLocatorDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static PLANARREFLECTIONLOCATORDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlanarReflectionLocatorDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(PlanarReflectionLocatorDynamicState, transform),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlanarReflectionLocatorDynamicState, enable),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PlanarReflectionLocatorDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PLANARREFLECTIONLOCATORDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PlanarReflectionLocatorDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        PLANARREFLECTIONLOCATORDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLANARREFLECTIONLOCATORDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PlanarReflectionLocatorDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlanarReflectionLocatorStaticState {
}

pub trait PlanarReflectionLocatorStaticStateTrait: TypeObject {
}

impl PlanarReflectionLocatorStaticStateTrait for PlanarReflectionLocatorStaticState {
}

pub static PLANARREFLECTIONLOCATORSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlanarReflectionLocatorStaticState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PLANARREFLECTIONLOCATORSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PlanarReflectionLocatorStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        PLANARREFLECTIONLOCATORSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLANARREFLECTIONLOCATORSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PlanarReflectionLocatorStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightProbeVolumeDynamicState {
    pub blend_distance: f32,
    pub priority: u32,
    pub enable: bool,
    pub field_flag_changed0: u8,
}

pub trait LightProbeVolumeDynamicStateTrait: TypeObject {
    fn blend_distance(&self) -> &f32;
    fn priority(&self) -> &u32;
    fn enable(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl LightProbeVolumeDynamicStateTrait for LightProbeVolumeDynamicState {
    fn blend_distance(&self) -> &f32 {
        &self.blend_distance
    }
    fn priority(&self) -> &u32 {
        &self.priority
    }
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static LIGHTPROBEVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeVolumeDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightProbeVolumeDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BlendDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightProbeVolumeDynamicState, blend_distance),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LightProbeVolumeDynamicState, priority),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LightProbeVolumeDynamicState, enable),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(LightProbeVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LIGHTPROBEVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LightProbeVolumeDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTPROBEVOLUMEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTPROBEVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LightProbeVolumeDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightProbeVolumeStaticState {
    pub guid: glacier_util::guid::Guid,
    pub field_flag_changed0: u8,
}

pub trait LightProbeVolumeStaticStateTrait: TypeObject {
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn field_flag_changed0(&self) -> &u8;
}

impl LightProbeVolumeStaticStateTrait for LightProbeVolumeStaticState {
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static LIGHTPROBEVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeVolumeStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightProbeVolumeStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(LightProbeVolumeStaticState, guid),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(LightProbeVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LIGHTPROBEVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LightProbeVolumeStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTPROBEVOLUMESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTPROBEVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LightProbeVolumeStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DynamicEnlightenDynamicState {
    pub enable: bool,
    pub database_version: i32,
    pub field_flag_changed0: u8,
}

pub trait DynamicEnlightenDynamicStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn database_version(&self) -> &i32;
    fn field_flag_changed0(&self) -> &u8;
}

impl DynamicEnlightenDynamicStateTrait for DynamicEnlightenDynamicState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn database_version(&self) -> &i32 {
        &self.database_version
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static DYNAMICENLIGHTENDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DynamicEnlightenDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DynamicEnlightenDynamicState, enable),
            },
            FieldInfoData {
                name: "DatabaseVersion",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DynamicEnlightenDynamicState, database_version),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DynamicEnlightenDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DYNAMICENLIGHTENDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DynamicEnlightenDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        DYNAMICENLIGHTENDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DYNAMICENLIGHTENDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DynamicEnlightenDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DynamicEnlightenStaticState {
    pub priority: i32,
    pub enlighten_data: Option<Arc<Mutex<dyn super::render_base::EnlightenBaseAssetTrait>>>,
    pub object_layers: u16,
    pub field_flag_changed0: u8,
}

pub trait DynamicEnlightenStaticStateTrait: TypeObject {
    fn priority(&self) -> &i32;
    fn enlighten_data(&self) -> &Option<Arc<Mutex<dyn super::render_base::EnlightenBaseAssetTrait>>>;
    fn object_layers(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u8;
}

impl DynamicEnlightenStaticStateTrait for DynamicEnlightenStaticState {
    fn priority(&self) -> &i32 {
        &self.priority
    }
    fn enlighten_data(&self) -> &Option<Arc<Mutex<dyn super::render_base::EnlightenBaseAssetTrait>>> {
        &self.enlighten_data
    }
    fn object_layers(&self) -> &u16 {
        &self.object_layers
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static DYNAMICENLIGHTENSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DynamicEnlightenStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DynamicEnlightenStaticState, priority),
            },
            FieldInfoData {
                name: "EnlightenData",
                flags: MemberInfoFlags::new(0),
                field_type: "EnlightenBaseAsset",
                rust_offset: offset_of!(DynamicEnlightenStaticState, enlighten_data),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(DynamicEnlightenStaticState, object_layers),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DynamicEnlightenStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DYNAMICENLIGHTENSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicEnlightenStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        DYNAMICENLIGHTENSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DYNAMICENLIGHTENSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DynamicEnlightenStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StaticEnlightenDynamicState {
    pub enable: bool,
    pub mixed: bool,
    pub field_flag_changed0: u8,
}

pub trait StaticEnlightenDynamicStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn mixed(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl StaticEnlightenDynamicStateTrait for StaticEnlightenDynamicState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn mixed(&self) -> &bool {
        &self.mixed
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static STATICENLIGHTENDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticEnlightenDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StaticEnlightenDynamicState, enable),
            },
            FieldInfoData {
                name: "Mixed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StaticEnlightenDynamicState, mixed),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(StaticEnlightenDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(STATICENLIGHTENDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StaticEnlightenDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        STATICENLIGHTENDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STATICENLIGHTENDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("StaticEnlightenDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StaticEnlightenStaticState {
    pub priority: i32,
    pub enlighten_data: Option<Arc<Mutex<dyn super::render_base::StaticEnlightenBaseAssetTrait>>>,
    pub dynamic_enlighten_data: Option<Arc<Mutex<dyn super::render_base::EnlightenBaseAssetTrait>>>,
    pub object_layers: u16,
    pub flux_auto_bake: bool,
    pub field_flag_changed0: u8,
}

pub trait StaticEnlightenStaticStateTrait: TypeObject {
    fn priority(&self) -> &i32;
    fn enlighten_data(&self) -> &Option<Arc<Mutex<dyn super::render_base::StaticEnlightenBaseAssetTrait>>>;
    fn dynamic_enlighten_data(&self) -> &Option<Arc<Mutex<dyn super::render_base::EnlightenBaseAssetTrait>>>;
    fn object_layers(&self) -> &u16;
    fn flux_auto_bake(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl StaticEnlightenStaticStateTrait for StaticEnlightenStaticState {
    fn priority(&self) -> &i32 {
        &self.priority
    }
    fn enlighten_data(&self) -> &Option<Arc<Mutex<dyn super::render_base::StaticEnlightenBaseAssetTrait>>> {
        &self.enlighten_data
    }
    fn dynamic_enlighten_data(&self) -> &Option<Arc<Mutex<dyn super::render_base::EnlightenBaseAssetTrait>>> {
        &self.dynamic_enlighten_data
    }
    fn object_layers(&self) -> &u16 {
        &self.object_layers
    }
    fn flux_auto_bake(&self) -> &bool {
        &self.flux_auto_bake
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static STATICENLIGHTENSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticEnlightenStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(StaticEnlightenStaticState, priority),
            },
            FieldInfoData {
                name: "EnlightenData",
                flags: MemberInfoFlags::new(0),
                field_type: "StaticEnlightenBaseAsset",
                rust_offset: offset_of!(StaticEnlightenStaticState, enlighten_data),
            },
            FieldInfoData {
                name: "DynamicEnlightenData",
                flags: MemberInfoFlags::new(0),
                field_type: "EnlightenBaseAsset",
                rust_offset: offset_of!(StaticEnlightenStaticState, dynamic_enlighten_data),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(StaticEnlightenStaticState, object_layers),
            },
            FieldInfoData {
                name: "FluxAutoBake",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StaticEnlightenStaticState, flux_auto_bake),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(StaticEnlightenStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(STATICENLIGHTENSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticEnlightenStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        STATICENLIGHTENSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STATICENLIGHTENSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("StaticEnlightenStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RadiosityModifierDynamicState {
    pub position: super::core::Vec3,
    pub radius: f32,
    pub bounce_scale: f32,
    pub sun_scale: f32,
    pub field_flag_changed0: u8,
}

pub trait RadiosityModifierDynamicStateTrait: TypeObject {
    fn position(&self) -> &super::core::Vec3;
    fn radius(&self) -> &f32;
    fn bounce_scale(&self) -> &f32;
    fn sun_scale(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u8;
}

impl RadiosityModifierDynamicStateTrait for RadiosityModifierDynamicState {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn bounce_scale(&self) -> &f32 {
        &self.bounce_scale
    }
    fn sun_scale(&self) -> &f32 {
        &self.sun_scale
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static RADIOSITYMODIFIERDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RadiosityModifierDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(RadiosityModifierDynamicState, position),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiosityModifierDynamicState, radius),
            },
            FieldInfoData {
                name: "BounceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiosityModifierDynamicState, bounce_scale),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiosityModifierDynamicState, sun_scale),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RadiosityModifierDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMODIFIERDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityModifierDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        RADIOSITYMODIFIERDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RADIOSITYMODIFIERDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityModifierDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RadiosityModifierStaticState {
}

pub trait RadiosityModifierStaticStateTrait: TypeObject {
}

impl RadiosityModifierStaticStateTrait for RadiosityModifierStaticState {
}

pub static RADIOSITYMODIFIERSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RadiosityModifierStaticState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RADIOSITYMODIFIERSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RadiosityModifierStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        RADIOSITYMODIFIERSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RADIOSITYMODIFIERSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityModifierStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RadiosityMaterialTriggerDynamicState {
    pub opacity: f32,
    pub color: super::core::Vec3,
    pub field_flag_changed0: u8,
}

pub trait RadiosityMaterialTriggerDynamicStateTrait: TypeObject {
    fn opacity(&self) -> &f32;
    fn color(&self) -> &super::core::Vec3;
    fn field_flag_changed0(&self) -> &u8;
}

impl RadiosityMaterialTriggerDynamicStateTrait for RadiosityMaterialTriggerDynamicState {
    fn opacity(&self) -> &f32 {
        &self.opacity
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static RADIOSITYMATERIALTRIGGERDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialTriggerDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RadiosityMaterialTriggerDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiosityMaterialTriggerDynamicState, opacity),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(RadiosityMaterialTriggerDynamicState, color),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RadiosityMaterialTriggerDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALTRIGGERDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityMaterialTriggerDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        RADIOSITYMATERIALTRIGGERDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RADIOSITYMATERIALTRIGGERDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialTriggerDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityMaterialTriggerDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RadiosityMaterialTriggerStaticState {
    pub material_guid: glacier_util::guid::Guid,
    pub field_flag_changed0: u8,
}

pub trait RadiosityMaterialTriggerStaticStateTrait: TypeObject {
    fn material_guid(&self) -> &glacier_util::guid::Guid;
    fn field_flag_changed0(&self) -> &u8;
}

impl RadiosityMaterialTriggerStaticStateTrait for RadiosityMaterialTriggerStaticState {
    fn material_guid(&self) -> &glacier_util::guid::Guid {
        &self.material_guid
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static RADIOSITYMATERIALTRIGGERSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialTriggerStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RadiosityMaterialTriggerStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaterialGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(RadiosityMaterialTriggerStaticState, material_guid),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RadiosityMaterialTriggerStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALTRIGGERSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RadiosityMaterialTriggerStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        RADIOSITYMATERIALTRIGGERSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RADIOSITYMATERIALTRIGGERSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialTriggerStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityMaterialTriggerStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RadiosityMaterialDynamicState {
    pub color: super::core::Vec3,
    pub emissive_intensity: f32,
    pub opacity: f32,
    pub backface_type: super::render_base::RadiosityBackfaceType,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait RadiosityMaterialDynamicStateTrait: TypeObject {
    fn color(&self) -> &super::core::Vec3;
    fn emissive_intensity(&self) -> &f32;
    fn opacity(&self) -> &f32;
    fn backface_type(&self) -> &super::render_base::RadiosityBackfaceType;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl RadiosityMaterialDynamicStateTrait for RadiosityMaterialDynamicState {
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn emissive_intensity(&self) -> &f32 {
        &self.emissive_intensity
    }
    fn opacity(&self) -> &f32 {
        &self.opacity
    }
    fn backface_type(&self) -> &super::render_base::RadiosityBackfaceType {
        &self.backface_type
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static RADIOSITYMATERIALDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RadiosityMaterialDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(RadiosityMaterialDynamicState, color),
            },
            FieldInfoData {
                name: "EmissiveIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiosityMaterialDynamicState, emissive_intensity),
            },
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiosityMaterialDynamicState, opacity),
            },
            FieldInfoData {
                name: "BackfaceType",
                flags: MemberInfoFlags::new(0),
                field_type: "RadiosityBackfaceType",
                rust_offset: offset_of!(RadiosityMaterialDynamicState, backface_type),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RadiosityMaterialDynamicState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RadiosityMaterialDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityMaterialDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        RADIOSITYMATERIALDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RADIOSITYMATERIALDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityMaterialDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RadiosityMaterialStaticState {
    pub material_guid: glacier_util::guid::Guid,
    pub material_update_mask: u8,
    pub field_flag_changed0: u8,
}

pub trait RadiosityMaterialStaticStateTrait: TypeObject {
    fn material_guid(&self) -> &glacier_util::guid::Guid;
    fn material_update_mask(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl RadiosityMaterialStaticStateTrait for RadiosityMaterialStaticState {
    fn material_guid(&self) -> &glacier_util::guid::Guid {
        &self.material_guid
    }
    fn material_update_mask(&self) -> &u8 {
        &self.material_update_mask
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static RADIOSITYMATERIALSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RadiosityMaterialStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaterialGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(RadiosityMaterialStaticState, material_guid),
            },
            FieldInfoData {
                name: "MaterialUpdateMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RadiosityMaterialStaticState, material_update_mask),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RadiosityMaterialStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RadiosityMaterialStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        RADIOSITYMATERIALSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RADIOSITYMATERIALSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityMaterialStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroundHeightDynamicState {
}

pub trait GroundHeightDynamicStateTrait: TypeObject {
}

impl GroundHeightDynamicStateTrait for GroundHeightDynamicState {
}

pub static GROUNDHEIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroundHeightDynamicState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GROUNDHEIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GroundHeightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        GROUNDHEIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GROUNDHEIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("GroundHeightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroundHeightStaticState {
    pub ground_origo: super::core::Vec3,
    pub data: GroundHeightData,
    pub field_flag_changed0: u8,
}

pub trait GroundHeightStaticStateTrait: TypeObject {
    fn ground_origo(&self) -> &super::core::Vec3;
    fn data(&self) -> &GroundHeightData;
    fn field_flag_changed0(&self) -> &u8;
}

impl GroundHeightStaticStateTrait for GroundHeightStaticState {
    fn ground_origo(&self) -> &super::core::Vec3 {
        &self.ground_origo
    }
    fn data(&self) -> &GroundHeightData {
        &self.data
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static GROUNDHEIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroundHeightStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GroundOrigo",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(GroundHeightStaticState, ground_origo),
            },
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "GroundHeightData",
                rust_offset: offset_of!(GroundHeightStaticState, data),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(GroundHeightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(GROUNDHEIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GroundHeightStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        GROUNDHEIGHTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GROUNDHEIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("GroundHeightStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroundHeightData {
    pub world_size: f32,
    pub height_span: super::core::Vec2,
    pub data: Vec<u16>,
}

pub trait GroundHeightDataTrait: TypeObject {
    fn world_size(&self) -> &f32;
    fn height_span(&self) -> &super::core::Vec2;
    fn data(&self) -> &Vec<u16>;
}

impl GroundHeightDataTrait for GroundHeightData {
    fn world_size(&self) -> &f32 {
        &self.world_size
    }
    fn height_span(&self) -> &super::core::Vec2 {
        &self.height_span
    }
    fn data(&self) -> &Vec<u16> {
        &self.data
    }
}

pub static GROUNDHEIGHTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightData",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroundHeightData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "WorldSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GroundHeightData, world_size),
            },
            FieldInfoData {
                name: "HeightSpan",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(GroundHeightData, height_span),
            },
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint16-Array",
                rust_offset: offset_of!(GroundHeightData, data),
            },
        ],
    }),
    array_type: Some(GROUNDHEIGHTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GroundHeightData {
    fn type_info(&self) -> &'static TypeInfo {
        GROUNDHEIGHTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GROUNDHEIGHTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("GroundHeightData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RenderVolumeDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub field_flag_changed0: u8,
}

pub trait RenderVolumeDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn enabled(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl RenderVolumeDynamicStateTrait for RenderVolumeDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static RENDERVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderVolumeDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(RenderVolumeDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RenderVolumeDynamicState, enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RenderVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RENDERVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RenderVolumeDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERVOLUMEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RENDERVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RenderVolumeDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RenderVolumeStaticState {
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub user_masks: super::core::Vec4,
    pub transform_type: RenderVolumeTransformType,
    pub field_flag_changed0: u8,
}

pub trait RenderVolumeStaticStateTrait: TypeObject {
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn user_masks(&self) -> &super::core::Vec4;
    fn transform_type(&self) -> &RenderVolumeTransformType;
    fn field_flag_changed0(&self) -> &u8;
}

impl RenderVolumeStaticStateTrait for RenderVolumeStaticState {
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn user_masks(&self) -> &super::core::Vec4 {
        &self.user_masks
    }
    fn transform_type(&self) -> &RenderVolumeTransformType {
        &self.transform_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static RENDERVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderVolumeStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(RenderVolumeStaticState, shader),
            },
            FieldInfoData {
                name: "UserMasks",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(RenderVolumeStaticState, user_masks),
            },
            FieldInfoData {
                name: "TransformType",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderVolumeTransformType",
                rust_offset: offset_of!(RenderVolumeStaticState, transform_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(RenderVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RENDERVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RenderVolumeStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERVOLUMESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RENDERVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RenderVolumeStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RenderVolumeTransformType {
    #[default]
    RenderVolumeTransformType_WorldSpaceInv = 0,
    RenderVolumeTransformType_WorldSpaceNoScale = 1,
}

pub static RENDERVOLUMETRANSFORMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeTransformType",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERVOLUMETRANSFORMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderVolumeTransformType {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERVOLUMETRANSFORMTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RENDERVOLUMETRANSFORMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeTransformType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RenderVolumeTransformType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PbrRectangularLightDynamicState {
    pub shape: super::render_base::RectangularLightShape,
    pub outer_angle: f32,
    pub aspect: f32,
    pub width: f32,
    pub height: f32,
    pub texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub shadow_max_angle: f32,
    pub shadow_fade_out_range: f32,
    pub color: super::core::Vec3,
    pub intensity: f32,
    pub exposure_compensation: f32,
    pub attenuation_radius: f32,
    pub emissive_shape_enable: bool,
    pub attenuation_offset: f32,
    pub light_unit: super::render_base::LightUnitType,
    pub affect_diffuse: bool,
    pub affect_specular: bool,
    pub particle_color_scale: super::core::Vec3,
    pub cast_shadows: super::core::QualityScalableEnabled,
    pub cast_volumetric: super::core::QualityScalableEnabled,
    pub cast_volumetric_shadows: super::core::QualityScalableEnabled,
    pub shadow_resolution: super::core::QualityLevel,
    pub shadow_near_radius: f32,
    pub shadow_far_radius: f32,
    pub shadow_dimmer: f32,
    pub cast_shadows_enable: bool,
    pub cast_volumetric_shadows_enable: bool,
    pub affect_radiosity: bool,
    pub radiosity_color_scale: super::core::Vec3,
    pub dimmer: f32,
    pub cull_screen_area: f32,
    pub fade_screen_area: f32,
    pub cull_distance: f32,
    pub fade_distance: f32,
    pub shadow_cull_screen_area: f32,
    pub shadow_fade_screen_area: f32,
    pub shadow_cull_distance: f32,
    pub shadow_fade_distance: f32,
    pub direct_lightmap_enable: bool,
    pub is_first_person: bool,
    pub is_enabled: bool,
    pub intensity_multiplier: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u16,
}

pub trait PbrRectangularLightDynamicStateTrait: TypeObject {
    fn shape(&self) -> &super::render_base::RectangularLightShape;
    fn outer_angle(&self) -> &f32;
    fn aspect(&self) -> &f32;
    fn width(&self) -> &f32;
    fn height(&self) -> &f32;
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn shadow_max_angle(&self) -> &f32;
    fn shadow_fade_out_range(&self) -> &f32;
    fn color(&self) -> &super::core::Vec3;
    fn intensity(&self) -> &f32;
    fn exposure_compensation(&self) -> &f32;
    fn attenuation_radius(&self) -> &f32;
    fn emissive_shape_enable(&self) -> &bool;
    fn attenuation_offset(&self) -> &f32;
    fn light_unit(&self) -> &super::render_base::LightUnitType;
    fn affect_diffuse(&self) -> &bool;
    fn affect_specular(&self) -> &bool;
    fn particle_color_scale(&self) -> &super::core::Vec3;
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn shadow_resolution(&self) -> &super::core::QualityLevel;
    fn shadow_near_radius(&self) -> &f32;
    fn shadow_far_radius(&self) -> &f32;
    fn shadow_dimmer(&self) -> &f32;
    fn cast_shadows_enable(&self) -> &bool;
    fn cast_volumetric_shadows_enable(&self) -> &bool;
    fn affect_radiosity(&self) -> &bool;
    fn radiosity_color_scale(&self) -> &super::core::Vec3;
    fn dimmer(&self) -> &f32;
    fn cull_screen_area(&self) -> &f32;
    fn fade_screen_area(&self) -> &f32;
    fn cull_distance(&self) -> &f32;
    fn fade_distance(&self) -> &f32;
    fn shadow_cull_screen_area(&self) -> &f32;
    fn shadow_fade_screen_area(&self) -> &f32;
    fn shadow_cull_distance(&self) -> &f32;
    fn shadow_fade_distance(&self) -> &f32;
    fn direct_lightmap_enable(&self) -> &bool;
    fn is_first_person(&self) -> &bool;
    fn is_enabled(&self) -> &bool;
    fn intensity_multiplier(&self) -> &f32;
    fn shadow_cache_enable(&self) -> &bool;
    fn shadow_cache_update_priority(&self) -> &f32;
    fn shadow_cache_update_counter(&self) -> &u32;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u16;
}

impl PbrRectangularLightDynamicStateTrait for PbrRectangularLightDynamicState {
    fn shape(&self) -> &super::render_base::RectangularLightShape {
        &self.shape
    }
    fn outer_angle(&self) -> &f32 {
        &self.outer_angle
    }
    fn aspect(&self) -> &f32 {
        &self.aspect
    }
    fn width(&self) -> &f32 {
        &self.width
    }
    fn height(&self) -> &f32 {
        &self.height
    }
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture
    }
    fn shadow_max_angle(&self) -> &f32 {
        &self.shadow_max_angle
    }
    fn shadow_fade_out_range(&self) -> &f32 {
        &self.shadow_fade_out_range
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn exposure_compensation(&self) -> &f32 {
        &self.exposure_compensation
    }
    fn attenuation_radius(&self) -> &f32 {
        &self.attenuation_radius
    }
    fn emissive_shape_enable(&self) -> &bool {
        &self.emissive_shape_enable
    }
    fn attenuation_offset(&self) -> &f32 {
        &self.attenuation_offset
    }
    fn light_unit(&self) -> &super::render_base::LightUnitType {
        &self.light_unit
    }
    fn affect_diffuse(&self) -> &bool {
        &self.affect_diffuse
    }
    fn affect_specular(&self) -> &bool {
        &self.affect_specular
    }
    fn particle_color_scale(&self) -> &super::core::Vec3 {
        &self.particle_color_scale
    }
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_shadows
    }
    fn cast_volumetric(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric
    }
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric_shadows
    }
    fn shadow_resolution(&self) -> &super::core::QualityLevel {
        &self.shadow_resolution
    }
    fn shadow_near_radius(&self) -> &f32 {
        &self.shadow_near_radius
    }
    fn shadow_far_radius(&self) -> &f32 {
        &self.shadow_far_radius
    }
    fn shadow_dimmer(&self) -> &f32 {
        &self.shadow_dimmer
    }
    fn cast_shadows_enable(&self) -> &bool {
        &self.cast_shadows_enable
    }
    fn cast_volumetric_shadows_enable(&self) -> &bool {
        &self.cast_volumetric_shadows_enable
    }
    fn affect_radiosity(&self) -> &bool {
        &self.affect_radiosity
    }
    fn radiosity_color_scale(&self) -> &super::core::Vec3 {
        &self.radiosity_color_scale
    }
    fn dimmer(&self) -> &f32 {
        &self.dimmer
    }
    fn cull_screen_area(&self) -> &f32 {
        &self.cull_screen_area
    }
    fn fade_screen_area(&self) -> &f32 {
        &self.fade_screen_area
    }
    fn cull_distance(&self) -> &f32 {
        &self.cull_distance
    }
    fn fade_distance(&self) -> &f32 {
        &self.fade_distance
    }
    fn shadow_cull_screen_area(&self) -> &f32 {
        &self.shadow_cull_screen_area
    }
    fn shadow_fade_screen_area(&self) -> &f32 {
        &self.shadow_fade_screen_area
    }
    fn shadow_cull_distance(&self) -> &f32 {
        &self.shadow_cull_distance
    }
    fn shadow_fade_distance(&self) -> &f32 {
        &self.shadow_fade_distance
    }
    fn direct_lightmap_enable(&self) -> &bool {
        &self.direct_lightmap_enable
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn intensity_multiplier(&self) -> &f32 {
        &self.intensity_multiplier
    }
    fn shadow_cache_enable(&self) -> &bool {
        &self.shadow_cache_enable
    }
    fn shadow_cache_update_priority(&self) -> &f32 {
        &self.shadow_cache_update_priority
    }
    fn shadow_cache_update_counter(&self) -> &u32 {
        &self.shadow_cache_update_counter
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u16 {
        &self.field_flag_changed1
    }
}

pub static PBRRECTANGULARLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PbrRectangularLightDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shape",
                flags: MemberInfoFlags::new(0),
                field_type: "RectangularLightShape",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shape),
            },
            FieldInfoData {
                name: "OuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, outer_angle),
            },
            FieldInfoData {
                name: "Aspect",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, aspect),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, height),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, texture),
            },
            FieldInfoData {
                name: "ShadowMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_max_angle),
            },
            FieldInfoData {
                name: "ShadowFadeOutRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_fade_out_range),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: "LightUnitType",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(PbrRectangularLightDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(PBRRECTANGULARLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrRectangularLightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        PBRRECTANGULARLIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PBRRECTANGULARLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrRectangularLightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PbrRectangularLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub trait PbrRectangularLightStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl PbrRectangularLightStaticStateTrait for PbrRectangularLightStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static PBRRECTANGULARLIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PbrRectangularLightStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(PbrRectangularLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PbrRectangularLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PBRRECTANGULARLIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PbrRectangularLightStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        PBRRECTANGULARLIGHTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PBRRECTANGULARLIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrRectangularLightStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PbrTubeLightDynamicState {
    pub tube_radius: f32,
    pub tube_width: f32,
    pub only_hempishere: bool,
    pub is_capsule: bool,
    pub color: super::core::Vec3,
    pub intensity: f32,
    pub exposure_compensation: f32,
    pub attenuation_radius: f32,
    pub emissive_shape_enable: bool,
    pub attenuation_offset: f32,
    pub light_unit: super::render_base::LightUnitType,
    pub affect_diffuse: bool,
    pub affect_specular: bool,
    pub particle_color_scale: super::core::Vec3,
    pub cast_shadows: super::core::QualityScalableEnabled,
    pub cast_volumetric: super::core::QualityScalableEnabled,
    pub cast_volumetric_shadows: super::core::QualityScalableEnabled,
    pub shadow_resolution: super::core::QualityLevel,
    pub shadow_near_radius: f32,
    pub shadow_far_radius: f32,
    pub shadow_dimmer: f32,
    pub cast_shadows_enable: bool,
    pub cast_volumetric_shadows_enable: bool,
    pub affect_radiosity: bool,
    pub radiosity_color_scale: super::core::Vec3,
    pub dimmer: f32,
    pub cull_screen_area: f32,
    pub fade_screen_area: f32,
    pub cull_distance: f32,
    pub fade_distance: f32,
    pub shadow_cull_screen_area: f32,
    pub shadow_fade_screen_area: f32,
    pub shadow_cull_distance: f32,
    pub shadow_fade_distance: f32,
    pub direct_lightmap_enable: bool,
    pub is_first_person: bool,
    pub is_enabled: bool,
    pub intensity_multiplier: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u16,
}

pub trait PbrTubeLightDynamicStateTrait: TypeObject {
    fn tube_radius(&self) -> &f32;
    fn tube_width(&self) -> &f32;
    fn only_hempishere(&self) -> &bool;
    fn is_capsule(&self) -> &bool;
    fn color(&self) -> &super::core::Vec3;
    fn intensity(&self) -> &f32;
    fn exposure_compensation(&self) -> &f32;
    fn attenuation_radius(&self) -> &f32;
    fn emissive_shape_enable(&self) -> &bool;
    fn attenuation_offset(&self) -> &f32;
    fn light_unit(&self) -> &super::render_base::LightUnitType;
    fn affect_diffuse(&self) -> &bool;
    fn affect_specular(&self) -> &bool;
    fn particle_color_scale(&self) -> &super::core::Vec3;
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn shadow_resolution(&self) -> &super::core::QualityLevel;
    fn shadow_near_radius(&self) -> &f32;
    fn shadow_far_radius(&self) -> &f32;
    fn shadow_dimmer(&self) -> &f32;
    fn cast_shadows_enable(&self) -> &bool;
    fn cast_volumetric_shadows_enable(&self) -> &bool;
    fn affect_radiosity(&self) -> &bool;
    fn radiosity_color_scale(&self) -> &super::core::Vec3;
    fn dimmer(&self) -> &f32;
    fn cull_screen_area(&self) -> &f32;
    fn fade_screen_area(&self) -> &f32;
    fn cull_distance(&self) -> &f32;
    fn fade_distance(&self) -> &f32;
    fn shadow_cull_screen_area(&self) -> &f32;
    fn shadow_fade_screen_area(&self) -> &f32;
    fn shadow_cull_distance(&self) -> &f32;
    fn shadow_fade_distance(&self) -> &f32;
    fn direct_lightmap_enable(&self) -> &bool;
    fn is_first_person(&self) -> &bool;
    fn is_enabled(&self) -> &bool;
    fn intensity_multiplier(&self) -> &f32;
    fn shadow_cache_enable(&self) -> &bool;
    fn shadow_cache_update_priority(&self) -> &f32;
    fn shadow_cache_update_counter(&self) -> &u32;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u16;
}

impl PbrTubeLightDynamicStateTrait for PbrTubeLightDynamicState {
    fn tube_radius(&self) -> &f32 {
        &self.tube_radius
    }
    fn tube_width(&self) -> &f32 {
        &self.tube_width
    }
    fn only_hempishere(&self) -> &bool {
        &self.only_hempishere
    }
    fn is_capsule(&self) -> &bool {
        &self.is_capsule
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn exposure_compensation(&self) -> &f32 {
        &self.exposure_compensation
    }
    fn attenuation_radius(&self) -> &f32 {
        &self.attenuation_radius
    }
    fn emissive_shape_enable(&self) -> &bool {
        &self.emissive_shape_enable
    }
    fn attenuation_offset(&self) -> &f32 {
        &self.attenuation_offset
    }
    fn light_unit(&self) -> &super::render_base::LightUnitType {
        &self.light_unit
    }
    fn affect_diffuse(&self) -> &bool {
        &self.affect_diffuse
    }
    fn affect_specular(&self) -> &bool {
        &self.affect_specular
    }
    fn particle_color_scale(&self) -> &super::core::Vec3 {
        &self.particle_color_scale
    }
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_shadows
    }
    fn cast_volumetric(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric
    }
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric_shadows
    }
    fn shadow_resolution(&self) -> &super::core::QualityLevel {
        &self.shadow_resolution
    }
    fn shadow_near_radius(&self) -> &f32 {
        &self.shadow_near_radius
    }
    fn shadow_far_radius(&self) -> &f32 {
        &self.shadow_far_radius
    }
    fn shadow_dimmer(&self) -> &f32 {
        &self.shadow_dimmer
    }
    fn cast_shadows_enable(&self) -> &bool {
        &self.cast_shadows_enable
    }
    fn cast_volumetric_shadows_enable(&self) -> &bool {
        &self.cast_volumetric_shadows_enable
    }
    fn affect_radiosity(&self) -> &bool {
        &self.affect_radiosity
    }
    fn radiosity_color_scale(&self) -> &super::core::Vec3 {
        &self.radiosity_color_scale
    }
    fn dimmer(&self) -> &f32 {
        &self.dimmer
    }
    fn cull_screen_area(&self) -> &f32 {
        &self.cull_screen_area
    }
    fn fade_screen_area(&self) -> &f32 {
        &self.fade_screen_area
    }
    fn cull_distance(&self) -> &f32 {
        &self.cull_distance
    }
    fn fade_distance(&self) -> &f32 {
        &self.fade_distance
    }
    fn shadow_cull_screen_area(&self) -> &f32 {
        &self.shadow_cull_screen_area
    }
    fn shadow_fade_screen_area(&self) -> &f32 {
        &self.shadow_fade_screen_area
    }
    fn shadow_cull_distance(&self) -> &f32 {
        &self.shadow_cull_distance
    }
    fn shadow_fade_distance(&self) -> &f32 {
        &self.shadow_fade_distance
    }
    fn direct_lightmap_enable(&self) -> &bool {
        &self.direct_lightmap_enable
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn intensity_multiplier(&self) -> &f32 {
        &self.intensity_multiplier
    }
    fn shadow_cache_enable(&self) -> &bool {
        &self.shadow_cache_enable
    }
    fn shadow_cache_update_priority(&self) -> &f32 {
        &self.shadow_cache_update_priority
    }
    fn shadow_cache_update_counter(&self) -> &u32 {
        &self.shadow_cache_update_counter
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u16 {
        &self.field_flag_changed1
    }
}

pub static PBRTUBELIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PbrTubeLightDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TubeRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, tube_radius),
            },
            FieldInfoData {
                name: "TubeWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, tube_width),
            },
            FieldInfoData {
                name: "OnlyHempishere",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, only_hempishere),
            },
            FieldInfoData {
                name: "IsCapsule",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, is_capsule),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrTubeLightDynamicState, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: "LightUnitType",
                rust_offset: offset_of!(PbrTubeLightDynamicState, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrTubeLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrTubeLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrTubeLightDynamicState, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrTubeLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrTubeLightDynamicState, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(PbrTubeLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(PbrTubeLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PbrTubeLightDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(PbrTubeLightDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(PBRTUBELIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrTubeLightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        PBRTUBELIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PBRTUBELIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrTubeLightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PbrTubeLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub trait PbrTubeLightStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl PbrTubeLightStaticStateTrait for PbrTubeLightStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static PBRTUBELIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PbrTubeLightStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(PbrTubeLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PbrTubeLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PBRTUBELIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PbrTubeLightStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        PBRTUBELIGHTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PBRTUBELIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrTubeLightStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PbrSpotLightDynamicState {
    pub disc_radius: f32,
    pub inner_angle: f32,
    pub outer_angle: f32,
    pub i_e_s_profile: Option<Arc<Mutex<dyn IesProfileAssetTrait>>>,
    pub use_i_e_s_profile_as_mask: bool,
    pub i_e_s_multiplier: f32,
    pub shadow_max_angle: f32,
    pub shadow_fade_out_range: f32,
    pub color: super::core::Vec3,
    pub intensity: f32,
    pub exposure_compensation: f32,
    pub attenuation_radius: f32,
    pub emissive_shape_enable: bool,
    pub attenuation_offset: f32,
    pub light_unit: super::render_base::LightUnitType,
    pub affect_diffuse: bool,
    pub affect_specular: bool,
    pub particle_color_scale: super::core::Vec3,
    pub cast_shadows: super::core::QualityScalableEnabled,
    pub cast_volumetric: super::core::QualityScalableEnabled,
    pub cast_volumetric_shadows: super::core::QualityScalableEnabled,
    pub shadow_resolution: super::core::QualityLevel,
    pub shadow_near_radius: f32,
    pub shadow_far_radius: f32,
    pub shadow_dimmer: f32,
    pub cast_shadows_enable: bool,
    pub cast_volumetric_shadows_enable: bool,
    pub affect_radiosity: bool,
    pub radiosity_color_scale: super::core::Vec3,
    pub dimmer: f32,
    pub cull_screen_area: f32,
    pub fade_screen_area: f32,
    pub cull_distance: f32,
    pub fade_distance: f32,
    pub shadow_cull_screen_area: f32,
    pub shadow_fade_screen_area: f32,
    pub shadow_cull_distance: f32,
    pub shadow_fade_distance: f32,
    pub direct_lightmap_enable: bool,
    pub is_first_person: bool,
    pub is_enabled: bool,
    pub intensity_multiplier: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u16,
}

pub trait PbrSpotLightDynamicStateTrait: TypeObject {
    fn disc_radius(&self) -> &f32;
    fn inner_angle(&self) -> &f32;
    fn outer_angle(&self) -> &f32;
    fn i_e_s_profile(&self) -> &Option<Arc<Mutex<dyn IesProfileAssetTrait>>>;
    fn use_i_e_s_profile_as_mask(&self) -> &bool;
    fn i_e_s_multiplier(&self) -> &f32;
    fn shadow_max_angle(&self) -> &f32;
    fn shadow_fade_out_range(&self) -> &f32;
    fn color(&self) -> &super::core::Vec3;
    fn intensity(&self) -> &f32;
    fn exposure_compensation(&self) -> &f32;
    fn attenuation_radius(&self) -> &f32;
    fn emissive_shape_enable(&self) -> &bool;
    fn attenuation_offset(&self) -> &f32;
    fn light_unit(&self) -> &super::render_base::LightUnitType;
    fn affect_diffuse(&self) -> &bool;
    fn affect_specular(&self) -> &bool;
    fn particle_color_scale(&self) -> &super::core::Vec3;
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn shadow_resolution(&self) -> &super::core::QualityLevel;
    fn shadow_near_radius(&self) -> &f32;
    fn shadow_far_radius(&self) -> &f32;
    fn shadow_dimmer(&self) -> &f32;
    fn cast_shadows_enable(&self) -> &bool;
    fn cast_volumetric_shadows_enable(&self) -> &bool;
    fn affect_radiosity(&self) -> &bool;
    fn radiosity_color_scale(&self) -> &super::core::Vec3;
    fn dimmer(&self) -> &f32;
    fn cull_screen_area(&self) -> &f32;
    fn fade_screen_area(&self) -> &f32;
    fn cull_distance(&self) -> &f32;
    fn fade_distance(&self) -> &f32;
    fn shadow_cull_screen_area(&self) -> &f32;
    fn shadow_fade_screen_area(&self) -> &f32;
    fn shadow_cull_distance(&self) -> &f32;
    fn shadow_fade_distance(&self) -> &f32;
    fn direct_lightmap_enable(&self) -> &bool;
    fn is_first_person(&self) -> &bool;
    fn is_enabled(&self) -> &bool;
    fn intensity_multiplier(&self) -> &f32;
    fn shadow_cache_enable(&self) -> &bool;
    fn shadow_cache_update_priority(&self) -> &f32;
    fn shadow_cache_update_counter(&self) -> &u32;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u16;
}

impl PbrSpotLightDynamicStateTrait for PbrSpotLightDynamicState {
    fn disc_radius(&self) -> &f32 {
        &self.disc_radius
    }
    fn inner_angle(&self) -> &f32 {
        &self.inner_angle
    }
    fn outer_angle(&self) -> &f32 {
        &self.outer_angle
    }
    fn i_e_s_profile(&self) -> &Option<Arc<Mutex<dyn IesProfileAssetTrait>>> {
        &self.i_e_s_profile
    }
    fn use_i_e_s_profile_as_mask(&self) -> &bool {
        &self.use_i_e_s_profile_as_mask
    }
    fn i_e_s_multiplier(&self) -> &f32 {
        &self.i_e_s_multiplier
    }
    fn shadow_max_angle(&self) -> &f32 {
        &self.shadow_max_angle
    }
    fn shadow_fade_out_range(&self) -> &f32 {
        &self.shadow_fade_out_range
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn exposure_compensation(&self) -> &f32 {
        &self.exposure_compensation
    }
    fn attenuation_radius(&self) -> &f32 {
        &self.attenuation_radius
    }
    fn emissive_shape_enable(&self) -> &bool {
        &self.emissive_shape_enable
    }
    fn attenuation_offset(&self) -> &f32 {
        &self.attenuation_offset
    }
    fn light_unit(&self) -> &super::render_base::LightUnitType {
        &self.light_unit
    }
    fn affect_diffuse(&self) -> &bool {
        &self.affect_diffuse
    }
    fn affect_specular(&self) -> &bool {
        &self.affect_specular
    }
    fn particle_color_scale(&self) -> &super::core::Vec3 {
        &self.particle_color_scale
    }
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_shadows
    }
    fn cast_volumetric(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric
    }
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric_shadows
    }
    fn shadow_resolution(&self) -> &super::core::QualityLevel {
        &self.shadow_resolution
    }
    fn shadow_near_radius(&self) -> &f32 {
        &self.shadow_near_radius
    }
    fn shadow_far_radius(&self) -> &f32 {
        &self.shadow_far_radius
    }
    fn shadow_dimmer(&self) -> &f32 {
        &self.shadow_dimmer
    }
    fn cast_shadows_enable(&self) -> &bool {
        &self.cast_shadows_enable
    }
    fn cast_volumetric_shadows_enable(&self) -> &bool {
        &self.cast_volumetric_shadows_enable
    }
    fn affect_radiosity(&self) -> &bool {
        &self.affect_radiosity
    }
    fn radiosity_color_scale(&self) -> &super::core::Vec3 {
        &self.radiosity_color_scale
    }
    fn dimmer(&self) -> &f32 {
        &self.dimmer
    }
    fn cull_screen_area(&self) -> &f32 {
        &self.cull_screen_area
    }
    fn fade_screen_area(&self) -> &f32 {
        &self.fade_screen_area
    }
    fn cull_distance(&self) -> &f32 {
        &self.cull_distance
    }
    fn fade_distance(&self) -> &f32 {
        &self.fade_distance
    }
    fn shadow_cull_screen_area(&self) -> &f32 {
        &self.shadow_cull_screen_area
    }
    fn shadow_fade_screen_area(&self) -> &f32 {
        &self.shadow_fade_screen_area
    }
    fn shadow_cull_distance(&self) -> &f32 {
        &self.shadow_cull_distance
    }
    fn shadow_fade_distance(&self) -> &f32 {
        &self.shadow_fade_distance
    }
    fn direct_lightmap_enable(&self) -> &bool {
        &self.direct_lightmap_enable
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn intensity_multiplier(&self) -> &f32 {
        &self.intensity_multiplier
    }
    fn shadow_cache_enable(&self) -> &bool {
        &self.shadow_cache_enable
    }
    fn shadow_cache_update_priority(&self) -> &f32 {
        &self.shadow_cache_update_priority
    }
    fn shadow_cache_update_counter(&self) -> &u32 {
        &self.shadow_cache_update_counter
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u16 {
        &self.field_flag_changed1
    }
}

pub static PBRSPOTLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PbrSpotLightDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DiscRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, disc_radius),
            },
            FieldInfoData {
                name: "InnerAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, inner_angle),
            },
            FieldInfoData {
                name: "OuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, outer_angle),
            },
            FieldInfoData {
                name: "IESProfile",
                flags: MemberInfoFlags::new(0),
                field_type: "IesProfileAsset",
                rust_offset: offset_of!(PbrSpotLightDynamicState, i_e_s_profile),
            },
            FieldInfoData {
                name: "UseIESProfileAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, use_i_e_s_profile_as_mask),
            },
            FieldInfoData {
                name: "IESMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, i_e_s_multiplier),
            },
            FieldInfoData {
                name: "ShadowMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_max_angle),
            },
            FieldInfoData {
                name: "ShadowFadeOutRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_fade_out_range),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrSpotLightDynamicState, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: "LightUnitType",
                rust_offset: offset_of!(PbrSpotLightDynamicState, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrSpotLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrSpotLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrSpotLightDynamicState, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrSpotLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrSpotLightDynamicState, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(PbrSpotLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(PbrSpotLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PbrSpotLightDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(PbrSpotLightDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(PBRSPOTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrSpotLightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        PBRSPOTLIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PBRSPOTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrSpotLightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PbrSpotLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub trait PbrSpotLightStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl PbrSpotLightStaticStateTrait for PbrSpotLightStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static PBRSPOTLIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PbrSpotLightStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(PbrSpotLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PbrSpotLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PBRSPOTLIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PbrSpotLightStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        PBRSPOTLIGHTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PBRSPOTLIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrSpotLightStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PbrSphereLightDynamicState {
    pub sphere_radius: f32,
    pub only_hempishere: bool,
    pub i_e_s_profile: Option<Arc<Mutex<dyn IesProfileAssetTrait>>>,
    pub use_i_e_s_profile_as_mask: bool,
    pub i_e_s_multiplier: f32,
    pub color: super::core::Vec3,
    pub intensity: f32,
    pub exposure_compensation: f32,
    pub attenuation_radius: f32,
    pub emissive_shape_enable: bool,
    pub attenuation_offset: f32,
    pub light_unit: super::render_base::LightUnitType,
    pub affect_diffuse: bool,
    pub affect_specular: bool,
    pub particle_color_scale: super::core::Vec3,
    pub cast_shadows: super::core::QualityScalableEnabled,
    pub cast_volumetric: super::core::QualityScalableEnabled,
    pub cast_volumetric_shadows: super::core::QualityScalableEnabled,
    pub shadow_resolution: super::core::QualityLevel,
    pub shadow_near_radius: f32,
    pub shadow_far_radius: f32,
    pub shadow_dimmer: f32,
    pub cast_shadows_enable: bool,
    pub cast_volumetric_shadows_enable: bool,
    pub affect_radiosity: bool,
    pub radiosity_color_scale: super::core::Vec3,
    pub dimmer: f32,
    pub cull_screen_area: f32,
    pub fade_screen_area: f32,
    pub cull_distance: f32,
    pub fade_distance: f32,
    pub shadow_cull_screen_area: f32,
    pub shadow_fade_screen_area: f32,
    pub shadow_cull_distance: f32,
    pub shadow_fade_distance: f32,
    pub direct_lightmap_enable: bool,
    pub is_first_person: bool,
    pub is_enabled: bool,
    pub intensity_multiplier: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u16,
}

pub trait PbrSphereLightDynamicStateTrait: TypeObject {
    fn sphere_radius(&self) -> &f32;
    fn only_hempishere(&self) -> &bool;
    fn i_e_s_profile(&self) -> &Option<Arc<Mutex<dyn IesProfileAssetTrait>>>;
    fn use_i_e_s_profile_as_mask(&self) -> &bool;
    fn i_e_s_multiplier(&self) -> &f32;
    fn color(&self) -> &super::core::Vec3;
    fn intensity(&self) -> &f32;
    fn exposure_compensation(&self) -> &f32;
    fn attenuation_radius(&self) -> &f32;
    fn emissive_shape_enable(&self) -> &bool;
    fn attenuation_offset(&self) -> &f32;
    fn light_unit(&self) -> &super::render_base::LightUnitType;
    fn affect_diffuse(&self) -> &bool;
    fn affect_specular(&self) -> &bool;
    fn particle_color_scale(&self) -> &super::core::Vec3;
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn shadow_resolution(&self) -> &super::core::QualityLevel;
    fn shadow_near_radius(&self) -> &f32;
    fn shadow_far_radius(&self) -> &f32;
    fn shadow_dimmer(&self) -> &f32;
    fn cast_shadows_enable(&self) -> &bool;
    fn cast_volumetric_shadows_enable(&self) -> &bool;
    fn affect_radiosity(&self) -> &bool;
    fn radiosity_color_scale(&self) -> &super::core::Vec3;
    fn dimmer(&self) -> &f32;
    fn cull_screen_area(&self) -> &f32;
    fn fade_screen_area(&self) -> &f32;
    fn cull_distance(&self) -> &f32;
    fn fade_distance(&self) -> &f32;
    fn shadow_cull_screen_area(&self) -> &f32;
    fn shadow_fade_screen_area(&self) -> &f32;
    fn shadow_cull_distance(&self) -> &f32;
    fn shadow_fade_distance(&self) -> &f32;
    fn direct_lightmap_enable(&self) -> &bool;
    fn is_first_person(&self) -> &bool;
    fn is_enabled(&self) -> &bool;
    fn intensity_multiplier(&self) -> &f32;
    fn shadow_cache_enable(&self) -> &bool;
    fn shadow_cache_update_priority(&self) -> &f32;
    fn shadow_cache_update_counter(&self) -> &u32;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u16;
}

impl PbrSphereLightDynamicStateTrait for PbrSphereLightDynamicState {
    fn sphere_radius(&self) -> &f32 {
        &self.sphere_radius
    }
    fn only_hempishere(&self) -> &bool {
        &self.only_hempishere
    }
    fn i_e_s_profile(&self) -> &Option<Arc<Mutex<dyn IesProfileAssetTrait>>> {
        &self.i_e_s_profile
    }
    fn use_i_e_s_profile_as_mask(&self) -> &bool {
        &self.use_i_e_s_profile_as_mask
    }
    fn i_e_s_multiplier(&self) -> &f32 {
        &self.i_e_s_multiplier
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn exposure_compensation(&self) -> &f32 {
        &self.exposure_compensation
    }
    fn attenuation_radius(&self) -> &f32 {
        &self.attenuation_radius
    }
    fn emissive_shape_enable(&self) -> &bool {
        &self.emissive_shape_enable
    }
    fn attenuation_offset(&self) -> &f32 {
        &self.attenuation_offset
    }
    fn light_unit(&self) -> &super::render_base::LightUnitType {
        &self.light_unit
    }
    fn affect_diffuse(&self) -> &bool {
        &self.affect_diffuse
    }
    fn affect_specular(&self) -> &bool {
        &self.affect_specular
    }
    fn particle_color_scale(&self) -> &super::core::Vec3 {
        &self.particle_color_scale
    }
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_shadows
    }
    fn cast_volumetric(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric
    }
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric_shadows
    }
    fn shadow_resolution(&self) -> &super::core::QualityLevel {
        &self.shadow_resolution
    }
    fn shadow_near_radius(&self) -> &f32 {
        &self.shadow_near_radius
    }
    fn shadow_far_radius(&self) -> &f32 {
        &self.shadow_far_radius
    }
    fn shadow_dimmer(&self) -> &f32 {
        &self.shadow_dimmer
    }
    fn cast_shadows_enable(&self) -> &bool {
        &self.cast_shadows_enable
    }
    fn cast_volumetric_shadows_enable(&self) -> &bool {
        &self.cast_volumetric_shadows_enable
    }
    fn affect_radiosity(&self) -> &bool {
        &self.affect_radiosity
    }
    fn radiosity_color_scale(&self) -> &super::core::Vec3 {
        &self.radiosity_color_scale
    }
    fn dimmer(&self) -> &f32 {
        &self.dimmer
    }
    fn cull_screen_area(&self) -> &f32 {
        &self.cull_screen_area
    }
    fn fade_screen_area(&self) -> &f32 {
        &self.fade_screen_area
    }
    fn cull_distance(&self) -> &f32 {
        &self.cull_distance
    }
    fn fade_distance(&self) -> &f32 {
        &self.fade_distance
    }
    fn shadow_cull_screen_area(&self) -> &f32 {
        &self.shadow_cull_screen_area
    }
    fn shadow_fade_screen_area(&self) -> &f32 {
        &self.shadow_fade_screen_area
    }
    fn shadow_cull_distance(&self) -> &f32 {
        &self.shadow_cull_distance
    }
    fn shadow_fade_distance(&self) -> &f32 {
        &self.shadow_fade_distance
    }
    fn direct_lightmap_enable(&self) -> &bool {
        &self.direct_lightmap_enable
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn intensity_multiplier(&self) -> &f32 {
        &self.intensity_multiplier
    }
    fn shadow_cache_enable(&self) -> &bool {
        &self.shadow_cache_enable
    }
    fn shadow_cache_update_priority(&self) -> &f32 {
        &self.shadow_cache_update_priority
    }
    fn shadow_cache_update_counter(&self) -> &u32 {
        &self.shadow_cache_update_counter
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u16 {
        &self.field_flag_changed1
    }
}

pub static PBRSPHERELIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PbrSphereLightDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SphereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, sphere_radius),
            },
            FieldInfoData {
                name: "OnlyHempishere",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, only_hempishere),
            },
            FieldInfoData {
                name: "IESProfile",
                flags: MemberInfoFlags::new(0),
                field_type: "IesProfileAsset",
                rust_offset: offset_of!(PbrSphereLightDynamicState, i_e_s_profile),
            },
            FieldInfoData {
                name: "UseIESProfileAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, use_i_e_s_profile_as_mask),
            },
            FieldInfoData {
                name: "IESMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, i_e_s_multiplier),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrSphereLightDynamicState, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: "LightUnitType",
                rust_offset: offset_of!(PbrSphereLightDynamicState, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrSphereLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrSphereLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrSphereLightDynamicState, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrSphereLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrSphereLightDynamicState, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(PbrSphereLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(PbrSphereLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PbrSphereLightDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(PbrSphereLightDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(PBRSPHERELIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrSphereLightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        PBRSPHERELIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PBRSPHERELIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrSphereLightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PbrSphereLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub trait PbrSphereLightStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl PbrSphereLightStaticStateTrait for PbrSphereLightStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static PBRSPHERELIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PbrSphereLightStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(PbrSphereLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PbrSphereLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PBRSPHERELIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PbrSphereLightStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        PBRSPHERELIGHTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PBRSPHERELIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrSphereLightStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PbrAnalyticLightDynamicState {
    pub color: super::core::Vec3,
    pub intensity: f32,
    pub exposure_compensation: f32,
    pub attenuation_radius: f32,
    pub emissive_shape_enable: bool,
    pub attenuation_offset: f32,
    pub light_unit: super::render_base::LightUnitType,
    pub affect_diffuse: bool,
    pub affect_specular: bool,
    pub particle_color_scale: super::core::Vec3,
    pub cast_shadows: super::core::QualityScalableEnabled,
    pub cast_volumetric: super::core::QualityScalableEnabled,
    pub cast_volumetric_shadows: super::core::QualityScalableEnabled,
    pub shadow_resolution: super::core::QualityLevel,
    pub shadow_near_radius: f32,
    pub shadow_far_radius: f32,
    pub shadow_dimmer: f32,
    pub cast_shadows_enable: bool,
    pub cast_volumetric_shadows_enable: bool,
    pub affect_radiosity: bool,
    pub radiosity_color_scale: super::core::Vec3,
    pub dimmer: f32,
    pub cull_screen_area: f32,
    pub fade_screen_area: f32,
    pub cull_distance: f32,
    pub fade_distance: f32,
    pub shadow_cull_screen_area: f32,
    pub shadow_fade_screen_area: f32,
    pub shadow_cull_distance: f32,
    pub shadow_fade_distance: f32,
    pub direct_lightmap_enable: bool,
    pub is_first_person: bool,
    pub is_enabled: bool,
    pub intensity_multiplier: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
}

pub trait PbrAnalyticLightDynamicStateTrait: TypeObject {
    fn color(&self) -> &super::core::Vec3;
    fn intensity(&self) -> &f32;
    fn exposure_compensation(&self) -> &f32;
    fn attenuation_radius(&self) -> &f32;
    fn emissive_shape_enable(&self) -> &bool;
    fn attenuation_offset(&self) -> &f32;
    fn light_unit(&self) -> &super::render_base::LightUnitType;
    fn affect_diffuse(&self) -> &bool;
    fn affect_specular(&self) -> &bool;
    fn particle_color_scale(&self) -> &super::core::Vec3;
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn shadow_resolution(&self) -> &super::core::QualityLevel;
    fn shadow_near_radius(&self) -> &f32;
    fn shadow_far_radius(&self) -> &f32;
    fn shadow_dimmer(&self) -> &f32;
    fn cast_shadows_enable(&self) -> &bool;
    fn cast_volumetric_shadows_enable(&self) -> &bool;
    fn affect_radiosity(&self) -> &bool;
    fn radiosity_color_scale(&self) -> &super::core::Vec3;
    fn dimmer(&self) -> &f32;
    fn cull_screen_area(&self) -> &f32;
    fn fade_screen_area(&self) -> &f32;
    fn cull_distance(&self) -> &f32;
    fn fade_distance(&self) -> &f32;
    fn shadow_cull_screen_area(&self) -> &f32;
    fn shadow_fade_screen_area(&self) -> &f32;
    fn shadow_cull_distance(&self) -> &f32;
    fn shadow_fade_distance(&self) -> &f32;
    fn direct_lightmap_enable(&self) -> &bool;
    fn is_first_person(&self) -> &bool;
    fn is_enabled(&self) -> &bool;
    fn intensity_multiplier(&self) -> &f32;
    fn shadow_cache_enable(&self) -> &bool;
    fn shadow_cache_update_priority(&self) -> &f32;
    fn shadow_cache_update_counter(&self) -> &u32;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
}

impl PbrAnalyticLightDynamicStateTrait for PbrAnalyticLightDynamicState {
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn exposure_compensation(&self) -> &f32 {
        &self.exposure_compensation
    }
    fn attenuation_radius(&self) -> &f32 {
        &self.attenuation_radius
    }
    fn emissive_shape_enable(&self) -> &bool {
        &self.emissive_shape_enable
    }
    fn attenuation_offset(&self) -> &f32 {
        &self.attenuation_offset
    }
    fn light_unit(&self) -> &super::render_base::LightUnitType {
        &self.light_unit
    }
    fn affect_diffuse(&self) -> &bool {
        &self.affect_diffuse
    }
    fn affect_specular(&self) -> &bool {
        &self.affect_specular
    }
    fn particle_color_scale(&self) -> &super::core::Vec3 {
        &self.particle_color_scale
    }
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_shadows
    }
    fn cast_volumetric(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric
    }
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric_shadows
    }
    fn shadow_resolution(&self) -> &super::core::QualityLevel {
        &self.shadow_resolution
    }
    fn shadow_near_radius(&self) -> &f32 {
        &self.shadow_near_radius
    }
    fn shadow_far_radius(&self) -> &f32 {
        &self.shadow_far_radius
    }
    fn shadow_dimmer(&self) -> &f32 {
        &self.shadow_dimmer
    }
    fn cast_shadows_enable(&self) -> &bool {
        &self.cast_shadows_enable
    }
    fn cast_volumetric_shadows_enable(&self) -> &bool {
        &self.cast_volumetric_shadows_enable
    }
    fn affect_radiosity(&self) -> &bool {
        &self.affect_radiosity
    }
    fn radiosity_color_scale(&self) -> &super::core::Vec3 {
        &self.radiosity_color_scale
    }
    fn dimmer(&self) -> &f32 {
        &self.dimmer
    }
    fn cull_screen_area(&self) -> &f32 {
        &self.cull_screen_area
    }
    fn fade_screen_area(&self) -> &f32 {
        &self.fade_screen_area
    }
    fn cull_distance(&self) -> &f32 {
        &self.cull_distance
    }
    fn fade_distance(&self) -> &f32 {
        &self.fade_distance
    }
    fn shadow_cull_screen_area(&self) -> &f32 {
        &self.shadow_cull_screen_area
    }
    fn shadow_fade_screen_area(&self) -> &f32 {
        &self.shadow_fade_screen_area
    }
    fn shadow_cull_distance(&self) -> &f32 {
        &self.shadow_cull_distance
    }
    fn shadow_fade_distance(&self) -> &f32 {
        &self.shadow_fade_distance
    }
    fn direct_lightmap_enable(&self) -> &bool {
        &self.direct_lightmap_enable
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn intensity_multiplier(&self) -> &f32 {
        &self.intensity_multiplier
    }
    fn shadow_cache_enable(&self) -> &bool {
        &self.shadow_cache_enable
    }
    fn shadow_cache_update_priority(&self) -> &f32 {
        &self.shadow_cache_update_priority
    }
    fn shadow_cache_update_counter(&self) -> &u32 {
        &self.shadow_cache_update_counter
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
}

pub static PBRANALYTICLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrAnalyticLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PbrAnalyticLightDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: "LightUnitType",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, excluded_cull_id),
            },
        ],
    }),
    array_type: Some(PBRANALYTICLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrAnalyticLightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        PBRANALYTICLIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PBRANALYTICLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrAnalyticLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrAnalyticLightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OldSpotLightDynamicState {
    pub shape: SpotLightShape,
    pub cone_inner_angle: f32,
    pub cone_outer_angle: f32,
    pub frustum_fov: f32,
    pub frustum_aspect: f32,
    pub ortho_width: f32,
    pub ortho_height: f32,
    pub near_plane: f32,
    pub texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub cast_shadows: super::core::QualityScalableEnabled,
    pub cast_volumetric_shadows: super::core::QualityScalableEnabled,
    pub shadow_resolution: super::core::QualityLevel,
    pub shadow_radius: f32,
    pub shadow_dimmer: f32,
    pub frustum_as_cone: super::core::QualityScalableEnabled,
    pub frustum_as_cone_angle: bool,
    pub frustum_as_cone_intensity_scale: f32,
    pub cast_shadows_enable: bool,
    pub cast_volumetric_shadows_enable: bool,
    pub cast_shadows_min_level: super::core::QualityLevel,
    pub direct_lightmap_enable: bool,
    pub color: super::core::Vec3,
    pub radius: f32,
    pub intensity: f32,
    pub attenuation_offset: f32,
    pub direct_light_enable: bool,
    pub specular_enable: bool,
    pub enlighten_color_mode: EnlightenColorMode,
    pub enlighten_enable: bool,
    pub enlighten_color_scale: super::core::Vec3,
    pub particle_color_scale: super::core::Vec3,
    pub exposure_compensation: f32,
    pub is_first_person: bool,
    pub is_enabled: bool,
    pub intensity_multiplier: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u8,
}

pub trait OldSpotLightDynamicStateTrait: TypeObject {
    fn shape(&self) -> &SpotLightShape;
    fn cone_inner_angle(&self) -> &f32;
    fn cone_outer_angle(&self) -> &f32;
    fn frustum_fov(&self) -> &f32;
    fn frustum_aspect(&self) -> &f32;
    fn ortho_width(&self) -> &f32;
    fn ortho_height(&self) -> &f32;
    fn near_plane(&self) -> &f32;
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled;
    fn shadow_resolution(&self) -> &super::core::QualityLevel;
    fn shadow_radius(&self) -> &f32;
    fn shadow_dimmer(&self) -> &f32;
    fn frustum_as_cone(&self) -> &super::core::QualityScalableEnabled;
    fn frustum_as_cone_angle(&self) -> &bool;
    fn frustum_as_cone_intensity_scale(&self) -> &f32;
    fn cast_shadows_enable(&self) -> &bool;
    fn cast_volumetric_shadows_enable(&self) -> &bool;
    fn cast_shadows_min_level(&self) -> &super::core::QualityLevel;
    fn direct_lightmap_enable(&self) -> &bool;
    fn color(&self) -> &super::core::Vec3;
    fn radius(&self) -> &f32;
    fn intensity(&self) -> &f32;
    fn attenuation_offset(&self) -> &f32;
    fn direct_light_enable(&self) -> &bool;
    fn specular_enable(&self) -> &bool;
    fn enlighten_color_mode(&self) -> &EnlightenColorMode;
    fn enlighten_enable(&self) -> &bool;
    fn enlighten_color_scale(&self) -> &super::core::Vec3;
    fn particle_color_scale(&self) -> &super::core::Vec3;
    fn exposure_compensation(&self) -> &f32;
    fn is_first_person(&self) -> &bool;
    fn is_enabled(&self) -> &bool;
    fn intensity_multiplier(&self) -> &f32;
    fn shadow_cache_enable(&self) -> &bool;
    fn shadow_cache_update_priority(&self) -> &f32;
    fn shadow_cache_update_counter(&self) -> &u32;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed1(&self) -> &u8;
}

impl OldSpotLightDynamicStateTrait for OldSpotLightDynamicState {
    fn shape(&self) -> &SpotLightShape {
        &self.shape
    }
    fn cone_inner_angle(&self) -> &f32 {
        &self.cone_inner_angle
    }
    fn cone_outer_angle(&self) -> &f32 {
        &self.cone_outer_angle
    }
    fn frustum_fov(&self) -> &f32 {
        &self.frustum_fov
    }
    fn frustum_aspect(&self) -> &f32 {
        &self.frustum_aspect
    }
    fn ortho_width(&self) -> &f32 {
        &self.ortho_width
    }
    fn ortho_height(&self) -> &f32 {
        &self.ortho_height
    }
    fn near_plane(&self) -> &f32 {
        &self.near_plane
    }
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture
    }
    fn cast_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_shadows
    }
    fn cast_volumetric_shadows(&self) -> &super::core::QualityScalableEnabled {
        &self.cast_volumetric_shadows
    }
    fn shadow_resolution(&self) -> &super::core::QualityLevel {
        &self.shadow_resolution
    }
    fn shadow_radius(&self) -> &f32 {
        &self.shadow_radius
    }
    fn shadow_dimmer(&self) -> &f32 {
        &self.shadow_dimmer
    }
    fn frustum_as_cone(&self) -> &super::core::QualityScalableEnabled {
        &self.frustum_as_cone
    }
    fn frustum_as_cone_angle(&self) -> &bool {
        &self.frustum_as_cone_angle
    }
    fn frustum_as_cone_intensity_scale(&self) -> &f32 {
        &self.frustum_as_cone_intensity_scale
    }
    fn cast_shadows_enable(&self) -> &bool {
        &self.cast_shadows_enable
    }
    fn cast_volumetric_shadows_enable(&self) -> &bool {
        &self.cast_volumetric_shadows_enable
    }
    fn cast_shadows_min_level(&self) -> &super::core::QualityLevel {
        &self.cast_shadows_min_level
    }
    fn direct_lightmap_enable(&self) -> &bool {
        &self.direct_lightmap_enable
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn attenuation_offset(&self) -> &f32 {
        &self.attenuation_offset
    }
    fn direct_light_enable(&self) -> &bool {
        &self.direct_light_enable
    }
    fn specular_enable(&self) -> &bool {
        &self.specular_enable
    }
    fn enlighten_color_mode(&self) -> &EnlightenColorMode {
        &self.enlighten_color_mode
    }
    fn enlighten_enable(&self) -> &bool {
        &self.enlighten_enable
    }
    fn enlighten_color_scale(&self) -> &super::core::Vec3 {
        &self.enlighten_color_scale
    }
    fn particle_color_scale(&self) -> &super::core::Vec3 {
        &self.particle_color_scale
    }
    fn exposure_compensation(&self) -> &f32 {
        &self.exposure_compensation
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn intensity_multiplier(&self) -> &f32 {
        &self.intensity_multiplier
    }
    fn shadow_cache_enable(&self) -> &bool {
        &self.shadow_cache_enable
    }
    fn shadow_cache_update_priority(&self) -> &f32 {
        &self.shadow_cache_update_priority
    }
    fn shadow_cache_update_counter(&self) -> &u32 {
        &self.shadow_cache_update_counter
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed1(&self) -> &u8 {
        &self.field_flag_changed1
    }
}

pub static OLDSPOTLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldSpotLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OldSpotLightDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Shape",
                flags: MemberInfoFlags::new(0),
                field_type: "SpotLightShape",
                rust_offset: offset_of!(OldSpotLightDynamicState, shape),
            },
            FieldInfoData {
                name: "ConeInnerAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, cone_inner_angle),
            },
            FieldInfoData {
                name: "ConeOuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, cone_outer_angle),
            },
            FieldInfoData {
                name: "FrustumFov",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, frustum_fov),
            },
            FieldInfoData {
                name: "FrustumAspect",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, frustum_aspect),
            },
            FieldInfoData {
                name: "OrthoWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, ortho_width),
            },
            FieldInfoData {
                name: "OrthoHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, ortho_height),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, near_plane),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(OldSpotLightDynamicState, texture),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(OldSpotLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(OldSpotLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "FrustumAsCone",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(OldSpotLightDynamicState, frustum_as_cone),
            },
            FieldInfoData {
                name: "FrustumAsConeAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldSpotLightDynamicState, frustum_as_cone_angle),
            },
            FieldInfoData {
                name: "FrustumAsConeIntensityScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, frustum_as_cone_intensity_scale),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldSpotLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldSpotLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "CastShadowsMinLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(OldSpotLightDynamicState, cast_shadows_min_level),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldSpotLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OldSpotLightDynamicState, color),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, radius),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "DirectLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldSpotLightDynamicState, direct_light_enable),
            },
            FieldInfoData {
                name: "SpecularEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldSpotLightDynamicState, specular_enable),
            },
            FieldInfoData {
                name: "EnlightenColorMode",
                flags: MemberInfoFlags::new(0),
                field_type: "EnlightenColorMode",
                rust_offset: offset_of!(OldSpotLightDynamicState, enlighten_color_mode),
            },
            FieldInfoData {
                name: "EnlightenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldSpotLightDynamicState, enlighten_enable),
            },
            FieldInfoData {
                name: "EnlightenColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OldSpotLightDynamicState, enlighten_color_scale),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OldSpotLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldSpotLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldSpotLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(OldSpotLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(OldSpotLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OldSpotLightDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OldSpotLightDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(OLDSPOTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OldSpotLightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        OLDSPOTLIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OLDSPOTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldSpotLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OldSpotLightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OldSpotLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub trait OldSpotLightStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl OldSpotLightStaticStateTrait for OldSpotLightStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static OLDSPOTLIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldSpotLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OldSpotLightStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(OldSpotLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OldSpotLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OLDSPOTLIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for OldSpotLightStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        OLDSPOTLIGHTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OLDSPOTLIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldSpotLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OldSpotLightStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OldPointLightDynamicState {
    pub width: f32,
    pub translucency_ambient: f32,
    pub translucency_scale: f32,
    pub translucency_power: f32,
    pub translucency_distortion: f32,
    pub direct_lightmap_enable: bool,
    pub color: super::core::Vec3,
    pub radius: f32,
    pub intensity: f32,
    pub attenuation_offset: f32,
    pub direct_light_enable: bool,
    pub specular_enable: bool,
    pub enlighten_color_mode: EnlightenColorMode,
    pub enlighten_enable: bool,
    pub enlighten_color_scale: super::core::Vec3,
    pub particle_color_scale: super::core::Vec3,
    pub exposure_compensation: f32,
    pub is_first_person: bool,
    pub is_enabled: bool,
    pub intensity_multiplier: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub field_flag_changed0: u32,
}

pub trait OldPointLightDynamicStateTrait: TypeObject {
    fn width(&self) -> &f32;
    fn translucency_ambient(&self) -> &f32;
    fn translucency_scale(&self) -> &f32;
    fn translucency_power(&self) -> &f32;
    fn translucency_distortion(&self) -> &f32;
    fn direct_lightmap_enable(&self) -> &bool;
    fn color(&self) -> &super::core::Vec3;
    fn radius(&self) -> &f32;
    fn intensity(&self) -> &f32;
    fn attenuation_offset(&self) -> &f32;
    fn direct_light_enable(&self) -> &bool;
    fn specular_enable(&self) -> &bool;
    fn enlighten_color_mode(&self) -> &EnlightenColorMode;
    fn enlighten_enable(&self) -> &bool;
    fn enlighten_color_scale(&self) -> &super::core::Vec3;
    fn particle_color_scale(&self) -> &super::core::Vec3;
    fn exposure_compensation(&self) -> &f32;
    fn is_first_person(&self) -> &bool;
    fn is_enabled(&self) -> &bool;
    fn intensity_multiplier(&self) -> &f32;
    fn shadow_cache_enable(&self) -> &bool;
    fn shadow_cache_update_priority(&self) -> &f32;
    fn shadow_cache_update_counter(&self) -> &u32;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn field_flag_changed0(&self) -> &u32;
}

impl OldPointLightDynamicStateTrait for OldPointLightDynamicState {
    fn width(&self) -> &f32 {
        &self.width
    }
    fn translucency_ambient(&self) -> &f32 {
        &self.translucency_ambient
    }
    fn translucency_scale(&self) -> &f32 {
        &self.translucency_scale
    }
    fn translucency_power(&self) -> &f32 {
        &self.translucency_power
    }
    fn translucency_distortion(&self) -> &f32 {
        &self.translucency_distortion
    }
    fn direct_lightmap_enable(&self) -> &bool {
        &self.direct_lightmap_enable
    }
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn attenuation_offset(&self) -> &f32 {
        &self.attenuation_offset
    }
    fn direct_light_enable(&self) -> &bool {
        &self.direct_light_enable
    }
    fn specular_enable(&self) -> &bool {
        &self.specular_enable
    }
    fn enlighten_color_mode(&self) -> &EnlightenColorMode {
        &self.enlighten_color_mode
    }
    fn enlighten_enable(&self) -> &bool {
        &self.enlighten_enable
    }
    fn enlighten_color_scale(&self) -> &super::core::Vec3 {
        &self.enlighten_color_scale
    }
    fn particle_color_scale(&self) -> &super::core::Vec3 {
        &self.particle_color_scale
    }
    fn exposure_compensation(&self) -> &f32 {
        &self.exposure_compensation
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn intensity_multiplier(&self) -> &f32 {
        &self.intensity_multiplier
    }
    fn shadow_cache_enable(&self) -> &bool {
        &self.shadow_cache_enable
    }
    fn shadow_cache_update_priority(&self) -> &f32 {
        &self.shadow_cache_update_priority
    }
    fn shadow_cache_update_counter(&self) -> &u32 {
        &self.shadow_cache_update_counter
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static OLDPOINTLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldPointLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OldPointLightDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, width),
            },
            FieldInfoData {
                name: "TranslucencyAmbient",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, translucency_ambient),
            },
            FieldInfoData {
                name: "TranslucencyScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, translucency_scale),
            },
            FieldInfoData {
                name: "TranslucencyPower",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, translucency_power),
            },
            FieldInfoData {
                name: "TranslucencyDistortion",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, translucency_distortion),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldPointLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OldPointLightDynamicState, color),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, radius),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "DirectLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldPointLightDynamicState, direct_light_enable),
            },
            FieldInfoData {
                name: "SpecularEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldPointLightDynamicState, specular_enable),
            },
            FieldInfoData {
                name: "EnlightenColorMode",
                flags: MemberInfoFlags::new(0),
                field_type: "EnlightenColorMode",
                rust_offset: offset_of!(OldPointLightDynamicState, enlighten_color_mode),
            },
            FieldInfoData {
                name: "EnlightenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldPointLightDynamicState, enlighten_enable),
            },
            FieldInfoData {
                name: "EnlightenColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OldPointLightDynamicState, enlighten_color_scale),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OldPointLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldPointLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldPointLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldPointLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldPointLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OldPointLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(OldPointLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(OldPointLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OldPointLightDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OLDPOINTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OldPointLightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        OLDPOINTLIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OLDPOINTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldPointLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OldPointLightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OldPointLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub trait OldPointLightStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl OldPointLightStaticStateTrait for OldPointLightStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static OLDPOINTLIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldPointLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OldPointLightStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(OldPointLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OldPointLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OLDPOINTLIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for OldPointLightStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        OLDPOINTLIGHTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OLDPOINTLIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldPointLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OldPointLightStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OldLightDynamicState {
    pub color: super::core::Vec3,
    pub radius: f32,
    pub intensity: f32,
    pub attenuation_offset: f32,
    pub direct_light_enable: bool,
    pub specular_enable: bool,
    pub enlighten_color_mode: EnlightenColorMode,
    pub enlighten_enable: bool,
    pub enlighten_color_scale: super::core::Vec3,
    pub particle_color_scale: super::core::Vec3,
    pub exposure_compensation: f32,
    pub is_first_person: bool,
    pub is_enabled: bool,
    pub intensity_multiplier: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
}

pub trait OldLightDynamicStateTrait: TypeObject {
    fn color(&self) -> &super::core::Vec3;
    fn radius(&self) -> &f32;
    fn intensity(&self) -> &f32;
    fn attenuation_offset(&self) -> &f32;
    fn direct_light_enable(&self) -> &bool;
    fn specular_enable(&self) -> &bool;
    fn enlighten_color_mode(&self) -> &EnlightenColorMode;
    fn enlighten_enable(&self) -> &bool;
    fn enlighten_color_scale(&self) -> &super::core::Vec3;
    fn particle_color_scale(&self) -> &super::core::Vec3;
    fn exposure_compensation(&self) -> &f32;
    fn is_first_person(&self) -> &bool;
    fn is_enabled(&self) -> &bool;
    fn intensity_multiplier(&self) -> &f32;
    fn shadow_cache_enable(&self) -> &bool;
    fn shadow_cache_update_priority(&self) -> &f32;
    fn shadow_cache_update_counter(&self) -> &u32;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
}

impl OldLightDynamicStateTrait for OldLightDynamicState {
    fn color(&self) -> &super::core::Vec3 {
        &self.color
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn intensity(&self) -> &f32 {
        &self.intensity
    }
    fn attenuation_offset(&self) -> &f32 {
        &self.attenuation_offset
    }
    fn direct_light_enable(&self) -> &bool {
        &self.direct_light_enable
    }
    fn specular_enable(&self) -> &bool {
        &self.specular_enable
    }
    fn enlighten_color_mode(&self) -> &EnlightenColorMode {
        &self.enlighten_color_mode
    }
    fn enlighten_enable(&self) -> &bool {
        &self.enlighten_enable
    }
    fn enlighten_color_scale(&self) -> &super::core::Vec3 {
        &self.enlighten_color_scale
    }
    fn particle_color_scale(&self) -> &super::core::Vec3 {
        &self.particle_color_scale
    }
    fn exposure_compensation(&self) -> &f32 {
        &self.exposure_compensation
    }
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn intensity_multiplier(&self) -> &f32 {
        &self.intensity_multiplier
    }
    fn shadow_cache_enable(&self) -> &bool {
        &self.shadow_cache_enable
    }
    fn shadow_cache_update_priority(&self) -> &f32 {
        &self.shadow_cache_update_priority
    }
    fn shadow_cache_update_counter(&self) -> &u32 {
        &self.shadow_cache_update_counter
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
}

pub static OLDLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OldLightDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OldLightDynamicState, color),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldLightDynamicState, radius),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "DirectLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldLightDynamicState, direct_light_enable),
            },
            FieldInfoData {
                name: "SpecularEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldLightDynamicState, specular_enable),
            },
            FieldInfoData {
                name: "EnlightenColorMode",
                flags: MemberInfoFlags::new(0),
                field_type: "EnlightenColorMode",
                rust_offset: offset_of!(OldLightDynamicState, enlighten_color_mode),
            },
            FieldInfoData {
                name: "EnlightenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldLightDynamicState, enlighten_enable),
            },
            FieldInfoData {
                name: "EnlightenColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OldLightDynamicState, enlighten_color_scale),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(OldLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OldLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OldLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OldLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(OldLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(OldLightDynamicState, excluded_cull_id),
            },
        ],
    }),
    array_type: Some(OLDLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OldLightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        OLDLIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OLDLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OldLightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightDynamicState {
    pub is_first_person: bool,
    pub is_enabled: bool,
    pub intensity_multiplier: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
}

pub trait LightDynamicStateTrait: TypeObject {
    fn is_first_person(&self) -> &bool;
    fn is_enabled(&self) -> &bool;
    fn intensity_multiplier(&self) -> &f32;
    fn shadow_cache_enable(&self) -> &bool;
    fn shadow_cache_update_priority(&self) -> &f32;
    fn shadow_cache_update_counter(&self) -> &u32;
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle;
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle;
}

impl LightDynamicStateTrait for LightDynamicState {
    fn is_first_person(&self) -> &bool {
        &self.is_first_person
    }
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn intensity_multiplier(&self) -> &f32 {
        &self.intensity_multiplier
    }
    fn shadow_cache_enable(&self) -> &bool {
        &self.shadow_cache_enable
    }
    fn shadow_cache_update_priority(&self) -> &f32 {
        &self.shadow_cache_update_priority
    }
    fn shadow_cache_update_counter(&self) -> &u32 {
        &self.shadow_cache_update_counter
    }
    fn included_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.included_cull_id
    }
    fn excluded_cull_id(&self) -> &super::render_base::CullIdHandle {
        &self.excluded_cull_id
    }
}

pub static LIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(LightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: "CullIdHandle",
                rust_offset: offset_of!(LightDynamicState, excluded_cull_id),
            },
        ],
    }),
    array_type: Some(LIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LightDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LightDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
}

pub trait LightStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
}

impl LightStaticStateTrait for LightStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
}

pub static LIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LightStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(LightStaticState, transform_space),
            },
        ],
    }),
    array_type: Some(LIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LightStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LightStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IesProfileAsset {
    pub _glacier_base: super::core::Asset,
    pub source_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait IesProfileAssetTrait: super::core::AssetTrait {
    fn source_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl IesProfileAssetTrait for IesProfileAsset {
    fn source_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.source_resource
    }
}

impl super::core::AssetTrait for IesProfileAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for IesProfileAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static IESPROFILEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IesProfileAsset",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IesProfileAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SourceResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(IesProfileAsset, source_resource),
            },
        ],
    }),
    array_type: Some(IESPROFILEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IesProfileAsset {
    fn type_info(&self) -> &'static TypeInfo {
        IESPROFILEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IESPROFILEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IesProfileAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("IesProfileAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SpotLightShape {
    #[default]
    SpotLightShape_Cone = 0,
    SpotLightShape_Frustum = 1,
    SpotLightShape_OrthoFrustum = 2,
}

pub static SPOTLIGHTSHAPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpotLightShape",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SPOTLIGHTSHAPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SpotLightShape {
    fn type_info(&self) -> &'static TypeInfo {
        SPOTLIGHTSHAPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPOTLIGHTSHAPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpotLightShape-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SpotLightShape"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EnlightenColorMode {
    #[default]
    EnlightenColorMode_Multiply = 0,
    EnlightenColorMode_Override = 1,
}

pub static ENLIGHTENCOLORMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenColorMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(ENLIGHTENCOLORMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EnlightenColorMode {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENCOLORMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENLIGHTENCOLORMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenColorMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EnlightenColorMode"),
    array_type: None,
    alignment: 8,
};



pub static SETLIGHTTRANSFORM_LIGHTHANDLE_LINEARTRANSFORM__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetLightTransform(LightHandle,LinearTransform)",
    flags: MemberInfoFlags::new(793),
    module: "WorldBase",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static CREATELIGHT_LIGHTHANDLE_LINEARTRANSFORM_VEC3__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreateLight(LightHandle,LinearTransform,Vec3)",
    flags: MemberInfoFlags::new(793),
    module: "WorldBase",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LensFlareCreateState {
    pub transform: super::core::LinearTransform,
    pub is_world_space: bool,
    pub parent_transform_space: super::state_stream::TransformSpaceHandle,
}

pub trait LensFlareCreateStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn is_world_space(&self) -> &bool;
    fn parent_transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
}

impl LensFlareCreateStateTrait for LensFlareCreateState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn is_world_space(&self) -> &bool {
        &self.is_world_space
    }
    fn parent_transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.parent_transform_space
    }
}

pub static LENSFLARECREATESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareCreateState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LensFlareCreateState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(LensFlareCreateState, transform),
            },
            FieldInfoData {
                name: "IsWorldSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LensFlareCreateState, is_world_space),
            },
            FieldInfoData {
                name: "ParentTransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(LensFlareCreateState, parent_transform_space),
            },
        ],
    }),
    array_type: Some(LENSFLARECREATESTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensFlareCreateState {
    fn type_info(&self) -> &'static TypeInfo {
        LENSFLARECREATESTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LENSFLARECREATESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareCreateState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LensFlareCreateState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LensFlareDynamicState {
    pub flare_direction_mode: FlareDirectionMode,
    pub visible: bool,
    pub dimmer: f32,
    pub field_flag_changed0: u8,
}

pub trait LensFlareDynamicStateTrait: TypeObject {
    fn flare_direction_mode(&self) -> &FlareDirectionMode;
    fn visible(&self) -> &bool;
    fn dimmer(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u8;
}

impl LensFlareDynamicStateTrait for LensFlareDynamicState {
    fn flare_direction_mode(&self) -> &FlareDirectionMode {
        &self.flare_direction_mode
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn dimmer(&self) -> &f32 {
        &self.dimmer
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static LENSFLAREDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LensFlareDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FlareDirectionMode",
                flags: MemberInfoFlags::new(0),
                field_type: "FlareDirectionMode",
                rust_offset: offset_of!(LensFlareDynamicState, flare_direction_mode),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LensFlareDynamicState, visible),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensFlareDynamicState, dimmer),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(LensFlareDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LENSFLAREDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LensFlareDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        LENSFLAREDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LENSFLAREDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LensFlareDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LensFlareStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub debug_draw_occluder: bool,
    pub half_res: bool,
    pub occluder_size: f32,
    pub render_mode: LensFlareRenderMode,
    pub screen_clip: bool,
    pub depth_bias: f32,
    pub elements: Vec<LensFlareElement>,
    pub field_flag_changed0: u8,
}

pub trait LensFlareStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn debug_draw_occluder(&self) -> &bool;
    fn half_res(&self) -> &bool;
    fn occluder_size(&self) -> &f32;
    fn render_mode(&self) -> &LensFlareRenderMode;
    fn screen_clip(&self) -> &bool;
    fn depth_bias(&self) -> &f32;
    fn elements(&self) -> &Vec<LensFlareElement>;
    fn field_flag_changed0(&self) -> &u8;
}

impl LensFlareStaticStateTrait for LensFlareStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn debug_draw_occluder(&self) -> &bool {
        &self.debug_draw_occluder
    }
    fn half_res(&self) -> &bool {
        &self.half_res
    }
    fn occluder_size(&self) -> &f32 {
        &self.occluder_size
    }
    fn render_mode(&self) -> &LensFlareRenderMode {
        &self.render_mode
    }
    fn screen_clip(&self) -> &bool {
        &self.screen_clip
    }
    fn depth_bias(&self) -> &f32 {
        &self.depth_bias
    }
    fn elements(&self) -> &Vec<LensFlareElement> {
        &self.elements
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static LENSFLARESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LensFlareStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(LensFlareStaticState, transform_space),
            },
            FieldInfoData {
                name: "DebugDrawOccluder",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LensFlareStaticState, debug_draw_occluder),
            },
            FieldInfoData {
                name: "HalfRes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LensFlareStaticState, half_res),
            },
            FieldInfoData {
                name: "OccluderSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensFlareStaticState, occluder_size),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: "LensFlareRenderMode",
                rust_offset: offset_of!(LensFlareStaticState, render_mode),
            },
            FieldInfoData {
                name: "ScreenClip",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LensFlareStaticState, screen_clip),
            },
            FieldInfoData {
                name: "DepthBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensFlareStaticState, depth_bias),
            },
            FieldInfoData {
                name: "Elements",
                flags: MemberInfoFlags::new(144),
                field_type: "LensFlareElement-Array",
                rust_offset: offset_of!(LensFlareStaticState, elements),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(LensFlareStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LENSFLARESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LensFlareStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        LENSFLARESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LENSFLARESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LensFlareStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LensFlareRenderMode {
    #[default]
    LensFlareRenderMode_Half_Res = 0,
    LensFlareRenderMode_Full_Res = 1,
    LensFlareRenderMode_Foreground = 2,
    LensFlareRenderModeCount = 3,
}

pub static LENSFLARERENDERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareRenderMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(LENSFLARERENDERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LensFlareRenderMode {
    fn type_info(&self) -> &'static TypeInfo {
        LENSFLARERENDERMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LENSFLARERENDERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareRenderMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LensFlareRenderMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum FlareDirectionMode {
    #[default]
    FlareDirectionMode_Entity = 0,
    FlareDirectionMode_Sun = 1,
}

pub static FLAREDIRECTIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlareDirectionMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(FLAREDIRECTIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FlareDirectionMode {
    fn type_info(&self) -> &'static TypeInfo {
        FLAREDIRECTIONMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLAREDIRECTIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlareDirectionMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FlareDirectionMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LensFlareElement {
    pub enable: super::core::QualityScalableEnabled,
    pub enable_element: bool,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub ray_distance: f32,
    pub ray_distance_x: f32,
    pub ray_distance_y: f32,
    pub center_offset: super::core::Vec2,
    pub size: super::core::Vec2,
    pub size_occluder_curve: super::core::Vec4,
    pub size_screen_pos_curve: super::core::Vec4,
    pub size_angle_curve: super::core::Vec4,
    pub size_cam_dist_curve: super::core::Vec4,
    pub size_cam_dist_max: f32,
    pub alpha_occluder_curve: super::core::Vec4,
    pub alpha_screen_pos_curve: super::core::Vec4,
    pub alpha_angle_curve: super::core::Vec4,
    pub alpha_cam_dist_curve: super::core::Vec4,
    pub alpha_cam_dist_max: f32,
    pub rotation_local: f32,
    pub rotation_aligned_to_ray: bool,
    pub rotation_dist_curve: super::core::Vec4,
    pub rotation_dist_multiplier: f32,
}

pub trait LensFlareElementTrait: TypeObject {
    fn enable(&self) -> &super::core::QualityScalableEnabled;
    fn enable_element(&self) -> &bool;
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn ray_distance(&self) -> &f32;
    fn ray_distance_x(&self) -> &f32;
    fn ray_distance_y(&self) -> &f32;
    fn center_offset(&self) -> &super::core::Vec2;
    fn size(&self) -> &super::core::Vec2;
    fn size_occluder_curve(&self) -> &super::core::Vec4;
    fn size_screen_pos_curve(&self) -> &super::core::Vec4;
    fn size_angle_curve(&self) -> &super::core::Vec4;
    fn size_cam_dist_curve(&self) -> &super::core::Vec4;
    fn size_cam_dist_max(&self) -> &f32;
    fn alpha_occluder_curve(&self) -> &super::core::Vec4;
    fn alpha_screen_pos_curve(&self) -> &super::core::Vec4;
    fn alpha_angle_curve(&self) -> &super::core::Vec4;
    fn alpha_cam_dist_curve(&self) -> &super::core::Vec4;
    fn alpha_cam_dist_max(&self) -> &f32;
    fn rotation_local(&self) -> &f32;
    fn rotation_aligned_to_ray(&self) -> &bool;
    fn rotation_dist_curve(&self) -> &super::core::Vec4;
    fn rotation_dist_multiplier(&self) -> &f32;
}

impl LensFlareElementTrait for LensFlareElement {
    fn enable(&self) -> &super::core::QualityScalableEnabled {
        &self.enable
    }
    fn enable_element(&self) -> &bool {
        &self.enable_element
    }
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn ray_distance(&self) -> &f32 {
        &self.ray_distance
    }
    fn ray_distance_x(&self) -> &f32 {
        &self.ray_distance_x
    }
    fn ray_distance_y(&self) -> &f32 {
        &self.ray_distance_y
    }
    fn center_offset(&self) -> &super::core::Vec2 {
        &self.center_offset
    }
    fn size(&self) -> &super::core::Vec2 {
        &self.size
    }
    fn size_occluder_curve(&self) -> &super::core::Vec4 {
        &self.size_occluder_curve
    }
    fn size_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.size_screen_pos_curve
    }
    fn size_angle_curve(&self) -> &super::core::Vec4 {
        &self.size_angle_curve
    }
    fn size_cam_dist_curve(&self) -> &super::core::Vec4 {
        &self.size_cam_dist_curve
    }
    fn size_cam_dist_max(&self) -> &f32 {
        &self.size_cam_dist_max
    }
    fn alpha_occluder_curve(&self) -> &super::core::Vec4 {
        &self.alpha_occluder_curve
    }
    fn alpha_screen_pos_curve(&self) -> &super::core::Vec4 {
        &self.alpha_screen_pos_curve
    }
    fn alpha_angle_curve(&self) -> &super::core::Vec4 {
        &self.alpha_angle_curve
    }
    fn alpha_cam_dist_curve(&self) -> &super::core::Vec4 {
        &self.alpha_cam_dist_curve
    }
    fn alpha_cam_dist_max(&self) -> &f32 {
        &self.alpha_cam_dist_max
    }
    fn rotation_local(&self) -> &f32 {
        &self.rotation_local
    }
    fn rotation_aligned_to_ray(&self) -> &bool {
        &self.rotation_aligned_to_ray
    }
    fn rotation_dist_curve(&self) -> &super::core::Vec4 {
        &self.rotation_dist_curve
    }
    fn rotation_dist_multiplier(&self) -> &f32 {
        &self.rotation_dist_multiplier
    }
}

pub static LENSFLAREELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareElement",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LensFlareElement as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableEnabled",
                rust_offset: offset_of!(LensFlareElement, enable),
            },
            FieldInfoData {
                name: "EnableElement",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LensFlareElement, enable_element),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(LensFlareElement, shader),
            },
            FieldInfoData {
                name: "RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensFlareElement, ray_distance),
            },
            FieldInfoData {
                name: "RayDistanceX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensFlareElement, ray_distance_x),
            },
            FieldInfoData {
                name: "RayDistanceY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensFlareElement, ray_distance_y),
            },
            FieldInfoData {
                name: "CenterOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(LensFlareElement, center_offset),
            },
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(LensFlareElement, size),
            },
            FieldInfoData {
                name: "SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(LensFlareElement, size_occluder_curve),
            },
            FieldInfoData {
                name: "SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(LensFlareElement, size_screen_pos_curve),
            },
            FieldInfoData {
                name: "SizeAngleCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(LensFlareElement, size_angle_curve),
            },
            FieldInfoData {
                name: "SizeCamDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(LensFlareElement, size_cam_dist_curve),
            },
            FieldInfoData {
                name: "SizeCamDistMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensFlareElement, size_cam_dist_max),
            },
            FieldInfoData {
                name: "AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(LensFlareElement, alpha_occluder_curve),
            },
            FieldInfoData {
                name: "AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(LensFlareElement, alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "AlphaAngleCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(LensFlareElement, alpha_angle_curve),
            },
            FieldInfoData {
                name: "AlphaCamDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(LensFlareElement, alpha_cam_dist_curve),
            },
            FieldInfoData {
                name: "AlphaCamDistMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensFlareElement, alpha_cam_dist_max),
            },
            FieldInfoData {
                name: "RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensFlareElement, rotation_local),
            },
            FieldInfoData {
                name: "RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LensFlareElement, rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(LensFlareElement, rotation_dist_curve),
            },
            FieldInfoData {
                name: "RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LensFlareElement, rotation_dist_multiplier),
            },
        ],
    }),
    array_type: Some(LENSFLAREELEMENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensFlareElement {
    fn type_info(&self) -> &'static TypeInfo {
        LENSFLAREELEMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LENSFLAREELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LensFlareElement"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ReflectionVolumeQueryDynamicState {
    pub query_counter: u32,
    pub field_flag_changed0: u8,
}

pub trait ReflectionVolumeQueryDynamicStateTrait: TypeObject {
    fn query_counter(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u8;
}

impl ReflectionVolumeQueryDynamicStateTrait for ReflectionVolumeQueryDynamicState {
    fn query_counter(&self) -> &u32 {
        &self.query_counter
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static REFLECTIONVOLUMEQUERYDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeQueryDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ReflectionVolumeQueryDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "QueryCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReflectionVolumeQueryDynamicState, query_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(ReflectionVolumeQueryDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(REFLECTIONVOLUMEQUERYDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ReflectionVolumeQueryDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        REFLECTIONVOLUMEQUERYDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static REFLECTIONVOLUMEQUERYDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeQueryDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ReflectionVolumeQueryDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ReflectionVolumeQueryHandle {
}

pub trait ReflectionVolumeQueryHandleTrait: TypeObject {
}

impl ReflectionVolumeQueryHandleTrait for ReflectionVolumeQueryHandle {
}

pub static REFLECTIONVOLUMEQUERYHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeQueryHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ReflectionVolumeQueryHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(REFLECTIONVOLUMEQUERYHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for ReflectionVolumeQueryHandle {
    fn type_info(&self) -> &'static TypeInfo {
        REFLECTIONVOLUMEQUERYHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static REFLECTIONVOLUMEQUERYHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeQueryHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ReflectionVolumeQueryHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalPlanarReflectionDynamicState {
    pub transform: super::core::LinearTransform,
    pub enable: bool,
    pub terrain_reflections_enable: bool,
    pub sky_reflection_enable: bool,
    pub field_flag_changed0: u8,
}

pub trait LocalPlanarReflectionDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn enable(&self) -> &bool;
    fn terrain_reflections_enable(&self) -> &bool;
    fn sky_reflection_enable(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl LocalPlanarReflectionDynamicStateTrait for LocalPlanarReflectionDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn terrain_reflections_enable(&self) -> &bool {
        &self.terrain_reflections_enable
    }
    fn sky_reflection_enable(&self) -> &bool {
        &self.sky_reflection_enable
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static LOCALPLANARREFLECTIONDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalPlanarReflectionDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(LocalPlanarReflectionDynamicState, transform),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LocalPlanarReflectionDynamicState, enable),
            },
            FieldInfoData {
                name: "TerrainReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LocalPlanarReflectionDynamicState, terrain_reflections_enable),
            },
            FieldInfoData {
                name: "SkyReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LocalPlanarReflectionDynamicState, sky_reflection_enable),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(LocalPlanarReflectionDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LOCALPLANARREFLECTIONDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalPlanarReflectionDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALPLANARREFLECTIONDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALPLANARREFLECTIONDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LocalPlanarReflectionDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalPlanarReflectionStaticState {
    pub guid: glacier_util::guid::Guid,
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub far_plane: f32,
    pub clipping_offset: f32,
    pub clipping_enable: bool,
    pub distance_tolerance: f32,
    pub distance_falloff: f32,
    pub normal_tolerance: f32,
    pub normal_falloff: f32,
    pub field_flag_changed0: u16,
}

pub trait LocalPlanarReflectionStaticStateTrait: TypeObject {
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn far_plane(&self) -> &f32;
    fn clipping_offset(&self) -> &f32;
    fn clipping_enable(&self) -> &bool;
    fn distance_tolerance(&self) -> &f32;
    fn distance_falloff(&self) -> &f32;
    fn normal_tolerance(&self) -> &f32;
    fn normal_falloff(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u16;
}

impl LocalPlanarReflectionStaticStateTrait for LocalPlanarReflectionStaticState {
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn far_plane(&self) -> &f32 {
        &self.far_plane
    }
    fn clipping_offset(&self) -> &f32 {
        &self.clipping_offset
    }
    fn clipping_enable(&self) -> &bool {
        &self.clipping_enable
    }
    fn distance_tolerance(&self) -> &f32 {
        &self.distance_tolerance
    }
    fn distance_falloff(&self) -> &f32 {
        &self.distance_falloff
    }
    fn normal_tolerance(&self) -> &f32 {
        &self.normal_tolerance
    }
    fn normal_falloff(&self) -> &f32 {
        &self.normal_falloff
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static LOCALPLANARREFLECTIONSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalPlanarReflectionStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, guid),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, transform_space),
            },
            FieldInfoData {
                name: "FarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, far_plane),
            },
            FieldInfoData {
                name: "ClippingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, clipping_offset),
            },
            FieldInfoData {
                name: "ClippingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, clipping_enable),
            },
            FieldInfoData {
                name: "DistanceTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, distance_tolerance),
            },
            FieldInfoData {
                name: "DistanceFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, distance_falloff),
            },
            FieldInfoData {
                name: "NormalTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, normal_tolerance),
            },
            FieldInfoData {
                name: "NormalFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, normal_falloff),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LOCALPLANARREFLECTIONSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LocalPlanarReflectionStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALPLANARREFLECTIONSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALPLANARREFLECTIONSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LocalPlanarReflectionStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DistantIBLDynamicState {
    pub enabled: bool,
    pub is_visible: bool,
    pub refresh_counter: u32,
    pub field_flag_changed0: u8,
}

pub trait DistantIBLDynamicStateTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn is_visible(&self) -> &bool;
    fn refresh_counter(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u8;
}

impl DistantIBLDynamicStateTrait for DistantIBLDynamicState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn is_visible(&self) -> &bool {
        &self.is_visible
    }
    fn refresh_counter(&self) -> &u32 {
        &self.refresh_counter
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static DISTANTIBLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistantIBLDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantIBLDynamicState, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantIBLDynamicState, is_visible),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DistantIBLDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DistantIBLDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DISTANTIBLDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DistantIBLDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANTIBLDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISTANTIBLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DistantIBLDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DistantIBLStaticState {
    pub location_type: super::render_base::DistantIBLLocationType,
    pub local_offset: super::core::Vec3,
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub guid: glacier_util::guid::Guid,
    pub mode: super::render_base::LocalIBLMode,
    pub capture_distance: f32,
    pub capture_fade_distance: f32,
    pub influence_expand_distance: f32,
    pub influence_fade_distance: f32,
    pub update_when_moving: bool,
    pub capture_sky: bool,
    pub capture_sky_mask: bool,
    pub use_sky_visibility_as_a_o: bool,
    pub use_sky_visibility_as_mask: bool,
    pub sharpen_sky_visibility: f32,
    pub bias_sky_visibility: f32,
    pub use_proxy_reprojection: bool,
    pub capture_fog: bool,
    pub object_layers: u16,
    pub do_not_update_baked_texture: bool,
    pub baked_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub field_flag_changed0: u32,
}

pub trait DistantIBLStaticStateTrait: TypeObject {
    fn location_type(&self) -> &super::render_base::DistantIBLLocationType;
    fn local_offset(&self) -> &super::core::Vec3;
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn mode(&self) -> &super::render_base::LocalIBLMode;
    fn capture_distance(&self) -> &f32;
    fn capture_fade_distance(&self) -> &f32;
    fn influence_expand_distance(&self) -> &f32;
    fn influence_fade_distance(&self) -> &f32;
    fn update_when_moving(&self) -> &bool;
    fn capture_sky(&self) -> &bool;
    fn capture_sky_mask(&self) -> &bool;
    fn use_sky_visibility_as_a_o(&self) -> &bool;
    fn use_sky_visibility_as_mask(&self) -> &bool;
    fn sharpen_sky_visibility(&self) -> &f32;
    fn bias_sky_visibility(&self) -> &f32;
    fn use_proxy_reprojection(&self) -> &bool;
    fn capture_fog(&self) -> &bool;
    fn object_layers(&self) -> &u16;
    fn do_not_update_baked_texture(&self) -> &bool;
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn field_flag_changed0(&self) -> &u32;
}

impl DistantIBLStaticStateTrait for DistantIBLStaticState {
    fn location_type(&self) -> &super::render_base::DistantIBLLocationType {
        &self.location_type
    }
    fn local_offset(&self) -> &super::core::Vec3 {
        &self.local_offset
    }
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn mode(&self) -> &super::render_base::LocalIBLMode {
        &self.mode
    }
    fn capture_distance(&self) -> &f32 {
        &self.capture_distance
    }
    fn capture_fade_distance(&self) -> &f32 {
        &self.capture_fade_distance
    }
    fn influence_expand_distance(&self) -> &f32 {
        &self.influence_expand_distance
    }
    fn influence_fade_distance(&self) -> &f32 {
        &self.influence_fade_distance
    }
    fn update_when_moving(&self) -> &bool {
        &self.update_when_moving
    }
    fn capture_sky(&self) -> &bool {
        &self.capture_sky
    }
    fn capture_sky_mask(&self) -> &bool {
        &self.capture_sky_mask
    }
    fn use_sky_visibility_as_a_o(&self) -> &bool {
        &self.use_sky_visibility_as_a_o
    }
    fn use_sky_visibility_as_mask(&self) -> &bool {
        &self.use_sky_visibility_as_mask
    }
    fn sharpen_sky_visibility(&self) -> &f32 {
        &self.sharpen_sky_visibility
    }
    fn bias_sky_visibility(&self) -> &f32 {
        &self.bias_sky_visibility
    }
    fn use_proxy_reprojection(&self) -> &bool {
        &self.use_proxy_reprojection
    }
    fn capture_fog(&self) -> &bool {
        &self.capture_fog
    }
    fn object_layers(&self) -> &u16 {
        &self.object_layers
    }
    fn do_not_update_baked_texture(&self) -> &bool {
        &self.do_not_update_baked_texture
    }
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.baked_texture
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static DISTANTIBLSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistantIBLStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LocationType",
                flags: MemberInfoFlags::new(0),
                field_type: "DistantIBLLocationType",
                rust_offset: offset_of!(DistantIBLStaticState, location_type),
            },
            FieldInfoData {
                name: "LocalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DistantIBLStaticState, local_offset),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(DistantIBLStaticState, transform_space),
            },
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(DistantIBLStaticState, guid),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalIBLMode",
                rust_offset: offset_of!(DistantIBLStaticState, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DistantIBLStaticState, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DistantIBLStaticState, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DistantIBLStaticState, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DistantIBLStaticState, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantIBLStaticState, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantIBLStaticState, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantIBLStaticState, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantIBLStaticState, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantIBLStaticState, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DistantIBLStaticState, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DistantIBLStaticState, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantIBLStaticState, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantIBLStaticState, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(DistantIBLStaticState, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantIBLStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(DistantIBLStaticState, baked_texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DistantIBLStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DISTANTIBLSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DistantIBLStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANTIBLSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISTANTIBLSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DistantIBLStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoxIBLDynamicState {
    pub enabled: bool,
    pub is_visible: bool,
    pub refresh_counter: u32,
    pub field_flag_changed0: u8,
}

pub trait BoxIBLDynamicStateTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn is_visible(&self) -> &bool;
    fn refresh_counter(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u8;
}

impl BoxIBLDynamicStateTrait for BoxIBLDynamicState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn is_visible(&self) -> &bool {
        &self.is_visible
    }
    fn refresh_counter(&self) -> &u32 {
        &self.refresh_counter
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static BOXIBLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxIBLDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoxIBLDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoxIBLDynamicState, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoxIBLDynamicState, is_visible),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(BoxIBLDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(BoxIBLDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(BOXIBLDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BoxIBLDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        BOXIBLDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BOXIBLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxIBLDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("BoxIBLDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoxIBLStaticState {
    pub influence_fade_normal: super::core::Vec3,
    pub side_fade_pos_x: f32,
    pub side_fade_neg_x: f32,
    pub side_fade_pos_y: f32,
    pub side_fade_neg_y: f32,
    pub side_fade_pos_z: f32,
    pub side_fade_neg_z: f32,
    pub local_offset: super::core::Vec3,
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub guid: glacier_util::guid::Guid,
    pub mode: super::render_base::LocalIBLMode,
    pub capture_distance: f32,
    pub capture_fade_distance: f32,
    pub influence_expand_distance: f32,
    pub influence_fade_distance: f32,
    pub update_when_moving: bool,
    pub capture_sky: bool,
    pub capture_sky_mask: bool,
    pub use_sky_visibility_as_a_o: bool,
    pub use_sky_visibility_as_mask: bool,
    pub sharpen_sky_visibility: f32,
    pub bias_sky_visibility: f32,
    pub use_proxy_reprojection: bool,
    pub capture_fog: bool,
    pub object_layers: u16,
    pub do_not_update_baked_texture: bool,
    pub baked_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub field_flag_changed0: u32,
}

pub trait BoxIBLStaticStateTrait: TypeObject {
    fn influence_fade_normal(&self) -> &super::core::Vec3;
    fn side_fade_pos_x(&self) -> &f32;
    fn side_fade_neg_x(&self) -> &f32;
    fn side_fade_pos_y(&self) -> &f32;
    fn side_fade_neg_y(&self) -> &f32;
    fn side_fade_pos_z(&self) -> &f32;
    fn side_fade_neg_z(&self) -> &f32;
    fn local_offset(&self) -> &super::core::Vec3;
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn mode(&self) -> &super::render_base::LocalIBLMode;
    fn capture_distance(&self) -> &f32;
    fn capture_fade_distance(&self) -> &f32;
    fn influence_expand_distance(&self) -> &f32;
    fn influence_fade_distance(&self) -> &f32;
    fn update_when_moving(&self) -> &bool;
    fn capture_sky(&self) -> &bool;
    fn capture_sky_mask(&self) -> &bool;
    fn use_sky_visibility_as_a_o(&self) -> &bool;
    fn use_sky_visibility_as_mask(&self) -> &bool;
    fn sharpen_sky_visibility(&self) -> &f32;
    fn bias_sky_visibility(&self) -> &f32;
    fn use_proxy_reprojection(&self) -> &bool;
    fn capture_fog(&self) -> &bool;
    fn object_layers(&self) -> &u16;
    fn do_not_update_baked_texture(&self) -> &bool;
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn field_flag_changed0(&self) -> &u32;
}

impl BoxIBLStaticStateTrait for BoxIBLStaticState {
    fn influence_fade_normal(&self) -> &super::core::Vec3 {
        &self.influence_fade_normal
    }
    fn side_fade_pos_x(&self) -> &f32 {
        &self.side_fade_pos_x
    }
    fn side_fade_neg_x(&self) -> &f32 {
        &self.side_fade_neg_x
    }
    fn side_fade_pos_y(&self) -> &f32 {
        &self.side_fade_pos_y
    }
    fn side_fade_neg_y(&self) -> &f32 {
        &self.side_fade_neg_y
    }
    fn side_fade_pos_z(&self) -> &f32 {
        &self.side_fade_pos_z
    }
    fn side_fade_neg_z(&self) -> &f32 {
        &self.side_fade_neg_z
    }
    fn local_offset(&self) -> &super::core::Vec3 {
        &self.local_offset
    }
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn mode(&self) -> &super::render_base::LocalIBLMode {
        &self.mode
    }
    fn capture_distance(&self) -> &f32 {
        &self.capture_distance
    }
    fn capture_fade_distance(&self) -> &f32 {
        &self.capture_fade_distance
    }
    fn influence_expand_distance(&self) -> &f32 {
        &self.influence_expand_distance
    }
    fn influence_fade_distance(&self) -> &f32 {
        &self.influence_fade_distance
    }
    fn update_when_moving(&self) -> &bool {
        &self.update_when_moving
    }
    fn capture_sky(&self) -> &bool {
        &self.capture_sky
    }
    fn capture_sky_mask(&self) -> &bool {
        &self.capture_sky_mask
    }
    fn use_sky_visibility_as_a_o(&self) -> &bool {
        &self.use_sky_visibility_as_a_o
    }
    fn use_sky_visibility_as_mask(&self) -> &bool {
        &self.use_sky_visibility_as_mask
    }
    fn sharpen_sky_visibility(&self) -> &f32 {
        &self.sharpen_sky_visibility
    }
    fn bias_sky_visibility(&self) -> &f32 {
        &self.bias_sky_visibility
    }
    fn use_proxy_reprojection(&self) -> &bool {
        &self.use_proxy_reprojection
    }
    fn capture_fog(&self) -> &bool {
        &self.capture_fog
    }
    fn object_layers(&self) -> &u16 {
        &self.object_layers
    }
    fn do_not_update_baked_texture(&self) -> &bool {
        &self.do_not_update_baked_texture
    }
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.baked_texture
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static BOXIBLSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxIBLStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoxIBLStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InfluenceFadeNormal",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(BoxIBLStaticState, influence_fade_normal),
            },
            FieldInfoData {
                name: "SideFadePosX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_pos_x),
            },
            FieldInfoData {
                name: "SideFadeNegX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_neg_x),
            },
            FieldInfoData {
                name: "SideFadePosY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_pos_y),
            },
            FieldInfoData {
                name: "SideFadeNegY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_neg_y),
            },
            FieldInfoData {
                name: "SideFadePosZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_pos_z),
            },
            FieldInfoData {
                name: "SideFadeNegZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_neg_z),
            },
            FieldInfoData {
                name: "LocalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(BoxIBLStaticState, local_offset),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(BoxIBLStaticState, transform_space),
            },
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(BoxIBLStaticState, guid),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalIBLMode",
                rust_offset: offset_of!(BoxIBLStaticState, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoxIBLStaticState, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoxIBLStaticState, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoxIBLStaticState, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoxIBLStaticState, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoxIBLStaticState, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoxIBLStaticState, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoxIBLStaticState, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoxIBLStaticState, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(BoxIBLStaticState, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoxIBLStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(BoxIBLStaticState, baked_texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(BoxIBLStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(BOXIBLSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoxIBLStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        BOXIBLSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BOXIBLSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxIBLStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("BoxIBLStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SphereIBLDynamicState {
    pub enabled: bool,
    pub is_visible: bool,
    pub refresh_counter: u32,
    pub field_flag_changed0: u8,
}

pub trait SphereIBLDynamicStateTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn is_visible(&self) -> &bool;
    fn refresh_counter(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u8;
}

impl SphereIBLDynamicStateTrait for SphereIBLDynamicState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn is_visible(&self) -> &bool {
        &self.is_visible
    }
    fn refresh_counter(&self) -> &u32 {
        &self.refresh_counter
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static SPHEREIBLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereIBLDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SphereIBLDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SphereIBLDynamicState, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SphereIBLDynamicState, is_visible),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SphereIBLDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(SphereIBLDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SPHEREIBLDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SphereIBLDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        SPHEREIBLDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPHEREIBLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereIBLDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SphereIBLDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SphereIBLStaticState {
    pub influence_fade_normal: f32,
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub guid: glacier_util::guid::Guid,
    pub mode: super::render_base::LocalIBLMode,
    pub capture_distance: f32,
    pub capture_fade_distance: f32,
    pub influence_expand_distance: f32,
    pub influence_fade_distance: f32,
    pub update_when_moving: bool,
    pub capture_sky: bool,
    pub capture_sky_mask: bool,
    pub use_sky_visibility_as_a_o: bool,
    pub use_sky_visibility_as_mask: bool,
    pub sharpen_sky_visibility: f32,
    pub bias_sky_visibility: f32,
    pub use_proxy_reprojection: bool,
    pub capture_fog: bool,
    pub object_layers: u16,
    pub do_not_update_baked_texture: bool,
    pub baked_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub field_flag_changed0: u32,
}

pub trait SphereIBLStaticStateTrait: TypeObject {
    fn influence_fade_normal(&self) -> &f32;
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn mode(&self) -> &super::render_base::LocalIBLMode;
    fn capture_distance(&self) -> &f32;
    fn capture_fade_distance(&self) -> &f32;
    fn influence_expand_distance(&self) -> &f32;
    fn influence_fade_distance(&self) -> &f32;
    fn update_when_moving(&self) -> &bool;
    fn capture_sky(&self) -> &bool;
    fn capture_sky_mask(&self) -> &bool;
    fn use_sky_visibility_as_a_o(&self) -> &bool;
    fn use_sky_visibility_as_mask(&self) -> &bool;
    fn sharpen_sky_visibility(&self) -> &f32;
    fn bias_sky_visibility(&self) -> &f32;
    fn use_proxy_reprojection(&self) -> &bool;
    fn capture_fog(&self) -> &bool;
    fn object_layers(&self) -> &u16;
    fn do_not_update_baked_texture(&self) -> &bool;
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn field_flag_changed0(&self) -> &u32;
}

impl SphereIBLStaticStateTrait for SphereIBLStaticState {
    fn influence_fade_normal(&self) -> &f32 {
        &self.influence_fade_normal
    }
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn mode(&self) -> &super::render_base::LocalIBLMode {
        &self.mode
    }
    fn capture_distance(&self) -> &f32 {
        &self.capture_distance
    }
    fn capture_fade_distance(&self) -> &f32 {
        &self.capture_fade_distance
    }
    fn influence_expand_distance(&self) -> &f32 {
        &self.influence_expand_distance
    }
    fn influence_fade_distance(&self) -> &f32 {
        &self.influence_fade_distance
    }
    fn update_when_moving(&self) -> &bool {
        &self.update_when_moving
    }
    fn capture_sky(&self) -> &bool {
        &self.capture_sky
    }
    fn capture_sky_mask(&self) -> &bool {
        &self.capture_sky_mask
    }
    fn use_sky_visibility_as_a_o(&self) -> &bool {
        &self.use_sky_visibility_as_a_o
    }
    fn use_sky_visibility_as_mask(&self) -> &bool {
        &self.use_sky_visibility_as_mask
    }
    fn sharpen_sky_visibility(&self) -> &f32 {
        &self.sharpen_sky_visibility
    }
    fn bias_sky_visibility(&self) -> &f32 {
        &self.bias_sky_visibility
    }
    fn use_proxy_reprojection(&self) -> &bool {
        &self.use_proxy_reprojection
    }
    fn capture_fog(&self) -> &bool {
        &self.capture_fog
    }
    fn object_layers(&self) -> &u16 {
        &self.object_layers
    }
    fn do_not_update_baked_texture(&self) -> &bool {
        &self.do_not_update_baked_texture
    }
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.baked_texture
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static SPHEREIBLSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereIBLStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SphereIBLStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InfluenceFadeNormal",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereIBLStaticState, influence_fade_normal),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(SphereIBLStaticState, transform_space),
            },
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(SphereIBLStaticState, guid),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalIBLMode",
                rust_offset: offset_of!(SphereIBLStaticState, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereIBLStaticState, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereIBLStaticState, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereIBLStaticState, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereIBLStaticState, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SphereIBLStaticState, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SphereIBLStaticState, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SphereIBLStaticState, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SphereIBLStaticState, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SphereIBLStaticState, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereIBLStaticState, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereIBLStaticState, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SphereIBLStaticState, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SphereIBLStaticState, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SphereIBLStaticState, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SphereIBLStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(SphereIBLStaticState, baked_texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SphereIBLStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SPHEREIBLSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SphereIBLStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        SPHEREIBLSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPHEREIBLSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereIBLStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SphereIBLStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IBLDynamicState {
    pub enabled: bool,
    pub is_visible: bool,
    pub refresh_counter: u32,
}

pub trait IBLDynamicStateTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn is_visible(&self) -> &bool;
    fn refresh_counter(&self) -> &u32;
}

impl IBLDynamicStateTrait for IBLDynamicState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn is_visible(&self) -> &bool {
        &self.is_visible
    }
    fn refresh_counter(&self) -> &u32 {
        &self.refresh_counter
    }
}

pub static IBLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IBLDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IBLDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IBLDynamicState, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IBLDynamicState, is_visible),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(IBLDynamicState, refresh_counter),
            },
        ],
    }),
    array_type: Some(IBLDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IBLDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        IBLDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IBLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IBLDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("IBLDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IBLStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub guid: glacier_util::guid::Guid,
    pub mode: super::render_base::LocalIBLMode,
    pub capture_distance: f32,
    pub capture_fade_distance: f32,
    pub influence_expand_distance: f32,
    pub influence_fade_distance: f32,
    pub update_when_moving: bool,
    pub capture_sky: bool,
    pub capture_sky_mask: bool,
    pub use_sky_visibility_as_a_o: bool,
    pub use_sky_visibility_as_mask: bool,
    pub sharpen_sky_visibility: f32,
    pub bias_sky_visibility: f32,
    pub use_proxy_reprojection: bool,
    pub capture_fog: bool,
    pub object_layers: u16,
    pub do_not_update_baked_texture: bool,
    pub baked_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
}

pub trait IBLStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn mode(&self) -> &super::render_base::LocalIBLMode;
    fn capture_distance(&self) -> &f32;
    fn capture_fade_distance(&self) -> &f32;
    fn influence_expand_distance(&self) -> &f32;
    fn influence_fade_distance(&self) -> &f32;
    fn update_when_moving(&self) -> &bool;
    fn capture_sky(&self) -> &bool;
    fn capture_sky_mask(&self) -> &bool;
    fn use_sky_visibility_as_a_o(&self) -> &bool;
    fn use_sky_visibility_as_mask(&self) -> &bool;
    fn sharpen_sky_visibility(&self) -> &f32;
    fn bias_sky_visibility(&self) -> &f32;
    fn use_proxy_reprojection(&self) -> &bool;
    fn capture_fog(&self) -> &bool;
    fn object_layers(&self) -> &u16;
    fn do_not_update_baked_texture(&self) -> &bool;
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
}

impl IBLStaticStateTrait for IBLStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn mode(&self) -> &super::render_base::LocalIBLMode {
        &self.mode
    }
    fn capture_distance(&self) -> &f32 {
        &self.capture_distance
    }
    fn capture_fade_distance(&self) -> &f32 {
        &self.capture_fade_distance
    }
    fn influence_expand_distance(&self) -> &f32 {
        &self.influence_expand_distance
    }
    fn influence_fade_distance(&self) -> &f32 {
        &self.influence_fade_distance
    }
    fn update_when_moving(&self) -> &bool {
        &self.update_when_moving
    }
    fn capture_sky(&self) -> &bool {
        &self.capture_sky
    }
    fn capture_sky_mask(&self) -> &bool {
        &self.capture_sky_mask
    }
    fn use_sky_visibility_as_a_o(&self) -> &bool {
        &self.use_sky_visibility_as_a_o
    }
    fn use_sky_visibility_as_mask(&self) -> &bool {
        &self.use_sky_visibility_as_mask
    }
    fn sharpen_sky_visibility(&self) -> &f32 {
        &self.sharpen_sky_visibility
    }
    fn bias_sky_visibility(&self) -> &f32 {
        &self.bias_sky_visibility
    }
    fn use_proxy_reprojection(&self) -> &bool {
        &self.use_proxy_reprojection
    }
    fn capture_fog(&self) -> &bool {
        &self.capture_fog
    }
    fn object_layers(&self) -> &u16 {
        &self.object_layers
    }
    fn do_not_update_baked_texture(&self) -> &bool {
        &self.do_not_update_baked_texture
    }
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.baked_texture
    }
}

pub static IBLSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IBLStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IBLStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(IBLStaticState, transform_space),
            },
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(IBLStaticState, guid),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalIBLMode",
                rust_offset: offset_of!(IBLStaticState, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IBLStaticState, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IBLStaticState, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IBLStaticState, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IBLStaticState, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IBLStaticState, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IBLStaticState, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IBLStaticState, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IBLStaticState, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IBLStaticState, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IBLStaticState, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IBLStaticState, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IBLStaticState, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IBLStaticState, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(IBLStaticState, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(IBLStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(IBLStaticState, baked_texture),
            },
        ],
    }),
    array_type: Some(IBLSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IBLStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        IBLSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IBLSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IBLStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("IBLStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FogExclusionVolumeDynamicState {
    pub enabled: bool,
    pub transform: super::core::LinearTransform,
    pub field_flag_changed0: u8,
}

pub trait FogExclusionVolumeDynamicStateTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn transform(&self) -> &super::core::LinearTransform;
    fn field_flag_changed0(&self) -> &u8;
}

impl FogExclusionVolumeDynamicStateTrait for FogExclusionVolumeDynamicState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static FOGEXCLUSIONVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FogExclusionVolumeDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FogExclusionVolumeDynamicState, enabled),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(FogExclusionVolumeDynamicState, transform),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(FogExclusionVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(FOGEXCLUSIONVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FogExclusionVolumeDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        FOGEXCLUSIONVOLUMEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FOGEXCLUSIONVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FogExclusionVolumeDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FogExclusionVolumeStaticState {
    pub fog_exclusion_volume_shape: FogExclusionVolumeShape,
    pub fog_volume_strength: f32,
    pub fade_out_start: f32,
    pub fade_out_end: f32,
    pub field_flag_changed0: u8,
}

pub trait FogExclusionVolumeStaticStateTrait: TypeObject {
    fn fog_exclusion_volume_shape(&self) -> &FogExclusionVolumeShape;
    fn fog_volume_strength(&self) -> &f32;
    fn fade_out_start(&self) -> &f32;
    fn fade_out_end(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u8;
}

impl FogExclusionVolumeStaticStateTrait for FogExclusionVolumeStaticState {
    fn fog_exclusion_volume_shape(&self) -> &FogExclusionVolumeShape {
        &self.fog_exclusion_volume_shape
    }
    fn fog_volume_strength(&self) -> &f32 {
        &self.fog_volume_strength
    }
    fn fade_out_start(&self) -> &f32 {
        &self.fade_out_start
    }
    fn fade_out_end(&self) -> &f32 {
        &self.fade_out_end
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static FOGEXCLUSIONVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FogExclusionVolumeStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FogExclusionVolumeShape",
                flags: MemberInfoFlags::new(0),
                field_type: "FogExclusionVolumeShape",
                rust_offset: offset_of!(FogExclusionVolumeStaticState, fog_exclusion_volume_shape),
            },
            FieldInfoData {
                name: "FogVolumeStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogExclusionVolumeStaticState, fog_volume_strength),
            },
            FieldInfoData {
                name: "FadeOutStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogExclusionVolumeStaticState, fade_out_start),
            },
            FieldInfoData {
                name: "FadeOutEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FogExclusionVolumeStaticState, fade_out_end),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(FogExclusionVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(FOGEXCLUSIONVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FogExclusionVolumeStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        FOGEXCLUSIONVOLUMESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FOGEXCLUSIONVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FogExclusionVolumeStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum FogExclusionVolumeShape {
    #[default]
    FogExclusionVolumeShape_Box = 0,
    FogExclusionVolumeShape_Sphere = 1,
}

pub static FOGEXCLUSIONVOLUMESHAPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeShape",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(FOGEXCLUSIONVOLUMESHAPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FogExclusionVolumeShape {
    fn type_info(&self) -> &'static TypeInfo {
        FOGEXCLUSIONVOLUMESHAPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FOGEXCLUSIONVOLUMESHAPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeShape-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FogExclusionVolumeShape"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DistantShadowCacheDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub refresh_counter: u32,
    pub field_flag_changed0: u8,
}

pub trait DistantShadowCacheDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn enabled(&self) -> &bool;
    fn refresh_counter(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u8;
}

impl DistantShadowCacheDynamicStateTrait for DistantShadowCacheDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn refresh_counter(&self) -> &u32 {
        &self.refresh_counter
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static DISTANTSHADOWCACHEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistantShadowCacheDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(DistantShadowCacheDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantShadowCacheDynamicState, enabled),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DistantShadowCacheDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DistantShadowCacheDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DISTANTSHADOWCACHEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DistantShadowCacheDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANTSHADOWCACHEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISTANTSHADOWCACHEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DistantShadowCacheDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DistantShadowCacheStaticState {
    pub guid: glacier_util::guid::Guid,
    pub mode: super::render_base::ShadowCacheMode,
    pub resolution: u32,
    pub doublebuffer: bool,
    pub depth_bias: super::render_base::ShadowCacheDepthBias,
    pub dynamic_prod_priority: u32,
    pub tiles_per_side: u32,
    pub do_not_update_baked_texture: bool,
    pub baked_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub field_flag_changed0: u16,
}

pub trait DistantShadowCacheStaticStateTrait: TypeObject {
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn mode(&self) -> &super::render_base::ShadowCacheMode;
    fn resolution(&self) -> &u32;
    fn doublebuffer(&self) -> &bool;
    fn depth_bias(&self) -> &super::render_base::ShadowCacheDepthBias;
    fn dynamic_prod_priority(&self) -> &u32;
    fn tiles_per_side(&self) -> &u32;
    fn do_not_update_baked_texture(&self) -> &bool;
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn field_flag_changed0(&self) -> &u16;
}

impl DistantShadowCacheStaticStateTrait for DistantShadowCacheStaticState {
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn mode(&self) -> &super::render_base::ShadowCacheMode {
        &self.mode
    }
    fn resolution(&self) -> &u32 {
        &self.resolution
    }
    fn doublebuffer(&self) -> &bool {
        &self.doublebuffer
    }
    fn depth_bias(&self) -> &super::render_base::ShadowCacheDepthBias {
        &self.depth_bias
    }
    fn dynamic_prod_priority(&self) -> &u32 {
        &self.dynamic_prod_priority
    }
    fn tiles_per_side(&self) -> &u32 {
        &self.tiles_per_side
    }
    fn do_not_update_baked_texture(&self) -> &bool {
        &self.do_not_update_baked_texture
    }
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.baked_texture
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static DISTANTSHADOWCACHESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistantShadowCacheStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(DistantShadowCacheStaticState, guid),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: "ShadowCacheMode",
                rust_offset: offset_of!(DistantShadowCacheStaticState, mode),
            },
            FieldInfoData {
                name: "Resolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DistantShadowCacheStaticState, resolution),
            },
            FieldInfoData {
                name: "Doublebuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantShadowCacheStaticState, doublebuffer),
            },
            FieldInfoData {
                name: "DepthBias",
                flags: MemberInfoFlags::new(0),
                field_type: "ShadowCacheDepthBias",
                rust_offset: offset_of!(DistantShadowCacheStaticState, depth_bias),
            },
            FieldInfoData {
                name: "DynamicProdPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DistantShadowCacheStaticState, dynamic_prod_priority),
            },
            FieldInfoData {
                name: "TilesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DistantShadowCacheStaticState, tiles_per_side),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistantShadowCacheStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(DistantShadowCacheStaticState, baked_texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(DistantShadowCacheStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DISTANTSHADOWCACHESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistantShadowCacheStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        DISTANTSHADOWCACHESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISTANTSHADOWCACHESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DistantShadowCacheStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BakeableTextureDynamicState {
}

pub trait BakeableTextureDynamicStateTrait: TypeObject {
}

impl BakeableTextureDynamicStateTrait for BakeableTextureDynamicState {
}

pub static BAKEABLETEXTUREDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BakeableTextureDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BakeableTextureDynamicState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BAKEABLETEXTUREDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BakeableTextureDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        BAKEABLETEXTUREDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BAKEABLETEXTUREDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BakeableTextureDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("BakeableTextureDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BakeableTextureStaticState {
    pub do_not_update_baked_texture: bool,
    pub baked_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
}

pub trait BakeableTextureStaticStateTrait: TypeObject {
    fn do_not_update_baked_texture(&self) -> &bool;
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
}

impl BakeableTextureStaticStateTrait for BakeableTextureStaticState {
    fn do_not_update_baked_texture(&self) -> &bool {
        &self.do_not_update_baked_texture
    }
    fn baked_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.baked_texture
    }
}

pub static BAKEABLETEXTURESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BakeableTextureStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BakeableTextureStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BakeableTextureStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(BakeableTextureStaticState, baked_texture),
            },
        ],
    }),
    array_type: Some(BAKEABLETEXTURESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BakeableTextureStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        BAKEABLETEXTURESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BAKEABLETEXTURESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BakeableTextureStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("BakeableTextureStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PoseConsumer {
}

pub trait PoseConsumerTrait: TypeObject {
}

impl PoseConsumerTrait for PoseConsumer {
}

pub static POSECONSUMER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseConsumer",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PoseConsumer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 0,
};

impl TypeObject for PoseConsumer {
    fn type_info(&self) -> &'static TypeInfo {
        POSECONSUMER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MeshHandle {
}

pub trait MeshHandleTrait: TypeObject {
}

impl MeshHandleTrait for MeshHandle {
}

pub static MESHHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshHandle",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 0,
};

impl TypeObject for MeshHandle {
    fn type_info(&self) -> &'static TypeInfo {
        MESHHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

