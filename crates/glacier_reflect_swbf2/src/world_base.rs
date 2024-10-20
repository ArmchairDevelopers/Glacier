use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Debug)]
pub struct LocalFogVolumeDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub refresh_counter: u32,
    pub participating_media: ParticipatingMedia,
    pub ambient_lighting_scale: f32,
    pub density_texture: super::render_base::TextureBaseAsset,
    pub field_flag_changed0: u8,
}

pub const LOCALFOGVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeDynamicState, enabled),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "ParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICIPATINGMEDIA_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeDynamicState, participating_media),
            },
            FieldInfoData {
                name: "AmbientLightingScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeDynamicState, ambient_lighting_scale),
            },
            FieldInfoData {
                name: "DensityTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeDynamicState, density_texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LOCALFOGVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalFogVolumeDynamicState {
    fn type_info() -> &'static TypeInfo {
        LOCALFOGVOLUMEDYNAMICSTATE_TYPE_INFO
    }
}


pub const LOCALFOGVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LocalFogVolumeDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalFogVolumeStaticState {
    pub guid: super::core::Guid,
    pub object_layers: u16,
    pub field_flag_changed0: u8,
}

pub const LOCALFOGVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeStaticState, guid),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeStaticState, object_layers),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LOCALFOGVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LocalFogVolumeStaticState {
    fn type_info() -> &'static TypeInfo {
        LOCALFOGVOLUMESTATICSTATE_TYPE_INFO
    }
}


pub const LOCALFOGVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LocalFogVolumeStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VolumetricDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub refresh_counter: u32,
    pub participating_media: ParticipatingMedia,
    pub ambient_lighting_scale: f32,
    pub density_texture: super::render_base::TextureBaseAsset,
}

pub const VOLUMETRICDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumetricDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(VolumetricDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VolumetricDynamicState, enabled),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VolumetricDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "ParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICIPATINGMEDIA_TYPE_INFO,
                rust_offset: offset_of!(VolumetricDynamicState, participating_media),
            },
            FieldInfoData {
                name: "AmbientLightingScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VolumetricDynamicState, ambient_lighting_scale),
            },
            FieldInfoData {
                name: "DensityTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(VolumetricDynamicState, density_texture),
            },
        ],
    }),
    array_type: Some(VOLUMETRICDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VolumetricDynamicState {
    fn type_info() -> &'static TypeInfo {
        VOLUMETRICDYNAMICSTATE_TYPE_INFO
    }
}


pub const VOLUMETRICDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumetricDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VolumetricDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VolumetricStaticState {
    pub guid: super::core::Guid,
    pub object_layers: u16,
}

pub const VOLUMETRICSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumetricStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(VolumetricStaticState, guid),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(VolumetricStaticState, object_layers),
            },
        ],
    }),
    array_type: Some(VOLUMETRICSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VolumetricStaticState {
    fn type_info() -> &'static TypeInfo {
        VOLUMETRICSTATICSTATE_TYPE_INFO
    }
}


pub const VOLUMETRICSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumetricStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VolumetricStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ParticipatingMedia {
    pub specification_mode: ParticipatingMediaSpecificationMode,
    pub absorption: f32,
    pub scattering: super::core::Vec3,
    pub exctinction: f32,
    pub albedo: super::core::Vec3,
    pub emissive: super::core::Vec3,
    pub phase: f32,
}

pub const PARTICIPATINGMEDIA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMedia",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SpecificationMode",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICIPATINGMEDIASPECIFICATIONMODE_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMedia, specification_mode),
            },
            FieldInfoData {
                name: "Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMedia, absorption),
            },
            FieldInfoData {
                name: "Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMedia, scattering),
            },
            FieldInfoData {
                name: "Exctinction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMedia, exctinction),
            },
            FieldInfoData {
                name: "Albedo",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMedia, albedo),
            },
            FieldInfoData {
                name: "Emissive",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMedia, emissive),
            },
            FieldInfoData {
                name: "Phase",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMedia, phase),
            },
        ],
    }),
    array_type: Some(PARTICIPATINGMEDIA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ParticipatingMedia {
    fn type_info() -> &'static TypeInfo {
        PARTICIPATINGMEDIA_TYPE_INFO
    }
}


pub const PARTICIPATINGMEDIA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMedia-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ParticipatingMedia-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ParticipatingMediaMaterialData {
    pub specification_mode: ParticipatingMediaSpecificationMode,
    pub absorption: f32,
    pub scattering: super::core::Vec3,
    pub exctinction: f32,
    pub albedo: super::core::Vec3,
    pub emissive: super::core::Vec3,
    pub phase: f32,
}

pub const PARTICIPATINGMEDIAMATERIALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaMaterialData",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SpecificationMode",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICIPATINGMEDIASPECIFICATIONMODE_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialData, specification_mode),
            },
            FieldInfoData {
                name: "Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialData, absorption),
            },
            FieldInfoData {
                name: "Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialData, scattering),
            },
            FieldInfoData {
                name: "Exctinction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialData, exctinction),
            },
            FieldInfoData {
                name: "Albedo",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialData, albedo),
            },
            FieldInfoData {
                name: "Emissive",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialData, emissive),
            },
            FieldInfoData {
                name: "Phase",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialData, phase),
            },
        ],
    }),
    array_type: Some(PARTICIPATINGMEDIAMATERIALDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ParticipatingMediaMaterialData {
    fn type_info() -> &'static TypeInfo {
        PARTICIPATINGMEDIAMATERIALDATA_TYPE_INFO
    }
}


pub const PARTICIPATINGMEDIAMATERIALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaMaterialData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ParticipatingMediaMaterialData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ParticipatingMediaSpecificationMode {
    #[default]
    ParticipatingMediaSpecificationMode_AbsorptionScattering = 0,
    ParticipatingMediaSpecificationMode_ExtinctionAlbedo = 1,
}

pub const PARTICIPATINGMEDIASPECIFICATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaSpecificationMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(PARTICIPATINGMEDIASPECIFICATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ParticipatingMediaSpecificationMode {
    fn type_info() -> &'static TypeInfo {
        PARTICIPATINGMEDIASPECIFICATIONMODE_TYPE_INFO
    }
}


pub const PARTICIPATINGMEDIASPECIFICATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaSpecificationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ParticipatingMediaSpecificationMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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
    pub shape_texture: super::render_base::TextureBaseAsset,
    pub shape_texture_scale: f32,
    pub shape_texture_contrast: f32,
    pub detail_texture: super::render_base::TextureBaseAsset,
    pub detail_texture_scale: f32,
    pub weather_texture: super::render_base::TextureBaseAsset,
    pub weather_texture_scale: f32,
    pub cloud_type_density_gradient_texture: super::render_base::TextureBaseAsset,
    pub planet_radius: f32,
    pub cut_off_distance: f32,
    pub cloud_layer_start_height: f32,
    pub cloud_layer_thickness: f32,
    pub wind_scale: f32,
    pub offset: super::core::Vec2,
    pub field_flag_override0: u32,
    pub field_flag_changed0: u32,
}

pub const CLOUDCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, enable),
            },
            FieldInfoData {
                name: "BaseToTopMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, base_to_top_multiplier),
            },
            FieldInfoData {
                name: "EdgeDetailMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, edge_detail_multiplier),
            },
            FieldInfoData {
                name: "CloudDensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, cloud_density_multiplier),
            },
            FieldInfoData {
                name: "Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, absorption),
            },
            FieldInfoData {
                name: "Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, scattering),
            },
            FieldInfoData {
                name: "PhaseG0",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, phase_g0),
            },
            FieldInfoData {
                name: "PhaseG1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, phase_g1),
            },
            FieldInfoData {
                name: "PhaseBlend",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, phase_blend),
            },
            FieldInfoData {
                name: "AmbientMultiplicator",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, ambient_multiplicator),
            },
            FieldInfoData {
                name: "AmbientDesaturate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, ambient_desaturate),
            },
            FieldInfoData {
                name: "AerialPerspectiveScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, aerial_perspective_scale),
            },
            FieldInfoData {
                name: "EnableShadow",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, enable_shadow),
            },
            FieldInfoData {
                name: "ScatteringOrder",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, scattering_order),
            },
            FieldInfoData {
                name: "ScatteringFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, scattering_factor),
            },
            FieldInfoData {
                name: "ExtinctionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, extinction_factor),
            },
            FieldInfoData {
                name: "PhaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, phase_factor),
            },
            FieldInfoData {
                name: "ShapeTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, shape_texture),
            },
            FieldInfoData {
                name: "ShapeTextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, shape_texture_scale),
            },
            FieldInfoData {
                name: "ShapeTextureContrast",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, shape_texture_contrast),
            },
            FieldInfoData {
                name: "DetailTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, detail_texture),
            },
            FieldInfoData {
                name: "DetailTextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, detail_texture_scale),
            },
            FieldInfoData {
                name: "WeatherTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, weather_texture),
            },
            FieldInfoData {
                name: "WeatherTextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, weather_texture_scale),
            },
            FieldInfoData {
                name: "CloudTypeDensityGradientTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, cloud_type_density_gradient_texture),
            },
            FieldInfoData {
                name: "PlanetRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, planet_radius),
            },
            FieldInfoData {
                name: "CutOffDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, cut_off_distance),
            },
            FieldInfoData {
                name: "CloudLayerStartHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, cloud_layer_start_height),
            },
            FieldInfoData {
                name: "CloudLayerThickness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, cloud_layer_thickness),
            },
            FieldInfoData {
                name: "WindScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, wind_scale),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, offset),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CLOUDCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CloudComponentState {
    fn type_info() -> &'static TypeInfo {
        CLOUDCOMPONENTSTATE_TYPE_INFO
    }
}


pub const CLOUDCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("CloudComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SunFlareEffectComponentState {
    pub enable: bool,
    pub debug_draw_occluder: bool,
    pub occluder_size: f32,
    pub element1_enable: bool,
    pub element1_shader: super::render_base::SurfaceShaderBaseAsset,
    pub element1_ray_distance: f32,
    pub element1_size: super::core::Vec2,
    pub element1_size_occluder_curve: super::core::Vec4,
    pub element1_size_screen_pos_curve: super::core::Vec4,
    pub element1_alpha_occluder_curve: super::core::Vec4,
    pub element1_alpha_screen_pos_curve: super::core::Vec4,
    pub element2_enable: bool,
    pub element2_shader: super::render_base::SurfaceShaderBaseAsset,
    pub element2_ray_distance: f32,
    pub element2_size: super::core::Vec2,
    pub element2_size_occluder_curve: super::core::Vec4,
    pub element2_size_screen_pos_curve: super::core::Vec4,
    pub element2_alpha_occluder_curve: super::core::Vec4,
    pub element2_alpha_screen_pos_curve: super::core::Vec4,
    pub element3_enable: bool,
    pub element3_shader: super::render_base::SurfaceShaderBaseAsset,
    pub element3_ray_distance: f32,
    pub element3_size: super::core::Vec2,
    pub element3_size_occluder_curve: super::core::Vec4,
    pub element3_size_screen_pos_curve: super::core::Vec4,
    pub element3_alpha_occluder_curve: super::core::Vec4,
    pub element3_alpha_screen_pos_curve: super::core::Vec4,
    pub element4_enable: bool,
    pub element4_shader: super::render_base::SurfaceShaderBaseAsset,
    pub element4_ray_distance: f32,
    pub element4_size: super::core::Vec2,
    pub element4_size_occluder_curve: super::core::Vec4,
    pub element4_size_screen_pos_curve: super::core::Vec4,
    pub element4_alpha_occluder_curve: super::core::Vec4,
    pub element4_alpha_screen_pos_curve: super::core::Vec4,
    pub element5_enable: bool,
    pub element5_shader: super::render_base::SurfaceShaderBaseAsset,
    pub element5_ray_distance: f32,
    pub element5_size: super::core::Vec2,
    pub element5_size_occluder_curve: super::core::Vec4,
    pub element5_size_screen_pos_curve: super::core::Vec4,
    pub element5_alpha_occluder_curve: super::core::Vec4,
    pub element5_alpha_screen_pos_curve: super::core::Vec4,
}

pub const SUNFLAREEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareEffectComponentState",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, enable),
            },
            FieldInfoData {
                name: "DebugDrawOccluder",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, debug_draw_occluder),
            },
            FieldInfoData {
                name: "OccluderSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, occluder_size),
            },
            FieldInfoData {
                name: "Element1Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_enable),
            },
            FieldInfoData {
                name: "Element1Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_shader),
            },
            FieldInfoData {
                name: "Element1RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_ray_distance),
            },
            FieldInfoData {
                name: "Element1Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_size),
            },
            FieldInfoData {
                name: "Element1SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element1SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element1AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element1AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element1_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element2Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_enable),
            },
            FieldInfoData {
                name: "Element2Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_shader),
            },
            FieldInfoData {
                name: "Element2RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_ray_distance),
            },
            FieldInfoData {
                name: "Element2Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_size),
            },
            FieldInfoData {
                name: "Element2SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element2SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element2AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element2AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element2_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element3Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_enable),
            },
            FieldInfoData {
                name: "Element3Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_shader),
            },
            FieldInfoData {
                name: "Element3RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_ray_distance),
            },
            FieldInfoData {
                name: "Element3Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_size),
            },
            FieldInfoData {
                name: "Element3SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element3SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element3AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element3AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element3_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element4Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_enable),
            },
            FieldInfoData {
                name: "Element4Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_shader),
            },
            FieldInfoData {
                name: "Element4RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_ray_distance),
            },
            FieldInfoData {
                name: "Element4Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_size),
            },
            FieldInfoData {
                name: "Element4SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element4SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element4AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element4AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element4_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element5Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_enable),
            },
            FieldInfoData {
                name: "Element5Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_shader),
            },
            FieldInfoData {
                name: "Element5RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_ray_distance),
            },
            FieldInfoData {
                name: "Element5Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_size),
            },
            FieldInfoData {
                name: "Element5SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element5SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element5AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element5AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareEffectComponentState, element5_alpha_screen_pos_curve),
            },
        ],
    }),
    array_type: Some(SUNFLAREEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SunFlareEffectComponentState {
    fn type_info() -> &'static TypeInfo {
        SUNFLAREEFFECTCOMPONENTSTATE_TYPE_INFO
    }
}


pub const SUNFLAREEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SunFlareEffectComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkyEffectComponentState {
    pub enable: bool,
    pub sun_size: f32,
    pub sun_scale: f32,
    pub sky_gradient_scale: f32,
    pub sky_gradient_texture: super::render_base::TextureBaseAsset,
    pub panoramic_u_v_min_x: f32,
    pub panoramic_u_v_max_x: f32,
    pub panoramic_u_v_min_y: f32,
    pub panoramic_u_v_max_y: f32,
    pub panoramic_tile_factor: f32,
    pub panoramic_rotation: f32,
    pub panoramic_texture: super::render_base::TextureBaseAsset,
    pub panoramic_alpha_texture: super::render_base::TextureBaseAsset,
    pub cloud_layer_sun_color: super::core::Vec3,
    pub cloud_layer_mask_texture: super::render_base::TextureBaseAsset,
    pub cloud_layer1: SkyCloudLayer,
    pub cloud_layer2: SkyCloudLayer,
    pub static_envmap_texture: super::render_base::TextureBaseAsset,
    pub wind_direction: f32,
}

pub const SKYEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyEffectComponentState",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, enable),
            },
            FieldInfoData {
                name: "SunSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, sun_size),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, sun_scale),
            },
            FieldInfoData {
                name: "SkyGradientScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, sky_gradient_scale),
            },
            FieldInfoData {
                name: "SkyGradientTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, sky_gradient_texture),
            },
            FieldInfoData {
                name: "PanoramicUVMinX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_u_v_min_x),
            },
            FieldInfoData {
                name: "PanoramicUVMaxX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_u_v_max_x),
            },
            FieldInfoData {
                name: "PanoramicUVMinY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_u_v_min_y),
            },
            FieldInfoData {
                name: "PanoramicUVMaxY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_u_v_max_y),
            },
            FieldInfoData {
                name: "PanoramicTileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_tile_factor),
            },
            FieldInfoData {
                name: "PanoramicRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_rotation),
            },
            FieldInfoData {
                name: "PanoramicTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_texture),
            },
            FieldInfoData {
                name: "PanoramicAlphaTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, panoramic_alpha_texture),
            },
            FieldInfoData {
                name: "CloudLayerSunColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, cloud_layer_sun_color),
            },
            FieldInfoData {
                name: "CloudLayerMaskTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, cloud_layer_mask_texture),
            },
            FieldInfoData {
                name: "CloudLayer1",
                flags: MemberInfoFlags::new(0),
                field_type: SKYCLOUDLAYER_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, cloud_layer1),
            },
            FieldInfoData {
                name: "CloudLayer2",
                flags: MemberInfoFlags::new(0),
                field_type: SKYCLOUDLAYER_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, cloud_layer2),
            },
            FieldInfoData {
                name: "StaticEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, static_envmap_texture),
            },
            FieldInfoData {
                name: "WindDirection",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyEffectComponentState, wind_direction),
            },
        ],
    }),
    array_type: Some(SKYEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyEffectComponentState {
    fn type_info() -> &'static TypeInfo {
        SKYEFFECTCOMPONENTSTATE_TYPE_INFO
    }
}


pub const SKYEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyEffectComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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
    pub texture: super::render_base::TextureBaseAsset,
}

pub const SKYCLOUDLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCloudLayer",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Altitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCloudLayer, altitude),
            },
            FieldInfoData {
                name: "TileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCloudLayer, tile_factor),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCloudLayer, rotation),
            },
            FieldInfoData {
                name: "Speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCloudLayer, speed),
            },
            FieldInfoData {
                name: "SunLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCloudLayer, sun_light_intensity),
            },
            FieldInfoData {
                name: "SunLightPower",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCloudLayer, sun_light_power),
            },
            FieldInfoData {
                name: "AmbientLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCloudLayer, ambient_light_intensity),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyCloudLayer, color),
            },
            FieldInfoData {
                name: "AlphaMul",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCloudLayer, alpha_mul),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyCloudLayer, texture),
            },
        ],
    }),
    array_type: Some(SKYCLOUDLAYER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCloudLayer {
    fn type_info() -> &'static TypeInfo {
        SKYCLOUDLAYER_TYPE_INFO
    }
}


pub const SKYCLOUDLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCloudLayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyCloudLayer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FogEffectComponentState {
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

pub const FOGEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogEffectComponentState",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, enable),
            },
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, start),
            },
            FieldInfoData {
                name: "End",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, end),
            },
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, curve),
            },
            FieldInfoData {
                name: "FogColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, fog_color),
            },
            FieldInfoData {
                name: "FogColorStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, fog_color_start),
            },
            FieldInfoData {
                name: "FogColorEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, fog_color_end),
            },
            FieldInfoData {
                name: "FogColorCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, fog_color_curve),
            },
            FieldInfoData {
                name: "TransparencyFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, transparency_fade_start),
            },
            FieldInfoData {
                name: "TransparencyFadeEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, transparency_fade_end),
            },
            FieldInfoData {
                name: "TransparencyFadeClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, transparency_fade_clamp),
            },
            FieldInfoData {
                name: "ForwardLightScatteringColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_color),
            },
            FieldInfoData {
                name: "ForwardLightScatteringEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_enabled),
            },
            FieldInfoData {
                name: "ForwardLightScatteringPhaseG",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_phase_g),
            },
            FieldInfoData {
                name: "ForwardLightScatteringStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_strength),
            },
            FieldInfoData {
                name: "ForwardLightScatteringPresence",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_presence),
            },
            FieldInfoData {
                name: "ForwardLightScatteringMaxBlurLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_max_blur_length),
            },
            FieldInfoData {
                name: "ForwardLightScatteringExtinction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_extinction),
            },
            FieldInfoData {
                name: "ForwardLightScatteringSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, forward_light_scattering_smoothness),
            },
            FieldInfoData {
                name: "HeightFogEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, height_fog_enable),
            },
            FieldInfoData {
                name: "HeightFogFollowCamera",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, height_fog_follow_camera),
            },
            FieldInfoData {
                name: "HeightFogAltitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, height_fog_altitude),
            },
            FieldInfoData {
                name: "HeightFogDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, height_fog_depth),
            },
            FieldInfoData {
                name: "HeightFogVisibilityRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogEffectComponentState, height_fog_visibility_range),
            },
        ],
    }),
    array_type: Some(FOGEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FogEffectComponentState {
    fn type_info() -> &'static TypeInfo {
        FOGEFFECTCOMPONENTSTATE_TYPE_INFO
    }
}


pub const FOGEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FogEffectComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OutdoorLightEffectComponentState {
    pub enable: bool,
    pub sun_rotation_x: f32,
    pub sun_rotation_y: f32,
    pub sun_color: super::core::Vec3,
    pub sky_color: super::core::Vec3,
    pub ground_color: super::core::Vec3,
    pub sky_light_angle_factor: f32,
    pub sun_shadow_height_scale: f32,
    pub cloud_shadow_enable: bool,
    pub cloud_shadow_texture: super::render_base::TextureBaseAsset,
    pub cloud_shadow_speed: super::core::Vec2,
    pub cloud_shadow_size: f32,
    pub cloud_shadow_coverage: f32,
    pub cloud_shadow_exponent: f32,
}

pub const OUTDOORLIGHTEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightEffectComponentState",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, enable),
            },
            FieldInfoData {
                name: "SunRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sun_rotation_x),
            },
            FieldInfoData {
                name: "SunRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sun_rotation_y),
            },
            FieldInfoData {
                name: "SunColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sun_color),
            },
            FieldInfoData {
                name: "SkyColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sky_color),
            },
            FieldInfoData {
                name: "GroundColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, ground_color),
            },
            FieldInfoData {
                name: "SkyLightAngleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sky_light_angle_factor),
            },
            FieldInfoData {
                name: "SunShadowHeightScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, sun_shadow_height_scale),
            },
            FieldInfoData {
                name: "CloudShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_enable),
            },
            FieldInfoData {
                name: "CloudShadowTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_texture),
            },
            FieldInfoData {
                name: "CloudShadowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_speed),
            },
            FieldInfoData {
                name: "CloudShadowSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_size),
            },
            FieldInfoData {
                name: "CloudShadowCoverage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_coverage),
            },
            FieldInfoData {
                name: "CloudShadowExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightEffectComponentState, cloud_shadow_exponent),
            },
        ],
    }),
    array_type: Some(OUTDOORLIGHTEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OutdoorLightEffectComponentState {
    fn type_info() -> &'static TypeInfo {
        OUTDOORLIGHTEFFECTCOMPONENTSTATE_TYPE_INFO
    }
}


pub const OUTDOORLIGHTEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OutdoorLightEffectComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SonarParamsComponentState {
    pub enable: bool,
    pub color1: super::core::Vec3,
    pub color2: super::core::Vec3,
    pub color3: super::core::Vec3,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const SONARPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SonarParamsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentState, enable),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentState, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentState, color2),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentState, color3),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SONARPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SonarParamsComponentState {
    fn type_info() -> &'static TypeInfo {
        SONARPARAMSCOMPONENTSTATE_TYPE_INFO
    }
}


pub const SONARPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SonarParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SonarParamsComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const HOLOGRAMPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramParamsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, enable),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: HOLOGRAMRENDERMODE_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, render_mode),
            },
            FieldInfoData {
                name: "KeyIlluminance",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, key_illuminance),
            },
            FieldInfoData {
                name: "KeyLightDir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, key_light_dir),
            },
            FieldInfoData {
                name: "ResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, resolution_scale),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, color2),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, color3),
            },
            FieldInfoData {
                name: "Color4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, color4),
            },
            FieldInfoData {
                name: "Color5",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, color5),
            },
            FieldInfoData {
                name: "Float1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, float1),
            },
            FieldInfoData {
                name: "Float2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, float2),
            },
            FieldInfoData {
                name: "Float3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, float3),
            },
            FieldInfoData {
                name: "Float4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, float4),
            },
            FieldInfoData {
                name: "Float5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, float5),
            },
            FieldInfoData {
                name: "Float6",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, float6),
            },
            FieldInfoData {
                name: "Float7",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, float7),
            },
            FieldInfoData {
                name: "Float8",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, float8),
            },
            FieldInfoData {
                name: "Float9",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, float9),
            },
            FieldInfoData {
                name: "Brightness2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, brightness2),
            },
            FieldInfoData {
                name: "Brightness3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, brightness3),
            },
            FieldInfoData {
                name: "Brightness4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, brightness4),
            },
            FieldInfoData {
                name: "Brightness5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, brightness5),
            },
            FieldInfoData {
                name: "StreaksEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, streaks_enabled),
            },
            FieldInfoData {
                name: "TimeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, time_offset),
            },
            FieldInfoData {
                name: "Opacity1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, opacity1),
            },
            FieldInfoData {
                name: "DistortionScale1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, distortion_scale1),
            },
            FieldInfoData {
                name: "SourcePos1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, source_pos1),
            },
            FieldInfoData {
                name: "TargetPos1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, target_pos1),
            },
            FieldInfoData {
                name: "SourceRadius1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, source_radius1),
            },
            FieldInfoData {
                name: "TargetRadius1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, target_radius1),
            },
            FieldInfoData {
                name: "Opacity2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, opacity2),
            },
            FieldInfoData {
                name: "DistortionScale2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, distortion_scale2),
            },
            FieldInfoData {
                name: "SourcePos2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, source_pos2),
            },
            FieldInfoData {
                name: "TargetPos2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, target_pos2),
            },
            FieldInfoData {
                name: "SourceRadius2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, source_radius2),
            },
            FieldInfoData {
                name: "TargetRadius2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, target_radius2),
            },
            FieldInfoData {
                name: "Opacity3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, opacity3),
            },
            FieldInfoData {
                name: "DistortionScale3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, distortion_scale3),
            },
            FieldInfoData {
                name: "SourcePos3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, source_pos3),
            },
            FieldInfoData {
                name: "TargetPos3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, target_pos3),
            },
            FieldInfoData {
                name: "SourceRadius3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, source_radius3),
            },
            FieldInfoData {
                name: "TargetRadius3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, target_radius3),
            },
            FieldInfoData {
                name: "Opacity4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, opacity4),
            },
            FieldInfoData {
                name: "DistortionScale4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, distortion_scale4),
            },
            FieldInfoData {
                name: "SourcePos4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, source_pos4),
            },
            FieldInfoData {
                name: "TargetPos4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, target_pos4),
            },
            FieldInfoData {
                name: "SourceRadius4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, source_radius4),
            },
            FieldInfoData {
                name: "TargetRadius4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, target_radius4),
            },
            FieldInfoData {
                name: "Opacity5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, opacity5),
            },
            FieldInfoData {
                name: "DistortionScale5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, distortion_scale5),
            },
            FieldInfoData {
                name: "SourcePos5",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, source_pos5),
            },
            FieldInfoData {
                name: "TargetPos5",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, target_pos5),
            },
            FieldInfoData {
                name: "SourceRadius5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, source_radius5),
            },
            FieldInfoData {
                name: "TargetRadius5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, target_radius5),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(HOLOGRAMPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HologramParamsComponentState {
    fn type_info() -> &'static TypeInfo {
        HOLOGRAMPARAMSCOMPONENTSTATE_TYPE_INFO
    }
}


pub const HOLOGRAMPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("HologramParamsComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum HologramPlaneCount {
    #[default]
    HologramPlaneCount_Max = 5,
}

pub const HOLOGRAMPLANECOUNT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramPlaneCount",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(HOLOGRAMPLANECOUNT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for HologramPlaneCount {
    fn type_info() -> &'static TypeInfo {
        HOLOGRAMPLANECOUNT_TYPE_INFO
    }
}


pub const HOLOGRAMPLANECOUNT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramPlaneCount-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("HologramPlaneCount-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum HologramRenderMode {
    #[default]
    HologramRenderMode_ForwardSimple = 0,
    HologramRenderMode_Forward = 1,
}

pub const HOLOGRAMRENDERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramRenderMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(HOLOGRAMRENDERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for HologramRenderMode {
    fn type_info() -> &'static TypeInfo {
        HOLOGRAMRENDERMODE_TYPE_INFO
    }
}


pub const HOLOGRAMRENDERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramRenderMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("HologramRenderMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const THREATALERTHIGHLIGHTPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreatAlertHighlightParamsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, enable),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, color2),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, color3),
            },
            FieldInfoData {
                name: "Color4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, color4),
            },
            FieldInfoData {
                name: "Color5",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, color5),
            },
            FieldInfoData {
                name: "UseOutline",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, use_outline),
            },
            FieldInfoData {
                name: "OutlineOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, outline_opacity),
            },
            FieldInfoData {
                name: "UseScanLines",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, use_scan_lines),
            },
            FieldInfoData {
                name: "ScanLineOffset",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, scan_line_offset),
            },
            FieldInfoData {
                name: "ScanLineOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, scan_line_opacity),
            },
            FieldInfoData {
                name: "ScanLineThickness",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, scan_line_thickness),
            },
            FieldInfoData {
                name: "ScanLineSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, scan_line_spacing),
            },
            FieldInfoData {
                name: "UseHorizontalScanLines",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, use_horizontal_scan_lines),
            },
            FieldInfoData {
                name: "UseAltLines",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, use_alt_lines),
            },
            FieldInfoData {
                name: "AltLineOffset",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, alt_line_offset),
            },
            FieldInfoData {
                name: "AltLineOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, alt_line_opacity),
            },
            FieldInfoData {
                name: "AltLineThickness",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, alt_line_thickness),
            },
            FieldInfoData {
                name: "AltLineSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, alt_line_spacing),
            },
            FieldInfoData {
                name: "UseHorizontalAltLines",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, use_horizontal_alt_lines),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(THREATALERTHIGHLIGHTPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ThreatAlertHighlightParamsComponentState {
    fn type_info() -> &'static TypeInfo {
        THREATALERTHIGHLIGHTPARAMSCOMPONENTSTATE_TYPE_INFO
    }
}


pub const THREATALERTHIGHLIGHTPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreatAlertHighlightParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ThreatAlertHighlightParamsComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const OBJECTHIGHLIGHTPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightParamsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, enable),
            },
            FieldInfoData {
                name: "Brightness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, brightness),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color1),
            },
            FieldInfoData {
                name: "Color1Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color1_alpha),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color2),
            },
            FieldInfoData {
                name: "Color2Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color2_alpha),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color3),
            },
            FieldInfoData {
                name: "Color3Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color3_alpha),
            },
            FieldInfoData {
                name: "Color4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color4),
            },
            FieldInfoData {
                name: "Color4Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, color4_alpha),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OBJECTHIGHLIGHTPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ObjectHighlightParamsComponentState {
    fn type_info() -> &'static TypeInfo {
        OBJECTHIGHLIGHTPARAMSCOMPONENTSTATE_TYPE_INFO
    }
}


pub const OBJECTHIGHLIGHTPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ObjectHighlightParamsComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AntiAliasComponentState {
    pub enable: bool,
    pub disocclusion_rejection_factor: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const ANTIALIASCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiAliasComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntiAliasComponentState, enable),
            },
            FieldInfoData {
                name: "DisocclusionRejectionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntiAliasComponentState, disocclusion_rejection_factor),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(AntiAliasComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(AntiAliasComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(ANTIALIASCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AntiAliasComponentState {
    fn type_info() -> &'static TypeInfo {
        ANTIALIASCOMPONENTSTATE_TYPE_INFO
    }
}


pub const ANTIALIASCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiAliasComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("AntiAliasComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkyCelestialMultiComponentState {
    pub enable: bool,
    pub enabled_on_quality_levels: super::core::QualityScalableBool,
    pub draw_order: u32,
    pub planar_reflection: super::core::QualityScalableBool,
    pub render_in_sky_env_map: bool,
    pub write_alpha: bool,
    pub quad_count: super::core::QualityScalableInt,
    pub texture: super::render_base::TextureBaseAsset,
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

pub const SKYCELESTIALMULTICOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialMultiComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, enable),
            },
            FieldInfoData {
                name: "EnabledOnQualityLevels",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, enabled_on_quality_levels),
            },
            FieldInfoData {
                name: "DrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, draw_order),
            },
            FieldInfoData {
                name: "PlanarReflection",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, planar_reflection),
            },
            FieldInfoData {
                name: "RenderInSkyEnvMap",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, render_in_sky_env_map),
            },
            FieldInfoData {
                name: "WriteAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, write_alpha),
            },
            FieldInfoData {
                name: "QuadCount",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, quad_count),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, texture),
            },
            FieldInfoData {
                name: "UVStart",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, u_v_start),
            },
            FieldInfoData {
                name: "UVEnd",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, u_v_end),
            },
            FieldInfoData {
                name: "UVGrid",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, u_v_grid),
            },
            FieldInfoData {
                name: "Tint",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, tint),
            },
            FieldInfoData {
                name: "SkyEnvmapTintScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, sky_envmap_tint_scale),
            },
            FieldInfoData {
                name: "RandomSeed",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, random_seed),
            },
            FieldInfoData {
                name: "MinScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, min_scale),
            },
            FieldInfoData {
                name: "MaxScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, max_scale),
            },
            FieldInfoData {
                name: "ScaleMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, scale_multiplier),
            },
            FieldInfoData {
                name: "ZenithStop",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, zenith_stop),
            },
            FieldInfoData {
                name: "NadirStop",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, nadir_stop),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SKYCELESTIALMULTICOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCelestialMultiComponentState {
    fn type_info() -> &'static TypeInfo {
        SKYCELESTIALMULTICOMPONENTSTATE_TYPE_INFO
    }
}


pub const SKYCELESTIALMULTICOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialMultiComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyCelestialMultiComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkyCelestialSingleComponentState {
    pub enable: bool,
    pub enabled_on_quality_levels: super::core::QualityScalableBool,
    pub draw_order: u32,
    pub planar_reflection: super::core::QualityScalableBool,
    pub render_in_sky_env_map: bool,
    pub write_alpha: bool,
    pub texture: super::render_base::TextureBaseAsset,
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

pub const SKYCELESTIALSINGLECOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialSingleComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, enable),
            },
            FieldInfoData {
                name: "EnabledOnQualityLevels",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, enabled_on_quality_levels),
            },
            FieldInfoData {
                name: "DrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, draw_order),
            },
            FieldInfoData {
                name: "PlanarReflection",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, planar_reflection),
            },
            FieldInfoData {
                name: "RenderInSkyEnvMap",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, render_in_sky_env_map),
            },
            FieldInfoData {
                name: "WriteAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, write_alpha),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, texture),
            },
            FieldInfoData {
                name: "UVStart",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, u_v_start),
            },
            FieldInfoData {
                name: "UVEnd",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, u_v_end),
            },
            FieldInfoData {
                name: "Tint",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, tint),
            },
            FieldInfoData {
                name: "SkyEnvmapTintScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, sky_envmap_tint_scale),
            },
            FieldInfoData {
                name: "Longitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, longitude),
            },
            FieldInfoData {
                name: "Latitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, latitude),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, rotation),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SKYCELESTIALSINGLECOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCelestialSingleComponentState {
    fn type_info() -> &'static TypeInfo {
        SKYCELESTIALSINGLECOMPONENTSTATE_TYPE_INFO
    }
}


pub const SKYCELESTIALSINGLECOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialSingleComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyCelestialSingleComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkyCelestialRotationComponentState {
    pub enable: bool,
    pub rotation: f32,
    pub rotation_axis: super::core::Vec3,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const SKYCELESTIALROTATIONCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialRotationComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialRotationComponentState, enable),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialRotationComponentState, rotation),
            },
            FieldInfoData {
                name: "RotationAxis",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialRotationComponentState, rotation_axis),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialRotationComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialRotationComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SKYCELESTIALROTATIONCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCelestialRotationComponentState {
    fn type_info() -> &'static TypeInfo {
        SKYCELESTIALROTATIONCOMPONENTSTATE_TYPE_INFO
    }
}


pub const SKYCELESTIALROTATIONCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialRotationComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyCelestialRotationComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SkyCelestialComponentCountMax {
    #[default]
    SkyCelestialComponentCountMax_Single = 4,
    SkyCelestialComponentCountMax_Total = 8,
}

pub const SKYCELESTIALCOMPONENTCOUNTMAX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialComponentCountMax",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SKYCELESTIALCOMPONENTCOUNTMAX_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SkyCelestialComponentCountMax {
    fn type_info() -> &'static TypeInfo {
        SKYCELESTIALCOMPONENTCOUNTMAX_TYPE_INFO
    }
}


pub const SKYCELESTIALCOMPONENTCOUNTMAX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialComponentCountMax-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyCelestialComponentCountMax-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterParamGlobalComponentState {
    pub parameter: super::effect_base::EffectParameter,
    pub value: super::core::Vec4,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const EMITTERPARAMGLOBALCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Parameter",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamGlobalComponentState, parameter),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamGlobalComponentState, value),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamGlobalComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamGlobalComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(EMITTERPARAMGLOBALCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterParamGlobalComponentState {
    fn type_info() -> &'static TypeInfo {
        EMITTERPARAMGLOBALCOMPONENTSTATE_TYPE_INFO
    }
}


pub const EMITTERPARAMGLOBALCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EmitterParamGlobalComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterParamGlobalCountMax {
    #[default]
    EmitterParamGlobal_Count = 10,
}

pub const EMITTERPARAMGLOBALCOUNTMAX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalCountMax",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERPARAMGLOBALCOUNTMAX_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterParamGlobalCountMax {
    fn type_info() -> &'static TypeInfo {
        EMITTERPARAMGLOBALCOUNTMAX_TYPE_INFO
    }
}


pub const EMITTERPARAMGLOBALCOUNTMAX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalCountMax-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EmitterParamGlobalCountMax-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterParamComponentState {
    pub value: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const EMITTERPARAMCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamComponentState, value),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(EMITTERPARAMCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EmitterParamComponentState {
    fn type_info() -> &'static TypeInfo {
        EMITTERPARAMCOMPONENTSTATE_TYPE_INFO
    }
}


pub const EMITTERPARAMCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EmitterParamComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterParamOverride {
    #[default]
    EmitterParamOverride_EmitterParameter1 = 0,
    EmitterParamOverride_EmitterParameter2 = 1,
    EmitterParamOverride_EmitterParameter3 = 2,
    EmitterParamOverride_Count = 3,
}

pub const EMITTERPARAMOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamOverride",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERPARAMOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterParamOverride {
    fn type_info() -> &'static TypeInfo {
        EMITTERPARAMOVERRIDE_TYPE_INFO
    }
}


pub const EMITTERPARAMOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EmitterParamOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RaytraceReflectionComponentState {
    pub enable: bool,
    pub min_smoothness: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const RAYTRACEREFLECTIONCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaytraceReflectionComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaytraceReflectionComponentState, enable),
            },
            FieldInfoData {
                name: "MinSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RaytraceReflectionComponentState, min_smoothness),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RaytraceReflectionComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RaytraceReflectionComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RAYTRACEREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RaytraceReflectionComponentState {
    fn type_info() -> &'static TypeInfo {
        RAYTRACEREFLECTIONCOMPONENTSTATE_TYPE_INFO
    }
}


pub const RAYTRACEREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaytraceReflectionComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RaytraceReflectionComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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
    pub normal_fade_texture: super::render_base::TextureBaseAsset,
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

pub const SCREENSPACERAYTRACECOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenSpaceRaytraceComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "RaytraceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, raytrace_enable),
            },
            FieldInfoData {
                name: "CameraFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, camera_fade_start),
            },
            FieldInfoData {
                name: "CameraFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, camera_fade_length),
            },
            FieldInfoData {
                name: "RadialFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, radial_fade_start),
            },
            FieldInfoData {
                name: "RadialFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, radial_fade_length),
            },
            FieldInfoData {
                name: "DistanceFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, distance_fade_start),
            },
            FieldInfoData {
                name: "DistanceFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, distance_fade_length),
            },
            FieldInfoData {
                name: "ScreenFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, screen_fade_start),
            },
            FieldInfoData {
                name: "ScreenFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, screen_fade_length),
            },
            FieldInfoData {
                name: "BorderFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, border_fade_start),
            },
            FieldInfoData {
                name: "BorderFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, border_fade_length),
            },
            FieldInfoData {
                name: "MirrorFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, mirror_fade_start),
            },
            FieldInfoData {
                name: "MirrorFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, mirror_fade_length),
            },
            FieldInfoData {
                name: "ThicknessFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, thickness_fade_start),
            },
            FieldInfoData {
                name: "ThicknessFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, thickness_fade_length),
            },
            FieldInfoData {
                name: "RoughnessFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, roughness_fade_start),
            },
            FieldInfoData {
                name: "RoughnessFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, roughness_fade_length),
            },
            FieldInfoData {
                name: "NormalFadeTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, normal_fade_texture),
            },
            FieldInfoData {
                name: "MinSamples",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, min_samples),
            },
            FieldInfoData {
                name: "MaxSamples",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, max_samples),
            },
            FieldInfoData {
                name: "TemporalSamples",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, temporal_samples),
            },
            FieldInfoData {
                name: "TemporalPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, temporal_period),
            },
            FieldInfoData {
                name: "MinRoughness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, min_roughness),
            },
            FieldInfoData {
                name: "MaxRoughness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, max_roughness),
            },
            FieldInfoData {
                name: "ResolveSamples",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, resolve_samples),
            },
            FieldInfoData {
                name: "NoiseThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, noise_threshold),
            },
            FieldInfoData {
                name: "ClampThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, clamp_threshold),
            },
            FieldInfoData {
                name: "ImportanceSamplingBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, importance_sampling_bias),
            },
            FieldInfoData {
                name: "FilterBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, filter_bias),
            },
            FieldInfoData {
                name: "FilterAngularBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, filter_angular_bias),
            },
            FieldInfoData {
                name: "TemporalFilterResponsiveness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, temporal_filter_responsiveness),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SCREENSPACERAYTRACECOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScreenSpaceRaytraceComponentState {
    fn type_info() -> &'static TypeInfo {
        SCREENSPACERAYTRACECOMPONENTSTATE_TYPE_INFO
    }
}


pub const SCREENSPACERAYTRACECOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenSpaceRaytraceComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ScreenSpaceRaytraceComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const MOTIONBLURCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionBlurComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MotionBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, motion_blur_enable),
            },
            FieldInfoData {
                name: "MotionBlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, motion_blur_scale),
            },
            FieldInfoData {
                name: "MotionBlurCentered",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, motion_blur_centered),
            },
            FieldInfoData {
                name: "DepthCheckMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, depth_check_max_distance),
            },
            FieldInfoData {
                name: "RadialBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, radial_blur_enable),
            },
            FieldInfoData {
                name: "RadialBlurCenter",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, radial_blur_center),
            },
            FieldInfoData {
                name: "RadialBlurOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, radial_blur_offset),
            },
            FieldInfoData {
                name: "CircularOffsetFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, circular_offset_factor),
            },
            FieldInfoData {
                name: "RadialBlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, radial_blur_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MOTIONBLURCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MotionBlurComponentState {
    fn type_info() -> &'static TypeInfo {
        MOTIONBLURCOMPONENTSTATE_TYPE_INFO
    }
}


pub const MOTIONBLURCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionBlurComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("MotionBlurComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const CHARACTERLIGHTINGCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CharacterLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, character_light_enable),
            },
            FieldInfoData {
                name: "FirstPersonEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, first_person_enable),
            },
            FieldInfoData {
                name: "LockToCameraDirection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, lock_to_camera_direction),
            },
            FieldInfoData {
                name: "CameraUpRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, camera_up_rotation),
            },
            FieldInfoData {
                name: "CharacterLightingMode",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERLIGHTINGMODE_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, character_lighting_mode),
            },
            FieldInfoData {
                name: "BlendFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, blend_factor),
            },
            FieldInfoData {
                name: "TopLight",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, top_light),
            },
            FieldInfoData {
                name: "BottomLight",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, bottom_light),
            },
            FieldInfoData {
                name: "TopLightDirX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, top_light_dir_x),
            },
            FieldInfoData {
                name: "TopLightDirY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, top_light_dir_y),
            },
            FieldInfoData {
                name: "StartFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, start_fade_distance),
            },
            FieldInfoData {
                name: "EndFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, end_fade_distance),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CHARACTERLIGHTINGCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterLightingComponentState {
    fn type_info() -> &'static TypeInfo {
        CHARACTERLIGHTINGCOMPONENTSTATE_TYPE_INFO
    }
}


pub const CHARACTERLIGHTINGCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("CharacterLightingComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CharacterLightingMode {
    #[default]
    CharacterLightingMode_Add = 0,
    CharacterLightingMode_Blend = 1,
    CharacterLightingMode_Multiply = 2,
}

pub const CHARACTERLIGHTINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(CHARACTERLIGHTINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CharacterLightingMode {
    fn type_info() -> &'static TypeInfo {
        CHARACTERLIGHTINGMODE_TYPE_INFO
    }
}


pub const CHARACTERLIGHTINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("CharacterLightingMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DamageEffectComponentState {
    pub shader: super::render_base::SurfaceShaderBaseAsset,
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

pub const DAMAGEEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageEffectComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, shader),
            },
            FieldInfoData {
                name: "FrameWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, frame_width),
            },
            FieldInfoData {
                name: "OuterFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, outer_frame_opacity),
            },
            FieldInfoData {
                name: "InnerFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, inner_frame_opacity),
            },
            FieldInfoData {
                name: "FallofTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, fallof_time),
            },
            FieldInfoData {
                name: "MinDamagePercentageThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, min_damage_percentage_threshold),
            },
            FieldInfoData {
                name: "MaxOpacityDamagePercentage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, max_opacity_damage_percentage),
            },
            FieldInfoData {
                name: "StartCriticalEffectHealthThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, start_critical_effect_health_threshold),
            },
            FieldInfoData {
                name: "EndCriticalEffectHealthThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, end_critical_effect_health_threshold),
            },
            FieldInfoData {
                name: "DebugDamage",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, debug_damage),
            },
            FieldInfoData {
                name: "TopDamage",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, top_damage),
            },
            FieldInfoData {
                name: "LeftDamage",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, left_damage),
            },
            FieldInfoData {
                name: "BottomDamage",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, bottom_damage),
            },
            FieldInfoData {
                name: "RightDamage",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, right_damage),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DAMAGEEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DamageEffectComponentState {
    fn type_info() -> &'static TypeInfo {
        DAMAGEEFFECTCOMPONENTSTATE_TYPE_INFO
    }
}


pub const DAMAGEEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DamageEffectComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ScreenEffectComponentState {
    pub frame_type: ScreenEffectFrameType,
    pub shader: super::render_base::SurfaceShaderBaseAsset,
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

pub const SCREENEFFECTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FrameType",
                flags: MemberInfoFlags::new(0),
                field_type: SCREENEFFECTFRAMETYPE_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, frame_type),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, shader),
            },
            FieldInfoData {
                name: "FrameWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, frame_width),
            },
            FieldInfoData {
                name: "OuterFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, outer_frame_opacity),
            },
            FieldInfoData {
                name: "InnerFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, inner_frame_opacity),
            },
            FieldInfoData {
                name: "Angle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, angle),
            },
            FieldInfoData {
                name: "ScreenEffectParams",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, screen_effect_params),
            },
            FieldInfoData {
                name: "ActivePosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, active_position),
            },
            FieldInfoData {
                name: "StateId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, state_id),
            },
            FieldInfoData {
                name: "Visibilty",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, visibilty),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SCREENEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ScreenEffectComponentState {
    fn type_info() -> &'static TypeInfo {
        SCREENEFFECTCOMPONENTSTATE_TYPE_INFO
    }
}


pub const SCREENEFFECTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ScreenEffectComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ScreenEffectFrameType {
    #[default]
    ScreenEffectFrameType_FullFrame = 0,
    ScreenEffectFrameType_SingleFramePart = 1,
    ScreenEffectFrameType_SingleSquareFramePart = 2,
}

pub const SCREENEFFECTFRAMETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectFrameType",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SCREENEFFECTFRAMETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ScreenEffectFrameType {
    fn type_info() -> &'static TypeInfo {
        SCREENEFFECTFRAMETYPE_TYPE_INFO
    }
}


pub const SCREENEFFECTFRAMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectFrameType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ScreenEffectFrameType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const SHADOWSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DefSunShadowmapViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, def_sun_shadowmap_view_distance),
            },
            FieldInfoData {
                name: "SunShadowmapViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_view_distance),
            },
            FieldInfoData {
                name: "SunShadowmapExtrusionLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_extrusion_length),
            },
            FieldInfoData {
                name: "SunShadowmapSliceSchemeWeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_slice_scheme_weight),
            },
            FieldInfoData {
                name: "SunShadowmapFirstSliceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_first_slice_scale),
            },
            FieldInfoData {
                name: "SunShadowmapFirstSliceExtrusionLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_first_slice_extrusion_length),
            },
            FieldInfoData {
                name: "SmoothTransitionToDistantShadows",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, smooth_transition_to_distant_shadows),
            },
            FieldInfoData {
                name: "SunShadowmapSliceCountOffset",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_slice_count_offset),
            },
            FieldInfoData {
                name: "SunShadowmapSliceCountMin",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_slice_count_min),
            },
            FieldInfoData {
                name: "SunShadowmapSliceCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_slice_count_max),
            },
            FieldInfoData {
                name: "SunShadowmapSliceResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, sun_shadowmap_slice_resolution_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SHADOWSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ShadowsComponentState {
    fn type_info() -> &'static TypeInfo {
        SHADOWSCOMPONENTSTATE_TYPE_INFO
    }
}


pub const SHADOWSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ShadowsComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshSettingsComponentState {
    pub lod_scale: f32,
    pub force_lod: i32,
    pub cull_screen_area_scale: f32,
    pub shadow_distance_scale: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const MESHSETTINGSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSettingsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "LodScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentState, lod_scale),
            },
            FieldInfoData {
                name: "ForceLod",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentState, force_lod),
            },
            FieldInfoData {
                name: "CullScreenAreaScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentState, cull_screen_area_scale),
            },
            FieldInfoData {
                name: "ShadowDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentState, shadow_distance_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MESHSETTINGSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MeshSettingsComponentState {
    fn type_info() -> &'static TypeInfo {
        MESHSETTINGSCOMPONENTSTATE_TYPE_INFO
    }
}


pub const MESHSETTINGSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSettingsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("MeshSettingsComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CameraParamsComponentState {
    pub view_distance: f32,
    pub near_plane: f32,
    pub vegetation_max_wiggle_distance: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const CAMERAPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraParamsComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraParamsComponentState, view_distance),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraParamsComponentState, near_plane),
            },
            FieldInfoData {
                name: "VegetationMaxWiggleDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraParamsComponentState, vegetation_max_wiggle_distance),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(CameraParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(CameraParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CAMERAPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CameraParamsComponentState {
    fn type_info() -> &'static TypeInfo {
        CAMERAPARAMSCOMPONENTSTATE_TYPE_INFO
    }
}


pub const CAMERAPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("CameraParamsComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShaderColorParamsComponentState {
    pub vec4_value: super::core::Vec4,
    pub parameter_name: String,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const SHADERCOLORPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderColorParamsComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Vec4Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ShaderColorParamsComponentState, vec4_value),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ShaderColorParamsComponentState, parameter_name),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ShaderColorParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ShaderColorParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SHADERCOLORPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderColorParamsComponentState {
    fn type_info() -> &'static TypeInfo {
        SHADERCOLORPARAMSCOMPONENTSTATE_TYPE_INFO
    }
}


pub const SHADERCOLORPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderColorParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ShaderColorParamsComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShaderParamsComponentState {
    pub vec4_value: super::core::Vec4,
    pub bool_value: bool,
    pub texture_value: super::render_base::TextureBaseAsset,
    pub value_type: super::render_base::ExternalValueConstantType,
    pub conditional_value: u32,
    pub conditional_name: String,
    pub parameter_name: String,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const SHADERPARAMSCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParamsComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Vec4Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentState, vec4_value),
            },
            FieldInfoData {
                name: "BoolValue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentState, bool_value),
            },
            FieldInfoData {
                name: "TextureValue",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentState, texture_value),
            },
            FieldInfoData {
                name: "ValueType",
                flags: MemberInfoFlags::new(0),
                field_type: EXTERNALVALUECONSTANTTYPE_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentState, value_type),
            },
            FieldInfoData {
                name: "ConditionalValue",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentState, conditional_value),
            },
            FieldInfoData {
                name: "ConditionalName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentState, conditional_name),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentState, parameter_name),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SHADERPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderParamsComponentState {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMSCOMPONENTSTATE_TYPE_INFO
    }
}


pub const SHADERPARAMSCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParamsComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ShaderParamsComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const ENLIGHTENCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BounceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, bounce_scale),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sun_scale),
            },
            FieldInfoData {
                name: "SunDirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sun_direct_lightmap_enable),
            },
            FieldInfoData {
                name: "TerrainColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, terrain_color),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, cull_distance),
            },
            FieldInfoData {
                name: "CullRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, cull_radius),
            },
            FieldInfoData {
                name: "SkyBoxEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_enable),
            },
            FieldInfoData {
                name: "SkyBoxCutBottom",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_cut_bottom),
            },
            FieldInfoData {
                name: "SkyBoxBlendMode",
                flags: MemberInfoFlags::new(0),
                field_type: SKYBOXBLENDMODE_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_blend_mode),
            },
            FieldInfoData {
                name: "SkyBoxBlend",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_blend),
            },
            FieldInfoData {
                name: "SkyBoxSkyColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_sky_color),
            },
            FieldInfoData {
                name: "SkyBoxGroundColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_ground_color),
            },
            FieldInfoData {
                name: "SkyBoxSunLightColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_sun_light_color),
            },
            FieldInfoData {
                name: "SkyBoxSunLightColorSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_sun_light_color_size),
            },
            FieldInfoData {
                name: "SkyBoxBackLightColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_back_light_color),
            },
            FieldInfoData {
                name: "SkyBoxBackLightColorSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_back_light_color_size),
            },
            FieldInfoData {
                name: "SkyBoxBackLightRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_back_light_rotation_x),
            },
            FieldInfoData {
                name: "SkyBoxBackLightRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, sky_box_back_light_rotation_y),
            },
            FieldInfoData {
                name: "OpaqueAlphaTestSimpleScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, opaque_alpha_test_simple_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(ENLIGHTENCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnlightenComponentState {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENCOMPONENTSTATE_TYPE_INFO
    }
}


pub const ENLIGHTENCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EnlightenComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SkyBoxBlendMode {
    #[default]
    SkyBoxBlendMode_Lerp = 0,
    SkyBoxBlendMode_Add = 1,
    SkyBoxBlendMode_Multiply = 2,
}

pub const SKYBOXBLENDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyBoxBlendMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SKYBOXBLENDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SkyBoxBlendMode {
    fn type_info() -> &'static TypeInfo {
        SKYBOXBLENDMODE_TYPE_INFO
    }
}


pub const SKYBOXBLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyBoxBlendMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyBoxBlendMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const SUBSURFACESCATTERINGCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceScatteringComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SimpleSssColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, simple_sss_color),
            },
            FieldInfoData {
                name: "SimpleSssRolloffKeyLight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, simple_sss_rolloff_key_light),
            },
            FieldInfoData {
                name: "SimpleSssRolloffLocalLight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, simple_sss_rolloff_local_light),
            },
            FieldInfoData {
                name: "LocalLightTranslucencyEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, local_light_translucency_enable),
            },
            FieldInfoData {
                name: "Profile0",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile0),
            },
            FieldInfoData {
                name: "Profile1",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile1),
            },
            FieldInfoData {
                name: "Profile2",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile2),
            },
            FieldInfoData {
                name: "Profile3",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile3),
            },
            FieldInfoData {
                name: "Profile4",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile4),
            },
            FieldInfoData {
                name: "Profile5",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile5),
            },
            FieldInfoData {
                name: "Profile6",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile6),
            },
            FieldInfoData {
                name: "ProfileOATS",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, profile_o_a_t_s),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SUBSURFACESCATTERINGCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SubSurfaceScatteringComponentState {
    fn type_info() -> &'static TypeInfo {
        SUBSURFACESCATTERINGCOMPONENTSTATE_TYPE_INFO
    }
}


pub const SUBSURFACESCATTERINGCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceScatteringComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SubSurfaceScatteringComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const DYNAMICAOCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicAOComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, enable),
            },
            FieldInfoData {
                name: "AffectOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, affect_outdoor_light),
            },
            FieldInfoData {
                name: "AffectLocalLight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, affect_local_light),
            },
            FieldInfoData {
                name: "DynamicAOFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, dynamic_a_o_factor),
            },
            FieldInfoData {
                name: "SsaoFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, ssao_fade),
            },
            FieldInfoData {
                name: "SsaoRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, ssao_radius),
            },
            FieldInfoData {
                name: "SsaoMaxDistanceInner",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, ssao_max_distance_inner),
            },
            FieldInfoData {
                name: "SsaoMaxDistanceOuter",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, ssao_max_distance_outer),
            },
            FieldInfoData {
                name: "HbaoRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, hbao_radius),
            },
            FieldInfoData {
                name: "HbaoAngleBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, hbao_angle_bias),
            },
            FieldInfoData {
                name: "HbaoAttenuation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, hbao_attenuation),
            },
            FieldInfoData {
                name: "HbaoContrast",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, hbao_contrast),
            },
            FieldInfoData {
                name: "HbaoMaxFootprintRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, hbao_max_footprint_radius),
            },
            FieldInfoData {
                name: "HbaoPowerExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, hbao_power_exponent),
            },
            FieldInfoData {
                name: "HbaoBlurRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, hbao_blur_radius),
            },
            FieldInfoData {
                name: "HbaoBlurSharpness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, hbao_blur_sharpness),
            },
            FieldInfoData {
                name: "TemporalFilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, temporal_filter_enable),
            },
            FieldInfoData {
                name: "AaoDynamicWeight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_dynamic_weight),
            },
            FieldInfoData {
                name: "AaoBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_bias),
            },
            FieldInfoData {
                name: "AaoIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_intensity),
            },
            FieldInfoData {
                name: "AaoContrast",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_contrast),
            },
            FieldInfoData {
                name: "AaoRangeReduction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_range_reduction),
            },
            FieldInfoData {
                name: "AaoScreenRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_screen_radius),
            },
            FieldInfoData {
                name: "AaoNearOcclusionMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_near_occlusion_max),
            },
            FieldInfoData {
                name: "AaoNearFalloffThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_near_falloff_threshold),
            },
            FieldInfoData {
                name: "AaoClipDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_clip_distance),
            },
            FieldInfoData {
                name: "AaoClipFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_clip_fade_distance),
            },
            FieldInfoData {
                name: "AaoBlurDepthThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_blur_depth_threshold),
            },
            FieldInfoData {
                name: "AaoBlurConstFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, aao_blur_const_falloff),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DYNAMICAOCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DynamicAOComponentState {
    fn type_info() -> &'static TypeInfo {
        DYNAMICAOCOMPONENTSTATE_TYPE_INFO
    }
}


pub const DYNAMICAOCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicAOComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DynamicAOComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const SUNFLARECOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, enable),
            },
            FieldInfoData {
                name: "DebugDrawOccluder",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, debug_draw_occluder),
            },
            FieldInfoData {
                name: "OccluderSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, occluder_size),
            },
            FieldInfoData {
                name: "ScreenClip",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, screen_clip),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: LENSFLARERENDERMODE_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, render_mode),
            },
            FieldInfoData {
                name: "UseSunPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, use_sun_position),
            },
            FieldInfoData {
                name: "RotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, rotation_x),
            },
            FieldInfoData {
                name: "RotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, rotation_y),
            },
            FieldInfoData {
                name: "Element1Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_enable),
            },
            FieldInfoData {
                name: "Element1Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_shader),
            },
            FieldInfoData {
                name: "Element1RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_ray_distance),
            },
            FieldInfoData {
                name: "Element1Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_size),
            },
            FieldInfoData {
                name: "Element1SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element1SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element1AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element1AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element1RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_rotation_local),
            },
            FieldInfoData {
                name: "Element1RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element1RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element1RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element1_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element2Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_enable),
            },
            FieldInfoData {
                name: "Element2Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_shader),
            },
            FieldInfoData {
                name: "Element2RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_ray_distance),
            },
            FieldInfoData {
                name: "Element2Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_size),
            },
            FieldInfoData {
                name: "Element2SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element2SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element2AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element2AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element2RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_rotation_local),
            },
            FieldInfoData {
                name: "Element2RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element2RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element2RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element2_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element3Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_enable),
            },
            FieldInfoData {
                name: "Element3Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_shader),
            },
            FieldInfoData {
                name: "Element3RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_ray_distance),
            },
            FieldInfoData {
                name: "Element3Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_size),
            },
            FieldInfoData {
                name: "Element3SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element3SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element3AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element3AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element3RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_rotation_local),
            },
            FieldInfoData {
                name: "Element3RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element3RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element3RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element3_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element4Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_enable),
            },
            FieldInfoData {
                name: "Element4Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_shader),
            },
            FieldInfoData {
                name: "Element4RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_ray_distance),
            },
            FieldInfoData {
                name: "Element4Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_size),
            },
            FieldInfoData {
                name: "Element4SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element4SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element4AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element4AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element4RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_rotation_local),
            },
            FieldInfoData {
                name: "Element4RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element4RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element4RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element4_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element5Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_enable),
            },
            FieldInfoData {
                name: "Element5Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_shader),
            },
            FieldInfoData {
                name: "Element5RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_ray_distance),
            },
            FieldInfoData {
                name: "Element5Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_size),
            },
            FieldInfoData {
                name: "Element5SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element5SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element5AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element5AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element5RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_rotation_local),
            },
            FieldInfoData {
                name: "Element5RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element5RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element5RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, element5_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagOverride2",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, field_flag_override2),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, field_flag_changed1),
            },
            FieldInfoData {
                name: "FieldFlagChanged2",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentState, field_flag_changed2),
            },
        ],
    }),
    array_type: Some(SUNFLARECOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SunFlareComponentState {
    fn type_info() -> &'static TypeInfo {
        SUNFLARECOMPONENTSTATE_TYPE_INFO
    }
}


pub const SUNFLARECOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SunFlareComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const WINDCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "WindDirection",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentState, wind_direction),
            },
            FieldInfoData {
                name: "WindStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentState, wind_strength),
            },
            FieldInfoData {
                name: "WindVariationMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentState, wind_variation_multiplier),
            },
            FieldInfoData {
                name: "WindVariationRateMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentState, wind_variation_rate_multiplier),
            },
            FieldInfoData {
                name: "WindMicroVariationMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentState, wind_micro_variation_multiplier),
            },
            FieldInfoData {
                name: "TurbulenceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentState, turbulence_multiplier),
            },
            FieldInfoData {
                name: "TurbulenceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentState, turbulence_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(WindComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(WindComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WINDCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WindComponentState {
    fn type_info() -> &'static TypeInfo {
        WINDCOMPONENTSTATE_TYPE_INFO
    }
}


pub const WINDCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("WindComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DynamicEnvmapComponentState {
    pub enable: bool,
    pub terrain_reflections_enable: bool,
    pub key_color_envmap: super::core::Vec3,
    pub sky_color_envmap: super::core::Vec3,
    pub ground_color_envmap: super::core::Vec3,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const DYNAMICENVMAPCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnvmapComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentState, enable),
            },
            FieldInfoData {
                name: "TerrainReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentState, terrain_reflections_enable),
            },
            FieldInfoData {
                name: "KeyColorEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentState, key_color_envmap),
            },
            FieldInfoData {
                name: "SkyColorEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentState, sky_color_envmap),
            },
            FieldInfoData {
                name: "GroundColorEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentState, ground_color_envmap),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DYNAMICENVMAPCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DynamicEnvmapComponentState {
    fn type_info() -> &'static TypeInfo {
        DYNAMICENVMAPCOMPONENTSTATE_TYPE_INFO
    }
}


pub const DYNAMICENVMAPCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnvmapComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DynamicEnvmapComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const PLANARREFLECTIONCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, enable),
            },
            FieldInfoData {
                name: "TerrainReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, terrain_reflections_enable),
            },
            FieldInfoData {
                name: "SkyRenderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, sky_render_enable),
            },
            FieldInfoData {
                name: "GroundHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, ground_height),
            },
            FieldInfoData {
                name: "ViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, view_distance),
            },
            FieldInfoData {
                name: "VerticalBlurFilter",
                flags: MemberInfoFlags::new(0),
                field_type: BLURFILTER_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, vertical_blur_filter),
            },
            FieldInfoData {
                name: "VerticalDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, vertical_deviation),
            },
            FieldInfoData {
                name: "HorizontalBlurFilter",
                flags: MemberInfoFlags::new(0),
                field_type: BLURFILTER_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, horizontal_blur_filter),
            },
            FieldInfoData {
                name: "HorizontalDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, horizontal_deviation),
            },
            FieldInfoData {
                name: "ClippingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, clipping_offset),
            },
            FieldInfoData {
                name: "OverideOutdoorLightColors",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, overide_outdoor_light_colors),
            },
            FieldInfoData {
                name: "KeyColorReflection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, key_color_reflection),
            },
            FieldInfoData {
                name: "SkyColorReflection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, sky_color_reflection),
            },
            FieldInfoData {
                name: "GroundColorReflection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, ground_color_reflection),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PLANARREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PlanarReflectionComponentState {
    fn type_info() -> &'static TypeInfo {
        PLANARREFLECTIONCOMPONENTSTATE_TYPE_INFO
    }
}


pub const PLANARREFLECTIONCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PlanarReflectionComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkyComponentState {
    pub enable: bool,
    pub draw_sky_geo: bool,
    pub sky_type: SkyType,
    pub luminance_scale: f32,
    pub sky_gradient_texture: super::render_base::TextureBaseAsset,
    pub alpha_output: AlphaOutputMode,
    pub use_sky_visibility_as_a_o: bool,
    pub hdri_rotation: f32,
    pub hdri_texture: super::render_base::TextureBaseAsset,
    pub sun_size: f32,
    pub sun_scale: f32,
    pub panoramic_u_v_min_x: f32,
    pub panoramic_u_v_max_x: f32,
    pub panoramic_u_v_min_y: f32,
    pub panoramic_u_v_max_y: f32,
    pub panoramic_tile_factor: f32,
    pub panoramic_rotation: f32,
    pub panoramic_texture: super::render_base::TextureBaseAsset,
    pub panoramic_alpha_texture: super::render_base::TextureBaseAsset,
    pub sky_gradient_follows_panoramic_u_vs: bool,
    pub flow_enable: bool,
    pub flow_period: f32,
    pub flow_distance: f32,
    pub flow_direction: f32,
    pub flow_height_mask_scale: f32,
    pub flow_height_mask_bias: f32,
    pub flow_mask_texture: super::render_base::TextureBaseAsset,
    pub cloud_layer_sun_color: super::core::Vec3,
    pub cloud_layer_mask_texture: super::render_base::TextureBaseAsset,
    pub cloud_layer1_altitude: f32,
    pub cloud_layer1_tile_factor: f32,
    pub cloud_layer1_rotation: f32,
    pub cloud_layer1_speed: f32,
    pub cloud_layer1_sun_light_intensity: f32,
    pub cloud_layer1_sun_light_power: f32,
    pub cloud_layer1_ambient_light_intensity: f32,
    pub cloud_layer1_color: super::core::Vec3,
    pub cloud_layer1_alpha_mul: f32,
    pub cloud_layer1_texture: super::render_base::TextureBaseAsset,
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
    pub cloud_layer2_texture: super::render_base::TextureBaseAsset,
    pub cloud_layer2_absorption: f32,
    pub cloud_layer2_scattering: f32,
    pub cloud_layer2_phase: f32,
    pub cloud_layer2_thickness: f32,
    pub static_envmap_texture: super::render_base::TextureBaseAsset,
    pub static_envmap_scale: f32,
    pub sky_envmap8_bit_tex_scale: f32,
    pub custom_envmap_texture: super::render_base::TextureBaseAsset,
    pub custom_envmap_scale: f32,
    pub custom_envmap_ambient: f32,
    pub sky_visibility_exponent: f32,
    pub interior_envmap_texture: super::render_base::TextureBaseAsset,
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
    pub gradient_blend_texture: super::render_base::TextureBaseAsset,
    pub panoramic_blend_texture: super::render_base::TextureBaseAsset,
    pub panoramic_alpha_blend_texture: super::render_base::TextureBaseAsset,
    pub static_envmap_blend_texture: super::render_base::TextureBaseAsset,
    pub flow_mask_blend_texture: super::render_base::TextureBaseAsset,
    pub field_flag_override0: u32,
    pub field_flag_override1: u32,
    pub field_flag_override2: u32,
    pub field_flag_override3: u32,
    pub field_flag_changed0: u32,
    pub field_flag_changed1: u32,
    pub field_flag_changed2: u32,
    pub field_flag_changed3: u32,
}

pub const SKYCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, enable),
            },
            FieldInfoData {
                name: "DrawSkyGeo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, draw_sky_geo),
            },
            FieldInfoData {
                name: "SkyType",
                flags: MemberInfoFlags::new(0),
                field_type: SKYTYPE_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, sky_type),
            },
            FieldInfoData {
                name: "LuminanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, luminance_scale),
            },
            FieldInfoData {
                name: "SkyGradientTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, sky_gradient_texture),
            },
            FieldInfoData {
                name: "AlphaOutput",
                flags: MemberInfoFlags::new(0),
                field_type: ALPHAOUTPUTMODE_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, alpha_output),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "HdriRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, hdri_rotation),
            },
            FieldInfoData {
                name: "HdriTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, hdri_texture),
            },
            FieldInfoData {
                name: "SunSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, sun_size),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, sun_scale),
            },
            FieldInfoData {
                name: "PanoramicUVMinX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_u_v_min_x),
            },
            FieldInfoData {
                name: "PanoramicUVMaxX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_u_v_max_x),
            },
            FieldInfoData {
                name: "PanoramicUVMinY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_u_v_min_y),
            },
            FieldInfoData {
                name: "PanoramicUVMaxY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_u_v_max_y),
            },
            FieldInfoData {
                name: "PanoramicTileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_tile_factor),
            },
            FieldInfoData {
                name: "PanoramicRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_rotation),
            },
            FieldInfoData {
                name: "PanoramicTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_texture),
            },
            FieldInfoData {
                name: "PanoramicAlphaTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_alpha_texture),
            },
            FieldInfoData {
                name: "SkyGradientFollowsPanoramicUVs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, sky_gradient_follows_panoramic_u_vs),
            },
            FieldInfoData {
                name: "FlowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, flow_enable),
            },
            FieldInfoData {
                name: "FlowPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, flow_period),
            },
            FieldInfoData {
                name: "FlowDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, flow_distance),
            },
            FieldInfoData {
                name: "FlowDirection",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, flow_direction),
            },
            FieldInfoData {
                name: "FlowHeightMaskScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, flow_height_mask_scale),
            },
            FieldInfoData {
                name: "FlowHeightMaskBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, flow_height_mask_bias),
            },
            FieldInfoData {
                name: "FlowMaskTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, flow_mask_texture),
            },
            FieldInfoData {
                name: "CloudLayerSunColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer_sun_color),
            },
            FieldInfoData {
                name: "CloudLayerMaskTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer_mask_texture),
            },
            FieldInfoData {
                name: "CloudLayer1Altitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_altitude),
            },
            FieldInfoData {
                name: "CloudLayer1TileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_tile_factor),
            },
            FieldInfoData {
                name: "CloudLayer1Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_rotation),
            },
            FieldInfoData {
                name: "CloudLayer1Speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_speed),
            },
            FieldInfoData {
                name: "CloudLayer1SunLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_sun_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer1SunLightPower",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_sun_light_power),
            },
            FieldInfoData {
                name: "CloudLayer1AmbientLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_ambient_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer1Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_color),
            },
            FieldInfoData {
                name: "CloudLayer1AlphaMul",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_alpha_mul),
            },
            FieldInfoData {
                name: "CloudLayer1Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_texture),
            },
            FieldInfoData {
                name: "CloudLayer1Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_absorption),
            },
            FieldInfoData {
                name: "CloudLayer1Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_scattering),
            },
            FieldInfoData {
                name: "CloudLayer1Phase",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_phase),
            },
            FieldInfoData {
                name: "CloudLayer1Thickness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer1_thickness),
            },
            FieldInfoData {
                name: "CloudLayer2Altitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_altitude),
            },
            FieldInfoData {
                name: "CloudLayer2TileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_tile_factor),
            },
            FieldInfoData {
                name: "CloudLayer2Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_rotation),
            },
            FieldInfoData {
                name: "CloudLayer2Speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_speed),
            },
            FieldInfoData {
                name: "CloudLayer2SunLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_sun_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer2SunLightPower",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_sun_light_power),
            },
            FieldInfoData {
                name: "CloudLayer2AmbientLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_ambient_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer2Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_color),
            },
            FieldInfoData {
                name: "CloudLayer2AlphaMul",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_alpha_mul),
            },
            FieldInfoData {
                name: "CloudLayer2Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_texture),
            },
            FieldInfoData {
                name: "CloudLayer2Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_absorption),
            },
            FieldInfoData {
                name: "CloudLayer2Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_scattering),
            },
            FieldInfoData {
                name: "CloudLayer2Phase",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_phase),
            },
            FieldInfoData {
                name: "CloudLayer2Thickness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, cloud_layer2_thickness),
            },
            FieldInfoData {
                name: "StaticEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, static_envmap_texture),
            },
            FieldInfoData {
                name: "StaticEnvmapScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, static_envmap_scale),
            },
            FieldInfoData {
                name: "SkyEnvmap8BitTexScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, sky_envmap8_bit_tex_scale),
            },
            FieldInfoData {
                name: "CustomEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, custom_envmap_texture),
            },
            FieldInfoData {
                name: "CustomEnvmapScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, custom_envmap_scale),
            },
            FieldInfoData {
                name: "CustomEnvmapAmbient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, custom_envmap_ambient),
            },
            FieldInfoData {
                name: "SkyVisibilityExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, sky_visibility_exponent),
            },
            FieldInfoData {
                name: "InteriorEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, interior_envmap_texture),
            },
            FieldInfoData {
                name: "InteriorEnvmapExp",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, interior_envmap_exp),
            },
            FieldInfoData {
                name: "InteriorEnvmapScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, interior_envmap_scale),
            },
            FieldInfoData {
                name: "InteriorEnvmapBias",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, interior_envmap_bias),
            },
            FieldInfoData {
                name: "InteriorEnvmapSkyVisibilityFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, interior_envmap_sky_visibility_fade_start),
            },
            FieldInfoData {
                name: "InteriorEnvmapSkyVisibilityFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, interior_envmap_sky_visibility_fade_length),
            },
            FieldInfoData {
                name: "EarthRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, earth_radius),
            },
            FieldInfoData {
                name: "AtmosphereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, atmosphere_radius),
            },
            FieldInfoData {
                name: "MieScatteringCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, mie_scattering_coefficient),
            },
            FieldInfoData {
                name: "MieG",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, mie_g),
            },
            FieldInfoData {
                name: "MieExtinctionCoefficientRelation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, mie_extinction_coefficient_relation),
            },
            FieldInfoData {
                name: "ScaleHeightMie",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, scale_height_mie),
            },
            FieldInfoData {
                name: "RayleighScatteringCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, rayleigh_scattering_coefficient),
            },
            FieldInfoData {
                name: "RayleighScatteringCoefficientScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, rayleigh_scattering_coefficient_scale),
            },
            FieldInfoData {
                name: "RayleighExtinctionCoefficientRelation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, rayleigh_extinction_coefficient_relation),
            },
            FieldInfoData {
                name: "ScaleHeightRayleigh",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, scale_height_rayleigh),
            },
            FieldInfoData {
                name: "UseOzone",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, use_ozone),
            },
            FieldInfoData {
                name: "OzonePercentage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, ozone_percentage),
            },
            FieldInfoData {
                name: "UseAerialPerspective",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, use_aerial_perspective),
            },
            FieldInfoData {
                name: "AerialPerspectiveScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, aerial_perspective_scale),
            },
            FieldInfoData {
                name: "AerialPerspectiveIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, aerial_perspective_intensity),
            },
            FieldInfoData {
                name: "AerialPerspectiveDithering",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, aerial_perspective_dithering),
            },
            FieldInfoData {
                name: "Light1Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, light1_color),
            },
            FieldInfoData {
                name: "Light1Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, light1_intensity),
            },
            FieldInfoData {
                name: "Light1FollowOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, light1_follow_outdoor_light),
            },
            FieldInfoData {
                name: "Light1TakesColorFromOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, light1_takes_color_from_outdoor_light),
            },
            FieldInfoData {
                name: "Light1RotX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, light1_rot_x),
            },
            FieldInfoData {
                name: "Light1RotY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, light1_rot_y),
            },
            FieldInfoData {
                name: "Light2Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, light2_color),
            },
            FieldInfoData {
                name: "Light2Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, light2_intensity),
            },
            FieldInfoData {
                name: "UseLightSource2",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, use_light_source2),
            },
            FieldInfoData {
                name: "Light2RotX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, light2_rot_x),
            },
            FieldInfoData {
                name: "Light2RotY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, light2_rot_y),
            },
            FieldInfoData {
                name: "UseNoise",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, use_noise),
            },
            FieldInfoData {
                name: "FogStartDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, fog_start_distance),
            },
            FieldInfoData {
                name: "RayleighPolarization",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, rayleigh_polarization),
            },
            FieldInfoData {
                name: "MiePolarization",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, mie_polarization),
            },
            FieldInfoData {
                name: "OutdoorLightScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, outdoor_light_scale),
            },
            FieldInfoData {
                name: "DrawSunDisc",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, draw_sun_disc),
            },
            FieldInfoData {
                name: "ForwardScatteringDepthVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, forward_scattering_depth_visibility),
            },
            FieldInfoData {
                name: "ForwardScatteringStartDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, forward_scattering_start_depth),
            },
            FieldInfoData {
                name: "ForwardScatteringEndDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, forward_scattering_end_depth),
            },
            FieldInfoData {
                name: "ForwardScatteringTakesColorFromOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, forward_scattering_takes_color_from_outdoor_light),
            },
            FieldInfoData {
                name: "ForwardScatteringOutdoorLightTint",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, forward_scattering_outdoor_light_tint),
            },
            FieldInfoData {
                name: "HeightFogColorAdd",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, height_fog_color_add),
            },
            FieldInfoData {
                name: "HeightFogColorMult",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, height_fog_color_mult),
            },
            FieldInfoData {
                name: "MinHeightFogTransmittance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, min_height_fog_transmittance),
            },
            FieldInfoData {
                name: "GradientTextureBlendFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, gradient_texture_blend_factor),
            },
            FieldInfoData {
                name: "PanoramicBlendRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_blend_rotation),
            },
            FieldInfoData {
                name: "GradientBlendTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, gradient_blend_texture),
            },
            FieldInfoData {
                name: "PanoramicBlendTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_blend_texture),
            },
            FieldInfoData {
                name: "PanoramicAlphaBlendTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, panoramic_alpha_blend_texture),
            },
            FieldInfoData {
                name: "StaticEnvmapBlendTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, static_envmap_blend_texture),
            },
            FieldInfoData {
                name: "FlowMaskBlendTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, flow_mask_blend_texture),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagOverride2",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, field_flag_override2),
            },
            FieldInfoData {
                name: "FieldFlagOverride3",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, field_flag_override3),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, field_flag_changed1),
            },
            FieldInfoData {
                name: "FieldFlagChanged2",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, field_flag_changed2),
            },
            FieldInfoData {
                name: "FieldFlagChanged3",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentState, field_flag_changed3),
            },
        ],
    }),
    array_type: Some(SKYCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyComponentState {
    fn type_info() -> &'static TypeInfo {
        SKYCOMPONENTSTATE_TYPE_INFO
    }
}


pub const SKYCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SkyType {
    #[default]
    SkyType_Procedural = 0,
    SkyType_Hdri = 1,
    SkyType_Physical = 2,
}

pub const SKYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyType",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SKYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SkyType {
    fn type_info() -> &'static TypeInfo {
        SKYTYPE_TYPE_INFO
    }
}


pub const SKYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SkyType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const ALPHAOUTPUTMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AlphaOutputMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(ALPHAOUTPUTMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AlphaOutputMode {
    fn type_info() -> &'static TypeInfo {
        ALPHAOUTPUTMODE_TYPE_INFO
    }
}


pub const ALPHAOUTPUTMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AlphaOutputMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("AlphaOutputMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const FOGCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, enable),
            },
            FieldInfoData {
                name: "FogDistanceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fog_distance_multiplier),
            },
            FieldInfoData {
                name: "FogGradientEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fog_gradient_enable),
            },
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, start),
            },
            FieldInfoData {
                name: "End",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, end),
            },
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, curve),
            },
            FieldInfoData {
                name: "FogGradientHeightFadeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fog_gradient_height_fade_enable),
            },
            FieldInfoData {
                name: "FadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fade_start),
            },
            FieldInfoData {
                name: "FadeEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fade_end),
            },
            FieldInfoData {
                name: "FogColorEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fog_color_enable),
            },
            FieldInfoData {
                name: "FogColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fog_color),
            },
            FieldInfoData {
                name: "FogColorStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fog_color_start),
            },
            FieldInfoData {
                name: "FogColorEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fog_color_end),
            },
            FieldInfoData {
                name: "FogColorCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fog_color_curve),
            },
            FieldInfoData {
                name: "TransparencyFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, transparency_fade_start),
            },
            FieldInfoData {
                name: "TransparencyFadeEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, transparency_fade_end),
            },
            FieldInfoData {
                name: "TransparencyFadeClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, transparency_fade_clamp),
            },
            FieldInfoData {
                name: "TransparencyFadeCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, transparency_fade_curve),
            },
            FieldInfoData {
                name: "ForwardLightScatteringEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_enabled),
            },
            FieldInfoData {
                name: "ForwardLightScatteringUseSunPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_use_sun_position),
            },
            FieldInfoData {
                name: "ForwardLightScatteringRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_rotation_x),
            },
            FieldInfoData {
                name: "ForwardLightScatteringRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_rotation_y),
            },
            FieldInfoData {
                name: "ForwardLightScatteringPhaseG",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_phase_g),
            },
            FieldInfoData {
                name: "ForwardLightScatteringStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_strength),
            },
            FieldInfoData {
                name: "ForwardLightScatteringColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_color),
            },
            FieldInfoData {
                name: "ForwardLightScatteringPresence",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_presence),
            },
            FieldInfoData {
                name: "ForwardLightScatteringMaxBlurLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_max_blur_length),
            },
            FieldInfoData {
                name: "ForwardLightScatteringExtinction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_extinction),
            },
            FieldInfoData {
                name: "ForwardLightScatteringSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_smoothness),
            },
            FieldInfoData {
                name: "ForwardLightScatteringAttenuationType",
                flags: MemberInfoFlags::new(0),
                field_type: FORWARDLIGHTSCATTERINGATTENUATION_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, forward_light_scattering_attenuation_type),
            },
            FieldInfoData {
                name: "HeightFogEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, height_fog_enable),
            },
            FieldInfoData {
                name: "HeightFogFollowCamera",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, height_fog_follow_camera),
            },
            FieldInfoData {
                name: "HeightFogAltitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, height_fog_altitude),
            },
            FieldInfoData {
                name: "HeightFogDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, height_fog_depth),
            },
            FieldInfoData {
                name: "HeightFogVisibilityRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, height_fog_visibility_range),
            },
            FieldInfoData {
                name: "ParticipatingMediaEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, participating_media_enable),
            },
            FieldInfoData {
                name: "DepthFogParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICIPATINGMEDIA_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, depth_fog_participating_media),
            },
            FieldInfoData {
                name: "HeightFogParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICIPATINGMEDIA_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, height_fog_participating_media),
            },
            FieldInfoData {
                name: "FogVolumeStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, fog_volume_strength),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(FogComponentState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(FOGCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FogComponentState {
    fn type_info() -> &'static TypeInfo {
        FOGCOMPONENTSTATE_TYPE_INFO
    }
}


pub const FOGCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FogComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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
    pub cloud_shadow_texture: super::render_base::TextureBaseAsset,
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
    pub secondary_cloud_shadow_texture: super::render_base::TextureBaseAsset,
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

pub const OUTDOORLIGHTCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightComponentState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, enable),
            },
            FieldInfoData {
                name: "SunRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_rotation_x),
            },
            FieldInfoData {
                name: "SunRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_rotation_y),
            },
            FieldInfoData {
                name: "ShadowSunRotationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, shadow_sun_rotation_enable),
            },
            FieldInfoData {
                name: "ShadowSunRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, shadow_sun_rotation_x),
            },
            FieldInfoData {
                name: "ShadowSunRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, shadow_sun_rotation_y),
            },
            FieldInfoData {
                name: "SunColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_color),
            },
            FieldInfoData {
                name: "SunIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_intensity),
            },
            FieldInfoData {
                name: "FinalSunLuminance",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, final_sun_luminance),
            },
            FieldInfoData {
                name: "FinalSunIlluminance",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, final_sun_illuminance),
            },
            FieldInfoData {
                name: "OuterSpaceSunLuminance",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, outer_space_sun_luminance),
            },
            FieldInfoData {
                name: "OuterSpaceSunIlluminance1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, outer_space_sun_illuminance1),
            },
            FieldInfoData {
                name: "OuterSpaceSunIlluminance2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, outer_space_sun_illuminance2),
            },
            FieldInfoData {
                name: "SunAngularRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_angular_radius),
            },
            FieldInfoData {
                name: "SkyColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sky_color),
            },
            FieldInfoData {
                name: "GroundColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, ground_color),
            },
            FieldInfoData {
                name: "SkyLightAngleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sky_light_angle_factor),
            },
            FieldInfoData {
                name: "SunSpecularScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_specular_scale),
            },
            FieldInfoData {
                name: "SkyEnvmapShadowScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sky_envmap_shadow_scale),
            },
            FieldInfoData {
                name: "CascadeShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cascade_shadow_enable),
            },
            FieldInfoData {
                name: "SunShadowHeightScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_shadow_height_scale),
            },
            FieldInfoData {
                name: "SunShadowFilterType",
                flags: MemberInfoFlags::new(0),
                field_type: SHADOWFILTERINGTYPE_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_shadow_filter_type),
            },
            FieldInfoData {
                name: "SunShadowForwardQuality",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERSHADOWMAPQUALITY_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_shadow_forward_quality),
            },
            FieldInfoData {
                name: "SunPcssFilterAdaptive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_pcss_filter_adaptive),
            },
            FieldInfoData {
                name: "SunPcssInitialSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_pcss_initial_sample_count),
            },
            FieldInfoData {
                name: "SunPcssMaximumSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_pcss_maximum_sample_count),
            },
            FieldInfoData {
                name: "SunPcssFilterErrorThresholdPct",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_pcss_filter_error_threshold_pct),
            },
            FieldInfoData {
                name: "SunPenumbraSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_penumbra_size),
            },
            FieldInfoData {
                name: "SunPcssShadowFilterScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, sun_pcss_shadow_filter_scale),
            },
            FieldInfoData {
                name: "CloudShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_enable),
            },
            FieldInfoData {
                name: "CloudShadowTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_texture),
            },
            FieldInfoData {
                name: "CloudShadowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_speed),
            },
            FieldInfoData {
                name: "CloudShadowSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_size),
            },
            FieldInfoData {
                name: "CloudShadowCoverage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_coverage),
            },
            FieldInfoData {
                name: "CloudShadowExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_exponent),
            },
            FieldInfoData {
                name: "CloudShadowIsTopDown",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_is_top_down),
            },
            FieldInfoData {
                name: "CloudShadowStartFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_start_fade),
            },
            FieldInfoData {
                name: "CloudShadowsFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadows_fade_distance),
            },
            FieldInfoData {
                name: "CloudShadowHeightFadeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_height_fade_enable),
            },
            FieldInfoData {
                name: "CloudShadowStartHeightFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_start_height_fade),
            },
            FieldInfoData {
                name: "CloudShadowsHeightFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadows_height_fade_distance),
            },
            FieldInfoData {
                name: "CloudXZTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_x_z_translation),
            },
            FieldInfoData {
                name: "CloudShadowAddressingMode",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_shadow_addressing_mode),
            },
            FieldInfoData {
                name: "CloudRadiosityEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cloud_radiosity_enable),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_texture),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_speed),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_size),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowCoverage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_coverage),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_exponent),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowIsTopDown",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_is_top_down),
            },
            FieldInfoData {
                name: "SecondaryCloudXZTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_x_z_translation),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowAddressingMode",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, secondary_cloud_shadow_addressing_mode),
            },
            FieldInfoData {
                name: "CastTerrainShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, cast_terrain_shadows_enable),
            },
            FieldInfoData {
                name: "TranslucencyAmbient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, translucency_ambient),
            },
            FieldInfoData {
                name: "TranslucencyScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, translucency_scale),
            },
            FieldInfoData {
                name: "TranslucencyPower",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, translucency_power),
            },
            FieldInfoData {
                name: "TranslucencyDistortion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, translucency_distortion),
            },
            FieldInfoData {
                name: "ParticleSunShadowFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, particle_sun_shadow_factor),
            },
            FieldInfoData {
                name: "ParticleSunShadowSmoothing",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, particle_sun_shadow_smoothing),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(OUTDOORLIGHTCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OutdoorLightComponentState {
    fn type_info() -> &'static TypeInfo {
        OUTDOORLIGHTCOMPONENTSTATE_TYPE_INFO
    }
}


pub const OUTDOORLIGHTCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OutdoorLightComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IndirectSpecularComponentState {
    pub enabled: bool,
    pub intensity: f32,
    pub reflectance_scale: f32,
    pub probes_intensity: f32,
    pub probes_reflectance_scale: f32,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const INDIRECTSPECULARCOMPONENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndirectSpecularComponentState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentState, enabled),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentState, intensity),
            },
            FieldInfoData {
                name: "ReflectanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentState, reflectance_scale),
            },
            FieldInfoData {
                name: "ProbesIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentState, probes_intensity),
            },
            FieldInfoData {
                name: "ProbesReflectanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentState, probes_reflectance_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(INDIRECTSPECULARCOMPONENTSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IndirectSpecularComponentState {
    fn type_info() -> &'static TypeInfo {
        INDIRECTSPECULARCOMPONENTSTATE_TYPE_INFO
    }
}


pub const INDIRECTSPECULARCOMPONENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndirectSpecularComponentState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("IndirectSpecularComponentState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShadowFilteringType {
    #[default]
    ShadowFilteringType_Pcf = 0,
    ShadowFilteringType_ContactHardening = 1,
    ShadowFilteringType_Pcss = 2,
}

pub const SHADOWFILTERINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowFilteringType",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SHADOWFILTERINGTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShadowFilteringType {
    fn type_info() -> &'static TypeInfo {
        SHADOWFILTERINGTYPE_TYPE_INFO
    }
}


pub const SHADOWFILTERINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowFilteringType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ShadowFilteringType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VisualEnvironmentComponentStaticState {
    pub visual_environment: VisualEnvironmentHandle,
    pub index: u32,
    pub field_flag_changed0: u8,
}

pub const VISUALENVIRONMENTCOMPONENTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentComponentStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "VisualEnvironment",
                flags: MemberInfoFlags::new(0),
                field_type: VISUALENVIRONMENTHANDLE_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentComponentStaticState, visual_environment),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentComponentStaticState, index),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentComponentStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTCOMPONENTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VisualEnvironmentComponentStaticState {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTCOMPONENTSTATICSTATE_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTCOMPONENTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentComponentStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisualEnvironmentComponentStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VisualEnvironmentDynamicState {
    pub visibility: f32,
    pub priority: i32,
    pub blend_mode: u8,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub override_visibility: bool,
    pub field_flag_changed0: u8,
}

pub const VISUALENVIRONMENTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Visibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentDynamicState, visibility),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentDynamicState, priority),
            },
            FieldInfoData {
                name: "BlendMode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentDynamicState, blend_mode),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "OverrideVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentDynamicState, override_visibility),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VisualEnvironmentDynamicState {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTDYNAMICSTATE_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisualEnvironmentDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VisualEnvironmentStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub debug_name: String,
    pub field_flag_changed0: u8,
}

pub const VISUALENVIRONMENTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentStaticState, transform_space),
            },
            FieldInfoData {
                name: "DebugName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentStaticState, debug_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualEnvironmentStaticState {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTSTATICSTATE_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisualEnvironmentStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VisualEnvironmentHandle {
}

pub const VISUALENVIRONMENTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(VISUALENVIRONMENTHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for VisualEnvironmentHandle {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTHANDLE_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisualEnvironmentHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ForwardLightScatteringAttenuation {
    #[default]
    ForwardLightScatteringAttenuation_None = 0,
    ForwardLightScatteringAttenuation_Corner = 1,
}

pub const FORWARDLIGHTSCATTERINGATTENUATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForwardLightScatteringAttenuation",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(FORWARDLIGHTSCATTERINGATTENUATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ForwardLightScatteringAttenuation {
    fn type_info() -> &'static TypeInfo {
        FORWARDLIGHTSCATTERINGATTENUATION_TYPE_INFO
    }
}


pub const FORWARDLIGHTSCATTERINGATTENUATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForwardLightScatteringAttenuation-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ForwardLightScatteringAttenuation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VisualEnvironmentSettings {
    pub sun_rotation_x: f32,
    pub sun_rotation_y: f32,
    pub sky_rotation_phi: f32,
    pub draw_stats: i32,
    pub draw_only_visible_stats: bool,
    pub hdr_grading_enable: bool,
}

pub const VISUALENVIRONMENTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentSettings",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SunRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentSettings, sun_rotation_x),
            },
            FieldInfoData {
                name: "SunRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentSettings, sun_rotation_y),
            },
            FieldInfoData {
                name: "SkyRotationPhi",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentSettings, sky_rotation_phi),
            },
            FieldInfoData {
                name: "DrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentSettings, draw_stats),
            },
            FieldInfoData {
                name: "DrawOnlyVisibleStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentSettings, draw_only_visible_stats),
            },
            FieldInfoData {
                name: "HdrGradingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentSettings, hdr_grading_enable),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualEnvironmentSettings {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTSETTINGS_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisualEnvironmentSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureSetDynamicState {
}

pub const TEXTURESETDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSetDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TEXTURESETDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureSetDynamicState {
    fn type_info() -> &'static TypeInfo {
        TEXTURESETDYNAMICSTATE_TYPE_INFO
    }
}


pub const TEXTURESETDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSetDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("TextureSetDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureSetStaticState {
    pub dds: Vec<u8>,
    pub texture: super::render_base::TextureResourceHandle,
    pub field_flag_changed0: u8,
}

pub const TEXTURESETSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSetStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Dds",
                flags: MemberInfoFlags::new(144),
                field_type: UINT8_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TextureSetStaticState, dds),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURERESOURCEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(TextureSetStaticState, texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TextureSetStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TEXTURESETSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureSetStaticState {
    fn type_info() -> &'static TypeInfo {
        TEXTURESETSTATICSTATE_TYPE_INFO
    }
}


pub const TEXTURESETSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSetStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("TextureSetStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshDeformerHandle {
}

pub const MESHDEFORMERHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDeformerHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(MESHDEFORMERHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MeshDeformerHandle {
    fn type_info() -> &'static TypeInfo {
        MESHDEFORMERHANDLE_TYPE_INFO
    }
}


pub const MESHDEFORMERHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDeformerHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("MeshDeformerHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LightHandle {
}

pub const LIGHTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(LIGHTHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LightHandle {
    fn type_info() -> &'static TypeInfo {
        LIGHTHANDLE_TYPE_INFO
    }
}


pub const LIGHTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LightHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ModelHandle {
}

pub const MODELHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(MODELHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ModelHandle {
    fn type_info() -> &'static TypeInfo {
        MODELHANDLE_TYPE_INFO
    }
}


pub const MODELHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ModelHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WasVisibleHandle {
}

pub const WASVISIBLEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WasVisibleHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(WASVISIBLEHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for WasVisibleHandle {
    fn type_info() -> &'static TypeInfo {
        WASVISIBLEHANDLE_TYPE_INFO
    }
}


pub const WASVISIBLEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WasVisibleHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("WasVisibleHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SimpleVolumetricsDynamicState {
    pub transform: super::core::LinearTransform,
    pub exponent: f32,
    pub emission: super::core::Vec3,
    pub emission_scale: f32,
    pub field_flag_changed0: u8,
}

pub const SIMPLEVOLUMETRICSDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsDynamicState, transform),
            },
            FieldInfoData {
                name: "Exponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsDynamicState, exponent),
            },
            FieldInfoData {
                name: "Emission",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsDynamicState, emission),
            },
            FieldInfoData {
                name: "EmissionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsDynamicState, emission_scale),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SIMPLEVOLUMETRICSDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SimpleVolumetricsDynamicState {
    fn type_info() -> &'static TypeInfo {
        SIMPLEVOLUMETRICSDYNAMICSTATE_TYPE_INFO
    }
}


pub const SIMPLEVOLUMETRICSDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SimpleVolumetricsDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const SIMPLEVOLUMETRICSSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FadeOutEndRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsStaticState, fade_out_end_radius),
            },
            FieldInfoData {
                name: "FadeOutStartRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsStaticState, fade_out_start_radius),
            },
            FieldInfoData {
                name: "FarFadeStartDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsStaticState, far_fade_start_distance),
            },
            FieldInfoData {
                name: "FarFadeEndDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsStaticState, far_fade_end_distance),
            },
            FieldInfoData {
                name: "UseClippingPlane",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsStaticState, use_clipping_plane),
            },
            FieldInfoData {
                name: "ClippingPlaneOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsStaticState, clipping_plane_offset),
            },
            FieldInfoData {
                name: "DrawPass",
                flags: MemberInfoFlags::new(0),
                field_type: SIMPLEVOLUMETRICSDRAWPASS_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsStaticState, draw_pass),
            },
            FieldInfoData {
                name: "ScaleToExposure",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsStaticState, scale_to_exposure),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SIMPLEVOLUMETRICSSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SimpleVolumetricsStaticState {
    fn type_info() -> &'static TypeInfo {
        SIMPLEVOLUMETRICSSTATICSTATE_TYPE_INFO
    }
}


pub const SIMPLEVOLUMETRICSSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SimpleVolumetricsStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SimpleVolumetricsDrawPass {
    #[default]
    SimpleVolumetricsDrawPass_BeforeTransparent = 0,
    SimpleVolumetricsDrawPass_AfterTransparent = 1,
    SimpleVolumetricsDrawPass_Count = 2,
}

pub const SIMPLEVOLUMETRICSDRAWPASS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsDrawPass",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SIMPLEVOLUMETRICSDRAWPASS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SimpleVolumetricsDrawPass {
    fn type_info() -> &'static TypeInfo {
        SIMPLEVOLUMETRICSDRAWPASS_TYPE_INFO
    }
}


pub const SIMPLEVOLUMETRICSDRAWPASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsDrawPass-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SimpleVolumetricsDrawPass-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResourceRefDynamicState {
}

pub const RESOURCEREFDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RESOURCEREFDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ResourceRefDynamicState {
    fn type_info() -> &'static TypeInfo {
        RESOURCEREFDYNAMICSTATE_TYPE_INFO
    }
}


pub const RESOURCEREFDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ResourceRefDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResourceRefStaticState {
    pub object: super::core::DataContainer,
    pub field_id: u32,
    pub field_flag_changed0: u8,
}

pub const RESOURCEREFSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Object",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(ResourceRefStaticState, object),
            },
            FieldInfoData {
                name: "FieldId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ResourceRefStaticState, field_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ResourceRefStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RESOURCEREFSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ResourceRefStaticState {
    fn type_info() -> &'static TypeInfo {
        RESOURCEREFSTATICSTATE_TYPE_INFO
    }
}


pub const RESOURCEREFSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ResourceRefStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResourceRefHandle {
}

pub const RESOURCEREFHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RESOURCEREFHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for ResourceRefHandle {
    fn type_info() -> &'static TypeInfo {
        RESOURCEREFHANDLE_TYPE_INFO
    }
}


pub const RESOURCEREFHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRefHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ResourceRefHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OccluderPlaneDynamicState {
    pub transform: super::core::LinearTransform,
    pub visible: bool,
    pub field_flag_changed0: u8,
}

pub const OCCLUDERPLANEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneDynamicState, transform),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneDynamicState, visible),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERPLANEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OccluderPlaneDynamicState {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERPLANEDYNAMICSTATE_TYPE_INFO
    }
}


pub const OCCLUDERPLANEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderPlaneDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OccluderPlaneStaticState {
    pub occluder_high_priority: bool,
    pub occluder_is_conservative: bool,
    pub doubled_sided: bool,
    pub coverage_value: f32,
    pub field_flag_changed0: u8,
}

pub const OCCLUDERPLANESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "OccluderHighPriority",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneStaticState, occluder_high_priority),
            },
            FieldInfoData {
                name: "OccluderIsConservative",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneStaticState, occluder_is_conservative),
            },
            FieldInfoData {
                name: "DoubledSided",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneStaticState, doubled_sided),
            },
            FieldInfoData {
                name: "CoverageValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneStaticState, coverage_value),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERPLANESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for OccluderPlaneStaticState {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERPLANESTATICSTATE_TYPE_INFO
    }
}


pub const OCCLUDERPLANESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderPlaneStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OccluderVolumeDynamicState {
    pub transform: super::core::LinearTransform,
    pub visible: bool,
    pub field_flag_changed0: u8,
}

pub const OCCLUDERVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeDynamicState, transform),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeDynamicState, visible),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OccluderVolumeDynamicState {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERVOLUMEDYNAMICSTATE_TYPE_INFO
    }
}


pub const OCCLUDERVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderVolumeDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OccluderVolumeStaticState {
    pub occluder_high_priority: bool,
    pub occluder_is_conservative: bool,
    pub coverage_value: f32,
    pub field_flag_changed0: u8,
}

pub const OCCLUDERVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "OccluderHighPriority",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeStaticState, occluder_high_priority),
            },
            FieldInfoData {
                name: "OccluderIsConservative",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeStaticState, occluder_is_conservative),
            },
            FieldInfoData {
                name: "CoverageValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeStaticState, coverage_value),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for OccluderVolumeStaticState {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERVOLUMESTATICSTATE_TYPE_INFO
    }
}


pub const OCCLUDERVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderVolumeStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OccluderMeshDynamicState {
    pub transform: super::core::LinearTransform,
    pub visible: bool,
    pub field_flag_changed0: u8,
}

pub const OCCLUDERMESHDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(OccluderMeshDynamicState, transform),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderMeshDynamicState, visible),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OccluderMeshDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERMESHDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OccluderMeshDynamicState {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERMESHDYNAMICSTATE_TYPE_INFO
    }
}


pub const OCCLUDERMESHDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderMeshDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OccluderMeshStaticState {
    pub mesh: super::render_base::MeshBaseAsset,
    pub field_flag_changed0: u8,
}

pub const OCCLUDERMESHSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(OccluderMeshStaticState, mesh),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OccluderMeshStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OCCLUDERMESHSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OccluderMeshStaticState {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERMESHSTATICSTATE_TYPE_INFO
    }
}


pub const OCCLUDERMESHSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OccluderMeshStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectHighlightDynamicState {
    pub object_highlight_type: ObjectHighlightType,
    pub index: i32,
    pub enable: bool,
    pub draw_at_foreground: bool,
    pub is_mask: bool,
    pub field_flag_changed0: u8,
}

pub const OBJECTHIGHLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ObjectHighlightType",
                flags: MemberInfoFlags::new(0),
                field_type: OBJECTHIGHLIGHTTYPE_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightDynamicState, object_highlight_type),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightDynamicState, index),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightDynamicState, enable),
            },
            FieldInfoData {
                name: "DrawAtForeground",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightDynamicState, draw_at_foreground),
            },
            FieldInfoData {
                name: "IsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightDynamicState, is_mask),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OBJECTHIGHLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ObjectHighlightDynamicState {
    fn type_info() -> &'static TypeInfo {
        OBJECTHIGHLIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const OBJECTHIGHLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ObjectHighlightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectHighlightStaticState {
    pub render_view_handle: super::render_base::RenderViewHandle,
    pub model_handle: ModelHandle,
    pub field_flag_changed0: u8,
}

pub const OBJECTHIGHLIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "RenderViewHandle",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERVIEWHANDLE_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightStaticState, render_view_handle),
            },
            FieldInfoData {
                name: "ModelHandle",
                flags: MemberInfoFlags::new(0),
                field_type: MODELHANDLE_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightStaticState, model_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OBJECTHIGHLIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ObjectHighlightStaticState {
    fn type_info() -> &'static TypeInfo {
        OBJECTHIGHLIGHTSTATICSTATE_TYPE_INFO
    }
}


pub const OBJECTHIGHLIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ObjectHighlightStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ObjectHighlightType {
    #[default]
    ObjectHighlightType_Default = 0,
    ObjectHighlightType_ThreatOverlay = 1,
    ObjectHighlightType_Sonar = 2,
    ObjectHighlightType_EdgeDetect = 3,
}

pub const OBJECTHIGHLIGHTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightType",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(OBJECTHIGHLIGHTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ObjectHighlightType {
    fn type_info() -> &'static TypeInfo {
        OBJECTHIGHLIGHTTYPE_TYPE_INFO
    }
}


pub const OBJECTHIGHLIGHTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ObjectHighlightType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EdgeModelsBaseData {
    pub instance_transforms: Vec<super::core::LinearTransform>,
    pub mesh_instance_ranges: Vec<u16>,
    pub connection_instance_lookup_table: Vec<u16>,
    pub connections: Vec<EdgeModelConnectionInfo>,
    pub part_connection_ranges: Vec<u16>,
}

pub const EDGEMODELSBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelsBaseData",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InstanceTransforms",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARTRANSFORM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EdgeModelsBaseData, instance_transforms),
            },
            FieldInfoData {
                name: "MeshInstanceRanges",
                flags: MemberInfoFlags::new(144),
                field_type: UINT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EdgeModelsBaseData, mesh_instance_ranges),
            },
            FieldInfoData {
                name: "ConnectionInstanceLookupTable",
                flags: MemberInfoFlags::new(144),
                field_type: UINT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EdgeModelsBaseData, connection_instance_lookup_table),
            },
            FieldInfoData {
                name: "Connections",
                flags: MemberInfoFlags::new(144),
                field_type: EDGEMODELCONNECTIONINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EdgeModelsBaseData, connections),
            },
            FieldInfoData {
                name: "PartConnectionRanges",
                flags: MemberInfoFlags::new(144),
                field_type: UINT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EdgeModelsBaseData, part_connection_ranges),
            },
        ],
    }),
    array_type: Some(EDGEMODELSBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EdgeModelsBaseData {
    fn type_info() -> &'static TypeInfo {
        EDGEMODELSBASEDATA_TYPE_INFO
    }
}


pub const EDGEMODELSBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelsBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EdgeModelsBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EdgeModelConnectionInfo {
    pub connection_instance_range: u16,
    pub neighbour_part_index: u16,
}

pub const EDGEMODELCONNECTIONINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelConnectionInfo",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ConnectionInstanceRange",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(EdgeModelConnectionInfo, connection_instance_range),
            },
            FieldInfoData {
                name: "NeighbourPartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(EdgeModelConnectionInfo, neighbour_part_index),
            },
        ],
    }),
    array_type: Some(EDGEMODELCONNECTIONINFO_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for EdgeModelConnectionInfo {
    fn type_info() -> &'static TypeInfo {
        EDGEMODELCONNECTIONINFO_TYPE_INFO
    }
}


pub const EDGEMODELCONNECTIONINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelConnectionInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EdgeModelConnectionInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestructionVolumeBaseAsset {
}

pub const DESTRUCTIONVOLUMEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionVolumeBaseAsset {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONVOLUMEBASEASSET_TYPE_INFO
    }
}


pub const DESTRUCTIONVOLUMEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DestructionVolumeBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestructionVolumeBaseData {
}

pub const DESTRUCTIONVOLUMEBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeBaseData",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMEBASEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionVolumeBaseData {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONVOLUMEBASEDATA_TYPE_INFO
    }
}


pub const DESTRUCTIONVOLUMEBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DestructionVolumeBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshDynamicState {
    pub object_variation_name_hash: u32,
    pub asset_dependancies: Vec<super::core::Asset>,
    pub shader_parameter_blocks: Vec<super::render_base::ShaderParameterBlockHandle>,
    pub is_streaming: bool,
    pub streaming_priority: f32,
    pub render_flags: u32,
    pub parent_render_flags_mask: u32,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub light_map_weight: f32,
    pub destruction_volume: DestructionVolumeBaseData,
    pub edge_models: EdgeModelsBaseData,
    pub stencil_bits: u8,
    pub render_object_stencil_mask: u8,
    pub is_visible: bool,
    pub vertex_shader_fragment_override: super::render_base::VertexShaderFragmentHandle,
    pub field_flag_changed0: u16,
}

pub const MESHDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ObjectVariationNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, object_variation_name_hash),
            },
            FieldInfoData {
                name: "AssetDependancies",
                flags: MemberInfoFlags::new(144),
                field_type: ASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, asset_dependancies),
            },
            FieldInfoData {
                name: "ShaderParameterBlocks",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERPARAMETERBLOCKHANDLE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, shader_parameter_blocks),
            },
            FieldInfoData {
                name: "IsStreaming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, is_streaming),
            },
            FieldInfoData {
                name: "StreamingPriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, streaming_priority),
            },
            FieldInfoData {
                name: "RenderFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, render_flags),
            },
            FieldInfoData {
                name: "ParentRenderFlagsMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, parent_render_flags_mask),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "LightMapWeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, light_map_weight),
            },
            FieldInfoData {
                name: "DestructionVolume",
                flags: MemberInfoFlags::new(0),
                field_type: DESTRUCTIONVOLUMEBASEDATA_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, destruction_volume),
            },
            FieldInfoData {
                name: "EdgeModels",
                flags: MemberInfoFlags::new(0),
                field_type: EDGEMODELSBASEDATA_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, edge_models),
            },
            FieldInfoData {
                name: "StencilBits",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, stencil_bits),
            },
            FieldInfoData {
                name: "RenderObjectStencilMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, render_object_stencil_mask),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, is_visible),
            },
            FieldInfoData {
                name: "VertexShaderFragmentOverride",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXSHADERFRAGMENTHANDLE_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, vertex_shader_fragment_override),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(MeshDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MESHDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshDynamicState {
    fn type_info() -> &'static TypeInfo {
        MESHDYNAMICSTATE_TYPE_INFO
    }
}


pub const MESHDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("MeshDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshStaticState {
    pub mesh: super::render_base::MeshBaseAsset,
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

pub const MESHSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, mesh),
            },
            FieldInfoData {
                name: "MeshDeformer",
                flags: MemberInfoFlags::new(0),
                field_type: MESHDEFORMERHANDLE_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, mesh_deformer),
            },
            FieldInfoData {
                name: "VertexShaderFragment",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXSHADERFRAGMENTHANDLE_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, vertex_shader_fragment),
            },
            FieldInfoData {
                name: "ProceduralAnimationMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, procedural_animation_max_distance),
            },
            FieldInfoData {
                name: "ShadowLodOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, shadow_lod_offset),
            },
            FieldInfoData {
                name: "IsTerrainShaderNodesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, is_terrain_shader_nodes_enable),
            },
            FieldInfoData {
                name: "UseForBoundingBoxCalculations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, use_for_bounding_box_calculations),
            },
            FieldInfoData {
                name: "UsePartsForBoundingBoxCalculations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, use_parts_for_bounding_box_calculations),
            },
            FieldInfoData {
                name: "DecalsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, decals_enable),
            },
            FieldInfoData {
                name: "ShaderDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, shader_draw_order),
            },
            FieldInfoData {
                name: "ShaderDrawOrderSubOrder",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, shader_draw_order_sub_order),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(MeshStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MESHSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshStaticState {
    fn type_info() -> &'static TypeInfo {
        MESHSTATICSTATE_TYPE_INFO
    }
}


pub const MESHSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("MeshStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ModelCreateInfo {
    pub transform: super::core::LinearTransform,
    pub bone_or_part_transforms: Vec<super::core::Vec>,
    pub should_apply_world_transform: bool,
    pub should_apply_reference_translation: bool,
    pub reference_translation: super::core::Vec3,
    pub is_visible: bool,
    pub is_world_space: bool,
    pub parent_transform_space: super::state_stream::TransformSpaceHandle,
}

pub const MODELCREATEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelCreateInfo",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(ModelCreateInfo, transform),
            },
            FieldInfoData {
                name: "BoneOrPartTransforms",
                flags: MemberInfoFlags::new(144),
                field_type: VEC_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ModelCreateInfo, bone_or_part_transforms),
            },
            FieldInfoData {
                name: "ShouldApplyWorldTransform",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ModelCreateInfo, should_apply_world_transform),
            },
            FieldInfoData {
                name: "ShouldApplyReferenceTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ModelCreateInfo, should_apply_reference_translation),
            },
            FieldInfoData {
                name: "ReferenceTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ModelCreateInfo, reference_translation),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ModelCreateInfo, is_visible),
            },
            FieldInfoData {
                name: "IsWorldSpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ModelCreateInfo, is_world_space),
            },
            FieldInfoData {
                name: "ParentTransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(ModelCreateInfo, parent_transform_space),
            },
        ],
    }),
    array_type: Some(MODELCREATEINFO_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ModelCreateInfo {
    fn type_info() -> &'static TypeInfo {
        MODELCREATEINFO_TYPE_INFO
    }
}


pub const MODELCREATEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelCreateInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ModelCreateInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const MODELDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BoneOrPartVisibleFlags",
                flags: MemberInfoFlags::new(144),
                field_type: UINT8_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, bone_or_part_visible_flags),
            },
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: MODELHANDLE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, children),
            },
            FieldInfoData {
                name: "ShaderParameterBlocks",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERPARAMETERBLOCKHANDLE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, shader_parameter_blocks),
            },
            FieldInfoData {
                name: "FallbackBranch",
                flags: MemberInfoFlags::new(0),
                field_type: MODELHANDLE_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, fallback_branch),
            },
            FieldInfoData {
                name: "ObjectVariationNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, object_variation_name_hash),
            },
            FieldInfoData {
                name: "RenderFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, render_flags),
            },
            FieldInfoData {
                name: "ParentRenderFlagsMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, parent_render_flags_mask),
            },
            FieldInfoData {
                name: "StreamingPriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, streaming_priority),
            },
            FieldInfoData {
                name: "IsMeshesVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, is_meshes_visible),
            },
            FieldInfoData {
                name: "IsOcclusionTestEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, is_occlusion_test_enable),
            },
            FieldInfoData {
                name: "WasVisibleCallback",
                flags: MemberInfoFlags::new(0),
                field_type: WASVISIBLEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, was_visible_callback),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "RenderFovDeg",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, render_fov_deg),
            },
            FieldInfoData {
                name: "CustomLightProbeType",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, custom_light_probe_type),
            },
            FieldInfoData {
                name: "CastShadowOverride",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, cast_shadow_override),
            },
            FieldInfoData {
                name: "StencilBits",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, stencil_bits),
            },
            FieldInfoData {
                name: "RenderObjectStencilMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, render_object_stencil_mask),
            },
            FieldInfoData {
                name: "FallbackDisableMinRequiredLodIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT8_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, fallback_disable_min_required_lod_index),
            },
            FieldInfoData {
                name: "LightProbeSampleOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, light_probe_sample_offset),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ModelDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MODELDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ModelDynamicState {
    fn type_info() -> &'static TypeInfo {
        MODELDYNAMICSTATE_TYPE_INFO
    }
}


pub const MODELDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ModelDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
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

pub const MODELSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MeshType",
                flags: MemberInfoFlags::new(0),
                field_type: MESHTYPE_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, mesh_type),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, transform_space),
            },
            FieldInfoData {
                name: "BoneOrPartCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, bone_or_part_count),
            },
            FieldInfoData {
                name: "MovableParts",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, movable_parts),
            },
            FieldInfoData {
                name: "RadiosityTypeOverride",
                flags: MemberInfoFlags::new(0),
                field_type: RADIOSITYTYPEOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, radiosity_type_override),
            },
            FieldInfoData {
                name: "HierarchyLevel",
                flags: MemberInfoFlags::new(0),
                field_type: MODELHIERARCHYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, hierarchy_level),
            },
            FieldInfoData {
                name: "RenderingOverrides",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERINGOVERRIDES_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, rendering_overrides),
            },
            FieldInfoData {
                name: "UseTopOfBoundingBoxForLightProbe",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, use_top_of_bounding_box_for_light_probe),
            },
            FieldInfoData {
                name: "IncludeBoundingBoxWhenNotVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, include_bounding_box_when_not_visible),
            },
            FieldInfoData {
                name: "LightmapHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, lightmap_hash),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ModelStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MODELSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ModelStaticState {
    fn type_info() -> &'static TypeInfo {
        MODELSTATICSTATE_TYPE_INFO
    }
}


pub const MODELSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ModelStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ModelHierarchyLevel {
    #[default]
    ModelHierarchyLevel_Root = 0,
    ModelHierarchyLevel_RootWithFallback = 1,
    ModelHierarchyLevel_Child = 2,
}

pub const MODELHIERARCHYLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelHierarchyLevel",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(MODELHIERARCHYLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ModelHierarchyLevel {
    fn type_info() -> &'static TypeInfo {
        MODELHIERARCHYLEVEL_TYPE_INFO
    }
}


pub const MODELHIERARCHYLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelHierarchyLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ModelHierarchyLevel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VisibleAreaDynamicState {
    pub transform: super::core::LinearTransform,
    pub field_flag_changed0: u8,
}

pub const VISIBLEAREADYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(VisibleAreaDynamicState, transform),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VisibleAreaDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISIBLEAREADYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VisibleAreaDynamicState {
    fn type_info() -> &'static TypeInfo {
        VISIBLEAREADYNAMICSTATE_TYPE_INFO
    }
}


pub const VISIBLEAREADYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisibleAreaDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VisibleAreaStaticState {
    pub visual_cull_screen_area: f32,
    pub was_visible_handle: WasVisibleHandle,
    pub field_flag_changed0: u8,
}

pub const VISIBLEAREASTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "VisualCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisibleAreaStaticState, visual_cull_screen_area),
            },
            FieldInfoData {
                name: "WasVisibleHandle",
                flags: MemberInfoFlags::new(0),
                field_type: WASVISIBLEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(VisibleAreaStaticState, was_visible_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VisibleAreaStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(VISIBLEAREASTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VisibleAreaStaticState {
    fn type_info() -> &'static TypeInfo {
        VISIBLEAREASTATICSTATE_TYPE_INFO
    }
}


pub const VISIBLEAREASTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("VisibleAreaStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PlanarReflectionLocatorDynamicState {
    pub transform: super::core::LinearTransform,
    pub enable: bool,
    pub field_flag_changed0: u8,
}

pub const PLANARREFLECTIONLOCATORDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionLocatorDynamicState, transform),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionLocatorDynamicState, enable),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionLocatorDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PLANARREFLECTIONLOCATORDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PlanarReflectionLocatorDynamicState {
    fn type_info() -> &'static TypeInfo {
        PLANARREFLECTIONLOCATORDYNAMICSTATE_TYPE_INFO
    }
}


pub const PLANARREFLECTIONLOCATORDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PlanarReflectionLocatorDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlanarReflectionLocatorStaticState {
}

pub const PLANARREFLECTIONLOCATORSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(PLANARREFLECTIONLOCATORSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PlanarReflectionLocatorStaticState {
    fn type_info() -> &'static TypeInfo {
        PLANARREFLECTIONLOCATORSTATICSTATE_TYPE_INFO
    }
}


pub const PLANARREFLECTIONLOCATORSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PlanarReflectionLocatorStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LightProbeVolumeDynamicState {
    pub blend_distance: f32,
    pub priority: u32,
    pub enable: bool,
    pub field_flag_changed0: u8,
}

pub const LIGHTPROBEVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeVolumeDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BlendDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeDynamicState, blend_distance),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeDynamicState, priority),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeDynamicState, enable),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LIGHTPROBEVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LightProbeVolumeDynamicState {
    fn type_info() -> &'static TypeInfo {
        LIGHTPROBEVOLUMEDYNAMICSTATE_TYPE_INFO
    }
}


pub const LIGHTPROBEVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LightProbeVolumeDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LightProbeVolumeStaticState {
    pub guid: super::core::Guid,
    pub field_flag_changed0: u8,
}

pub const LIGHTPROBEVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeVolumeStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeStaticState, guid),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LIGHTPROBEVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LightProbeVolumeStaticState {
    fn type_info() -> &'static TypeInfo {
        LIGHTPROBEVOLUMESTATICSTATE_TYPE_INFO
    }
}


pub const LIGHTPROBEVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LightProbeVolumeStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicEnlightenDynamicState {
    pub enable: bool,
    pub database_version: i32,
    pub field_flag_changed0: u8,
}

pub const DYNAMICENLIGHTENDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenDynamicState, enable),
            },
            FieldInfoData {
                name: "DatabaseVersion",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenDynamicState, database_version),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DYNAMICENLIGHTENDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DynamicEnlightenDynamicState {
    fn type_info() -> &'static TypeInfo {
        DYNAMICENLIGHTENDYNAMICSTATE_TYPE_INFO
    }
}


pub const DYNAMICENLIGHTENDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DynamicEnlightenDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicEnlightenStaticState {
    pub priority: i32,
    pub enlighten_data: super::render_base::EnlightenBaseAsset,
    pub object_layers: u16,
    pub field_flag_changed0: u8,
}

pub const DYNAMICENLIGHTENSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenStaticState, priority),
            },
            FieldInfoData {
                name: "EnlightenData",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenStaticState, enlighten_data),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenStaticState, object_layers),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DYNAMICENLIGHTENSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicEnlightenStaticState {
    fn type_info() -> &'static TypeInfo {
        DYNAMICENLIGHTENSTATICSTATE_TYPE_INFO
    }
}


pub const DYNAMICENLIGHTENSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DynamicEnlightenStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticEnlightenDynamicState {
    pub enable: bool,
    pub mixed: bool,
    pub field_flag_changed0: u8,
}

pub const STATICENLIGHTENDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenDynamicState, enable),
            },
            FieldInfoData {
                name: "Mixed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenDynamicState, mixed),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(STATICENLIGHTENDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StaticEnlightenDynamicState {
    fn type_info() -> &'static TypeInfo {
        STATICENLIGHTENDYNAMICSTATE_TYPE_INFO
    }
}


pub const STATICENLIGHTENDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("StaticEnlightenDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticEnlightenStaticState {
    pub priority: i32,
    pub enlighten_data: super::render_base::StaticEnlightenBaseAsset,
    pub dynamic_enlighten_data: super::render_base::EnlightenBaseAsset,
    pub object_layers: u16,
    pub flux_auto_bake: bool,
    pub field_flag_changed0: u8,
}

pub const STATICENLIGHTENSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenStaticState, priority),
            },
            FieldInfoData {
                name: "EnlightenData",
                flags: MemberInfoFlags::new(0),
                field_type: STATICENLIGHTENBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenStaticState, enlighten_data),
            },
            FieldInfoData {
                name: "DynamicEnlightenData",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenStaticState, dynamic_enlighten_data),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenStaticState, object_layers),
            },
            FieldInfoData {
                name: "FluxAutoBake",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenStaticState, flux_auto_bake),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(STATICENLIGHTENSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticEnlightenStaticState {
    fn type_info() -> &'static TypeInfo {
        STATICENLIGHTENSTATICSTATE_TYPE_INFO
    }
}


pub const STATICENLIGHTENSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("StaticEnlightenStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RadiosityModifierDynamicState {
    pub position: super::core::Vec3,
    pub radius: f32,
    pub bounce_scale: f32,
    pub sun_scale: f32,
    pub field_flag_changed0: u8,
}

pub const RADIOSITYMODIFIERDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RadiosityModifierDynamicState, position),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityModifierDynamicState, radius),
            },
            FieldInfoData {
                name: "BounceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityModifierDynamicState, bounce_scale),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityModifierDynamicState, sun_scale),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityModifierDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMODIFIERDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityModifierDynamicState {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMODIFIERDYNAMICSTATE_TYPE_INFO
    }
}


pub const RADIOSITYMODIFIERDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityModifierDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RadiosityModifierStaticState {
}

pub const RADIOSITYMODIFIERSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RADIOSITYMODIFIERSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RadiosityModifierStaticState {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMODIFIERSTATICSTATE_TYPE_INFO
    }
}


pub const RADIOSITYMODIFIERSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityModifierStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RadiosityMaterialTriggerDynamicState {
    pub opacity: f32,
    pub color: super::core::Vec3,
    pub field_flag_changed0: u8,
}

pub const RADIOSITYMATERIALTRIGGERDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialTriggerDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialTriggerDynamicState, opacity),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialTriggerDynamicState, color),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialTriggerDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALTRIGGERDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityMaterialTriggerDynamicState {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIALTRIGGERDYNAMICSTATE_TYPE_INFO
    }
}


pub const RADIOSITYMATERIALTRIGGERDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialTriggerDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityMaterialTriggerDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RadiosityMaterialTriggerStaticState {
    pub material_guid: super::core::Guid,
    pub field_flag_changed0: u8,
}

pub const RADIOSITYMATERIALTRIGGERSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialTriggerStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MaterialGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialTriggerStaticState, material_guid),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialTriggerStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALTRIGGERSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RadiosityMaterialTriggerStaticState {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIALTRIGGERSTATICSTATE_TYPE_INFO
    }
}


pub const RADIOSITYMATERIALTRIGGERSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialTriggerStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityMaterialTriggerStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RadiosityMaterialDynamicState {
    pub color: super::core::Vec3,
    pub emissive_intensity: f32,
    pub opacity: f32,
    pub backface_type: super::render_base::RadiosityBackfaceType,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const RADIOSITYMATERIALDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialDynamicState, color),
            },
            FieldInfoData {
                name: "EmissiveIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialDynamicState, emissive_intensity),
            },
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialDynamicState, opacity),
            },
            FieldInfoData {
                name: "BackfaceType",
                flags: MemberInfoFlags::new(0),
                field_type: RADIOSITYBACKFACETYPE_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialDynamicState, backface_type),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialDynamicState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityMaterialDynamicState {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIALDYNAMICSTATE_TYPE_INFO
    }
}


pub const RADIOSITYMATERIALDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityMaterialDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RadiosityMaterialStaticState {
    pub material_guid: super::core::Guid,
    pub material_update_mask: u8,
    pub field_flag_changed0: u8,
}

pub const RADIOSITYMATERIALSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MaterialGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialStaticState, material_guid),
            },
            FieldInfoData {
                name: "MaterialUpdateMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialStaticState, material_update_mask),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RadiosityMaterialStaticState {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIALSTATICSTATE_TYPE_INFO
    }
}


pub const RADIOSITYMATERIALSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RadiosityMaterialStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GroundHeightDynamicState {
}

pub const GROUNDHEIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(GROUNDHEIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GroundHeightDynamicState {
    fn type_info() -> &'static TypeInfo {
        GROUNDHEIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const GROUNDHEIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("GroundHeightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GroundHeightStaticState {
    pub ground_origo: super::core::Vec3,
    pub data: GroundHeightData,
    pub field_flag_changed0: u8,
}

pub const GROUNDHEIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "GroundOrigo",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(GroundHeightStaticState, ground_origo),
            },
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: GROUNDHEIGHTDATA_TYPE_INFO,
                rust_offset: offset_of!(GroundHeightStaticState, data),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(GroundHeightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(GROUNDHEIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GroundHeightStaticState {
    fn type_info() -> &'static TypeInfo {
        GROUNDHEIGHTSTATICSTATE_TYPE_INFO
    }
}


pub const GROUNDHEIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("GroundHeightStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GroundHeightData {
    pub world_size: f32,
    pub height_span: super::core::Vec2,
    pub data: Vec<u16>,
}

pub const GROUNDHEIGHTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightData",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "WorldSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GroundHeightData, world_size),
            },
            FieldInfoData {
                name: "HeightSpan",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(GroundHeightData, height_span),
            },
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(144),
                field_type: UINT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GroundHeightData, data),
            },
        ],
    }),
    array_type: Some(GROUNDHEIGHTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GroundHeightData {
    fn type_info() -> &'static TypeInfo {
        GROUNDHEIGHTDATA_TYPE_INFO
    }
}


pub const GROUNDHEIGHTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("GroundHeightData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RenderVolumeDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub field_flag_changed0: u8,
}

pub const RENDERVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeDynamicState, enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RENDERVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RenderVolumeDynamicState {
    fn type_info() -> &'static TypeInfo {
        RENDERVOLUMEDYNAMICSTATE_TYPE_INFO
    }
}


pub const RENDERVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RenderVolumeDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RenderVolumeStaticState {
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub user_masks: super::core::Vec4,
    pub transform_type: RenderVolumeTransformType,
    pub field_flag_changed0: u8,
}

pub const RENDERVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeStaticState, shader),
            },
            FieldInfoData {
                name: "UserMasks",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeStaticState, user_masks),
            },
            FieldInfoData {
                name: "TransformType",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERVOLUMETRANSFORMTYPE_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeStaticState, transform_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(RENDERVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RenderVolumeStaticState {
    fn type_info() -> &'static TypeInfo {
        RENDERVOLUMESTATICSTATE_TYPE_INFO
    }
}


pub const RENDERVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RenderVolumeStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderVolumeTransformType {
    #[default]
    RenderVolumeTransformType_WorldSpaceInv = 0,
    RenderVolumeTransformType_WorldSpaceNoScale = 1,
}

pub const RENDERVOLUMETRANSFORMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeTransformType",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERVOLUMETRANSFORMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderVolumeTransformType {
    fn type_info() -> &'static TypeInfo {
        RENDERVOLUMETRANSFORMTYPE_TYPE_INFO
    }
}


pub const RENDERVOLUMETRANSFORMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeTransformType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("RenderVolumeTransformType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrRectangularLightDynamicState {
    pub shape: super::render_base::RectangularLightShape,
    pub outer_angle: f32,
    pub aspect: f32,
    pub width: f32,
    pub height: f32,
    pub texture: super::render_base::TextureBaseAsset,
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

pub const PBRRECTANGULARLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Shape",
                flags: MemberInfoFlags::new(0),
                field_type: RECTANGULARLIGHTSHAPE_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shape),
            },
            FieldInfoData {
                name: "OuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, outer_angle),
            },
            FieldInfoData {
                name: "Aspect",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, aspect),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, height),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, texture),
            },
            FieldInfoData {
                name: "ShadowMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_max_angle),
            },
            FieldInfoData {
                name: "ShadowFadeOutRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_fade_out_range),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTUNITTYPE_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(PBRRECTANGULARLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrRectangularLightDynamicState {
    fn type_info() -> &'static TypeInfo {
        PBRRECTANGULARLIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const PBRRECTANGULARLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrRectangularLightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PbrRectangularLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub const PBRRECTANGULARLIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PBRRECTANGULARLIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PbrRectangularLightStaticState {
    fn type_info() -> &'static TypeInfo {
        PBRRECTANGULARLIGHTSTATICSTATE_TYPE_INFO
    }
}


pub const PBRRECTANGULARLIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrRectangularLightStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const PBRTUBELIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TubeRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, tube_radius),
            },
            FieldInfoData {
                name: "TubeWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, tube_width),
            },
            FieldInfoData {
                name: "OnlyHempishere",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, only_hempishere),
            },
            FieldInfoData {
                name: "IsCapsule",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, is_capsule),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTUNITTYPE_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(PBRTUBELIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrTubeLightDynamicState {
    fn type_info() -> &'static TypeInfo {
        PBRTUBELIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const PBRTUBELIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrTubeLightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PbrTubeLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub const PBRTUBELIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PBRTUBELIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PbrTubeLightStaticState {
    fn type_info() -> &'static TypeInfo {
        PBRTUBELIGHTSTATICSTATE_TYPE_INFO
    }
}


pub const PBRTUBELIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrTubeLightStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrSpotLightDynamicState {
    pub disc_radius: f32,
    pub inner_angle: f32,
    pub outer_angle: f32,
    pub i_e_s_profile: IesProfileAsset,
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

pub const PBRSPOTLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DiscRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, disc_radius),
            },
            FieldInfoData {
                name: "InnerAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, inner_angle),
            },
            FieldInfoData {
                name: "OuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, outer_angle),
            },
            FieldInfoData {
                name: "IESProfile",
                flags: MemberInfoFlags::new(0),
                field_type: IESPROFILEASSET_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, i_e_s_profile),
            },
            FieldInfoData {
                name: "UseIESProfileAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, use_i_e_s_profile_as_mask),
            },
            FieldInfoData {
                name: "IESMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, i_e_s_multiplier),
            },
            FieldInfoData {
                name: "ShadowMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_max_angle),
            },
            FieldInfoData {
                name: "ShadowFadeOutRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_fade_out_range),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTUNITTYPE_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(PBRSPOTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrSpotLightDynamicState {
    fn type_info() -> &'static TypeInfo {
        PBRSPOTLIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const PBRSPOTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrSpotLightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PbrSpotLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub const PBRSPOTLIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PBRSPOTLIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PbrSpotLightStaticState {
    fn type_info() -> &'static TypeInfo {
        PBRSPOTLIGHTSTATICSTATE_TYPE_INFO
    }
}


pub const PBRSPOTLIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrSpotLightStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrSphereLightDynamicState {
    pub sphere_radius: f32,
    pub only_hempishere: bool,
    pub i_e_s_profile: IesProfileAsset,
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

pub const PBRSPHERELIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SphereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, sphere_radius),
            },
            FieldInfoData {
                name: "OnlyHempishere",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, only_hempishere),
            },
            FieldInfoData {
                name: "IESProfile",
                flags: MemberInfoFlags::new(0),
                field_type: IESPROFILEASSET_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, i_e_s_profile),
            },
            FieldInfoData {
                name: "UseIESProfileAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, use_i_e_s_profile_as_mask),
            },
            FieldInfoData {
                name: "IESMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, i_e_s_multiplier),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTUNITTYPE_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(PBRSPHERELIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrSphereLightDynamicState {
    fn type_info() -> &'static TypeInfo {
        PBRSPHERELIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const PBRSPHERELIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrSphereLightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PbrSphereLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub const PBRSPHERELIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(PBRSPHERELIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PbrSphereLightStaticState {
    fn type_info() -> &'static TypeInfo {
        PBRSPHERELIGHTSTATICSTATE_TYPE_INFO
    }
}


pub const PBRSPHERELIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrSphereLightStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const PBRANALYTICLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrAnalyticLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTUNITTYPE_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(PbrAnalyticLightDynamicState, excluded_cull_id),
            },
        ],
    }),
    array_type: Some(PBRANALYTICLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrAnalyticLightDynamicState {
    fn type_info() -> &'static TypeInfo {
        PBRANALYTICLIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const PBRANALYTICLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrAnalyticLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("PbrAnalyticLightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OldSpotLightDynamicState {
    pub shape: SpotLightShape,
    pub cone_inner_angle: f32,
    pub cone_outer_angle: f32,
    pub frustum_fov: f32,
    pub frustum_aspect: f32,
    pub ortho_width: f32,
    pub ortho_height: f32,
    pub near_plane: f32,
    pub texture: super::render_base::TextureBaseAsset,
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

pub const OLDSPOTLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldSpotLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Shape",
                flags: MemberInfoFlags::new(0),
                field_type: SPOTLIGHTSHAPE_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, shape),
            },
            FieldInfoData {
                name: "ConeInnerAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, cone_inner_angle),
            },
            FieldInfoData {
                name: "ConeOuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, cone_outer_angle),
            },
            FieldInfoData {
                name: "FrustumFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, frustum_fov),
            },
            FieldInfoData {
                name: "FrustumAspect",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, frustum_aspect),
            },
            FieldInfoData {
                name: "OrthoWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, ortho_width),
            },
            FieldInfoData {
                name: "OrthoHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, ortho_height),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, near_plane),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, texture),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_dimmer),
            },
            FieldInfoData {
                name: "FrustumAsCone",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, frustum_as_cone),
            },
            FieldInfoData {
                name: "FrustumAsConeAngle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, frustum_as_cone_angle),
            },
            FieldInfoData {
                name: "FrustumAsConeIntensityScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, frustum_as_cone_intensity_scale),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "CastShadowsMinLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, cast_shadows_min_level),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, color),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, radius),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "DirectLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, direct_light_enable),
            },
            FieldInfoData {
                name: "SpecularEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, specular_enable),
            },
            FieldInfoData {
                name: "EnlightenColorMode",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENCOLORMODE_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, enlighten_color_mode),
            },
            FieldInfoData {
                name: "EnlightenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, enlighten_enable),
            },
            FieldInfoData {
                name: "EnlightenColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, enlighten_color_scale),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, field_flag_changed0),
            },
            FieldInfoData {
                name: "FieldFlagChanged1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightDynamicState, field_flag_changed1),
            },
        ],
    }),
    array_type: Some(OLDSPOTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OldSpotLightDynamicState {
    fn type_info() -> &'static TypeInfo {
        OLDSPOTLIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const OLDSPOTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldSpotLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OldSpotLightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OldSpotLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub const OLDSPOTLIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldSpotLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OldSpotLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OLDSPOTLIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for OldSpotLightStaticState {
    fn type_info() -> &'static TypeInfo {
        OLDSPOTLIGHTSTATICSTATE_TYPE_INFO
    }
}


pub const OLDSPOTLIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldSpotLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OldSpotLightStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const OLDPOINTLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldPointLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, width),
            },
            FieldInfoData {
                name: "TranslucencyAmbient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, translucency_ambient),
            },
            FieldInfoData {
                name: "TranslucencyScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, translucency_scale),
            },
            FieldInfoData {
                name: "TranslucencyPower",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, translucency_power),
            },
            FieldInfoData {
                name: "TranslucencyDistortion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, translucency_distortion),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, color),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, radius),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "DirectLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, direct_light_enable),
            },
            FieldInfoData {
                name: "SpecularEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, specular_enable),
            },
            FieldInfoData {
                name: "EnlightenColorMode",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENCOLORMODE_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, enlighten_color_mode),
            },
            FieldInfoData {
                name: "EnlightenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, enlighten_enable),
            },
            FieldInfoData {
                name: "EnlightenColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, enlighten_color_scale),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OLDPOINTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OldPointLightDynamicState {
    fn type_info() -> &'static TypeInfo {
        OLDPOINTLIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const OLDPOINTLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldPointLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OldPointLightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OldPointLightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub const OLDPOINTLIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldPointLightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OldPointLightStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(OLDPOINTLIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for OldPointLightStaticState {
    fn type_info() -> &'static TypeInfo {
        OLDPOINTLIGHTSTATICSTATE_TYPE_INFO
    }
}


pub const OLDPOINTLIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldPointLightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OldPointLightStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const OLDLIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldLightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, color),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, radius),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, intensity),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, attenuation_offset),
            },
            FieldInfoData {
                name: "DirectLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, direct_light_enable),
            },
            FieldInfoData {
                name: "SpecularEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, specular_enable),
            },
            FieldInfoData {
                name: "EnlightenColorMode",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENCOLORMODE_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, enlighten_color_mode),
            },
            FieldInfoData {
                name: "EnlightenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, enlighten_enable),
            },
            FieldInfoData {
                name: "EnlightenColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, enlighten_color_scale),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, particle_color_scale),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, exposure_compensation),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(OldLightDynamicState, excluded_cull_id),
            },
        ],
    }),
    array_type: Some(OLDLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OldLightDynamicState {
    fn type_info() -> &'static TypeInfo {
        OLDLIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const OLDLIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldLightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("OldLightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const LIGHTDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LightDynamicState, is_first_person),
            },
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LightDynamicState, is_enabled),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightDynamicState, intensity_multiplier),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LightDynamicState, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightDynamicState, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LightDynamicState, shadow_cache_update_counter),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(LightDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(LightDynamicState, excluded_cull_id),
            },
        ],
    }),
    array_type: Some(LIGHTDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LightDynamicState {
    fn type_info() -> &'static TypeInfo {
        LIGHTDYNAMICSTATE_TYPE_INFO
    }
}


pub const LIGHTDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LightDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LightStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
}

pub const LIGHTSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(LightStaticState, transform_space),
            },
        ],
    }),
    array_type: Some(LIGHTSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LightStaticState {
    fn type_info() -> &'static TypeInfo {
        LIGHTSTATICSTATE_TYPE_INFO
    }
}


pub const LIGHTSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LightStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IesProfileAsset {
    pub source_resource: super::core::ResourceRef,
}

pub const IESPROFILEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IesProfileAsset",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SourceResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(IesProfileAsset, source_resource),
            },
        ],
    }),
    array_type: Some(IESPROFILEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IesProfileAsset {
    fn type_info() -> &'static TypeInfo {
        IESPROFILEASSET_TYPE_INFO
    }
}


pub const IESPROFILEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IesProfileAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("IesProfileAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SpotLightShape {
    #[default]
    SpotLightShape_Cone = 0,
    SpotLightShape_Frustum = 1,
    SpotLightShape_OrthoFrustum = 2,
}

pub const SPOTLIGHTSHAPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpotLightShape",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(SPOTLIGHTSHAPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SpotLightShape {
    fn type_info() -> &'static TypeInfo {
        SPOTLIGHTSHAPE_TYPE_INFO
    }
}


pub const SPOTLIGHTSHAPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpotLightShape-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SpotLightShape-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EnlightenColorMode {
    #[default]
    EnlightenColorMode_Multiply = 0,
    EnlightenColorMode_Override = 1,
}

pub const ENLIGHTENCOLORMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenColorMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(ENLIGHTENCOLORMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EnlightenColorMode {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENCOLORMODE_TYPE_INFO
    }
}


pub const ENLIGHTENCOLORMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenColorMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("EnlightenColorMode-Array"),
    array_type: None,
    alignment: 8,
};



pub const SETLIGHTTRANSFORM_LIGHTHANDLE_LINEARTRANSFORM__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetLightTransform(LightHandle,LinearTransform)",
    flags: MemberInfoFlags::new(793),
    module: "WorldBase",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const CREATELIGHT_LIGHTHANDLE_LINEARTRANSFORM_VEC3__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreateLight(LightHandle,LinearTransform,Vec3)",
    flags: MemberInfoFlags::new(793),
    module: "WorldBase",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LensFlareCreateState {
    pub transform: super::core::LinearTransform,
    pub is_world_space: bool,
    pub parent_transform_space: super::state_stream::TransformSpaceHandle,
}

pub const LENSFLARECREATESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareCreateState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(LensFlareCreateState, transform),
            },
            FieldInfoData {
                name: "IsWorldSpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareCreateState, is_world_space),
            },
            FieldInfoData {
                name: "ParentTransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(LensFlareCreateState, parent_transform_space),
            },
        ],
    }),
    array_type: Some(LENSFLARECREATESTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensFlareCreateState {
    fn type_info() -> &'static TypeInfo {
        LENSFLARECREATESTATE_TYPE_INFO
    }
}


pub const LENSFLARECREATESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareCreateState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LensFlareCreateState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LensFlareDynamicState {
    pub flare_direction_mode: FlareDirectionMode,
    pub visible: bool,
    pub dimmer: f32,
    pub field_flag_changed0: u8,
}

pub const LENSFLAREDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FlareDirectionMode",
                flags: MemberInfoFlags::new(0),
                field_type: FLAREDIRECTIONMODE_TYPE_INFO,
                rust_offset: offset_of!(LensFlareDynamicState, flare_direction_mode),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareDynamicState, visible),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareDynamicState, dimmer),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(LensFlareDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LENSFLAREDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LensFlareDynamicState {
    fn type_info() -> &'static TypeInfo {
        LENSFLAREDYNAMICSTATE_TYPE_INFO
    }
}


pub const LENSFLAREDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LensFlareDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const LENSFLARESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(LensFlareStaticState, transform_space),
            },
            FieldInfoData {
                name: "DebugDrawOccluder",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareStaticState, debug_draw_occluder),
            },
            FieldInfoData {
                name: "HalfRes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareStaticState, half_res),
            },
            FieldInfoData {
                name: "OccluderSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareStaticState, occluder_size),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: LENSFLARERENDERMODE_TYPE_INFO,
                rust_offset: offset_of!(LensFlareStaticState, render_mode),
            },
            FieldInfoData {
                name: "ScreenClip",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareStaticState, screen_clip),
            },
            FieldInfoData {
                name: "DepthBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareStaticState, depth_bias),
            },
            FieldInfoData {
                name: "Elements",
                flags: MemberInfoFlags::new(144),
                field_type: LENSFLAREELEMENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LensFlareStaticState, elements),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(LensFlareStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LENSFLARESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LensFlareStaticState {
    fn type_info() -> &'static TypeInfo {
        LENSFLARESTATICSTATE_TYPE_INFO
    }
}


pub const LENSFLARESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LensFlareStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LensFlareRenderMode {
    #[default]
    LensFlareRenderMode_Half_Res = 0,
    LensFlareRenderMode_Full_Res = 1,
    LensFlareRenderMode_Foreground = 2,
    LensFlareRenderModeCount = 3,
}

pub const LENSFLARERENDERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareRenderMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(LENSFLARERENDERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LensFlareRenderMode {
    fn type_info() -> &'static TypeInfo {
        LENSFLARERENDERMODE_TYPE_INFO
    }
}


pub const LENSFLARERENDERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareRenderMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LensFlareRenderMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FlareDirectionMode {
    #[default]
    FlareDirectionMode_Entity = 0,
    FlareDirectionMode_Sun = 1,
}

pub const FLAREDIRECTIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlareDirectionMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(FLAREDIRECTIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FlareDirectionMode {
    fn type_info() -> &'static TypeInfo {
        FLAREDIRECTIONMODE_TYPE_INFO
    }
}


pub const FLAREDIRECTIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlareDirectionMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FlareDirectionMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const LENSFLAREELEMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareElement",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, enable),
            },
            FieldInfoData {
                name: "EnableElement",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, enable_element),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, shader),
            },
            FieldInfoData {
                name: "RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, ray_distance),
            },
            FieldInfoData {
                name: "RayDistanceX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, ray_distance_x),
            },
            FieldInfoData {
                name: "RayDistanceY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, ray_distance_y),
            },
            FieldInfoData {
                name: "CenterOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, center_offset),
            },
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, size),
            },
            FieldInfoData {
                name: "SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, size_occluder_curve),
            },
            FieldInfoData {
                name: "SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, size_screen_pos_curve),
            },
            FieldInfoData {
                name: "SizeAngleCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, size_angle_curve),
            },
            FieldInfoData {
                name: "SizeCamDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, size_cam_dist_curve),
            },
            FieldInfoData {
                name: "SizeCamDistMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, size_cam_dist_max),
            },
            FieldInfoData {
                name: "AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, alpha_occluder_curve),
            },
            FieldInfoData {
                name: "AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "AlphaAngleCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, alpha_angle_curve),
            },
            FieldInfoData {
                name: "AlphaCamDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, alpha_cam_dist_curve),
            },
            FieldInfoData {
                name: "AlphaCamDistMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, alpha_cam_dist_max),
            },
            FieldInfoData {
                name: "RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, rotation_local),
            },
            FieldInfoData {
                name: "RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, rotation_dist_curve),
            },
            FieldInfoData {
                name: "RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareElement, rotation_dist_multiplier),
            },
        ],
    }),
    array_type: Some(LENSFLAREELEMENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensFlareElement {
    fn type_info() -> &'static TypeInfo {
        LENSFLAREELEMENT_TYPE_INFO
    }
}


pub const LENSFLAREELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LensFlareElement-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReflectionVolumeQueryDynamicState {
    pub query_counter: u32,
    pub field_flag_changed0: u8,
}

pub const REFLECTIONVOLUMEQUERYDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeQueryDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "QueryCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReflectionVolumeQueryDynamicState, query_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ReflectionVolumeQueryDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(REFLECTIONVOLUMEQUERYDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ReflectionVolumeQueryDynamicState {
    fn type_info() -> &'static TypeInfo {
        REFLECTIONVOLUMEQUERYDYNAMICSTATE_TYPE_INFO
    }
}


pub const REFLECTIONVOLUMEQUERYDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeQueryDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ReflectionVolumeQueryDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReflectionVolumeQueryHandle {
}

pub const REFLECTIONVOLUMEQUERYHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeQueryHandle",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(REFLECTIONVOLUMEQUERYHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for ReflectionVolumeQueryHandle {
    fn type_info() -> &'static TypeInfo {
        REFLECTIONVOLUMEQUERYHANDLE_TYPE_INFO
    }
}


pub const REFLECTIONVOLUMEQUERYHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeQueryHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("ReflectionVolumeQueryHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalPlanarReflectionDynamicState {
    pub transform: super::core::LinearTransform,
    pub enable: bool,
    pub terrain_reflections_enable: bool,
    pub sky_reflection_enable: bool,
    pub field_flag_changed0: u8,
}

pub const LOCALPLANARREFLECTIONDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionDynamicState, transform),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionDynamicState, enable),
            },
            FieldInfoData {
                name: "TerrainReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionDynamicState, terrain_reflections_enable),
            },
            FieldInfoData {
                name: "SkyReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionDynamicState, sky_reflection_enable),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LOCALPLANARREFLECTIONDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalPlanarReflectionDynamicState {
    fn type_info() -> &'static TypeInfo {
        LOCALPLANARREFLECTIONDYNAMICSTATE_TYPE_INFO
    }
}


pub const LOCALPLANARREFLECTIONDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LocalPlanarReflectionDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalPlanarReflectionStaticState {
    pub guid: super::core::Guid,
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

pub const LOCALPLANARREFLECTIONSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, guid),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, transform_space),
            },
            FieldInfoData {
                name: "FarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, far_plane),
            },
            FieldInfoData {
                name: "ClippingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, clipping_offset),
            },
            FieldInfoData {
                name: "ClippingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, clipping_enable),
            },
            FieldInfoData {
                name: "DistanceTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, distance_tolerance),
            },
            FieldInfoData {
                name: "DistanceFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, distance_falloff),
            },
            FieldInfoData {
                name: "NormalTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, normal_tolerance),
            },
            FieldInfoData {
                name: "NormalFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, normal_falloff),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(LOCALPLANARREFLECTIONSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LocalPlanarReflectionStaticState {
    fn type_info() -> &'static TypeInfo {
        LOCALPLANARREFLECTIONSTATICSTATE_TYPE_INFO
    }
}


pub const LOCALPLANARREFLECTIONSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("LocalPlanarReflectionStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DistantIBLDynamicState {
    pub enabled: bool,
    pub is_visible: bool,
    pub refresh_counter: u32,
    pub field_flag_changed0: u8,
}

pub const DISTANTIBLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLDynamicState, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLDynamicState, is_visible),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DISTANTIBLDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DistantIBLDynamicState {
    fn type_info() -> &'static TypeInfo {
        DISTANTIBLDYNAMICSTATE_TYPE_INFO
    }
}


pub const DISTANTIBLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DistantIBLDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DistantIBLStaticState {
    pub location_type: super::render_base::DistantIBLLocationType,
    pub local_offset: super::core::Vec3,
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub guid: super::core::Guid,
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
    pub baked_texture: super::render_base::TextureBaseAsset,
    pub field_flag_changed0: u32,
}

pub const DISTANTIBLSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "LocationType",
                flags: MemberInfoFlags::new(0),
                field_type: DISTANTIBLLOCATIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, location_type),
            },
            FieldInfoData {
                name: "LocalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, local_offset),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, transform_space),
            },
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, guid),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIBLMODE_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, baked_texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DistantIBLStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DISTANTIBLSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DistantIBLStaticState {
    fn type_info() -> &'static TypeInfo {
        DISTANTIBLSTATICSTATE_TYPE_INFO
    }
}


pub const DISTANTIBLSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DistantIBLStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoxIBLDynamicState {
    pub enabled: bool,
    pub is_visible: bool,
    pub refresh_counter: u32,
    pub field_flag_changed0: u8,
}

pub const BOXIBLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxIBLDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLDynamicState, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLDynamicState, is_visible),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(BOXIBLDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BoxIBLDynamicState {
    fn type_info() -> &'static TypeInfo {
        BOXIBLDYNAMICSTATE_TYPE_INFO
    }
}


pub const BOXIBLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxIBLDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("BoxIBLDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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
    pub guid: super::core::Guid,
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
    pub baked_texture: super::render_base::TextureBaseAsset,
    pub field_flag_changed0: u32,
}

pub const BOXIBLSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxIBLStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "InfluenceFadeNormal",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, influence_fade_normal),
            },
            FieldInfoData {
                name: "SideFadePosX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_pos_x),
            },
            FieldInfoData {
                name: "SideFadeNegX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_neg_x),
            },
            FieldInfoData {
                name: "SideFadePosY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_pos_y),
            },
            FieldInfoData {
                name: "SideFadeNegY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_neg_y),
            },
            FieldInfoData {
                name: "SideFadePosZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_pos_z),
            },
            FieldInfoData {
                name: "SideFadeNegZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, side_fade_neg_z),
            },
            FieldInfoData {
                name: "LocalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, local_offset),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, transform_space),
            },
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, guid),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIBLMODE_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, baked_texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BoxIBLStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(BOXIBLSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoxIBLStaticState {
    fn type_info() -> &'static TypeInfo {
        BOXIBLSTATICSTATE_TYPE_INFO
    }
}


pub const BOXIBLSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxIBLStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("BoxIBLStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SphereIBLDynamicState {
    pub enabled: bool,
    pub is_visible: bool,
    pub refresh_counter: u32,
    pub field_flag_changed0: u8,
}

pub const SPHEREIBLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereIBLDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLDynamicState, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLDynamicState, is_visible),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SPHEREIBLDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SphereIBLDynamicState {
    fn type_info() -> &'static TypeInfo {
        SPHEREIBLDYNAMICSTATE_TYPE_INFO
    }
}


pub const SPHEREIBLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereIBLDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SphereIBLDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SphereIBLStaticState {
    pub influence_fade_normal: f32,
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub guid: super::core::Guid,
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
    pub baked_texture: super::render_base::TextureBaseAsset,
    pub field_flag_changed0: u32,
}

pub const SPHEREIBLSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereIBLStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "InfluenceFadeNormal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, influence_fade_normal),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, transform_space),
            },
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, guid),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIBLMODE_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, baked_texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SphereIBLStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SPHEREIBLSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SphereIBLStaticState {
    fn type_info() -> &'static TypeInfo {
        SPHEREIBLSTATICSTATE_TYPE_INFO
    }
}


pub const SPHEREIBLSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereIBLStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("SphereIBLStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IBLDynamicState {
    pub enabled: bool,
    pub is_visible: bool,
    pub refresh_counter: u32,
}

pub const IBLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IBLDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IBLDynamicState, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IBLDynamicState, is_visible),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(IBLDynamicState, refresh_counter),
            },
        ],
    }),
    array_type: Some(IBLDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IBLDynamicState {
    fn type_info() -> &'static TypeInfo {
        IBLDYNAMICSTATE_TYPE_INFO
    }
}


pub const IBLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IBLDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("IBLDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IBLStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub guid: super::core::Guid,
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
    pub baked_texture: super::render_base::TextureBaseAsset,
}

pub const IBLSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IBLStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, transform_space),
            },
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, guid),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIBLMODE_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(IBLStaticState, baked_texture),
            },
        ],
    }),
    array_type: Some(IBLSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IBLStaticState {
    fn type_info() -> &'static TypeInfo {
        IBLSTATICSTATE_TYPE_INFO
    }
}


pub const IBLSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IBLStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("IBLStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FogExclusionVolumeDynamicState {
    pub enabled: bool,
    pub transform: super::core::LinearTransform,
    pub field_flag_changed0: u8,
}

pub const FOGEXCLUSIONVOLUMEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeDynamicState, enabled),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeDynamicState, transform),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(FOGEXCLUSIONVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FogExclusionVolumeDynamicState {
    fn type_info() -> &'static TypeInfo {
        FOGEXCLUSIONVOLUMEDYNAMICSTATE_TYPE_INFO
    }
}


pub const FOGEXCLUSIONVOLUMEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FogExclusionVolumeDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FogExclusionVolumeStaticState {
    pub fog_exclusion_volume_shape: FogExclusionVolumeShape,
    pub fog_volume_strength: f32,
    pub fade_out_start: f32,
    pub fade_out_end: f32,
    pub field_flag_changed0: u8,
}

pub const FOGEXCLUSIONVOLUMESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FogExclusionVolumeShape",
                flags: MemberInfoFlags::new(0),
                field_type: FOGEXCLUSIONVOLUMESHAPE_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeStaticState, fog_exclusion_volume_shape),
            },
            FieldInfoData {
                name: "FogVolumeStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeStaticState, fog_volume_strength),
            },
            FieldInfoData {
                name: "FadeOutStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeStaticState, fade_out_start),
            },
            FieldInfoData {
                name: "FadeOutEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeStaticState, fade_out_end),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(FOGEXCLUSIONVOLUMESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FogExclusionVolumeStaticState {
    fn type_info() -> &'static TypeInfo {
        FOGEXCLUSIONVOLUMESTATICSTATE_TYPE_INFO
    }
}


pub const FOGEXCLUSIONVOLUMESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FogExclusionVolumeStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FogExclusionVolumeShape {
    #[default]
    FogExclusionVolumeShape_Box = 0,
    FogExclusionVolumeShape_Sphere = 1,
}

pub const FOGEXCLUSIONVOLUMESHAPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeShape",
    flags: MemberInfoFlags::new(49429),
    module: "WorldBase",
    data: TypeInfoData::Enum,
    array_type: Some(FOGEXCLUSIONVOLUMESHAPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FogExclusionVolumeShape {
    fn type_info() -> &'static TypeInfo {
        FOGEXCLUSIONVOLUMESHAPE_TYPE_INFO
    }
}


pub const FOGEXCLUSIONVOLUMESHAPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeShape-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("FogExclusionVolumeShape-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DistantShadowCacheDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub refresh_counter: u32,
    pub field_flag_changed0: u8,
}

pub const DISTANTSHADOWCACHEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheDynamicState, enabled),
            },
            FieldInfoData {
                name: "RefreshCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheDynamicState, refresh_counter),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DISTANTSHADOWCACHEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DistantShadowCacheDynamicState {
    fn type_info() -> &'static TypeInfo {
        DISTANTSHADOWCACHEDYNAMICSTATE_TYPE_INFO
    }
}


pub const DISTANTSHADOWCACHEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DistantShadowCacheDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DistantShadowCacheStaticState {
    pub guid: super::core::Guid,
    pub mode: super::render_base::ShadowCacheMode,
    pub resolution: u32,
    pub doublebuffer: bool,
    pub depth_bias: super::render_base::ShadowCacheDepthBias,
    pub dynamic_prod_priority: u32,
    pub tiles_per_side: u32,
    pub do_not_update_baked_texture: bool,
    pub baked_texture: super::render_base::TextureBaseAsset,
    pub field_flag_changed0: u16,
}

pub const DISTANTSHADOWCACHESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheStaticState, guid),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: SHADOWCACHEMODE_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheStaticState, mode),
            },
            FieldInfoData {
                name: "Resolution",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheStaticState, resolution),
            },
            FieldInfoData {
                name: "Doublebuffer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheStaticState, doublebuffer),
            },
            FieldInfoData {
                name: "DepthBias",
                flags: MemberInfoFlags::new(0),
                field_type: SHADOWCACHEDEPTHBIAS_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheStaticState, depth_bias),
            },
            FieldInfoData {
                name: "DynamicProdPriority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheStaticState, dynamic_prod_priority),
            },
            FieldInfoData {
                name: "TilesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheStaticState, tiles_per_side),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheStaticState, baked_texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DISTANTSHADOWCACHESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistantShadowCacheStaticState {
    fn type_info() -> &'static TypeInfo {
        DISTANTSHADOWCACHESTATICSTATE_TYPE_INFO
    }
}


pub const DISTANTSHADOWCACHESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("DistantShadowCacheStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BakeableTextureDynamicState {
}

pub const BAKEABLETEXTUREDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BakeableTextureDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(BAKEABLETEXTUREDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BakeableTextureDynamicState {
    fn type_info() -> &'static TypeInfo {
        BAKEABLETEXTUREDYNAMICSTATE_TYPE_INFO
    }
}


pub const BAKEABLETEXTUREDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BakeableTextureDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("BakeableTextureDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BakeableTextureStaticState {
    pub do_not_update_baked_texture: bool,
    pub baked_texture: super::render_base::TextureBaseAsset,
}

pub const BAKEABLETEXTURESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BakeableTextureStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BakeableTextureStaticState, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(BakeableTextureStaticState, baked_texture),
            },
        ],
    }),
    array_type: Some(BAKEABLETEXTURESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BakeableTextureStaticState {
    fn type_info() -> &'static TypeInfo {
        BAKEABLETEXTURESTATICSTATE_TYPE_INFO
    }
}


pub const BAKEABLETEXTURESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BakeableTextureStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldBase",
    data: TypeInfoData::Array("BakeableTextureStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PoseConsumer {
}

pub const POSECONSUMER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseConsumer",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 0,
};

impl TypeObject for PoseConsumer {
    fn type_info() -> &'static TypeInfo {
        POSECONSUMER_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshHandle {
}

pub const MESHHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshHandle",
    flags: MemberInfoFlags::new(101),
    module: "WorldBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 0,
};

impl TypeObject for MeshHandle {
    fn type_info() -> &'static TypeInfo {
        MESHHANDLE_TYPE_INFO
    }
}

