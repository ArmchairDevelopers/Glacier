use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_world_sim_types(registry: &mut TypeRegistry) {
    registry.register_type(FORCEMANAGERSETTINGS_TYPE_INFO);
    registry.register_type(FORCEMANAGERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCEBAKED3DAS2X2DTEXENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALWINDFORCEBAKED3DAS2X2DTEXENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCECONEENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALWINDFORCECONEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCESPHEREENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALWINDFORCESPHEREENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCESAMPLERENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALWINDFORCESAMPLERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCEENTITYBASEDATA_TYPE_INFO);
    registry.register_type(LOCALWINDFORCEENTITYBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(LOCALWINDFORCEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCESTATICSTATE_TYPE_INFO);
    registry.register_type(LOCALWINDFORCESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(FORCEGROUPASSET_TYPE_INFO);
    registry.register_type(FORCEGROUPASSET_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCEBAKED3DAS2X2DTEXCOMPONENTDATA_TYPE_INFO);
    registry.register_type(LOCALWINDFORCEBAKED3DAS2X2DTEXCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCECONECOMPONENTDATA_TYPE_INFO);
    registry.register_type(LOCALWINDFORCECONECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCESPHERECOMPONENTDATA_TYPE_INFO);
    registry.register_type(LOCALWINDFORCESPHERECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCECOMPONENTBASEDATA_TYPE_INFO);
    registry.register_type(LOCALWINDFORCECOMPONENTBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOGICVISUALENVIRONMENTREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(LOGICVISUALENVIRONMENTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOGICVISUALENVIRONMENTENTITYDATA_TYPE_INFO);
    registry.register_type(LOGICVISUALENVIRONMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTBLUEPRINT_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTENTITYDATA_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTPRIORITYOFFSET_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTPRIORITYOFFSET_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTBLENDMODE_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTBLENDMODE_ARRAY_TYPE_INFO);
    registry.register_type(SONARPARAMSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(SONARPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(HOLOGRAMPARAMSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(HOLOGRAMPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(THREATALERTHIGHLIGHTPARAMSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(THREATALERTHIGHLIGHTPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTPARAMSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(FILMICEFFECTSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(FILMICEFFECTSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DEBUGCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DEBUGCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LENSSCOPECOMPONENTDATA_TYPE_INFO);
    registry.register_type(LENSSCOPECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(FILMGRAINCOMPONENTDATA_TYPE_INFO);
    registry.register_type(FILMGRAINCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(VIGNETTECOMPONENTDATA_TYPE_INFO);
    registry.register_type(VIGNETTECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DOFCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DOFCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(COLORCORRECTIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(COLORCORRECTIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(TONEMAPCOMPONENTDATA_TYPE_INFO);
    registry.register_type(TONEMAPCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SKYCELESTIALMULTICOMPONENTDATA_TYPE_INFO);
    registry.register_type(SKYCELESTIALMULTICOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SKYCELESTIALSINGLECOMPONENTDATA_TYPE_INFO);
    registry.register_type(SKYCELESTIALSINGLECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SKYCELESTIALROTATIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(SKYCELESTIALROTATIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERPARAMGLOBALCOMPONENTDATA_TYPE_INFO);
    registry.register_type(EMITTERPARAMGLOBALCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERPARAMCOMPONENTDATA_TYPE_INFO);
    registry.register_type(EMITTERPARAMCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(RAYTRACEREFLECTIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(RAYTRACEREFLECTIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SCREENSPACERAYTRACECOMPONENTDATA_TYPE_INFO);
    registry.register_type(SCREENSPACERAYTRACECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOTIONBLURCOMPONENTDATA_TYPE_INFO);
    registry.register_type(MOTIONBLURCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERLIGHTINGCOMPONENTDATA_TYPE_INFO);
    registry.register_type(CHARACTERLIGHTINGCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DAMAGEEFFECTCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DAMAGEEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SCREENEFFECTCOMPONENTDATA_TYPE_INFO);
    registry.register_type(SCREENEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(SHADOWSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(MESHSETTINGSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(MESHSETTINGSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CAMERAPARAMSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(CAMERAPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADERCOLORPARAMSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(SHADERCOLORPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(SHADERPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENCOMPONENTDATA_TYPE_INFO);
    registry.register_type(ENLIGHTENCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SUBSURFACESCATTERINGCOMPONENTDATA_TYPE_INFO);
    registry.register_type(SUBSURFACESCATTERINGCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICAOCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DYNAMICAOCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANTIALIASCOMPONENTDATA_TYPE_INFO);
    registry.register_type(ANTIALIASCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LENSREFLECTIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(LENSREFLECTIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SUNFLARECOMPONENTDATA_TYPE_INFO);
    registry.register_type(SUNFLARECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLOUDCOMPONENTDATA_TYPE_INFO);
    registry.register_type(CLOUDCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WINDCOMPONENTDATA_TYPE_INFO);
    registry.register_type(WINDCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICENVMAPCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DYNAMICENVMAPCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SKYCOMPONENTDATA_TYPE_INFO);
    registry.register_type(SKYCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(FOGCOMPONENTDATA_TYPE_INFO);
    registry.register_type(FOGCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(OUTDOORLIGHTCOMPONENTDATA_TYPE_INFO);
    registry.register_type(OUTDOORLIGHTCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(INDIRECTSPECULARCOMPONENTDATA_TYPE_INFO);
    registry.register_type(INDIRECTSPECULARCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMSTRINGPARAM_TYPE_INFO);
    registry.register_type(CUSTOMSTRINGPARAM_ARRAY_TYPE_INFO);
    registry.register_type(FILEFRAMENAMINGENUM_TYPE_INFO);
    registry.register_type(FILEFRAMENAMINGENUM_ARRAY_TYPE_INFO);
    registry.register_type(LAYERMODEENUM_TYPE_INFO);
    registry.register_type(LAYERMODEENUM_ARRAY_TYPE_INFO);
    registry.register_type(IMAGETYPEENUM_TYPE_INFO);
    registry.register_type(IMAGETYPEENUM_ARRAY_TYPE_INFO);
    registry.register_type(RENDERFRAMESTRACKDATA_TYPE_INFO);
    registry.register_type(RENDERFRAMESTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(RENDERFRAMESKEYFRAME_TYPE_INFO);
    registry.register_type(RENDERFRAMESKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLEVOLUMETRICSENTITYDATA_TYPE_INFO);
    registry.register_type(SIMPLEVOLUMETRICSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SCREENSHOTCAPTUREENTITYDATA_TYPE_INFO);
    registry.register_type(SCREENSHOTCAPTUREENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(OCCLUDERVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERPLANEENTITYDATA_TYPE_INFO);
    registry.register_type(OCCLUDERPLANEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERMESHENTITYDATA_TYPE_INFO);
    registry.register_type(OCCLUDERMESHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VISIBLEAREAENTITYDATA_TYPE_INFO);
    registry.register_type(VISIBLEAREAENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONLOCATORENTITYDATA_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONLOCATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTPROBEVOLUMEDATA_TYPE_INFO);
    registry.register_type(LIGHTPROBEVOLUMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATICENLIGHTENENTITYDATA_TYPE_INFO);
    registry.register_type(STATICENLIGHTENENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICENLIGHTENENTITYDATA_TYPE_INFO);
    registry.register_type(DYNAMICENLIGHTENENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENENTITYDATA_TYPE_INFO);
    registry.register_type(ENLIGHTENENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMODIFIERENTITYDATA_TYPE_INFO);
    registry.register_type(RADIOSITYMODIFIERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALOPACITYTRIGGERENTITYDATA_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALOPACITYTRIGGERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALINSTANCEENTITYDATA_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALINSTANCEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALENTITYDATA_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(GROUNDHEIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(GROUNDHEIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RENDERVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(RENDERVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MESHPROXYENTITYDATA_TYPE_INFO);
    registry.register_type(MESHPROXYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTICIPATINGMEDIAMATERIALENTITYDATA_TYPE_INFO);
    registry.register_type(PARTICIPATINGMEDIAMATERIALENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALFOGVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALFOGVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALVOLUMETRICENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALVOLUMETRICENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(REFLECTIONVOLUMESYNCHRONIZERENTITYDATA_TYPE_INFO);
    registry.register_type(REFLECTIONVOLUMESYNCHRONIZERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALPLANARREFLECTIONENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALPLANARREFLECTIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PBRDISTANTREFLECTIONVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(PBRDISTANTREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PBRBOXREFLECTIONVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(PBRBOXREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PBRGENERICBOXREFLECTIONVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(PBRGENERICBOXREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PBRSPHEREREFLECTIONVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(PBRSPHEREREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PBRREFLECTIONVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(PBRREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PBRRECTANGULARLIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(PBRRECTANGULARLIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PBRTUBELIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(PBRTUBELIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PBRSPOTLIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(PBRSPOTLIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PBRSPHERELIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(PBRSPHERELIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PBRANALYTICLIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(PBRANALYTICLIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPOTLIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(SPOTLIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(POINTLIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(POINTLIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALLIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALLIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LENSFLAREREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(LENSFLAREREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LENSFLAREBLUEPRINT_TYPE_INFO);
    registry.register_type(LENSFLAREBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(LENSFLAREENTITYDATA_TYPE_INFO);
    registry.register_type(LENSFLAREENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FOGEXCLUSIONVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(FOGEXCLUSIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DISTANTSHADOWCACHEVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(DISTANTSHADOWCACHEVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLEVOLUMETRICSENTITY_TYPE_INFO);
    registry.register_type(SIMPLEVOLUMETRICSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SCREENSHOTCAPTUREENTITY_TYPE_INFO);
    registry.register_type(SCREENSHOTCAPTUREENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RENDERVOLUMEENTITY_TYPE_INFO);
    registry.register_type(RENDERVOLUMEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMODIFIERENTITY_TYPE_INFO);
    registry.register_type(RADIOSITYMODIFIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALOPACITYTRIGGERENTITY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALOPACITYTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALINSTANCEENTITY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALINSTANCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALENTITY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIALENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONLOCATORENTITY_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONLOCATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERVOLUMEENTITY_TYPE_INFO);
    registry.register_type(OCCLUDERVOLUMEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERPLANEENTITY_TYPE_INFO);
    registry.register_type(OCCLUDERPLANEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERMESHENTITY_TYPE_INFO);
    registry.register_type(OCCLUDERMESHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(MESHPROXYENTITY_TYPE_INFO);
    registry.register_type(MESHPROXYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PARTICIPATINGMEDIAMATERIALENTITY_TYPE_INFO);
    registry.register_type(PARTICIPATINGMEDIAMATERIALENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOCALFOGVOLUMEENTITY_TYPE_INFO);
    registry.register_type(LOCALFOGVOLUMEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOCALPLANARREFLECTIONENTITY_TYPE_INFO);
    registry.register_type(LOCALPLANARREFLECTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(REFLECTIONVOLUMESYNCHRONIZERENTITY_TYPE_INFO);
    registry.register_type(REFLECTIONVOLUMESYNCHRONIZERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DISTANTIBLENTITY_TYPE_INFO);
    registry.register_type(DISTANTIBLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOCALBOXIBLENTITY_TYPE_INFO);
    registry.register_type(LOCALBOXIBLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOCALSPHEREIBLENTITY_TYPE_INFO);
    registry.register_type(LOCALSPHEREIBLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PBRRECTANGULARLIGHTENTITY_TYPE_INFO);
    registry.register_type(PBRRECTANGULARLIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PBRTUBELIGHTENTITY_TYPE_INFO);
    registry.register_type(PBRTUBELIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PBRSPOTLIGHTENTITY_TYPE_INFO);
    registry.register_type(PBRSPOTLIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PBRSPHERELIGHTENTITY_TYPE_INFO);
    registry.register_type(PBRSPHERELIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OLDSPOTLIGHTENTITY_TYPE_INFO);
    registry.register_type(OLDSPOTLIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OLDPOINTLIGHTENTITY_TYPE_INFO);
    registry.register_type(OLDPOINTLIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOCALLIGHTENTITY_TYPE_INFO);
    registry.register_type(LOCALLIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LENSFLAREENTITY_TYPE_INFO);
    registry.register_type(LENSFLAREENTITY_ARRAY_TYPE_INFO);
    registry.register_type(GROUNDHEIGHTENTITY_TYPE_INFO);
    registry.register_type(GROUNDHEIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FOGEXCLUSIONVOLUMEENTITY_TYPE_INFO);
    registry.register_type(FOGEXCLUSIONVOLUMEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICENLIGHTENENTITY_TYPE_INFO);
    registry.register_type(DYNAMICENLIGHTENENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DISTANTSHADOWCACHEVOLUMEENTITY_TYPE_INFO);
    registry.register_type(DISTANTSHADOWCACHEVOLUMEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BAKEABLETEXTUREENTITYDATA_TYPE_INFO);
    registry.register_type(BAKEABLETEXTUREENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WINDFORCESPHEREENTITY_TYPE_INFO);
    registry.register_type(WINDFORCESPHEREENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WINDFORCESPHERECOMPONENT_TYPE_INFO);
    registry.register_type(WINDFORCESPHERECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WINDFORCESAMPLERENTITY_TYPE_INFO);
    registry.register_type(WINDFORCESAMPLERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WINDFORCEENTITY_TYPE_INFO);
    registry.register_type(WINDFORCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WINDFORCECONEENTITY_TYPE_INFO);
    registry.register_type(WINDFORCECONEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WINDFORCECONECOMPONENT_TYPE_INFO);
    registry.register_type(WINDFORCECONECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WINDFORCECOMPONENT_TYPE_INFO);
    registry.register_type(WINDFORCECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WINDFORCEBAKED3DAS2X2DTEXENTITY_TYPE_INFO);
    registry.register_type(WINDFORCEBAKED3DAS2X2DTEXENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WINDFORCEBAKED3DAS2X2DTEXCOMPONENT_TYPE_INFO);
    registry.register_type(WINDFORCEBAKED3DAS2X2DTEXCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTENTITY_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RAYTRACEREFLECTIONCOMPONENT_TYPE_INFO);
    registry.register_type(RAYTRACEREFLECTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SONARPARAMSCOMPONENT_TYPE_INFO);
    registry.register_type(SONARPARAMSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(HOLOGRAMPARAMSCOMPONENT_TYPE_INFO);
    registry.register_type(HOLOGRAMPARAMSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(THREATALERTHIGHLIGHTPARAMSCOMPONENT_TYPE_INFO);
    registry.register_type(THREATALERTHIGHLIGHTPARAMSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTPARAMSCOMPONENT_TYPE_INFO);
    registry.register_type(OBJECTHIGHLIGHTPARAMSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(ANTIALIASCOMPONENT_TYPE_INFO);
    registry.register_type(ANTIALIASCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SCREENSPACERAYTRACECOMPONENT_TYPE_INFO);
    registry.register_type(SCREENSPACERAYTRACECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(DEBUGCOMPONENT_TYPE_INFO);
    registry.register_type(DEBUGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(LENSREFLECTIONCOMPONENT_TYPE_INFO);
    registry.register_type(LENSREFLECTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SKYCELESTIALROTATIONCOMPONENT_TYPE_INFO);
    registry.register_type(SKYCELESTIALROTATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SKYCELESTIALMULTICOMPONENT_TYPE_INFO);
    registry.register_type(SKYCELESTIALMULTICOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SKYCELESTIALSINGLECOMPONENT_TYPE_INFO);
    registry.register_type(SKYCELESTIALSINGLECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERPARAMGLOBALCOMPONENT_TYPE_INFO);
    registry.register_type(EMITTERPARAMGLOBALCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERPARAMCOMPONENT_TYPE_INFO);
    registry.register_type(EMITTERPARAMCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMSCOMPONENT_TYPE_INFO);
    registry.register_type(SHADERPARAMSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(MOTIONBLURCOMPONENT_TYPE_INFO);
    registry.register_type(MOTIONBLURCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SUBSURFACESCATTERINGCOMPONENT_TYPE_INFO);
    registry.register_type(SUBSURFACESCATTERINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERLIGHTINGCOMPONENT_TYPE_INFO);
    registry.register_type(CHARACTERLIGHTINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICENVMAPCOMPONENT_TYPE_INFO);
    registry.register_type(DYNAMICENVMAPCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONCOMPONENT_TYPE_INFO);
    registry.register_type(PLANARREFLECTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(DAMAGEEFFECTCOMPONENT_TYPE_INFO);
    registry.register_type(DAMAGEEFFECTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SCREENEFFECTCOMPONENT_TYPE_INFO);
    registry.register_type(SCREENEFFECTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWSCOMPONENT_TYPE_INFO);
    registry.register_type(SHADOWSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(MESHSETTINGSCOMPONENT_TYPE_INFO);
    registry.register_type(MESHSETTINGSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CAMERAPARAMSCOMPONENT_TYPE_INFO);
    registry.register_type(CAMERAPARAMSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(LENSSCOPECOMPONENT_TYPE_INFO);
    registry.register_type(LENSSCOPECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(FILMGRAINCOMPONENT_TYPE_INFO);
    registry.register_type(FILMGRAINCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(VIGNETTECOMPONENT_TYPE_INFO);
    registry.register_type(VIGNETTECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(DOFCOMPONENT_TYPE_INFO);
    registry.register_type(DOFCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(FILMICEFFECTSCOMPONENT_TYPE_INFO);
    registry.register_type(FILMICEFFECTSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICAOCOMPONENT_TYPE_INFO);
    registry.register_type(DYNAMICAOCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SUNFLARECOMPONENT_TYPE_INFO);
    registry.register_type(SUNFLARECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLOUDCOMPONENT_TYPE_INFO);
    registry.register_type(CLOUDCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WINDCOMPONENT_TYPE_INFO);
    registry.register_type(WINDCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(FOGCOMPONENT_TYPE_INFO);
    registry.register_type(FOGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(COLORCORRECTIONCOMPONENT_TYPE_INFO);
    registry.register_type(COLORCORRECTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(TONEMAPCOMPONENT_TYPE_INFO);
    registry.register_type(TONEMAPCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENCOMPONENT_TYPE_INFO);
    registry.register_type(ENLIGHTENCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(OUTDOORLIGHTCOMPONENT_TYPE_INFO);
    registry.register_type(OUTDOORLIGHTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(INDIRECTSPECULARCOMPONENT_TYPE_INFO);
    registry.register_type(INDIRECTSPECULARCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SKYCOMPONENT_TYPE_INFO);
    registry.register_type(SKYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(RENDERFRAMESTRACK_TYPE_INFO);
    registry.register_type(RENDERFRAMESTRACK_ARRAY_TYPE_INFO);
    registry.register_type(SERVERVISIBLEAREAENTITY_TYPE_INFO);
    registry.register_type(SERVERVISIBLEAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTVISIBLEAREAENTITY_TYPE_INFO);
    registry.register_type(CLIENTVISIBLEAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(STATICENLIGHTENENTITY_TYPE_INFO);
    registry.register_type(STATICENLIGHTENENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct ForceManagerSettings {
    pub vector_field_norm_value: f32,
    pub vector_field_cell_size: f32,
    pub vector_field_planar_height_position: f32,
    pub wind_enable: bool,
    pub forces_enable: bool,
    pub draw_stats: bool,
    pub draw_wind_graph: bool,
    pub draw_bounding_volumes: bool,
    pub draw_selection_bounding_volumes: bool,
    pub draw_selection_vector_field: bool,
    pub draw_selection_force_vector_field: bool,
    pub draw_planar_vector_field: bool,
    pub draw_external_vector_field: bool,
    pub vector_field_enable_wind: bool,
    pub vector_field_enable_sphere: bool,
    pub vector_field_enable_cone: bool,
    pub vector_field_enable_baked: bool,
    pub vector_field_camera_centered: bool,
    pub vector_field_size_x: f32,
    pub vector_field_size_y: f32,
    pub vector_field_size_z: f32,
    pub vector_field_center_x: f32,
    pub vector_field_center_y: f32,
    pub vector_field_center_z: f32,
    pub vector_field_point_size: f32,
    pub vector_field_arrow: bool,
    pub vector_field_lock_selection: bool,
}

pub const FORCEMANAGERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceManagerSettings",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "VectorFieldNormValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_norm_value),
            },
            FieldInfoData {
                name: "VectorFieldCellSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_cell_size),
            },
            FieldInfoData {
                name: "VectorFieldPlanarHeightPosition",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_planar_height_position),
            },
            FieldInfoData {
                name: "WindEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, wind_enable),
            },
            FieldInfoData {
                name: "ForcesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, forces_enable),
            },
            FieldInfoData {
                name: "DrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, draw_stats),
            },
            FieldInfoData {
                name: "DrawWindGraph",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, draw_wind_graph),
            },
            FieldInfoData {
                name: "DrawBoundingVolumes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, draw_bounding_volumes),
            },
            FieldInfoData {
                name: "DrawSelectionBoundingVolumes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, draw_selection_bounding_volumes),
            },
            FieldInfoData {
                name: "DrawSelectionVectorField",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, draw_selection_vector_field),
            },
            FieldInfoData {
                name: "DrawSelectionForceVectorField",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, draw_selection_force_vector_field),
            },
            FieldInfoData {
                name: "DrawPlanarVectorField",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, draw_planar_vector_field),
            },
            FieldInfoData {
                name: "DrawExternalVectorField",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, draw_external_vector_field),
            },
            FieldInfoData {
                name: "VectorFieldEnableWind",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_enable_wind),
            },
            FieldInfoData {
                name: "VectorFieldEnableSphere",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_enable_sphere),
            },
            FieldInfoData {
                name: "VectorFieldEnableCone",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_enable_cone),
            },
            FieldInfoData {
                name: "VectorFieldEnableBaked",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_enable_baked),
            },
            FieldInfoData {
                name: "VectorFieldCameraCentered",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_camera_centered),
            },
            FieldInfoData {
                name: "VectorFieldSizeX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_size_x),
            },
            FieldInfoData {
                name: "VectorFieldSizeY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_size_y),
            },
            FieldInfoData {
                name: "VectorFieldSizeZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_size_z),
            },
            FieldInfoData {
                name: "VectorFieldCenterX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_center_x),
            },
            FieldInfoData {
                name: "VectorFieldCenterY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_center_y),
            },
            FieldInfoData {
                name: "VectorFieldCenterZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_center_z),
            },
            FieldInfoData {
                name: "VectorFieldPointSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_point_size),
            },
            FieldInfoData {
                name: "VectorFieldArrow",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_arrow),
            },
            FieldInfoData {
                name: "VectorFieldLockSelection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceManagerSettings, vector_field_lock_selection),
            },
        ],
    }),
    array_type: Some(FORCEMANAGERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ForceManagerSettings {
    fn type_info() -> &'static TypeInfo {
        FORCEMANAGERSETTINGS_TYPE_INFO
    }
}


pub const FORCEMANAGERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceManagerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ForceManagerSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalWindForceBaked3DAs2x2DTexEntityData {
    pub size_x: f32,
    pub size_y: f32,
    pub size_z: f32,
    pub attenuation: f32,
    pub volume_slice_z_x: super::render_base::TextureBaseAsset,
    pub volume_slice_z_x_scale: super::core::Vec3,
    pub volume_slice_z_y: super::render_base::TextureBaseAsset,
    pub volume_slice_z_y_scale: super::core::Vec3,
}

pub const LOCALWINDFORCEBAKED3DAS2X2DTEXENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceBaked3DAs2x2DTexEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALWINDFORCEENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SizeX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexEntityData, size_x),
            },
            FieldInfoData {
                name: "SizeY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexEntityData, size_y),
            },
            FieldInfoData {
                name: "SizeZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexEntityData, size_z),
            },
            FieldInfoData {
                name: "Attenuation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexEntityData, attenuation),
            },
            FieldInfoData {
                name: "VolumeSliceZX",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexEntityData, volume_slice_z_x),
            },
            FieldInfoData {
                name: "VolumeSliceZXScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexEntityData, volume_slice_z_x_scale),
            },
            FieldInfoData {
                name: "VolumeSliceZY",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexEntityData, volume_slice_z_y),
            },
            FieldInfoData {
                name: "VolumeSliceZYScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexEntityData, volume_slice_z_y_scale),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCEBAKED3DAS2X2DTEXENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalWindForceBaked3DAs2x2DTexEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCEBAKED3DAS2X2DTEXENTITYDATA_TYPE_INFO
    }
}


pub const LOCALWINDFORCEBAKED3DAS2X2DTEXENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceBaked3DAs2x2DTexEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceBaked3DAs2x2DTexEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalWindForceConeEntityData {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub cone_inner_angle: f32,
    pub cone_outer_angle: f32,
}

pub const LOCALWINDFORCECONEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceConeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALWINDFORCEENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InnerRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceConeEntityData, inner_radius),
            },
            FieldInfoData {
                name: "OuterRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceConeEntityData, outer_radius),
            },
            FieldInfoData {
                name: "ConeInnerAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceConeEntityData, cone_inner_angle),
            },
            FieldInfoData {
                name: "ConeOuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceConeEntityData, cone_outer_angle),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCECONEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalWindForceConeEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCECONEENTITYDATA_TYPE_INFO
    }
}


pub const LOCALWINDFORCECONEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceConeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceConeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalWindForceSphereEntityData {
    pub radius: f32,
}

pub const LOCALWINDFORCESPHEREENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceSphereEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALWINDFORCEENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceSphereEntityData, radius),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCESPHEREENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalWindForceSphereEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCESPHEREENTITYDATA_TYPE_INFO
    }
}


pub const LOCALWINDFORCESPHEREENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceSphereEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceSphereEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalWindForceSamplerEntityData {
    pub radius: f32,
}

pub const LOCALWINDFORCESAMPLERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceSamplerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceSamplerEntityData, radius),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCESAMPLERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalWindForceSamplerEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCESAMPLERENTITYDATA_TYPE_INFO
    }
}


pub const LOCALWINDFORCESAMPLERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceSamplerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceSamplerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalWindForceEntityBaseData {
    pub force_group: ForceGroupAsset,
    pub strength: f32,
    pub variation: f32,
    pub variation_rate: f32,
    pub micro_variation: f32,
    pub hardness: f32,
    pub force_as_instant_velocity: f32,
}

pub const LOCALWINDFORCEENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceEntityBaseData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ForceGroup",
                flags: MemberInfoFlags::new(0),
                field_type: FORCEGROUPASSET_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceEntityBaseData, force_group),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceEntityBaseData, strength),
            },
            FieldInfoData {
                name: "Variation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceEntityBaseData, variation),
            },
            FieldInfoData {
                name: "VariationRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceEntityBaseData, variation_rate),
            },
            FieldInfoData {
                name: "MicroVariation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceEntityBaseData, micro_variation),
            },
            FieldInfoData {
                name: "Hardness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceEntityBaseData, hardness),
            },
            FieldInfoData {
                name: "ForceAsInstantVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceEntityBaseData, force_as_instant_velocity),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCEENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalWindForceEntityBaseData {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCEENTITYBASEDATA_TYPE_INFO
    }
}


pub const LOCALWINDFORCEENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceEntityBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceEntityBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalWindForceDynamicState {
    pub strength: f32,
    pub variation: f32,
    pub variation_rate: f32,
    pub micro_variation: f32,
    pub hardness: f32,
    pub force_as_instant_velocity: f32,
}

pub const LOCALWINDFORCEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WorldSim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceDynamicState, strength),
            },
            FieldInfoData {
                name: "Variation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceDynamicState, variation),
            },
            FieldInfoData {
                name: "VariationRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceDynamicState, variation_rate),
            },
            FieldInfoData {
                name: "MicroVariation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceDynamicState, micro_variation),
            },
            FieldInfoData {
                name: "Hardness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceDynamicState, hardness),
            },
            FieldInfoData {
                name: "ForceAsInstantVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceDynamicState, force_as_instant_velocity),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LocalWindForceDynamicState {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCEDYNAMICSTATE_TYPE_INFO
    }
}


pub const LOCALWINDFORCEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalWindForceStaticState {
    pub force_group: ForceGroupAsset,
}

pub const LOCALWINDFORCESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WorldSim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ForceGroup",
                flags: MemberInfoFlags::new(0),
                field_type: FORCEGROUPASSET_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceStaticState, force_group),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalWindForceStaticState {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCESTATICSTATE_TYPE_INFO
    }
}


pub const LOCALWINDFORCESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForceGroupAsset {
    pub mesh_scattering: bool,
    pub vegetation: bool,
    pub effects: bool,
    pub cloth: bool,
    pub physics: bool,
}

pub const FORCEGROUPASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceGroupAsset",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MeshScattering",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceGroupAsset, mesh_scattering),
            },
            FieldInfoData {
                name: "Vegetation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceGroupAsset, vegetation),
            },
            FieldInfoData {
                name: "Effects",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceGroupAsset, effects),
            },
            FieldInfoData {
                name: "Cloth",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceGroupAsset, cloth),
            },
            FieldInfoData {
                name: "Physics",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForceGroupAsset, physics),
            },
        ],
    }),
    array_type: Some(FORCEGROUPASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ForceGroupAsset {
    fn type_info() -> &'static TypeInfo {
        FORCEGROUPASSET_TYPE_INFO
    }
}


pub const FORCEGROUPASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForceGroupAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ForceGroupAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalWindForceBaked3DAs2x2DTexComponentData {
    pub size_x: f32,
    pub size_y: f32,
    pub size_z: f32,
    pub attenuation: f32,
    pub volume_slice_z_x: super::render_base::TextureBaseAsset,
    pub volume_slice_z_x_scale: super::core::Vec3,
    pub volume_slice_z_y: super::render_base::TextureBaseAsset,
    pub volume_slice_z_y_scale: super::core::Vec3,
}

pub const LOCALWINDFORCEBAKED3DAS2X2DTEXCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceBaked3DAs2x2DTexComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALWINDFORCECOMPONENTBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SizeX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexComponentData, size_x),
            },
            FieldInfoData {
                name: "SizeY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexComponentData, size_y),
            },
            FieldInfoData {
                name: "SizeZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexComponentData, size_z),
            },
            FieldInfoData {
                name: "Attenuation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexComponentData, attenuation),
            },
            FieldInfoData {
                name: "VolumeSliceZX",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexComponentData, volume_slice_z_x),
            },
            FieldInfoData {
                name: "VolumeSliceZXScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexComponentData, volume_slice_z_x_scale),
            },
            FieldInfoData {
                name: "VolumeSliceZY",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexComponentData, volume_slice_z_y),
            },
            FieldInfoData {
                name: "VolumeSliceZYScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceBaked3DAs2x2DTexComponentData, volume_slice_z_y_scale),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCEBAKED3DAS2X2DTEXCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalWindForceBaked3DAs2x2DTexComponentData {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCEBAKED3DAS2X2DTEXCOMPONENTDATA_TYPE_INFO
    }
}


pub const LOCALWINDFORCEBAKED3DAS2X2DTEXCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceBaked3DAs2x2DTexComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceBaked3DAs2x2DTexComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalWindForceConeComponentData {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub cone_inner_angle: f32,
    pub cone_outer_angle: f32,
}

pub const LOCALWINDFORCECONECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceConeComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALWINDFORCECOMPONENTBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InnerRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceConeComponentData, inner_radius),
            },
            FieldInfoData {
                name: "OuterRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceConeComponentData, outer_radius),
            },
            FieldInfoData {
                name: "ConeInnerAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceConeComponentData, cone_inner_angle),
            },
            FieldInfoData {
                name: "ConeOuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceConeComponentData, cone_outer_angle),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCECONECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalWindForceConeComponentData {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCECONECOMPONENTDATA_TYPE_INFO
    }
}


pub const LOCALWINDFORCECONECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceConeComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceConeComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalWindForceSphereComponentData {
    pub radius: f32,
}

pub const LOCALWINDFORCESPHERECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceSphereComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALWINDFORCECOMPONENTBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceSphereComponentData, radius),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCESPHERECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalWindForceSphereComponentData {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCESPHERECOMPONENTDATA_TYPE_INFO
    }
}


pub const LOCALWINDFORCESPHERECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceSphereComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceSphereComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalWindForceComponentBaseData {
    pub force_group: ForceGroupAsset,
    pub strength: f32,
    pub variation: f32,
    pub variation_rate: f32,
    pub micro_variation: f32,
    pub hardness: f32,
    pub force_as_instant_velocity: f32,
}

pub const LOCALWINDFORCECOMPONENTBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceComponentBaseData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ForceGroup",
                flags: MemberInfoFlags::new(0),
                field_type: FORCEGROUPASSET_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceComponentBaseData, force_group),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceComponentBaseData, strength),
            },
            FieldInfoData {
                name: "Variation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceComponentBaseData, variation),
            },
            FieldInfoData {
                name: "VariationRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceComponentBaseData, variation_rate),
            },
            FieldInfoData {
                name: "MicroVariation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceComponentBaseData, micro_variation),
            },
            FieldInfoData {
                name: "Hardness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceComponentBaseData, hardness),
            },
            FieldInfoData {
                name: "ForceAsInstantVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalWindForceComponentBaseData, force_as_instant_velocity),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCECOMPONENTBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalWindForceComponentBaseData {
    fn type_info() -> &'static TypeInfo {
        LOCALWINDFORCECOMPONENTBASEDATA_TYPE_INFO
    }
}


pub const LOCALWINDFORCECOMPONENTBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceComponentBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalWindForceComponentBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogicVisualEnvironmentReferenceObjectData {
}

pub const LOGICVISUALENVIRONMENTREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicVisualEnvironmentReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGICVISUALENVIRONMENTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogicVisualEnvironmentReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        LOGICVISUALENVIRONMENTREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const LOGICVISUALENVIRONMENTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicVisualEnvironmentReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LogicVisualEnvironmentReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VisualEnvironmentReferenceObjectData {
    pub priority: i32,
    pub override_visibility: bool,
    pub owned_by_lighting_context_pad: bool,
}

pub const VISUALENVIRONMENTREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentReferenceObjectData, priority),
            },
            FieldInfoData {
                name: "OverrideVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentReferenceObjectData, override_visibility),
            },
            FieldInfoData {
                name: "OwnedByLightingContextPad",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentReferenceObjectData, owned_by_lighting_context_pad),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VisualEnvironmentReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("VisualEnvironmentReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogicVisualEnvironmentEntityData {
    pub visual_environment: VisualEnvironmentBlueprint,
    pub visibility: f32,
}

pub const LOGICVISUALENVIRONMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicVisualEnvironmentEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "VisualEnvironment",
                flags: MemberInfoFlags::new(0),
                field_type: VISUALENVIRONMENTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(LogicVisualEnvironmentEntityData, visual_environment),
            },
            FieldInfoData {
                name: "Visibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LogicVisualEnvironmentEntityData, visibility),
            },
        ],
    }),
    array_type: Some(LOGICVISUALENVIRONMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LogicVisualEnvironmentEntityData {
    fn type_info() -> &'static TypeInfo {
        LOGICVISUALENVIRONMENTENTITYDATA_TYPE_INFO
    }
}


pub const LOGICVISUALENVIRONMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicVisualEnvironmentEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LogicVisualEnvironmentEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VisualEnvironmentBlueprint {
    pub time_delta_type: super::entity::TimeDeltaType,
}

pub const VISUALENVIRONMENTBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBJECTBLUEPRINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: TIMEDELTATYPE_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentBlueprint, time_delta_type),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualEnvironmentBlueprint {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTBLUEPRINT_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("VisualEnvironmentBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VisualEnvironmentEntityData {
    pub visibility: f32,
    pub priority: i32,
    pub local_player_view_id: super::render_base::LocalPlayerViewId,
    pub explicit_priority_enable: bool,
    pub blend_mode: VisualEnvironmentBlendMode,
    pub view_id: super::render_base::LocalPlayerViewId,
}

pub const VISUALENVIRONMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Visibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEntityData, visibility),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEntityData, priority),
            },
            FieldInfoData {
                name: "LocalPlayerViewId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERVIEWID_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEntityData, local_player_view_id),
            },
            FieldInfoData {
                name: "ExplicitPriorityEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEntityData, explicit_priority_enable),
            },
            FieldInfoData {
                name: "BlendMode",
                flags: MemberInfoFlags::new(0),
                field_type: VISUALENVIRONMENTBLENDMODE_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEntityData, blend_mode),
            },
            FieldInfoData {
                name: "ViewId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERVIEWID_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEntityData, view_id),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VisualEnvironmentEntityData {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTENTITYDATA_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("VisualEnvironmentEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VisualEnvironmentPriorityOffset {
    #[default]
    VisualEnvironmentPriorityOffset_Camera = 10000,
    VisualEnvironmentPriorityOffset_Logic = 10000000,
}

pub const VISUALENVIRONMENTPRIORITYOFFSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentPriorityOffset",
    flags: MemberInfoFlags::new(49429),
    module: "WorldSim",
    data: TypeInfoData::Enum,
    array_type: Some(VISUALENVIRONMENTPRIORITYOFFSET_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VisualEnvironmentPriorityOffset {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTPRIORITYOFFSET_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTPRIORITYOFFSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentPriorityOffset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("VisualEnvironmentPriorityOffset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VisualEnvironmentBlendMode {
    #[default]
    VisualEnvironmentBlendMode_Lerp = 0,
    VisualEnvironmentBlendMode_Add = 1,
    VisualEnvironmentBlendMode_Subtract = 2,
}

pub const VISUALENVIRONMENTBLENDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentBlendMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldSim",
    data: TypeInfoData::Enum,
    array_type: Some(VISUALENVIRONMENTBLENDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VisualEnvironmentBlendMode {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTBLENDMODE_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTBLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentBlendMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("VisualEnvironmentBlendMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SonarParamsComponentData {
    pub enable: bool,
    pub color1: super::core::Vec3,
    pub color2: super::core::Vec3,
    pub color3: super::core::Vec3,
    pub field_flag_override0: u8,
}

pub const SONARPARAMSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SonarParamsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentData, enable),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentData, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentData, color2),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentData, color3),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SonarParamsComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(SONARPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SonarParamsComponentData {
    fn type_info() -> &'static TypeInfo {
        SONARPARAMSCOMPONENTDATA_TYPE_INFO
    }
}


pub const SONARPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SonarParamsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SonarParamsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HologramParamsComponentData {
    pub enable: bool,
    pub render_mode: super::world_base::HologramRenderMode,
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
}

pub const HOLOGRAMPARAMSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramParamsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, enable),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: HOLOGRAMRENDERMODE_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, render_mode),
            },
            FieldInfoData {
                name: "KeyIlluminance",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, key_illuminance),
            },
            FieldInfoData {
                name: "KeyLightDir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, key_light_dir),
            },
            FieldInfoData {
                name: "ResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, resolution_scale),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, color2),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, color3),
            },
            FieldInfoData {
                name: "Color4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, color4),
            },
            FieldInfoData {
                name: "Color5",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, color5),
            },
            FieldInfoData {
                name: "Float1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, float1),
            },
            FieldInfoData {
                name: "Float2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, float2),
            },
            FieldInfoData {
                name: "Float3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, float3),
            },
            FieldInfoData {
                name: "Float4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, float4),
            },
            FieldInfoData {
                name: "Float5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, float5),
            },
            FieldInfoData {
                name: "Float6",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, float6),
            },
            FieldInfoData {
                name: "Float7",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, float7),
            },
            FieldInfoData {
                name: "Float8",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, float8),
            },
            FieldInfoData {
                name: "Float9",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, float9),
            },
            FieldInfoData {
                name: "Brightness2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, brightness2),
            },
            FieldInfoData {
                name: "Brightness3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, brightness3),
            },
            FieldInfoData {
                name: "Brightness4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, brightness4),
            },
            FieldInfoData {
                name: "Brightness5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, brightness5),
            },
            FieldInfoData {
                name: "StreaksEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, streaks_enabled),
            },
            FieldInfoData {
                name: "TimeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, time_offset),
            },
            FieldInfoData {
                name: "Opacity1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, opacity1),
            },
            FieldInfoData {
                name: "DistortionScale1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, distortion_scale1),
            },
            FieldInfoData {
                name: "SourcePos1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, source_pos1),
            },
            FieldInfoData {
                name: "TargetPos1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, target_pos1),
            },
            FieldInfoData {
                name: "SourceRadius1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, source_radius1),
            },
            FieldInfoData {
                name: "TargetRadius1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, target_radius1),
            },
            FieldInfoData {
                name: "Opacity2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, opacity2),
            },
            FieldInfoData {
                name: "DistortionScale2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, distortion_scale2),
            },
            FieldInfoData {
                name: "SourcePos2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, source_pos2),
            },
            FieldInfoData {
                name: "TargetPos2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, target_pos2),
            },
            FieldInfoData {
                name: "SourceRadius2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, source_radius2),
            },
            FieldInfoData {
                name: "TargetRadius2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, target_radius2),
            },
            FieldInfoData {
                name: "Opacity3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, opacity3),
            },
            FieldInfoData {
                name: "DistortionScale3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, distortion_scale3),
            },
            FieldInfoData {
                name: "SourcePos3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, source_pos3),
            },
            FieldInfoData {
                name: "TargetPos3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, target_pos3),
            },
            FieldInfoData {
                name: "SourceRadius3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, source_radius3),
            },
            FieldInfoData {
                name: "TargetRadius3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, target_radius3),
            },
            FieldInfoData {
                name: "Opacity4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, opacity4),
            },
            FieldInfoData {
                name: "DistortionScale4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, distortion_scale4),
            },
            FieldInfoData {
                name: "SourcePos4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, source_pos4),
            },
            FieldInfoData {
                name: "TargetPos4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, target_pos4),
            },
            FieldInfoData {
                name: "SourceRadius4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, source_radius4),
            },
            FieldInfoData {
                name: "TargetRadius4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, target_radius4),
            },
            FieldInfoData {
                name: "Opacity5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, opacity5),
            },
            FieldInfoData {
                name: "DistortionScale5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, distortion_scale5),
            },
            FieldInfoData {
                name: "SourcePos5",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, source_pos5),
            },
            FieldInfoData {
                name: "TargetPos5",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, target_pos5),
            },
            FieldInfoData {
                name: "SourceRadius5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, source_radius5),
            },
            FieldInfoData {
                name: "TargetRadius5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, target_radius5),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HologramParamsComponentData, field_flag_override1),
            },
        ],
    }),
    array_type: Some(HOLOGRAMPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HologramParamsComponentData {
    fn type_info() -> &'static TypeInfo {
        HOLOGRAMPARAMSCOMPONENTDATA_TYPE_INFO
    }
}


pub const HOLOGRAMPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramParamsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("HologramParamsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ThreatAlertHighlightParamsComponentData {
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
}

pub const THREATALERTHIGHLIGHTPARAMSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreatAlertHighlightParamsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, enable),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, color1),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, color2),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, color3),
            },
            FieldInfoData {
                name: "Color4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, color4),
            },
            FieldInfoData {
                name: "Color5",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, color5),
            },
            FieldInfoData {
                name: "UseOutline",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, use_outline),
            },
            FieldInfoData {
                name: "OutlineOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, outline_opacity),
            },
            FieldInfoData {
                name: "UseScanLines",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, use_scan_lines),
            },
            FieldInfoData {
                name: "ScanLineOffset",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, scan_line_offset),
            },
            FieldInfoData {
                name: "ScanLineOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, scan_line_opacity),
            },
            FieldInfoData {
                name: "ScanLineThickness",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, scan_line_thickness),
            },
            FieldInfoData {
                name: "ScanLineSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, scan_line_spacing),
            },
            FieldInfoData {
                name: "UseHorizontalScanLines",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, use_horizontal_scan_lines),
            },
            FieldInfoData {
                name: "UseAltLines",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, use_alt_lines),
            },
            FieldInfoData {
                name: "AltLineOffset",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, alt_line_offset),
            },
            FieldInfoData {
                name: "AltLineOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, alt_line_opacity),
            },
            FieldInfoData {
                name: "AltLineThickness",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, alt_line_thickness),
            },
            FieldInfoData {
                name: "AltLineSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, alt_line_spacing),
            },
            FieldInfoData {
                name: "UseHorizontalAltLines",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, use_horizontal_alt_lines),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ThreatAlertHighlightParamsComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(THREATALERTHIGHLIGHTPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ThreatAlertHighlightParamsComponentData {
    fn type_info() -> &'static TypeInfo {
        THREATALERTHIGHLIGHTPARAMSCOMPONENTDATA_TYPE_INFO
    }
}


pub const THREATALERTHIGHLIGHTPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreatAlertHighlightParamsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ThreatAlertHighlightParamsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ObjectHighlightParamsComponentData {
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
}

pub const OBJECTHIGHLIGHTPARAMSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightParamsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, enable),
            },
            FieldInfoData {
                name: "Brightness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, brightness),
            },
            FieldInfoData {
                name: "Color1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, color1),
            },
            FieldInfoData {
                name: "Color1Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, color1_alpha),
            },
            FieldInfoData {
                name: "Color2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, color2),
            },
            FieldInfoData {
                name: "Color2Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, color2_alpha),
            },
            FieldInfoData {
                name: "Color3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, color3),
            },
            FieldInfoData {
                name: "Color3Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, color3_alpha),
            },
            FieldInfoData {
                name: "Color4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, color4),
            },
            FieldInfoData {
                name: "Color4Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, color4_alpha),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ObjectHighlightParamsComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(OBJECTHIGHLIGHTPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ObjectHighlightParamsComponentData {
    fn type_info() -> &'static TypeInfo {
        OBJECTHIGHLIGHTPARAMSCOMPONENTDATA_TYPE_INFO
    }
}


pub const OBJECTHIGHLIGHTPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightParamsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ObjectHighlightParamsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FilmicEffectsComponentData {
    pub enable: bool,
    pub enable_chromatic_abberation: bool,
    pub chromatic_abberation_scale: f32,
    pub chromatic_abberation_aspect_ratio: f32,
    pub enable_vignetting: bool,
    pub vignetting_falloff: f32,
    pub vignetting_luminance_percent: f32,
    pub enable_lens_distortion: bool,
    pub lens_distortion_gain: f32,
    pub lens_distortion_cubic_gain: f32,
    pub lens_distortion_stretch: f32,
    pub enable_frame_flash: bool,
    pub frame_flash_gain: f32,
    pub enable_depth_flash: bool,
    pub depth_flash_atmos_color: super::core::Vec3,
    pub depth_flash_half_distance: f32,
    pub enable_distance_blur: bool,
    pub distance_blur_gain: f32,
    pub distance_blur_half_distance: f32,
    pub enable_edge_blur: bool,
    pub edge_blur_gain: f32,
    pub edge_blur_depth_target_scale: f32,
    pub edge_blur_fade_near_depth: f32,
    pub edge_blur_fade_far_depth: f32,
    pub edge_blur_matte_dilate_size: f32,
    pub edge_blur_matte_blur_kernel_size: f32,
    pub enable_heat_ripple: bool,
    pub heat_ripple_gain: f32,
    pub heat_ripple_horizontal_speed: f32,
    pub heat_ripple_vertical_speed: f32,
    pub heat_ripple_noise_scale: f32,
    pub heat_ripple_near_distance: f32,
    pub heat_ripple_far_distance: f32,
    pub heat_ripple_near_gain: f32,
    pub heat_ripple_far_gain: f32,
    pub heat_ripple_texture: super::render_base::TextureBaseAsset,
    pub field_flag_override0: u32,
    pub field_flag_override1: u8,
}

pub const FILMICEFFECTSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmicEffectsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, enable),
            },
            FieldInfoData {
                name: "EnableChromaticAbberation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, enable_chromatic_abberation),
            },
            FieldInfoData {
                name: "ChromaticAbberationScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, chromatic_abberation_scale),
            },
            FieldInfoData {
                name: "ChromaticAbberationAspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, chromatic_abberation_aspect_ratio),
            },
            FieldInfoData {
                name: "EnableVignetting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, enable_vignetting),
            },
            FieldInfoData {
                name: "VignettingFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, vignetting_falloff),
            },
            FieldInfoData {
                name: "VignettingLuminancePercent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, vignetting_luminance_percent),
            },
            FieldInfoData {
                name: "EnableLensDistortion",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, enable_lens_distortion),
            },
            FieldInfoData {
                name: "LensDistortionGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, lens_distortion_gain),
            },
            FieldInfoData {
                name: "LensDistortionCubicGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, lens_distortion_cubic_gain),
            },
            FieldInfoData {
                name: "LensDistortionStretch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, lens_distortion_stretch),
            },
            FieldInfoData {
                name: "EnableFrameFlash",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, enable_frame_flash),
            },
            FieldInfoData {
                name: "FrameFlashGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, frame_flash_gain),
            },
            FieldInfoData {
                name: "EnableDepthFlash",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, enable_depth_flash),
            },
            FieldInfoData {
                name: "DepthFlashAtmosColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, depth_flash_atmos_color),
            },
            FieldInfoData {
                name: "DepthFlashHalfDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, depth_flash_half_distance),
            },
            FieldInfoData {
                name: "EnableDistanceBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, enable_distance_blur),
            },
            FieldInfoData {
                name: "DistanceBlurGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, distance_blur_gain),
            },
            FieldInfoData {
                name: "DistanceBlurHalfDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, distance_blur_half_distance),
            },
            FieldInfoData {
                name: "EnableEdgeBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, enable_edge_blur),
            },
            FieldInfoData {
                name: "EdgeBlurGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, edge_blur_gain),
            },
            FieldInfoData {
                name: "EdgeBlurDepthTargetScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, edge_blur_depth_target_scale),
            },
            FieldInfoData {
                name: "EdgeBlurFadeNearDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, edge_blur_fade_near_depth),
            },
            FieldInfoData {
                name: "EdgeBlurFadeFarDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, edge_blur_fade_far_depth),
            },
            FieldInfoData {
                name: "EdgeBlurMatteDilateSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, edge_blur_matte_dilate_size),
            },
            FieldInfoData {
                name: "EdgeBlurMatteBlurKernelSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, edge_blur_matte_blur_kernel_size),
            },
            FieldInfoData {
                name: "EnableHeatRipple",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, enable_heat_ripple),
            },
            FieldInfoData {
                name: "HeatRippleGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, heat_ripple_gain),
            },
            FieldInfoData {
                name: "HeatRippleHorizontalSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, heat_ripple_horizontal_speed),
            },
            FieldInfoData {
                name: "HeatRippleVerticalSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, heat_ripple_vertical_speed),
            },
            FieldInfoData {
                name: "HeatRippleNoiseScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, heat_ripple_noise_scale),
            },
            FieldInfoData {
                name: "HeatRippleNearDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, heat_ripple_near_distance),
            },
            FieldInfoData {
                name: "HeatRippleFarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, heat_ripple_far_distance),
            },
            FieldInfoData {
                name: "HeatRippleNearGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, heat_ripple_near_gain),
            },
            FieldInfoData {
                name: "HeatRippleFarGain",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, heat_ripple_far_gain),
            },
            FieldInfoData {
                name: "HeatRippleTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, heat_ripple_texture),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(FilmicEffectsComponentData, field_flag_override1),
            },
        ],
    }),
    array_type: Some(FILMICEFFECTSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FilmicEffectsComponentData {
    fn type_info() -> &'static TypeInfo {
        FILMICEFFECTSCOMPONENTDATA_TYPE_INFO
    }
}


pub const FILMICEFFECTSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmicEffectsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("FilmicEffectsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DebugComponentData {
    pub enable: bool,
    pub fullscreen: bool,
    pub debug_texture: super::render_base::TextureBaseAsset,
    pub field_flag_override0: u8,
}

pub const DEBUGCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebugComponentData, enable),
            },
            FieldInfoData {
                name: "Fullscreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebugComponentData, fullscreen),
            },
            FieldInfoData {
                name: "DebugTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DebugComponentData, debug_texture),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DebugComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(DEBUGCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DebugComponentData {
    fn type_info() -> &'static TypeInfo {
        DEBUGCOMPONENTDATA_TYPE_INFO
    }
}


pub const DEBUGCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DebugComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LensScopeComponentData {
    pub enable: bool,
    pub blur_scale: f32,
    pub blur_center: super::core::Vec2,
    pub chromatic_aberration_color1: super::core::Vec3,
    pub chromatic_aberration_color2: super::core::Vec3,
    pub chromatic_aberration_strengths: super::core::Vec2,
    pub chromatic_aberration_displacement1: super::core::Vec2,
    pub chromatic_aberration_displacement2: super::core::Vec2,
    pub radial_blend_distance_coefficients: super::core::Vec2,
    pub field_flag_override0: u16,
}

pub const LENSSCOPECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensScopeComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentData, enable),
            },
            FieldInfoData {
                name: "BlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentData, blur_scale),
            },
            FieldInfoData {
                name: "BlurCenter",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentData, blur_center),
            },
            FieldInfoData {
                name: "ChromaticAberrationColor1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentData, chromatic_aberration_color1),
            },
            FieldInfoData {
                name: "ChromaticAberrationColor2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentData, chromatic_aberration_color2),
            },
            FieldInfoData {
                name: "ChromaticAberrationStrengths",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentData, chromatic_aberration_strengths),
            },
            FieldInfoData {
                name: "ChromaticAberrationDisplacement1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentData, chromatic_aberration_displacement1),
            },
            FieldInfoData {
                name: "ChromaticAberrationDisplacement2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentData, chromatic_aberration_displacement2),
            },
            FieldInfoData {
                name: "RadialBlendDistanceCoefficients",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentData, radial_blend_distance_coefficients),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(LensScopeComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(LENSSCOPECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensScopeComponentData {
    fn type_info() -> &'static TypeInfo {
        LENSSCOPECOMPONENTDATA_TYPE_INFO
    }
}


pub const LENSSCOPECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensScopeComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LensScopeComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FilmGrainComponentData {
    pub enable: bool,
    pub texture_scale: super::core::Vec2,
    pub color_scale: super::core::Vec3,
    pub linear_filtering_enable: bool,
    pub random_enable: bool,
    pub texture: super::render_base::TextureBaseAsset,
    pub grain_grey_fraction: f32,
    pub grain_luminance_control_enable: bool,
    pub grain_shadow_threshold: f32,
    pub grain_highlight_threshold: f32,
    pub grain_shadow_intensity: f32,
    pub grain_highlight_intensity: f32,
    pub field_flag_override0: u16,
}

pub const FILMGRAINCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmGrainComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, enable),
            },
            FieldInfoData {
                name: "TextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, texture_scale),
            },
            FieldInfoData {
                name: "ColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, color_scale),
            },
            FieldInfoData {
                name: "LinearFilteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, linear_filtering_enable),
            },
            FieldInfoData {
                name: "RandomEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, random_enable),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, texture),
            },
            FieldInfoData {
                name: "GrainGreyFraction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, grain_grey_fraction),
            },
            FieldInfoData {
                name: "GrainLuminanceControlEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, grain_luminance_control_enable),
            },
            FieldInfoData {
                name: "GrainShadowThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, grain_shadow_threshold),
            },
            FieldInfoData {
                name: "GrainHighlightThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, grain_highlight_threshold),
            },
            FieldInfoData {
                name: "GrainShadowIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, grain_shadow_intensity),
            },
            FieldInfoData {
                name: "GrainHighlightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, grain_highlight_intensity),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(FilmGrainComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(FILMGRAINCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FilmGrainComponentData {
    fn type_info() -> &'static TypeInfo {
        FILMGRAINCOMPONENTDATA_TYPE_INFO
    }
}


pub const FILMGRAINCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmGrainComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("FilmGrainComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VignetteComponentData {
    pub enable: bool,
    pub scale: super::core::Vec2,
    pub exponent: f32,
    pub color: super::core::Vec3,
    pub opacity: f32,
    pub field_flag_override0: u8,
}

pub const VIGNETTECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignetteComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentData, enable),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentData, scale),
            },
            FieldInfoData {
                name: "Exponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentData, exponent),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentData, color),
            },
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentData, opacity),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(VignetteComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(VIGNETTECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VignetteComponentData {
    fn type_info() -> &'static TypeInfo {
        VIGNETTECOMPONENTDATA_TYPE_INFO
    }
}


pub const VIGNETTECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignetteComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("VignetteComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DofComponentData {
    pub enable: bool,
    pub physical_camera_tweak_enable: bool,
    pub pbc_background_blur_add: f32,
    pub pbc_foreground_blur_add: f32,
    pub pbc_focus_range_add: f32,
    pub dof_source: super::render_base::DofSource,
    pub debug_draw_focus_plane: bool,
    pub focus_dof_max_blur: f32,
    pub blur_factor: f32,
    pub blur_add: f32,
    pub focus_distance: f32,
    pub radial_blur_enable_common_dof: bool,
    pub radial_blur_amount_common_dof: f32,
    pub radial_blur_start_radius_common_dof: f32,
    pub radial_blur_transition_width_common_dof: f32,
    pub radial_blur_tilt_common_dof: f32,
    pub radial_blur_horizontal_scale_common_dof: f32,
    pub radial_blur_aspect_ratio_blend: f32,
    pub radial_blur_position_common_dof: super::core::Vec2,
    pub simple_dof_blur_filter: super::render_base::BlurFilter,
    pub simple_dof_standard_deviation: f32,
    pub sprite_dof_bokeh_texture: super::render_base::TextureBaseAsset,
    pub focus_dof_near_start: f32,
    pub focus_dof_near_end: f32,
    pub focus_dof_far_start: f32,
    pub focus_dof_far_end: f32,
    pub pbr_focus_length_dof: f32,
    pub pbr_film_width_dof: f32,
    pub pbr_f_stop_dof: f32,
    pub optical_vignetting_enable: bool,
    pub optical_vignetting_amount: f32,
    pub optical_vignetting_aspect_ratio: f32,
    pub optical_vignetting_anamorphic_squeeze: f32,
    pub optical_vignetting_size_compensation: f32,
    pub optical_vignetting_operation: super::render_base::VignettingOperation,
    pub r_g_b_bokeh_texture_enable: bool,
    pub bokeh_chromatic_aberration_enable: bool,
    pub bokeh_chromatic_aberration_scale: f32,
    pub bokeh_chromatic_aberration_radius: f32,
    pub bokeh_chromatic_aberration_width: f32,
    pub bokeh_chromatic_aberration_radius_threshold: f32,
    pub bokeh_chromatic_aberration_radius_threshold_width: f32,
    pub bokeh_chromatic_aberration_energy_threshold: f32,
    pub bokeh_chromatic_aberration_fg_color: super::core::Vec3,
    pub bokeh_chromatic_aberration_bg_color: super::core::Vec3,
    pub ironsights_dof_active: bool,
    pub ironsights_dof_extra_blur: bool,
    pub hip_to_ironsights_fade: f32,
    pub ironsights_dof_start_fade: f32,
    pub ironsights_focal_distance: f32,
    pub ironsights_dof_circle_blur: bool,
    pub ironsights_dof_circle_distance: f32,
    pub ironsights_dof_circle_fade_distance: f32,
    pub masked_blur_enabled: bool,
    pub masked_blur_amount: f32,
    pub masked_blur_texture: super::render_base::TextureBaseAsset,
    pub circular_dof_anti_band_artifact: bool,
    pub use_camera_settings: bool,
    pub simple_dof_max_blur: f32,
    pub simple_dof_near_start: f32,
    pub simple_dof_near_end: f32,
    pub simple_dof_far_start: f32,
    pub simple_dof_far_end: f32,
    pub sprite_dof_near_start: f32,
    pub sprite_dof_near_end: f32,
    pub sprite_dof_far_start: f32,
    pub sprite_dof_far_end: f32,
    pub sprite_dof_max_blur: f32,
    pub anisotropy: f32,
    pub full_screen_blur_add_common_dof: f32,
    pub field_flag_override0: u32,
    pub field_flag_override1: u32,
    pub field_flag_override2: u8,
}

pub const DOFCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, enable),
            },
            FieldInfoData {
                name: "PhysicalCameraTweakEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, physical_camera_tweak_enable),
            },
            FieldInfoData {
                name: "PbcBackgroundBlurAdd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, pbc_background_blur_add),
            },
            FieldInfoData {
                name: "PbcForegroundBlurAdd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, pbc_foreground_blur_add),
            },
            FieldInfoData {
                name: "PbcFocusRangeAdd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, pbc_focus_range_add),
            },
            FieldInfoData {
                name: "DofSource",
                flags: MemberInfoFlags::new(0),
                field_type: DOFSOURCE_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, dof_source),
            },
            FieldInfoData {
                name: "DebugDrawFocusPlane",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, debug_draw_focus_plane),
            },
            FieldInfoData {
                name: "FocusDofMaxBlur",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, focus_dof_max_blur),
            },
            FieldInfoData {
                name: "BlurFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, blur_factor),
            },
            FieldInfoData {
                name: "BlurAdd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, blur_add),
            },
            FieldInfoData {
                name: "FocusDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, focus_distance),
            },
            FieldInfoData {
                name: "RadialBlurEnableCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, radial_blur_enable_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurAmountCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, radial_blur_amount_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurStartRadiusCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, radial_blur_start_radius_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurTransitionWidthCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, radial_blur_transition_width_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurTiltCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, radial_blur_tilt_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurHorizontalScaleCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, radial_blur_horizontal_scale_common_dof),
            },
            FieldInfoData {
                name: "RadialBlurAspectRatioBlend",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, radial_blur_aspect_ratio_blend),
            },
            FieldInfoData {
                name: "RadialBlurPositionCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, radial_blur_position_common_dof),
            },
            FieldInfoData {
                name: "SimpleDofBlurFilter",
                flags: MemberInfoFlags::new(0),
                field_type: BLURFILTER_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, simple_dof_blur_filter),
            },
            FieldInfoData {
                name: "SimpleDofStandardDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, simple_dof_standard_deviation),
            },
            FieldInfoData {
                name: "SpriteDofBokehTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, sprite_dof_bokeh_texture),
            },
            FieldInfoData {
                name: "FocusDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, focus_dof_near_start),
            },
            FieldInfoData {
                name: "FocusDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, focus_dof_near_end),
            },
            FieldInfoData {
                name: "FocusDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, focus_dof_far_start),
            },
            FieldInfoData {
                name: "FocusDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, focus_dof_far_end),
            },
            FieldInfoData {
                name: "PbrFocusLengthDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, pbr_focus_length_dof),
            },
            FieldInfoData {
                name: "PbrFilmWidthDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, pbr_film_width_dof),
            },
            FieldInfoData {
                name: "PbrFStopDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, pbr_f_stop_dof),
            },
            FieldInfoData {
                name: "OpticalVignettingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, optical_vignetting_enable),
            },
            FieldInfoData {
                name: "OpticalVignettingAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, optical_vignetting_amount),
            },
            FieldInfoData {
                name: "OpticalVignettingAspectRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, optical_vignetting_aspect_ratio),
            },
            FieldInfoData {
                name: "OpticalVignettingAnamorphicSqueeze",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, optical_vignetting_anamorphic_squeeze),
            },
            FieldInfoData {
                name: "OpticalVignettingSizeCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, optical_vignetting_size_compensation),
            },
            FieldInfoData {
                name: "OpticalVignettingOperation",
                flags: MemberInfoFlags::new(0),
                field_type: VIGNETTINGOPERATION_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, optical_vignetting_operation),
            },
            FieldInfoData {
                name: "RGBBokehTextureEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, r_g_b_bokeh_texture_enable),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, bokeh_chromatic_aberration_enable),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, bokeh_chromatic_aberration_scale),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, bokeh_chromatic_aberration_radius),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, bokeh_chromatic_aberration_width),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationRadiusThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, bokeh_chromatic_aberration_radius_threshold),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationRadiusThresholdWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, bokeh_chromatic_aberration_radius_threshold_width),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationEnergyThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, bokeh_chromatic_aberration_energy_threshold),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationFgColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, bokeh_chromatic_aberration_fg_color),
            },
            FieldInfoData {
                name: "BokehChromaticAberrationBgColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, bokeh_chromatic_aberration_bg_color),
            },
            FieldInfoData {
                name: "IronsightsDofActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, ironsights_dof_active),
            },
            FieldInfoData {
                name: "IronsightsDofExtraBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, ironsights_dof_extra_blur),
            },
            FieldInfoData {
                name: "HipToIronsightsFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, hip_to_ironsights_fade),
            },
            FieldInfoData {
                name: "IronsightsDofStartFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, ironsights_dof_start_fade),
            },
            FieldInfoData {
                name: "IronsightsFocalDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, ironsights_focal_distance),
            },
            FieldInfoData {
                name: "IronsightsDofCircleBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, ironsights_dof_circle_blur),
            },
            FieldInfoData {
                name: "IronsightsDofCircleDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, ironsights_dof_circle_distance),
            },
            FieldInfoData {
                name: "IronsightsDofCircleFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, ironsights_dof_circle_fade_distance),
            },
            FieldInfoData {
                name: "MaskedBlurEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, masked_blur_enabled),
            },
            FieldInfoData {
                name: "MaskedBlurAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, masked_blur_amount),
            },
            FieldInfoData {
                name: "MaskedBlurTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, masked_blur_texture),
            },
            FieldInfoData {
                name: "CircularDofAntiBandArtifact",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, circular_dof_anti_band_artifact),
            },
            FieldInfoData {
                name: "UseCameraSettings",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, use_camera_settings),
            },
            FieldInfoData {
                name: "SimpleDofMaxBlur",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, simple_dof_max_blur),
            },
            FieldInfoData {
                name: "SimpleDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, simple_dof_near_start),
            },
            FieldInfoData {
                name: "SimpleDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, simple_dof_near_end),
            },
            FieldInfoData {
                name: "SimpleDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, simple_dof_far_start),
            },
            FieldInfoData {
                name: "SimpleDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, simple_dof_far_end),
            },
            FieldInfoData {
                name: "SpriteDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, sprite_dof_near_start),
            },
            FieldInfoData {
                name: "SpriteDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, sprite_dof_near_end),
            },
            FieldInfoData {
                name: "SpriteDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, sprite_dof_far_start),
            },
            FieldInfoData {
                name: "SpriteDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, sprite_dof_far_end),
            },
            FieldInfoData {
                name: "SpriteDofMaxBlur",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, sprite_dof_max_blur),
            },
            FieldInfoData {
                name: "Anisotropy",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, anisotropy),
            },
            FieldInfoData {
                name: "FullScreenBlurAddCommonDof",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, full_screen_blur_add_common_dof),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagOverride2",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DofComponentData, field_flag_override2),
            },
        ],
    }),
    array_type: Some(DOFCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DofComponentData {
    fn type_info() -> &'static TypeInfo {
        DOFCOMPONENTDATA_TYPE_INFO
    }
}


pub const DOFCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DofComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ColorCorrectionComponentData {
    pub enable: bool,
    pub brightness: super::core::Vec3,
    pub contrast: super::core::Vec3,
    pub saturation: super::core::Vec3,
    pub hue: f32,
    pub color_grading_enable: bool,
    pub color_grading_texture: super::render_base::TextureBaseAsset,
    pub color_grading_max_hdr_value: f32,
    pub hdr_color_grading_lut: super::render_base::TextureBaseAsset,
    pub field_flag_override0: u16,
}

pub const COLORCORRECTIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorCorrectionComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentData, enable),
            },
            FieldInfoData {
                name: "Brightness",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentData, brightness),
            },
            FieldInfoData {
                name: "Contrast",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentData, contrast),
            },
            FieldInfoData {
                name: "Saturation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentData, saturation),
            },
            FieldInfoData {
                name: "Hue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentData, hue),
            },
            FieldInfoData {
                name: "ColorGradingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentData, color_grading_enable),
            },
            FieldInfoData {
                name: "ColorGradingTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentData, color_grading_texture),
            },
            FieldInfoData {
                name: "ColorGradingMaxHdrValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentData, color_grading_max_hdr_value),
            },
            FieldInfoData {
                name: "HdrColorGradingLut",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentData, hdr_color_grading_lut),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ColorCorrectionComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(COLORCORRECTIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ColorCorrectionComponentData {
    fn type_info() -> &'static TypeInfo {
        COLORCORRECTIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const COLORCORRECTIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorCorrectionComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ColorCorrectionComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TonemapComponentData {
    pub e_v: f32,
    pub exposure_compensation: f32,
    pub auto_exposure_darkest_exclude: f32,
    pub auto_exposure_brightest_exclude: f32,
    pub dark_adaptation_time: f32,
    pub light_adaptation_time: f32,
    pub automatic_exposure: bool,
    pub auto_exposure_method: super::render_base::AutoExposureMethod,
    pub auto_exposure_higher_threshold: f32,
    pub auto_exposure_lower_threshold: f32,
    pub clamp_e_v: bool,
    pub min_e_v: f32,
    pub max_e_v: f32,
    pub spot_meter_scale: f32,
    pub spot_meter_offset_x: f32,
    pub spot_meter_offset_y: f32,
    pub tonemap_method: super::render_base::TonemapMethod,
    pub bloom_direction: super::render_base::BloomDirection,
    pub directional_bloom_clamp: f32,
    pub bloom_scale: super::core::Vec3,
    pub bloom_soft_clip: f32,
    pub bloom_method: super::render_base::BloomMethod,
    pub gaussian_sharpness: f32,
    pub gaussian1_color: super::core::Vec3,
    pub gaussian1_weight: f32,
    pub gaussian2_color: super::core::Vec3,
    pub gaussian2_weight: f32,
    pub gaussian3_color: super::core::Vec3,
    pub gaussian3_weight: f32,
    pub gaussian4_color: super::core::Vec3,
    pub gaussian4_weight: f32,
    pub gaussian5_color: super::core::Vec3,
    pub gaussian5_weight: f32,
    pub f_f_t_threshold: f32,
    pub f_f_t_cutoff: f32,
    pub f_f_t_kernel_scale: f32,
    pub f_f_t_kernel_rotation: f32,
    pub f_f_t_spike_scale_limit_enable: bool,
    pub f_f_t_spike_scale_limit: f32,
    pub f_f_t_kernel_texture: super::render_base::TextureBaseAsset,
    pub chromostereopsis_enable: bool,
    pub chromostereopsis_scale: f32,
    pub chromostereopsis_offset: f32,
    pub lens_dirt_texture: super::render_base::TextureBaseAsset,
    pub lens_dirt_bias: super::core::Vec3,
    pub lens_dirt_factor: super::core::Vec3,
    pub lens_dirt_exponent: super::core::Vec3,
    pub field_flag_override0: u32,
    pub field_flag_override1: u16,
}

pub const TONEMAPCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, e_v),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, exposure_compensation),
            },
            FieldInfoData {
                name: "AutoExposureDarkestExclude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, auto_exposure_darkest_exclude),
            },
            FieldInfoData {
                name: "AutoExposureBrightestExclude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, auto_exposure_brightest_exclude),
            },
            FieldInfoData {
                name: "DarkAdaptationTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, dark_adaptation_time),
            },
            FieldInfoData {
                name: "LightAdaptationTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, light_adaptation_time),
            },
            FieldInfoData {
                name: "AutomaticExposure",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, automatic_exposure),
            },
            FieldInfoData {
                name: "AutoExposureMethod",
                flags: MemberInfoFlags::new(0),
                field_type: AUTOEXPOSUREMETHOD_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, auto_exposure_method),
            },
            FieldInfoData {
                name: "AutoExposureHigherThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, auto_exposure_higher_threshold),
            },
            FieldInfoData {
                name: "AutoExposureLowerThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, auto_exposure_lower_threshold),
            },
            FieldInfoData {
                name: "ClampEV",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, clamp_e_v),
            },
            FieldInfoData {
                name: "MinEV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, min_e_v),
            },
            FieldInfoData {
                name: "MaxEV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, max_e_v),
            },
            FieldInfoData {
                name: "SpotMeterScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, spot_meter_scale),
            },
            FieldInfoData {
                name: "SpotMeterOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, spot_meter_offset_x),
            },
            FieldInfoData {
                name: "SpotMeterOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, spot_meter_offset_y),
            },
            FieldInfoData {
                name: "TonemapMethod",
                flags: MemberInfoFlags::new(0),
                field_type: TONEMAPMETHOD_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, tonemap_method),
            },
            FieldInfoData {
                name: "BloomDirection",
                flags: MemberInfoFlags::new(0),
                field_type: BLOOMDIRECTION_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, bloom_direction),
            },
            FieldInfoData {
                name: "DirectionalBloomClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, directional_bloom_clamp),
            },
            FieldInfoData {
                name: "BloomScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, bloom_scale),
            },
            FieldInfoData {
                name: "BloomSoftClip",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, bloom_soft_clip),
            },
            FieldInfoData {
                name: "BloomMethod",
                flags: MemberInfoFlags::new(0),
                field_type: BLOOMMETHOD_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, bloom_method),
            },
            FieldInfoData {
                name: "GaussianSharpness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian_sharpness),
            },
            FieldInfoData {
                name: "Gaussian1Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian1_color),
            },
            FieldInfoData {
                name: "Gaussian1Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian1_weight),
            },
            FieldInfoData {
                name: "Gaussian2Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian2_color),
            },
            FieldInfoData {
                name: "Gaussian2Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian2_weight),
            },
            FieldInfoData {
                name: "Gaussian3Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian3_color),
            },
            FieldInfoData {
                name: "Gaussian3Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian3_weight),
            },
            FieldInfoData {
                name: "Gaussian4Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian4_color),
            },
            FieldInfoData {
                name: "Gaussian4Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian4_weight),
            },
            FieldInfoData {
                name: "Gaussian5Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian5_color),
            },
            FieldInfoData {
                name: "Gaussian5Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, gaussian5_weight),
            },
            FieldInfoData {
                name: "FFTThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, f_f_t_threshold),
            },
            FieldInfoData {
                name: "FFTCutoff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, f_f_t_cutoff),
            },
            FieldInfoData {
                name: "FFTKernelScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, f_f_t_kernel_scale),
            },
            FieldInfoData {
                name: "FFTKernelRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, f_f_t_kernel_rotation),
            },
            FieldInfoData {
                name: "FFTSpikeScaleLimitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, f_f_t_spike_scale_limit_enable),
            },
            FieldInfoData {
                name: "FFTSpikeScaleLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, f_f_t_spike_scale_limit),
            },
            FieldInfoData {
                name: "FFTKernelTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, f_f_t_kernel_texture),
            },
            FieldInfoData {
                name: "ChromostereopsisEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, chromostereopsis_enable),
            },
            FieldInfoData {
                name: "ChromostereopsisScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, chromostereopsis_scale),
            },
            FieldInfoData {
                name: "ChromostereopsisOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, chromostereopsis_offset),
            },
            FieldInfoData {
                name: "LensDirtTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, lens_dirt_texture),
            },
            FieldInfoData {
                name: "LensDirtBias",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, lens_dirt_bias),
            },
            FieldInfoData {
                name: "LensDirtFactor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, lens_dirt_factor),
            },
            FieldInfoData {
                name: "LensDirtExponent",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, lens_dirt_exponent),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TonemapComponentData, field_flag_override1),
            },
        ],
    }),
    array_type: Some(TONEMAPCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TonemapComponentData {
    fn type_info() -> &'static TypeInfo {
        TONEMAPCOMPONENTDATA_TYPE_INFO
    }
}


pub const TONEMAPCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("TonemapComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkyCelestialMultiComponentData {
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
}

pub const SKYCELESTIALMULTICOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialMultiComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, enable),
            },
            FieldInfoData {
                name: "EnabledOnQualityLevels",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, enabled_on_quality_levels),
            },
            FieldInfoData {
                name: "DrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, draw_order),
            },
            FieldInfoData {
                name: "PlanarReflection",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, planar_reflection),
            },
            FieldInfoData {
                name: "RenderInSkyEnvMap",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, render_in_sky_env_map),
            },
            FieldInfoData {
                name: "WriteAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, write_alpha),
            },
            FieldInfoData {
                name: "QuadCount",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, quad_count),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, texture),
            },
            FieldInfoData {
                name: "UVStart",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, u_v_start),
            },
            FieldInfoData {
                name: "UVEnd",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, u_v_end),
            },
            FieldInfoData {
                name: "UVGrid",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, u_v_grid),
            },
            FieldInfoData {
                name: "Tint",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, tint),
            },
            FieldInfoData {
                name: "SkyEnvmapTintScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, sky_envmap_tint_scale),
            },
            FieldInfoData {
                name: "RandomSeed",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, random_seed),
            },
            FieldInfoData {
                name: "MinScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, min_scale),
            },
            FieldInfoData {
                name: "MaxScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, max_scale),
            },
            FieldInfoData {
                name: "ScaleMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, scale_multiplier),
            },
            FieldInfoData {
                name: "ZenithStop",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, zenith_stop),
            },
            FieldInfoData {
                name: "NadirStop",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, nadir_stop),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialMultiComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(SKYCELESTIALMULTICOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCelestialMultiComponentData {
    fn type_info() -> &'static TypeInfo {
        SKYCELESTIALMULTICOMPONENTDATA_TYPE_INFO
    }
}


pub const SKYCELESTIALMULTICOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialMultiComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SkyCelestialMultiComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkyCelestialSingleComponentData {
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
}

pub const SKYCELESTIALSINGLECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialSingleComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, enable),
            },
            FieldInfoData {
                name: "EnabledOnQualityLevels",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, enabled_on_quality_levels),
            },
            FieldInfoData {
                name: "DrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, draw_order),
            },
            FieldInfoData {
                name: "PlanarReflection",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, planar_reflection),
            },
            FieldInfoData {
                name: "RenderInSkyEnvMap",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, render_in_sky_env_map),
            },
            FieldInfoData {
                name: "WriteAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, write_alpha),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, texture),
            },
            FieldInfoData {
                name: "UVStart",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, u_v_start),
            },
            FieldInfoData {
                name: "UVEnd",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, u_v_end),
            },
            FieldInfoData {
                name: "Tint",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, tint),
            },
            FieldInfoData {
                name: "SkyEnvmapTintScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, sky_envmap_tint_scale),
            },
            FieldInfoData {
                name: "Longitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, longitude),
            },
            FieldInfoData {
                name: "Latitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, latitude),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, rotation),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialSingleComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(SKYCELESTIALSINGLECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCelestialSingleComponentData {
    fn type_info() -> &'static TypeInfo {
        SKYCELESTIALSINGLECOMPONENTDATA_TYPE_INFO
    }
}


pub const SKYCELESTIALSINGLECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialSingleComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SkyCelestialSingleComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkyCelestialRotationComponentData {
    pub enable: bool,
    pub rotation: f32,
    pub rotation_axis: super::core::Vec3,
    pub field_flag_override0: u8,
}

pub const SKYCELESTIALROTATIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialRotationComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialRotationComponentData, enable),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialRotationComponentData, rotation),
            },
            FieldInfoData {
                name: "RotationAxis",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialRotationComponentData, rotation_axis),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SkyCelestialRotationComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(SKYCELESTIALROTATIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyCelestialRotationComponentData {
    fn type_info() -> &'static TypeInfo {
        SKYCELESTIALROTATIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const SKYCELESTIALROTATIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialRotationComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SkyCelestialRotationComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterParamGlobalComponentData {
    pub parameter: super::effect_base::EffectParameter,
    pub value: super::core::Vec4,
    pub field_flag_override0: u8,
}

pub const EMITTERPARAMGLOBALCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Parameter",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamGlobalComponentData, parameter),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamGlobalComponentData, value),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamGlobalComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(EMITTERPARAMGLOBALCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterParamGlobalComponentData {
    fn type_info() -> &'static TypeInfo {
        EMITTERPARAMGLOBALCOMPONENTDATA_TYPE_INFO
    }
}


pub const EMITTERPARAMGLOBALCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("EmitterParamGlobalComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterParamComponentData {
    pub parameter: super::world_base::EmitterParamOverride,
    pub value: f32,
    pub field_flag_override0: u8,
}

pub const EMITTERPARAMCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Parameter",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERPARAMOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamComponentData, parameter),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamComponentData, value),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EmitterParamComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(EMITTERPARAMCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterParamComponentData {
    fn type_info() -> &'static TypeInfo {
        EMITTERPARAMCOMPONENTDATA_TYPE_INFO
    }
}


pub const EMITTERPARAMCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("EmitterParamComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RaytraceReflectionComponentData {
    pub enable: bool,
    pub min_smoothness: f32,
    pub field_flag_override0: u8,
}

pub const RAYTRACEREFLECTIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaytraceReflectionComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaytraceReflectionComponentData, enable),
            },
            FieldInfoData {
                name: "MinSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RaytraceReflectionComponentData, min_smoothness),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RaytraceReflectionComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(RAYTRACEREFLECTIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RaytraceReflectionComponentData {
    fn type_info() -> &'static TypeInfo {
        RAYTRACEREFLECTIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const RAYTRACEREFLECTIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaytraceReflectionComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RaytraceReflectionComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ScreenSpaceRaytraceComponentData {
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
}

pub const SCREENSPACERAYTRACECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenSpaceRaytraceComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RaytraceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, raytrace_enable),
            },
            FieldInfoData {
                name: "CameraFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, camera_fade_start),
            },
            FieldInfoData {
                name: "CameraFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, camera_fade_length),
            },
            FieldInfoData {
                name: "RadialFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, radial_fade_start),
            },
            FieldInfoData {
                name: "RadialFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, radial_fade_length),
            },
            FieldInfoData {
                name: "DistanceFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, distance_fade_start),
            },
            FieldInfoData {
                name: "DistanceFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, distance_fade_length),
            },
            FieldInfoData {
                name: "ScreenFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, screen_fade_start),
            },
            FieldInfoData {
                name: "ScreenFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, screen_fade_length),
            },
            FieldInfoData {
                name: "BorderFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, border_fade_start),
            },
            FieldInfoData {
                name: "BorderFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, border_fade_length),
            },
            FieldInfoData {
                name: "MirrorFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, mirror_fade_start),
            },
            FieldInfoData {
                name: "MirrorFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, mirror_fade_length),
            },
            FieldInfoData {
                name: "ThicknessFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, thickness_fade_start),
            },
            FieldInfoData {
                name: "ThicknessFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, thickness_fade_length),
            },
            FieldInfoData {
                name: "RoughnessFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, roughness_fade_start),
            },
            FieldInfoData {
                name: "RoughnessFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, roughness_fade_length),
            },
            FieldInfoData {
                name: "NormalFadeTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, normal_fade_texture),
            },
            FieldInfoData {
                name: "MinSamples",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, min_samples),
            },
            FieldInfoData {
                name: "MaxSamples",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, max_samples),
            },
            FieldInfoData {
                name: "TemporalSamples",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, temporal_samples),
            },
            FieldInfoData {
                name: "TemporalPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, temporal_period),
            },
            FieldInfoData {
                name: "MinRoughness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, min_roughness),
            },
            FieldInfoData {
                name: "MaxRoughness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, max_roughness),
            },
            FieldInfoData {
                name: "ResolveSamples",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, resolve_samples),
            },
            FieldInfoData {
                name: "NoiseThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, noise_threshold),
            },
            FieldInfoData {
                name: "ClampThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, clamp_threshold),
            },
            FieldInfoData {
                name: "ImportanceSamplingBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, importance_sampling_bias),
            },
            FieldInfoData {
                name: "FilterBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, filter_bias),
            },
            FieldInfoData {
                name: "FilterAngularBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, filter_angular_bias),
            },
            FieldInfoData {
                name: "TemporalFilterResponsiveness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, temporal_filter_responsiveness),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenSpaceRaytraceComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(SCREENSPACERAYTRACECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ScreenSpaceRaytraceComponentData {
    fn type_info() -> &'static TypeInfo {
        SCREENSPACERAYTRACECOMPONENTDATA_TYPE_INFO
    }
}


pub const SCREENSPACERAYTRACECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenSpaceRaytraceComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ScreenSpaceRaytraceComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MotionBlurComponentData {
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
}

pub const MOTIONBLURCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionBlurComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MotionBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentData, motion_blur_enable),
            },
            FieldInfoData {
                name: "MotionBlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentData, motion_blur_scale),
            },
            FieldInfoData {
                name: "MotionBlurCentered",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentData, motion_blur_centered),
            },
            FieldInfoData {
                name: "DepthCheckMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentData, depth_check_max_distance),
            },
            FieldInfoData {
                name: "RadialBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentData, radial_blur_enable),
            },
            FieldInfoData {
                name: "RadialBlurCenter",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentData, radial_blur_center),
            },
            FieldInfoData {
                name: "RadialBlurOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentData, radial_blur_offset),
            },
            FieldInfoData {
                name: "CircularOffsetFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentData, circular_offset_factor),
            },
            FieldInfoData {
                name: "RadialBlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentData, radial_blur_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(MotionBlurComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(MOTIONBLURCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MotionBlurComponentData {
    fn type_info() -> &'static TypeInfo {
        MOTIONBLURCOMPONENTDATA_TYPE_INFO
    }
}


pub const MOTIONBLURCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionBlurComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("MotionBlurComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterLightingComponentData {
    pub character_light_enable: bool,
    pub first_person_enable: bool,
    pub lock_to_camera_direction: bool,
    pub camera_up_rotation: f32,
    pub character_lighting_mode: super::world_base::CharacterLightingMode,
    pub blend_factor: f32,
    pub top_light: super::core::Vec3,
    pub bottom_light: super::core::Vec3,
    pub top_light_dir_x: f32,
    pub top_light_dir_y: f32,
    pub start_fade_distance: f32,
    pub end_fade_distance: f32,
    pub field_flag_override0: u16,
}

pub const CHARACTERLIGHTINGCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CharacterLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, character_light_enable),
            },
            FieldInfoData {
                name: "FirstPersonEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, first_person_enable),
            },
            FieldInfoData {
                name: "LockToCameraDirection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, lock_to_camera_direction),
            },
            FieldInfoData {
                name: "CameraUpRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, camera_up_rotation),
            },
            FieldInfoData {
                name: "CharacterLightingMode",
                flags: MemberInfoFlags::new(0),
                field_type: CHARACTERLIGHTINGMODE_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, character_lighting_mode),
            },
            FieldInfoData {
                name: "BlendFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, blend_factor),
            },
            FieldInfoData {
                name: "TopLight",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, top_light),
            },
            FieldInfoData {
                name: "BottomLight",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, bottom_light),
            },
            FieldInfoData {
                name: "TopLightDirX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, top_light_dir_x),
            },
            FieldInfoData {
                name: "TopLightDirY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, top_light_dir_y),
            },
            FieldInfoData {
                name: "StartFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, start_fade_distance),
            },
            FieldInfoData {
                name: "EndFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, end_fade_distance),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(CharacterLightingComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(CHARACTERLIGHTINGCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterLightingComponentData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERLIGHTINGCOMPONENTDATA_TYPE_INFO
    }
}


pub const CHARACTERLIGHTINGCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("CharacterLightingComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DamageEffectComponentData {
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
}

pub const DAMAGEEFFECTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageEffectComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, shader),
            },
            FieldInfoData {
                name: "FrameWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, frame_width),
            },
            FieldInfoData {
                name: "OuterFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, outer_frame_opacity),
            },
            FieldInfoData {
                name: "InnerFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, inner_frame_opacity),
            },
            FieldInfoData {
                name: "FallofTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, fallof_time),
            },
            FieldInfoData {
                name: "MinDamagePercentageThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, min_damage_percentage_threshold),
            },
            FieldInfoData {
                name: "MaxOpacityDamagePercentage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, max_opacity_damage_percentage),
            },
            FieldInfoData {
                name: "StartCriticalEffectHealthThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, start_critical_effect_health_threshold),
            },
            FieldInfoData {
                name: "EndCriticalEffectHealthThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, end_critical_effect_health_threshold),
            },
            FieldInfoData {
                name: "DebugDamage",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, debug_damage),
            },
            FieldInfoData {
                name: "TopDamage",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, top_damage),
            },
            FieldInfoData {
                name: "LeftDamage",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, left_damage),
            },
            FieldInfoData {
                name: "BottomDamage",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, bottom_damage),
            },
            FieldInfoData {
                name: "RightDamage",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, right_damage),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(DamageEffectComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(DAMAGEEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DamageEffectComponentData {
    fn type_info() -> &'static TypeInfo {
        DAMAGEEFFECTCOMPONENTDATA_TYPE_INFO
    }
}


pub const DAMAGEEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageEffectComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DamageEffectComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ScreenEffectComponentData {
    pub frame_type: super::world_base::ScreenEffectFrameType,
    pub shader: super::render_base::SurfaceShaderBaseAsset,
    pub frame_width: f32,
    pub outer_frame_opacity: f32,
    pub inner_frame_opacity: f32,
    pub angle: f32,
    pub screen_effect_params: super::core::Vec4,
    pub field_flag_override0: u16,
}

pub const SCREENEFFECTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FrameType",
                flags: MemberInfoFlags::new(0),
                field_type: SCREENEFFECTFRAMETYPE_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentData, frame_type),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentData, shader),
            },
            FieldInfoData {
                name: "FrameWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentData, frame_width),
            },
            FieldInfoData {
                name: "OuterFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentData, outer_frame_opacity),
            },
            FieldInfoData {
                name: "InnerFrameOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentData, inner_frame_opacity),
            },
            FieldInfoData {
                name: "Angle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentData, angle),
            },
            FieldInfoData {
                name: "ScreenEffectParams",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentData, screen_effect_params),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ScreenEffectComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(SCREENEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ScreenEffectComponentData {
    fn type_info() -> &'static TypeInfo {
        SCREENEFFECTCOMPONENTDATA_TYPE_INFO
    }
}


pub const SCREENEFFECTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ScreenEffectComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShadowsComponentData {
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
}

pub const SHADOWSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DefSunShadowmapViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, def_sun_shadowmap_view_distance),
            },
            FieldInfoData {
                name: "SunShadowmapViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, sun_shadowmap_view_distance),
            },
            FieldInfoData {
                name: "SunShadowmapExtrusionLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, sun_shadowmap_extrusion_length),
            },
            FieldInfoData {
                name: "SunShadowmapSliceSchemeWeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, sun_shadowmap_slice_scheme_weight),
            },
            FieldInfoData {
                name: "SunShadowmapFirstSliceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, sun_shadowmap_first_slice_scale),
            },
            FieldInfoData {
                name: "SunShadowmapFirstSliceExtrusionLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, sun_shadowmap_first_slice_extrusion_length),
            },
            FieldInfoData {
                name: "SmoothTransitionToDistantShadows",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, smooth_transition_to_distant_shadows),
            },
            FieldInfoData {
                name: "SunShadowmapSliceCountOffset",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, sun_shadowmap_slice_count_offset),
            },
            FieldInfoData {
                name: "SunShadowmapSliceCountMin",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, sun_shadowmap_slice_count_min),
            },
            FieldInfoData {
                name: "SunShadowmapSliceCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, sun_shadowmap_slice_count_max),
            },
            FieldInfoData {
                name: "SunShadowmapSliceResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, sun_shadowmap_slice_resolution_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ShadowsComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(SHADOWSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShadowsComponentData {
    fn type_info() -> &'static TypeInfo {
        SHADOWSCOMPONENTDATA_TYPE_INFO
    }
}


pub const SHADOWSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ShadowsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshSettingsComponentData {
    pub lod_scale: f32,
    pub force_lod: i32,
    pub cull_screen_area_scale: f32,
    pub shadow_distance_scale: f32,
    pub field_flag_override0: u8,
}

pub const MESHSETTINGSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSettingsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LodScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentData, lod_scale),
            },
            FieldInfoData {
                name: "ForceLod",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentData, force_lod),
            },
            FieldInfoData {
                name: "CullScreenAreaScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentData, cull_screen_area_scale),
            },
            FieldInfoData {
                name: "ShadowDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentData, shadow_distance_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MeshSettingsComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(MESHSETTINGSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MeshSettingsComponentData {
    fn type_info() -> &'static TypeInfo {
        MESHSETTINGSCOMPONENTDATA_TYPE_INFO
    }
}


pub const MESHSETTINGSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSettingsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("MeshSettingsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CameraParamsComponentData {
    pub view_distance: f32,
    pub near_plane: f32,
    pub vegetation_max_wiggle_distance: f32,
    pub field_flag_override0: u8,
}

pub const CAMERAPARAMSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraParamsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraParamsComponentData, view_distance),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraParamsComponentData, near_plane),
            },
            FieldInfoData {
                name: "VegetationMaxWiggleDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CameraParamsComponentData, vegetation_max_wiggle_distance),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(CameraParamsComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(CAMERAPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CameraParamsComponentData {
    fn type_info() -> &'static TypeInfo {
        CAMERAPARAMSCOMPONENTDATA_TYPE_INFO
    }
}


pub const CAMERAPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraParamsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("CameraParamsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShaderColorParamsComponentData {
    pub vec4_value: super::core::Vec4,
    pub parameter_name: String,
    pub field_flag_override0: u8,
}

pub const SHADERCOLORPARAMSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderColorParamsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Vec4Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ShaderColorParamsComponentData, vec4_value),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ShaderColorParamsComponentData, parameter_name),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ShaderColorParamsComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(SHADERCOLORPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderColorParamsComponentData {
    fn type_info() -> &'static TypeInfo {
        SHADERCOLORPARAMSCOMPONENTDATA_TYPE_INFO
    }
}


pub const SHADERCOLORPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderColorParamsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ShaderColorParamsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShaderParamsComponentData {
    pub vec4_value: super::core::Vec4,
    pub bool_value: bool,
    pub texture_value: super::render_base::TextureBaseAsset,
    pub value_type: super::render_base::ExternalValueConstantType,
    pub conditional_value: u32,
    pub conditional_name: String,
    pub parameter_name: String,
    pub field_flag_override0: u8,
}

pub const SHADERPARAMSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParamsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Vec4Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentData, vec4_value),
            },
            FieldInfoData {
                name: "BoolValue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentData, bool_value),
            },
            FieldInfoData {
                name: "TextureValue",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentData, texture_value),
            },
            FieldInfoData {
                name: "ValueType",
                flags: MemberInfoFlags::new(0),
                field_type: EXTERNALVALUECONSTANTTYPE_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentData, value_type),
            },
            FieldInfoData {
                name: "ConditionalValue",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentData, conditional_value),
            },
            FieldInfoData {
                name: "ConditionalName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentData, conditional_name),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentData, parameter_name),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ShaderParamsComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(SHADERPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShaderParamsComponentData {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMSCOMPONENTDATA_TYPE_INFO
    }
}


pub const SHADERPARAMSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParamsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ShaderParamsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EnlightenComponentData {
    pub bounce_scale: f32,
    pub sun_scale: f32,
    pub sun_direct_lightmap_enable: bool,
    pub terrain_color: super::core::Vec3,
    pub cull_distance: f32,
    pub cull_radius: f32,
    pub sky_box_enable: bool,
    pub sky_box_cut_bottom: bool,
    pub sky_box_blend_mode: super::world_base::SkyBoxBlendMode,
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
}

pub const ENLIGHTENCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BounceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, bounce_scale),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sun_scale),
            },
            FieldInfoData {
                name: "SunDirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sun_direct_lightmap_enable),
            },
            FieldInfoData {
                name: "TerrainColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, terrain_color),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, cull_distance),
            },
            FieldInfoData {
                name: "CullRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, cull_radius),
            },
            FieldInfoData {
                name: "SkyBoxEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_enable),
            },
            FieldInfoData {
                name: "SkyBoxCutBottom",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_cut_bottom),
            },
            FieldInfoData {
                name: "SkyBoxBlendMode",
                flags: MemberInfoFlags::new(0),
                field_type: SKYBOXBLENDMODE_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_blend_mode),
            },
            FieldInfoData {
                name: "SkyBoxBlend",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_blend),
            },
            FieldInfoData {
                name: "SkyBoxSkyColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_sky_color),
            },
            FieldInfoData {
                name: "SkyBoxGroundColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_ground_color),
            },
            FieldInfoData {
                name: "SkyBoxSunLightColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_sun_light_color),
            },
            FieldInfoData {
                name: "SkyBoxSunLightColorSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_sun_light_color_size),
            },
            FieldInfoData {
                name: "SkyBoxBackLightColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_back_light_color),
            },
            FieldInfoData {
                name: "SkyBoxBackLightColorSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_back_light_color_size),
            },
            FieldInfoData {
                name: "SkyBoxBackLightRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_back_light_rotation_x),
            },
            FieldInfoData {
                name: "SkyBoxBackLightRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, sky_box_back_light_rotation_y),
            },
            FieldInfoData {
                name: "OpaqueAlphaTestSimpleScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, opaque_alpha_test_simple_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(ENLIGHTENCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnlightenComponentData {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENCOMPONENTDATA_TYPE_INFO
    }
}


pub const ENLIGHTENCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("EnlightenComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SubSurfaceScatteringComponentData {
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
}

pub const SUBSURFACESCATTERINGCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceScatteringComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SimpleSssColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, simple_sss_color),
            },
            FieldInfoData {
                name: "SimpleSssRolloffKeyLight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, simple_sss_rolloff_key_light),
            },
            FieldInfoData {
                name: "SimpleSssRolloffLocalLight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, simple_sss_rolloff_local_light),
            },
            FieldInfoData {
                name: "LocalLightTranslucencyEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, local_light_translucency_enable),
            },
            FieldInfoData {
                name: "Profile0",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, profile0),
            },
            FieldInfoData {
                name: "Profile1",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, profile1),
            },
            FieldInfoData {
                name: "Profile2",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, profile2),
            },
            FieldInfoData {
                name: "Profile3",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, profile3),
            },
            FieldInfoData {
                name: "Profile4",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, profile4),
            },
            FieldInfoData {
                name: "Profile5",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, profile5),
            },
            FieldInfoData {
                name: "Profile6",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, profile6),
            },
            FieldInfoData {
                name: "ProfileOATS",
                flags: MemberInfoFlags::new(0),
                field_type: SUBSURFACEPROFILE_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, profile_o_a_t_s),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(SubSurfaceScatteringComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(SUBSURFACESCATTERINGCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SubSurfaceScatteringComponentData {
    fn type_info() -> &'static TypeInfo {
        SUBSURFACESCATTERINGCOMPONENTDATA_TYPE_INFO
    }
}


pub const SUBSURFACESCATTERINGCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceScatteringComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SubSurfaceScatteringComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DynamicAOComponentData {
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
}

pub const DYNAMICAOCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicAOComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, enable),
            },
            FieldInfoData {
                name: "AffectOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, affect_outdoor_light),
            },
            FieldInfoData {
                name: "AffectLocalLight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, affect_local_light),
            },
            FieldInfoData {
                name: "DynamicAOFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, dynamic_a_o_factor),
            },
            FieldInfoData {
                name: "SsaoFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, ssao_fade),
            },
            FieldInfoData {
                name: "SsaoRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, ssao_radius),
            },
            FieldInfoData {
                name: "SsaoMaxDistanceInner",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, ssao_max_distance_inner),
            },
            FieldInfoData {
                name: "SsaoMaxDistanceOuter",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, ssao_max_distance_outer),
            },
            FieldInfoData {
                name: "HbaoRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, hbao_radius),
            },
            FieldInfoData {
                name: "HbaoAngleBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, hbao_angle_bias),
            },
            FieldInfoData {
                name: "HbaoAttenuation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, hbao_attenuation),
            },
            FieldInfoData {
                name: "HbaoContrast",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, hbao_contrast),
            },
            FieldInfoData {
                name: "HbaoMaxFootprintRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, hbao_max_footprint_radius),
            },
            FieldInfoData {
                name: "HbaoPowerExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, hbao_power_exponent),
            },
            FieldInfoData {
                name: "HbaoBlurRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, hbao_blur_radius),
            },
            FieldInfoData {
                name: "HbaoBlurSharpness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, hbao_blur_sharpness),
            },
            FieldInfoData {
                name: "TemporalFilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, temporal_filter_enable),
            },
            FieldInfoData {
                name: "AaoDynamicWeight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_dynamic_weight),
            },
            FieldInfoData {
                name: "AaoBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_bias),
            },
            FieldInfoData {
                name: "AaoIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_intensity),
            },
            FieldInfoData {
                name: "AaoContrast",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_contrast),
            },
            FieldInfoData {
                name: "AaoRangeReduction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_range_reduction),
            },
            FieldInfoData {
                name: "AaoScreenRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_screen_radius),
            },
            FieldInfoData {
                name: "AaoNearOcclusionMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_near_occlusion_max),
            },
            FieldInfoData {
                name: "AaoNearFalloffThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_near_falloff_threshold),
            },
            FieldInfoData {
                name: "AaoClipDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_clip_distance),
            },
            FieldInfoData {
                name: "AaoClipFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_clip_fade_distance),
            },
            FieldInfoData {
                name: "AaoBlurDepthThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_blur_depth_threshold),
            },
            FieldInfoData {
                name: "AaoBlurConstFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, aao_blur_const_falloff),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicAOComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(DYNAMICAOCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DynamicAOComponentData {
    fn type_info() -> &'static TypeInfo {
        DYNAMICAOCOMPONENTDATA_TYPE_INFO
    }
}


pub const DYNAMICAOCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicAOComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DynamicAOComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AntiAliasComponentData {
    pub enable: bool,
    pub disocclusion_rejection_factor: f32,
    pub field_flag_override0: u8,
}

pub const ANTIALIASCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiAliasComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntiAliasComponentData, enable),
            },
            FieldInfoData {
                name: "DisocclusionRejectionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntiAliasComponentData, disocclusion_rejection_factor),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(AntiAliasComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(ANTIALIASCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AntiAliasComponentData {
    fn type_info() -> &'static TypeInfo {
        ANTIALIASCOMPONENTDATA_TYPE_INFO
    }
}


pub const ANTIALIASCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiAliasComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("AntiAliasComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LensReflectionComponentData {
    pub enable: bool,
    pub inner_color: super::core::Vec3,
    pub outer_color: super::core::Vec3,
    pub mix_start: f32,
    pub mix_stop: f32,
    pub input_exponent: f32,
    pub luminance_threshold: f32,
    pub input_scale: f32,
    pub max_opacity: f32,
    pub transpose_reflection: bool,
    pub scale: f32,
    pub distortion_factor: f32,
    pub vertical_stretch: f32,
    pub field_flag_override0: u16,
}

pub const LENSREFLECTIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, enable),
            },
            FieldInfoData {
                name: "InnerColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, inner_color),
            },
            FieldInfoData {
                name: "OuterColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, outer_color),
            },
            FieldInfoData {
                name: "MixStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, mix_start),
            },
            FieldInfoData {
                name: "MixStop",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, mix_stop),
            },
            FieldInfoData {
                name: "InputExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, input_exponent),
            },
            FieldInfoData {
                name: "LuminanceThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, luminance_threshold),
            },
            FieldInfoData {
                name: "InputScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, input_scale),
            },
            FieldInfoData {
                name: "MaxOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, max_opacity),
            },
            FieldInfoData {
                name: "TransposeReflection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, transpose_reflection),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, scale),
            },
            FieldInfoData {
                name: "DistortionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, distortion_factor),
            },
            FieldInfoData {
                name: "VerticalStretch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, vertical_stretch),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(LensReflectionComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(LENSREFLECTIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensReflectionComponentData {
    fn type_info() -> &'static TypeInfo {
        LENSREFLECTIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const LENSREFLECTIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LensReflectionComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SunFlareComponentData {
    pub enable: bool,
    pub debug_draw_occluder: bool,
    pub occluder_size: f32,
    pub screen_clip: bool,
    pub render_mode: super::world_base::LensFlareRenderMode,
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
}

pub const SUNFLARECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, enable),
            },
            FieldInfoData {
                name: "DebugDrawOccluder",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, debug_draw_occluder),
            },
            FieldInfoData {
                name: "OccluderSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, occluder_size),
            },
            FieldInfoData {
                name: "ScreenClip",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, screen_clip),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: LENSFLARERENDERMODE_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, render_mode),
            },
            FieldInfoData {
                name: "UseSunPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, use_sun_position),
            },
            FieldInfoData {
                name: "RotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, rotation_x),
            },
            FieldInfoData {
                name: "RotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, rotation_y),
            },
            FieldInfoData {
                name: "Element1Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_enable),
            },
            FieldInfoData {
                name: "Element1Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_shader),
            },
            FieldInfoData {
                name: "Element1RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_ray_distance),
            },
            FieldInfoData {
                name: "Element1Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_size),
            },
            FieldInfoData {
                name: "Element1SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element1SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element1AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element1AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element1RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_rotation_local),
            },
            FieldInfoData {
                name: "Element1RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element1RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element1RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element1_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element2Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_enable),
            },
            FieldInfoData {
                name: "Element2Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_shader),
            },
            FieldInfoData {
                name: "Element2RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_ray_distance),
            },
            FieldInfoData {
                name: "Element2Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_size),
            },
            FieldInfoData {
                name: "Element2SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element2SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element2AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element2AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element2RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_rotation_local),
            },
            FieldInfoData {
                name: "Element2RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element2RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element2RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element2_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element3Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_enable),
            },
            FieldInfoData {
                name: "Element3Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_shader),
            },
            FieldInfoData {
                name: "Element3RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_ray_distance),
            },
            FieldInfoData {
                name: "Element3Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_size),
            },
            FieldInfoData {
                name: "Element3SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element3SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element3AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element3AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element3RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_rotation_local),
            },
            FieldInfoData {
                name: "Element3RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element3RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element3RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element3_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element4Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_enable),
            },
            FieldInfoData {
                name: "Element4Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_shader),
            },
            FieldInfoData {
                name: "Element4RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_ray_distance),
            },
            FieldInfoData {
                name: "Element4Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_size),
            },
            FieldInfoData {
                name: "Element4SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element4SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element4AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element4AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element4RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_rotation_local),
            },
            FieldInfoData {
                name: "Element4RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element4RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element4RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element4_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "Element5Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_enable),
            },
            FieldInfoData {
                name: "Element5Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_shader),
            },
            FieldInfoData {
                name: "Element5RayDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_ray_distance),
            },
            FieldInfoData {
                name: "Element5Size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_size),
            },
            FieldInfoData {
                name: "Element5SizeOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_size_occluder_curve),
            },
            FieldInfoData {
                name: "Element5SizeScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_size_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element5AlphaOccluderCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_alpha_occluder_curve),
            },
            FieldInfoData {
                name: "Element5AlphaScreenPosCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_alpha_screen_pos_curve),
            },
            FieldInfoData {
                name: "Element5RotationLocal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_rotation_local),
            },
            FieldInfoData {
                name: "Element5RotationAlignedToRay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_rotation_aligned_to_ray),
            },
            FieldInfoData {
                name: "Element5RotationDistCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_rotation_dist_curve),
            },
            FieldInfoData {
                name: "Element5RotationDistMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, element5_rotation_dist_multiplier),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagOverride2",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SunFlareComponentData, field_flag_override2),
            },
        ],
    }),
    array_type: Some(SUNFLARECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SunFlareComponentData {
    fn type_info() -> &'static TypeInfo {
        SUNFLARECOMPONENTDATA_TYPE_INFO
    }
}


pub const SUNFLARECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SunFlareComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CloudComponentData {
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
}

pub const CLOUDCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, enable),
            },
            FieldInfoData {
                name: "BaseToTopMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, base_to_top_multiplier),
            },
            FieldInfoData {
                name: "EdgeDetailMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, edge_detail_multiplier),
            },
            FieldInfoData {
                name: "CloudDensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, cloud_density_multiplier),
            },
            FieldInfoData {
                name: "Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, absorption),
            },
            FieldInfoData {
                name: "Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, scattering),
            },
            FieldInfoData {
                name: "PhaseG0",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, phase_g0),
            },
            FieldInfoData {
                name: "PhaseG1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, phase_g1),
            },
            FieldInfoData {
                name: "PhaseBlend",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, phase_blend),
            },
            FieldInfoData {
                name: "AmbientMultiplicator",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, ambient_multiplicator),
            },
            FieldInfoData {
                name: "AmbientDesaturate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, ambient_desaturate),
            },
            FieldInfoData {
                name: "AerialPerspectiveScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, aerial_perspective_scale),
            },
            FieldInfoData {
                name: "EnableShadow",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, enable_shadow),
            },
            FieldInfoData {
                name: "ScatteringOrder",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, scattering_order),
            },
            FieldInfoData {
                name: "ScatteringFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, scattering_factor),
            },
            FieldInfoData {
                name: "ExtinctionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, extinction_factor),
            },
            FieldInfoData {
                name: "PhaseFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, phase_factor),
            },
            FieldInfoData {
                name: "ShapeTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, shape_texture),
            },
            FieldInfoData {
                name: "ShapeTextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, shape_texture_scale),
            },
            FieldInfoData {
                name: "ShapeTextureContrast",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, shape_texture_contrast),
            },
            FieldInfoData {
                name: "DetailTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, detail_texture),
            },
            FieldInfoData {
                name: "DetailTextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, detail_texture_scale),
            },
            FieldInfoData {
                name: "WeatherTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, weather_texture),
            },
            FieldInfoData {
                name: "WeatherTextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, weather_texture_scale),
            },
            FieldInfoData {
                name: "CloudTypeDensityGradientTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, cloud_type_density_gradient_texture),
            },
            FieldInfoData {
                name: "PlanetRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, planet_radius),
            },
            FieldInfoData {
                name: "CutOffDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, cut_off_distance),
            },
            FieldInfoData {
                name: "CloudLayerStartHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, cloud_layer_start_height),
            },
            FieldInfoData {
                name: "CloudLayerThickness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, cloud_layer_thickness),
            },
            FieldInfoData {
                name: "WindScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, wind_scale),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, offset),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CloudComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(CLOUDCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CloudComponentData {
    fn type_info() -> &'static TypeInfo {
        CLOUDCOMPONENTDATA_TYPE_INFO
    }
}


pub const CLOUDCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("CloudComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WindComponentData {
    pub wind_direction: f32,
    pub wind_strength: f32,
    pub wind_variation_multiplier: f32,
    pub wind_variation_rate_multiplier: f32,
    pub wind_micro_variation_multiplier: f32,
    pub turbulence_multiplier: f32,
    pub turbulence_scale: f32,
    pub field_flag_override0: u8,
}

pub const WINDCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WindDirection",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentData, wind_direction),
            },
            FieldInfoData {
                name: "WindStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentData, wind_strength),
            },
            FieldInfoData {
                name: "WindVariationMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentData, wind_variation_multiplier),
            },
            FieldInfoData {
                name: "WindVariationRateMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentData, wind_variation_rate_multiplier),
            },
            FieldInfoData {
                name: "WindMicroVariationMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentData, wind_micro_variation_multiplier),
            },
            FieldInfoData {
                name: "TurbulenceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentData, turbulence_multiplier),
            },
            FieldInfoData {
                name: "TurbulenceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WindComponentData, turbulence_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(WindComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(WINDCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WindComponentData {
    fn type_info() -> &'static TypeInfo {
        WINDCOMPONENTDATA_TYPE_INFO
    }
}


pub const WINDCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DynamicEnvmapComponentData {
    pub enable: bool,
    pub terrain_reflections_enable: bool,
    pub key_color_envmap: super::core::Vec3,
    pub sky_color_envmap: super::core::Vec3,
    pub ground_color_envmap: super::core::Vec3,
    pub field_flag_override0: u8,
}

pub const DYNAMICENVMAPCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnvmapComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentData, enable),
            },
            FieldInfoData {
                name: "TerrainReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentData, terrain_reflections_enable),
            },
            FieldInfoData {
                name: "KeyColorEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentData, key_color_envmap),
            },
            FieldInfoData {
                name: "SkyColorEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentData, sky_color_envmap),
            },
            FieldInfoData {
                name: "GroundColorEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentData, ground_color_envmap),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnvmapComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(DYNAMICENVMAPCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DynamicEnvmapComponentData {
    fn type_info() -> &'static TypeInfo {
        DYNAMICENVMAPCOMPONENTDATA_TYPE_INFO
    }
}


pub const DYNAMICENVMAPCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnvmapComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DynamicEnvmapComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PlanarReflectionComponentData {
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
}

pub const PLANARREFLECTIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, enable),
            },
            FieldInfoData {
                name: "TerrainReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, terrain_reflections_enable),
            },
            FieldInfoData {
                name: "SkyRenderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, sky_render_enable),
            },
            FieldInfoData {
                name: "GroundHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, ground_height),
            },
            FieldInfoData {
                name: "ViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, view_distance),
            },
            FieldInfoData {
                name: "VerticalBlurFilter",
                flags: MemberInfoFlags::new(0),
                field_type: BLURFILTER_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, vertical_blur_filter),
            },
            FieldInfoData {
                name: "VerticalDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, vertical_deviation),
            },
            FieldInfoData {
                name: "HorizontalBlurFilter",
                flags: MemberInfoFlags::new(0),
                field_type: BLURFILTER_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, horizontal_blur_filter),
            },
            FieldInfoData {
                name: "HorizontalDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, horizontal_deviation),
            },
            FieldInfoData {
                name: "ClippingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, clipping_offset),
            },
            FieldInfoData {
                name: "OverideOutdoorLightColors",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, overide_outdoor_light_colors),
            },
            FieldInfoData {
                name: "KeyColorReflection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, key_color_reflection),
            },
            FieldInfoData {
                name: "SkyColorReflection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, sky_color_reflection),
            },
            FieldInfoData {
                name: "GroundColorReflection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, ground_color_reflection),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(PLANARREFLECTIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PlanarReflectionComponentData {
    fn type_info() -> &'static TypeInfo {
        PLANARREFLECTIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const PLANARREFLECTIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PlanarReflectionComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkyComponentData {
    pub enable: bool,
    pub draw_sky_geo: bool,
    pub sky_type: super::world_base::SkyType,
    pub luminance_scale: f32,
    pub sky_gradient_texture: super::render_base::TextureBaseAsset,
    pub alpha_output: super::world_base::AlphaOutputMode,
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
    pub field_flag_override0: u32,
    pub field_flag_override1: u32,
    pub field_flag_override2: u32,
    pub field_flag_override3: u32,
}

pub const SKYCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, enable),
            },
            FieldInfoData {
                name: "DrawSkyGeo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, draw_sky_geo),
            },
            FieldInfoData {
                name: "SkyType",
                flags: MemberInfoFlags::new(0),
                field_type: SKYTYPE_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, sky_type),
            },
            FieldInfoData {
                name: "LuminanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, luminance_scale),
            },
            FieldInfoData {
                name: "SkyGradientTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, sky_gradient_texture),
            },
            FieldInfoData {
                name: "AlphaOutput",
                flags: MemberInfoFlags::new(0),
                field_type: ALPHAOUTPUTMODE_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, alpha_output),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "HdriRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, hdri_rotation),
            },
            FieldInfoData {
                name: "HdriTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, hdri_texture),
            },
            FieldInfoData {
                name: "SunSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, sun_size),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, sun_scale),
            },
            FieldInfoData {
                name: "PanoramicUVMinX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, panoramic_u_v_min_x),
            },
            FieldInfoData {
                name: "PanoramicUVMaxX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, panoramic_u_v_max_x),
            },
            FieldInfoData {
                name: "PanoramicUVMinY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, panoramic_u_v_min_y),
            },
            FieldInfoData {
                name: "PanoramicUVMaxY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, panoramic_u_v_max_y),
            },
            FieldInfoData {
                name: "PanoramicTileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, panoramic_tile_factor),
            },
            FieldInfoData {
                name: "PanoramicRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, panoramic_rotation),
            },
            FieldInfoData {
                name: "PanoramicTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, panoramic_texture),
            },
            FieldInfoData {
                name: "PanoramicAlphaTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, panoramic_alpha_texture),
            },
            FieldInfoData {
                name: "SkyGradientFollowsPanoramicUVs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, sky_gradient_follows_panoramic_u_vs),
            },
            FieldInfoData {
                name: "FlowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, flow_enable),
            },
            FieldInfoData {
                name: "FlowPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, flow_period),
            },
            FieldInfoData {
                name: "FlowDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, flow_distance),
            },
            FieldInfoData {
                name: "FlowDirection",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, flow_direction),
            },
            FieldInfoData {
                name: "FlowHeightMaskScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, flow_height_mask_scale),
            },
            FieldInfoData {
                name: "FlowHeightMaskBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, flow_height_mask_bias),
            },
            FieldInfoData {
                name: "FlowMaskTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, flow_mask_texture),
            },
            FieldInfoData {
                name: "CloudLayerSunColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer_sun_color),
            },
            FieldInfoData {
                name: "CloudLayerMaskTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer_mask_texture),
            },
            FieldInfoData {
                name: "CloudLayer1Altitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_altitude),
            },
            FieldInfoData {
                name: "CloudLayer1TileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_tile_factor),
            },
            FieldInfoData {
                name: "CloudLayer1Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_rotation),
            },
            FieldInfoData {
                name: "CloudLayer1Speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_speed),
            },
            FieldInfoData {
                name: "CloudLayer1SunLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_sun_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer1SunLightPower",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_sun_light_power),
            },
            FieldInfoData {
                name: "CloudLayer1AmbientLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_ambient_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer1Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_color),
            },
            FieldInfoData {
                name: "CloudLayer1AlphaMul",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_alpha_mul),
            },
            FieldInfoData {
                name: "CloudLayer1Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_texture),
            },
            FieldInfoData {
                name: "CloudLayer1Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_absorption),
            },
            FieldInfoData {
                name: "CloudLayer1Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_scattering),
            },
            FieldInfoData {
                name: "CloudLayer1Phase",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_phase),
            },
            FieldInfoData {
                name: "CloudLayer1Thickness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer1_thickness),
            },
            FieldInfoData {
                name: "CloudLayer2Altitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_altitude),
            },
            FieldInfoData {
                name: "CloudLayer2TileFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_tile_factor),
            },
            FieldInfoData {
                name: "CloudLayer2Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_rotation),
            },
            FieldInfoData {
                name: "CloudLayer2Speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_speed),
            },
            FieldInfoData {
                name: "CloudLayer2SunLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_sun_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer2SunLightPower",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_sun_light_power),
            },
            FieldInfoData {
                name: "CloudLayer2AmbientLightIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_ambient_light_intensity),
            },
            FieldInfoData {
                name: "CloudLayer2Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_color),
            },
            FieldInfoData {
                name: "CloudLayer2AlphaMul",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_alpha_mul),
            },
            FieldInfoData {
                name: "CloudLayer2Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_texture),
            },
            FieldInfoData {
                name: "CloudLayer2Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_absorption),
            },
            FieldInfoData {
                name: "CloudLayer2Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_scattering),
            },
            FieldInfoData {
                name: "CloudLayer2Phase",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_phase),
            },
            FieldInfoData {
                name: "CloudLayer2Thickness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, cloud_layer2_thickness),
            },
            FieldInfoData {
                name: "StaticEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, static_envmap_texture),
            },
            FieldInfoData {
                name: "StaticEnvmapScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, static_envmap_scale),
            },
            FieldInfoData {
                name: "SkyEnvmap8BitTexScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, sky_envmap8_bit_tex_scale),
            },
            FieldInfoData {
                name: "CustomEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, custom_envmap_texture),
            },
            FieldInfoData {
                name: "CustomEnvmapScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, custom_envmap_scale),
            },
            FieldInfoData {
                name: "CustomEnvmapAmbient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, custom_envmap_ambient),
            },
            FieldInfoData {
                name: "SkyVisibilityExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, sky_visibility_exponent),
            },
            FieldInfoData {
                name: "InteriorEnvmapTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, interior_envmap_texture),
            },
            FieldInfoData {
                name: "InteriorEnvmapExp",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, interior_envmap_exp),
            },
            FieldInfoData {
                name: "InteriorEnvmapScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, interior_envmap_scale),
            },
            FieldInfoData {
                name: "InteriorEnvmapBias",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, interior_envmap_bias),
            },
            FieldInfoData {
                name: "InteriorEnvmapSkyVisibilityFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, interior_envmap_sky_visibility_fade_start),
            },
            FieldInfoData {
                name: "InteriorEnvmapSkyVisibilityFadeLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, interior_envmap_sky_visibility_fade_length),
            },
            FieldInfoData {
                name: "EarthRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, earth_radius),
            },
            FieldInfoData {
                name: "AtmosphereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, atmosphere_radius),
            },
            FieldInfoData {
                name: "MieScatteringCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, mie_scattering_coefficient),
            },
            FieldInfoData {
                name: "MieG",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, mie_g),
            },
            FieldInfoData {
                name: "MieExtinctionCoefficientRelation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, mie_extinction_coefficient_relation),
            },
            FieldInfoData {
                name: "ScaleHeightMie",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, scale_height_mie),
            },
            FieldInfoData {
                name: "RayleighScatteringCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, rayleigh_scattering_coefficient),
            },
            FieldInfoData {
                name: "RayleighScatteringCoefficientScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, rayleigh_scattering_coefficient_scale),
            },
            FieldInfoData {
                name: "RayleighExtinctionCoefficientRelation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, rayleigh_extinction_coefficient_relation),
            },
            FieldInfoData {
                name: "ScaleHeightRayleigh",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, scale_height_rayleigh),
            },
            FieldInfoData {
                name: "UseOzone",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, use_ozone),
            },
            FieldInfoData {
                name: "OzonePercentage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, ozone_percentage),
            },
            FieldInfoData {
                name: "UseAerialPerspective",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, use_aerial_perspective),
            },
            FieldInfoData {
                name: "AerialPerspectiveScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, aerial_perspective_scale),
            },
            FieldInfoData {
                name: "AerialPerspectiveIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, aerial_perspective_intensity),
            },
            FieldInfoData {
                name: "AerialPerspectiveDithering",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, aerial_perspective_dithering),
            },
            FieldInfoData {
                name: "Light1Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, light1_color),
            },
            FieldInfoData {
                name: "Light1Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, light1_intensity),
            },
            FieldInfoData {
                name: "Light1FollowOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, light1_follow_outdoor_light),
            },
            FieldInfoData {
                name: "Light1TakesColorFromOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, light1_takes_color_from_outdoor_light),
            },
            FieldInfoData {
                name: "Light1RotX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, light1_rot_x),
            },
            FieldInfoData {
                name: "Light1RotY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, light1_rot_y),
            },
            FieldInfoData {
                name: "Light2Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, light2_color),
            },
            FieldInfoData {
                name: "Light2Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, light2_intensity),
            },
            FieldInfoData {
                name: "UseLightSource2",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, use_light_source2),
            },
            FieldInfoData {
                name: "Light2RotX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, light2_rot_x),
            },
            FieldInfoData {
                name: "Light2RotY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, light2_rot_y),
            },
            FieldInfoData {
                name: "UseNoise",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, use_noise),
            },
            FieldInfoData {
                name: "FogStartDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, fog_start_distance),
            },
            FieldInfoData {
                name: "RayleighPolarization",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, rayleigh_polarization),
            },
            FieldInfoData {
                name: "MiePolarization",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, mie_polarization),
            },
            FieldInfoData {
                name: "OutdoorLightScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, outdoor_light_scale),
            },
            FieldInfoData {
                name: "DrawSunDisc",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, draw_sun_disc),
            },
            FieldInfoData {
                name: "ForwardScatteringDepthVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, forward_scattering_depth_visibility),
            },
            FieldInfoData {
                name: "ForwardScatteringStartDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, forward_scattering_start_depth),
            },
            FieldInfoData {
                name: "ForwardScatteringEndDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, forward_scattering_end_depth),
            },
            FieldInfoData {
                name: "ForwardScatteringTakesColorFromOutdoorLight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, forward_scattering_takes_color_from_outdoor_light),
            },
            FieldInfoData {
                name: "ForwardScatteringOutdoorLightTint",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, forward_scattering_outdoor_light_tint),
            },
            FieldInfoData {
                name: "HeightFogColorAdd",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, height_fog_color_add),
            },
            FieldInfoData {
                name: "HeightFogColorMult",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, height_fog_color_mult),
            },
            FieldInfoData {
                name: "MinHeightFogTransmittance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, min_height_fog_transmittance),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, field_flag_override1),
            },
            FieldInfoData {
                name: "FieldFlagOverride2",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, field_flag_override2),
            },
            FieldInfoData {
                name: "FieldFlagOverride3",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SkyComponentData, field_flag_override3),
            },
        ],
    }),
    array_type: Some(SKYCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkyComponentData {
    fn type_info() -> &'static TypeInfo {
        SKYCOMPONENTDATA_TYPE_INFO
    }
}


pub const SKYCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SkyComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FogComponentData {
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
    pub forward_light_scattering_attenuation_type: super::world_base::ForwardLightScatteringAttenuation,
    pub height_fog_enable: bool,
    pub height_fog_follow_camera: f32,
    pub height_fog_altitude: f32,
    pub height_fog_depth: f32,
    pub height_fog_visibility_range: f32,
    pub participating_media_enable: bool,
    pub depth_fog_participating_media: super::world_base::ParticipatingMedia,
    pub height_fog_participating_media: super::world_base::ParticipatingMedia,
    pub fog_volume_strength: f32,
    pub field_flag_override0: u32,
    pub field_flag_override1: u8,
}

pub const FOGCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, enable),
            },
            FieldInfoData {
                name: "FogDistanceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fog_distance_multiplier),
            },
            FieldInfoData {
                name: "FogGradientEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fog_gradient_enable),
            },
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, start),
            },
            FieldInfoData {
                name: "End",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, end),
            },
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, curve),
            },
            FieldInfoData {
                name: "FogGradientHeightFadeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fog_gradient_height_fade_enable),
            },
            FieldInfoData {
                name: "FadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fade_start),
            },
            FieldInfoData {
                name: "FadeEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fade_end),
            },
            FieldInfoData {
                name: "FogColorEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fog_color_enable),
            },
            FieldInfoData {
                name: "FogColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fog_color),
            },
            FieldInfoData {
                name: "FogColorStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fog_color_start),
            },
            FieldInfoData {
                name: "FogColorEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fog_color_end),
            },
            FieldInfoData {
                name: "FogColorCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fog_color_curve),
            },
            FieldInfoData {
                name: "TransparencyFadeStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, transparency_fade_start),
            },
            FieldInfoData {
                name: "TransparencyFadeEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, transparency_fade_end),
            },
            FieldInfoData {
                name: "TransparencyFadeClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, transparency_fade_clamp),
            },
            FieldInfoData {
                name: "TransparencyFadeCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, transparency_fade_curve),
            },
            FieldInfoData {
                name: "ForwardLightScatteringEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_enabled),
            },
            FieldInfoData {
                name: "ForwardLightScatteringUseSunPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_use_sun_position),
            },
            FieldInfoData {
                name: "ForwardLightScatteringRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_rotation_x),
            },
            FieldInfoData {
                name: "ForwardLightScatteringRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_rotation_y),
            },
            FieldInfoData {
                name: "ForwardLightScatteringPhaseG",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_phase_g),
            },
            FieldInfoData {
                name: "ForwardLightScatteringStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_strength),
            },
            FieldInfoData {
                name: "ForwardLightScatteringColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_color),
            },
            FieldInfoData {
                name: "ForwardLightScatteringPresence",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_presence),
            },
            FieldInfoData {
                name: "ForwardLightScatteringMaxBlurLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_max_blur_length),
            },
            FieldInfoData {
                name: "ForwardLightScatteringExtinction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_extinction),
            },
            FieldInfoData {
                name: "ForwardLightScatteringSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_smoothness),
            },
            FieldInfoData {
                name: "ForwardLightScatteringAttenuationType",
                flags: MemberInfoFlags::new(0),
                field_type: FORWARDLIGHTSCATTERINGATTENUATION_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, forward_light_scattering_attenuation_type),
            },
            FieldInfoData {
                name: "HeightFogEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, height_fog_enable),
            },
            FieldInfoData {
                name: "HeightFogFollowCamera",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, height_fog_follow_camera),
            },
            FieldInfoData {
                name: "HeightFogAltitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, height_fog_altitude),
            },
            FieldInfoData {
                name: "HeightFogDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, height_fog_depth),
            },
            FieldInfoData {
                name: "HeightFogVisibilityRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, height_fog_visibility_range),
            },
            FieldInfoData {
                name: "ParticipatingMediaEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, participating_media_enable),
            },
            FieldInfoData {
                name: "DepthFogParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICIPATINGMEDIA_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, depth_fog_participating_media),
            },
            FieldInfoData {
                name: "HeightFogParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICIPATINGMEDIA_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, height_fog_participating_media),
            },
            FieldInfoData {
                name: "FogVolumeStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, fog_volume_strength),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(FogComponentData, field_flag_override1),
            },
        ],
    }),
    array_type: Some(FOGCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FogComponentData {
    fn type_info() -> &'static TypeInfo {
        FOGCOMPONENTDATA_TYPE_INFO
    }
}


pub const FOGCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("FogComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OutdoorLightComponentData {
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
    pub sun_shadow_filter_type: super::world_base::ShadowFilteringType,
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
}

pub const OUTDOORLIGHTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, enable),
            },
            FieldInfoData {
                name: "SunRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_rotation_x),
            },
            FieldInfoData {
                name: "SunRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_rotation_y),
            },
            FieldInfoData {
                name: "ShadowSunRotationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, shadow_sun_rotation_enable),
            },
            FieldInfoData {
                name: "ShadowSunRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, shadow_sun_rotation_x),
            },
            FieldInfoData {
                name: "ShadowSunRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, shadow_sun_rotation_y),
            },
            FieldInfoData {
                name: "SunColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_color),
            },
            FieldInfoData {
                name: "SunIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_intensity),
            },
            FieldInfoData {
                name: "FinalSunLuminance",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, final_sun_luminance),
            },
            FieldInfoData {
                name: "FinalSunIlluminance",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, final_sun_illuminance),
            },
            FieldInfoData {
                name: "OuterSpaceSunLuminance",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, outer_space_sun_luminance),
            },
            FieldInfoData {
                name: "OuterSpaceSunIlluminance1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, outer_space_sun_illuminance1),
            },
            FieldInfoData {
                name: "OuterSpaceSunIlluminance2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, outer_space_sun_illuminance2),
            },
            FieldInfoData {
                name: "SunAngularRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_angular_radius),
            },
            FieldInfoData {
                name: "SkyColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sky_color),
            },
            FieldInfoData {
                name: "GroundColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, ground_color),
            },
            FieldInfoData {
                name: "SkyLightAngleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sky_light_angle_factor),
            },
            FieldInfoData {
                name: "SunSpecularScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_specular_scale),
            },
            FieldInfoData {
                name: "SkyEnvmapShadowScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sky_envmap_shadow_scale),
            },
            FieldInfoData {
                name: "CascadeShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cascade_shadow_enable),
            },
            FieldInfoData {
                name: "SunShadowHeightScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_shadow_height_scale),
            },
            FieldInfoData {
                name: "SunShadowFilterType",
                flags: MemberInfoFlags::new(0),
                field_type: SHADOWFILTERINGTYPE_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_shadow_filter_type),
            },
            FieldInfoData {
                name: "SunShadowForwardQuality",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERSHADOWMAPQUALITY_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_shadow_forward_quality),
            },
            FieldInfoData {
                name: "SunPcssFilterAdaptive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_pcss_filter_adaptive),
            },
            FieldInfoData {
                name: "SunPcssInitialSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_pcss_initial_sample_count),
            },
            FieldInfoData {
                name: "SunPcssMaximumSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_pcss_maximum_sample_count),
            },
            FieldInfoData {
                name: "SunPcssFilterErrorThresholdPct",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_pcss_filter_error_threshold_pct),
            },
            FieldInfoData {
                name: "SunPenumbraSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_penumbra_size),
            },
            FieldInfoData {
                name: "SunPcssShadowFilterScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, sun_pcss_shadow_filter_scale),
            },
            FieldInfoData {
                name: "CloudShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_enable),
            },
            FieldInfoData {
                name: "CloudShadowTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_texture),
            },
            FieldInfoData {
                name: "CloudShadowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_speed),
            },
            FieldInfoData {
                name: "CloudShadowSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_size),
            },
            FieldInfoData {
                name: "CloudShadowCoverage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_coverage),
            },
            FieldInfoData {
                name: "CloudShadowExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_exponent),
            },
            FieldInfoData {
                name: "CloudShadowIsTopDown",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_is_top_down),
            },
            FieldInfoData {
                name: "CloudShadowStartFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_start_fade),
            },
            FieldInfoData {
                name: "CloudShadowsFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadows_fade_distance),
            },
            FieldInfoData {
                name: "CloudShadowHeightFadeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_height_fade_enable),
            },
            FieldInfoData {
                name: "CloudShadowStartHeightFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_start_height_fade),
            },
            FieldInfoData {
                name: "CloudShadowsHeightFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadows_height_fade_distance),
            },
            FieldInfoData {
                name: "CloudXZTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_x_z_translation),
            },
            FieldInfoData {
                name: "CloudShadowAddressingMode",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_shadow_addressing_mode),
            },
            FieldInfoData {
                name: "CloudRadiosityEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cloud_radiosity_enable),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, secondary_cloud_shadow_texture),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, secondary_cloud_shadow_speed),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, secondary_cloud_shadow_size),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowCoverage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, secondary_cloud_shadow_coverage),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, secondary_cloud_shadow_exponent),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowIsTopDown",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, secondary_cloud_shadow_is_top_down),
            },
            FieldInfoData {
                name: "SecondaryCloudXZTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, secondary_cloud_x_z_translation),
            },
            FieldInfoData {
                name: "SecondaryCloudShadowAddressingMode",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, secondary_cloud_shadow_addressing_mode),
            },
            FieldInfoData {
                name: "CastTerrainShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, cast_terrain_shadows_enable),
            },
            FieldInfoData {
                name: "TranslucencyAmbient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, translucency_ambient),
            },
            FieldInfoData {
                name: "TranslucencyScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, translucency_scale),
            },
            FieldInfoData {
                name: "TranslucencyPower",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, translucency_power),
            },
            FieldInfoData {
                name: "TranslucencyDistortion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, translucency_distortion),
            },
            FieldInfoData {
                name: "ParticleSunShadowFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, particle_sun_shadow_factor),
            },
            FieldInfoData {
                name: "ParticleSunShadowSmoothing",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, particle_sun_shadow_smoothing),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagOverride1",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OutdoorLightComponentData, field_flag_override1),
            },
        ],
    }),
    array_type: Some(OUTDOORLIGHTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OutdoorLightComponentData {
    fn type_info() -> &'static TypeInfo {
        OUTDOORLIGHTCOMPONENTDATA_TYPE_INFO
    }
}


pub const OUTDOORLIGHTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("OutdoorLightComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IndirectSpecularComponentData {
    pub enabled: bool,
    pub intensity: f32,
    pub reflectance_scale: f32,
    pub probes_intensity: f32,
    pub probes_reflectance_scale: f32,
    pub field_flag_override0: u8,
}

pub const INDIRECTSPECULARCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndirectSpecularComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentData, enabled),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentData, intensity),
            },
            FieldInfoData {
                name: "ReflectanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentData, reflectance_scale),
            },
            FieldInfoData {
                name: "ProbesIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentData, probes_intensity),
            },
            FieldInfoData {
                name: "ProbesReflectanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentData, probes_reflectance_scale),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(IndirectSpecularComponentData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(INDIRECTSPECULARCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for IndirectSpecularComponentData {
    fn type_info() -> &'static TypeInfo {
        INDIRECTSPECULARCOMPONENTDATA_TYPE_INFO
    }
}


pub const INDIRECTSPECULARCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndirectSpecularComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("IndirectSpecularComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VisualEnvironmentComponentData {
    pub property_overrides: Vec<String>,
}

pub const VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PropertyOverrides",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentComponentData, property_overrides),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VisualEnvironmentComponentData {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTCOMPONENTDATA_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("VisualEnvironmentComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomStringParam {
    pub param_name: String,
    pub param_value: String,
}

pub const CUSTOMSTRINGPARAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomStringParam",
    flags: MemberInfoFlags::new(73),
    module: "WorldSim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParamName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CustomStringParam, param_name),
            },
            FieldInfoData {
                name: "ParamValue",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CustomStringParam, param_value),
            },
        ],
    }),
    array_type: Some(CUSTOMSTRINGPARAM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomStringParam {
    fn type_info() -> &'static TypeInfo {
        CUSTOMSTRINGPARAM_TYPE_INFO
    }
}


pub const CUSTOMSTRINGPARAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomStringParam-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("CustomStringParam-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FileFrameNamingEnum {
    #[default]
    Sequential = 0,
    Absolute = 1,
}

pub const FILEFRAMENAMINGENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FileFrameNamingEnum",
    flags: MemberInfoFlags::new(49429),
    module: "WorldSim",
    data: TypeInfoData::Enum,
    array_type: Some(FILEFRAMENAMINGENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FileFrameNamingEnum {
    fn type_info() -> &'static TypeInfo {
        FILEFRAMENAMINGENUM_TYPE_INFO
    }
}


pub const FILEFRAMENAMINGENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FileFrameNamingEnum-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("FileFrameNamingEnum-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LayerModeEnum {
    #[default]
    Single = 0,
    Common = 1,
    All = 2,
}

pub const LAYERMODEENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerModeEnum",
    flags: MemberInfoFlags::new(49429),
    module: "WorldSim",
    data: TypeInfoData::Enum,
    array_type: Some(LAYERMODEENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LayerModeEnum {
    fn type_info() -> &'static TypeInfo {
        LAYERMODEENUM_TYPE_INFO
    }
}


pub const LAYERMODEENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerModeEnum-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LayerModeEnum-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ImageTypeEnum {
    #[default]
    tga = 0,
    png = 1,
    png16 = 2,
    exr = 3,
}

pub const IMAGETYPEENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ImageTypeEnum",
    flags: MemberInfoFlags::new(49429),
    module: "WorldSim",
    data: TypeInfoData::Enum,
    array_type: Some(IMAGETYPEENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ImageTypeEnum {
    fn type_info() -> &'static TypeInfo {
        IMAGETYPEENUM_TYPE_INFO
    }
}


pub const IMAGETYPEENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ImageTypeEnum-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ImageTypeEnum-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RenderFramesTrackData {
    pub keyframes: Vec<RenderFramesKeyframe>,
    pub quick_render_name: String,
}

pub const RENDERFRAMESTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFramesTrackData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: RENDERFRAMESKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RenderFramesTrackData, keyframes),
            },
            FieldInfoData {
                name: "QuickRenderName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RenderFramesTrackData, quick_render_name),
            },
        ],
    }),
    array_type: Some(RENDERFRAMESTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RenderFramesTrackData {
    fn type_info() -> &'static TypeInfo {
        RENDERFRAMESTRACKDATA_TYPE_INFO
    }
}


pub const RENDERFRAMESTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFramesTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RenderFramesTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RenderFramesKeyframe {
    pub time: f32,
    pub length: f32,
    pub render_file_name: String,
    pub render_folder_name: String,
}

pub const RENDERFRAMESKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFramesKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderFramesKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RenderFramesKeyframe, length),
            },
            FieldInfoData {
                name: "RenderFileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RenderFramesKeyframe, render_file_name),
            },
            FieldInfoData {
                name: "RenderFolderName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RenderFramesKeyframe, render_folder_name),
            },
        ],
    }),
    array_type: Some(RENDERFRAMESKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RenderFramesKeyframe {
    fn type_info() -> &'static TypeInfo {
        RENDERFRAMESKEYFRAME_TYPE_INFO
    }
}


pub const RENDERFRAMESKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFramesKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RenderFramesKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SimpleVolumetricsEntityData {
    pub enabled: bool,
    pub fade_out_end_radius: f32,
    pub fade_out_start_radius: f32,
    pub far_fade_start_distance: f32,
    pub far_fade_end_distance: f32,
    pub use_clipping_plane: bool,
    pub clipping_plane_offset: f32,
    pub draw_pass: super::world_base::SimpleVolumetricsDrawPass,
    pub scale_to_exposure: bool,
    pub exponent: f32,
    pub emission: super::core::Vec3,
    pub emission_scale: f32,
}

pub const SIMPLEVOLUMETRICSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, enabled),
            },
            FieldInfoData {
                name: "FadeOutEndRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, fade_out_end_radius),
            },
            FieldInfoData {
                name: "FadeOutStartRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, fade_out_start_radius),
            },
            FieldInfoData {
                name: "FarFadeStartDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, far_fade_start_distance),
            },
            FieldInfoData {
                name: "FarFadeEndDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, far_fade_end_distance),
            },
            FieldInfoData {
                name: "UseClippingPlane",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, use_clipping_plane),
            },
            FieldInfoData {
                name: "ClippingPlaneOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, clipping_plane_offset),
            },
            FieldInfoData {
                name: "DrawPass",
                flags: MemberInfoFlags::new(0),
                field_type: SIMPLEVOLUMETRICSDRAWPASS_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, draw_pass),
            },
            FieldInfoData {
                name: "ScaleToExposure",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, scale_to_exposure),
            },
            FieldInfoData {
                name: "Exponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, exponent),
            },
            FieldInfoData {
                name: "Emission",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, emission),
            },
            FieldInfoData {
                name: "EmissionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleVolumetricsEntityData, emission_scale),
            },
        ],
    }),
    array_type: Some(SIMPLEVOLUMETRICSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SimpleVolumetricsEntityData {
    fn type_info() -> &'static TypeInfo {
        SIMPLEVOLUMETRICSENTITYDATA_TYPE_INFO
    }
}


pub const SIMPLEVOLUMETRICSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SimpleVolumetricsEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScreenshotCaptureEntityData {
    pub file_name: String,
    pub sub_folder: String,
    pub format: super::render_base::ScreenshotFormat,
    pub layer_mode: super::render_base::ScreenshotLayerMode,
    pub resolution_multiplier: i32,
    pub antialias_multiplier: i32,
    pub enable_alpha: bool,
    pub surround_capture: bool,
    pub force_auto_render: bool,
    pub use_native_file_system: bool,
    pub starting_x_pos: i32,
    pub starting_y_pos: i32,
    pub width: i32,
    pub height: i32,
    pub delay_frames: i32,
    pub upload_to_juice: bool,
    pub upload_to_ensemble_whiteboard: bool,
    pub overwrite: bool,
}

pub const SCREENSHOTCAPTUREENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotCaptureEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, file_name),
            },
            FieldInfoData {
                name: "SubFolder",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, sub_folder),
            },
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: SCREENSHOTFORMAT_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, format),
            },
            FieldInfoData {
                name: "LayerMode",
                flags: MemberInfoFlags::new(0),
                field_type: SCREENSHOTLAYERMODE_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, layer_mode),
            },
            FieldInfoData {
                name: "ResolutionMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, resolution_multiplier),
            },
            FieldInfoData {
                name: "AntialiasMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, antialias_multiplier),
            },
            FieldInfoData {
                name: "EnableAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, enable_alpha),
            },
            FieldInfoData {
                name: "SurroundCapture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, surround_capture),
            },
            FieldInfoData {
                name: "ForceAutoRender",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, force_auto_render),
            },
            FieldInfoData {
                name: "UseNativeFileSystem",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, use_native_file_system),
            },
            FieldInfoData {
                name: "StartingXPos",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, starting_x_pos),
            },
            FieldInfoData {
                name: "StartingYPos",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, starting_y_pos),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, height),
            },
            FieldInfoData {
                name: "DelayFrames",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, delay_frames),
            },
            FieldInfoData {
                name: "UploadToJuice",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, upload_to_juice),
            },
            FieldInfoData {
                name: "UploadToEnsembleWhiteboard",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, upload_to_ensemble_whiteboard),
            },
            FieldInfoData {
                name: "Overwrite",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScreenshotCaptureEntityData, overwrite),
            },
        ],
    }),
    array_type: Some(SCREENSHOTCAPTUREENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScreenshotCaptureEntityData {
    fn type_info() -> &'static TypeInfo {
        SCREENSHOTCAPTUREENTITYDATA_TYPE_INFO
    }
}


pub const SCREENSHOTCAPTUREENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotCaptureEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ScreenshotCaptureEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OccluderVolumeEntityData {
    pub occluder_high_priority: bool,
    pub occluder_is_conservative: bool,
    pub coverage_value: f32,
    pub visible: bool,
}

pub const OCCLUDERVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OccluderHighPriority",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeEntityData, occluder_high_priority),
            },
            FieldInfoData {
                name: "OccluderIsConservative",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeEntityData, occluder_is_conservative),
            },
            FieldInfoData {
                name: "CoverageValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeEntityData, coverage_value),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderVolumeEntityData, visible),
            },
        ],
    }),
    array_type: Some(OCCLUDERVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OccluderVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const OCCLUDERVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("OccluderVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OccluderPlaneEntityData {
    pub occluder_high_priority: bool,
    pub occluder_is_conservative: bool,
    pub doubled_sided: bool,
    pub coverage_value: f32,
    pub visible: bool,
}

pub const OCCLUDERPLANEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OccluderHighPriority",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneEntityData, occluder_high_priority),
            },
            FieldInfoData {
                name: "OccluderIsConservative",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneEntityData, occluder_is_conservative),
            },
            FieldInfoData {
                name: "DoubledSided",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneEntityData, doubled_sided),
            },
            FieldInfoData {
                name: "CoverageValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneEntityData, coverage_value),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderPlaneEntityData, visible),
            },
        ],
    }),
    array_type: Some(OCCLUDERPLANEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OccluderPlaneEntityData {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERPLANEENTITYDATA_TYPE_INFO
    }
}


pub const OCCLUDERPLANEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("OccluderPlaneEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OccluderMeshEntityData {
    pub mesh: super::render_base::MeshBaseAsset,
    pub visible: bool,
}

pub const OCCLUDERMESHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(OccluderMeshEntityData, mesh),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OccluderMeshEntityData, visible),
            },
        ],
    }),
    array_type: Some(OCCLUDERMESHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OccluderMeshEntityData {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERMESHENTITYDATA_TYPE_INFO
    }
}


pub const OCCLUDERMESHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("OccluderMeshEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VisibleAreaEntityData {
    pub realm: super::core::Realm,
    pub hide_treshold: u32,
    pub visual_cull_screen_area: f32,
}

pub const VISIBLEAREAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(VisibleAreaEntityData, realm),
            },
            FieldInfoData {
                name: "HideTreshold",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisibleAreaEntityData, hide_treshold),
            },
            FieldInfoData {
                name: "VisualCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisibleAreaEntityData, visual_cull_screen_area),
            },
        ],
    }),
    array_type: Some(VISIBLEAREAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VisibleAreaEntityData {
    fn type_info() -> &'static TypeInfo {
        VISIBLEAREAENTITYDATA_TYPE_INFO
    }
}


pub const VISIBLEAREAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("VisibleAreaEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PlanarReflectionLocatorEntityData {
    pub enable: bool,
}

pub const PLANARREFLECTIONLOCATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlanarReflectionLocatorEntityData, enable),
            },
        ],
    }),
    array_type: Some(PLANARREFLECTIONLOCATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PlanarReflectionLocatorEntityData {
    fn type_info() -> &'static TypeInfo {
        PLANARREFLECTIONLOCATORENTITYDATA_TYPE_INFO
    }
}


pub const PLANARREFLECTIONLOCATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PlanarReflectionLocatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LightProbeVolumeData {
    pub xres: u32,
    pub yres: u32,
    pub zres: u32,
    pub blend_distance: f32,
    pub priority: u32,
    pub enable: bool,
}

pub const LIGHTPROBEVOLUMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeVolumeData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Xres",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeData, xres),
            },
            FieldInfoData {
                name: "Yres",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeData, yres),
            },
            FieldInfoData {
                name: "Zres",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeData, zres),
            },
            FieldInfoData {
                name: "BlendDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeData, blend_distance),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeData, priority),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LightProbeVolumeData, enable),
            },
        ],
    }),
    array_type: Some(LIGHTPROBEVOLUMEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LightProbeVolumeData {
    fn type_info() -> &'static TypeInfo {
        LIGHTPROBEVOLUMEDATA_TYPE_INFO
    }
}


pub const LIGHTPROBEVOLUMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeVolumeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LightProbeVolumeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticEnlightenEntityData {
    pub priority: i32,
    pub enlighten_data: super::render_base::StaticEnlightenBaseAsset,
    pub dynamic_enlighten_data: super::render_base::EnlightenBaseAsset,
    pub object_layers: u16,
    pub flux_auto_bake: bool,
    pub enable: bool,
    pub mixed: bool,
}

pub const STATICENLIGHTENENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENLIGHTENENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenEntityData, priority),
            },
            FieldInfoData {
                name: "EnlightenData",
                flags: MemberInfoFlags::new(0),
                field_type: STATICENLIGHTENBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenEntityData, enlighten_data),
            },
            FieldInfoData {
                name: "DynamicEnlightenData",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenEntityData, dynamic_enlighten_data),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenEntityData, object_layers),
            },
            FieldInfoData {
                name: "FluxAutoBake",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenEntityData, flux_auto_bake),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenEntityData, enable),
            },
            FieldInfoData {
                name: "Mixed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenEntityData, mixed),
            },
        ],
    }),
    array_type: Some(STATICENLIGHTENENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticEnlightenEntityData {
    fn type_info() -> &'static TypeInfo {
        STATICENLIGHTENENTITYDATA_TYPE_INFO
    }
}


pub const STATICENLIGHTENENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("StaticEnlightenEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicEnlightenEntityData {
    pub priority: i32,
    pub enlighten_data: super::render_base::EnlightenBaseAsset,
    pub object_layers: u16,
    pub enable: bool,
    pub database_version: i32,
}

pub const DYNAMICENLIGHTENENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENLIGHTENENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenEntityData, priority),
            },
            FieldInfoData {
                name: "EnlightenData",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenEntityData, enlighten_data),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenEntityData, object_layers),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenEntityData, enable),
            },
            FieldInfoData {
                name: "DatabaseVersion",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicEnlightenEntityData, database_version),
            },
        ],
    }),
    array_type: Some(DYNAMICENLIGHTENENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicEnlightenEntityData {
    fn type_info() -> &'static TypeInfo {
        DYNAMICENLIGHTENENTITYDATA_TYPE_INFO
    }
}


pub const DYNAMICENLIGHTENENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DynamicEnlightenEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnlightenEntityData {
}

pub const ENLIGHTENENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENLIGHTENENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnlightenEntityData {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENENTITYDATA_TYPE_INFO
    }
}


pub const ENLIGHTENENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("EnlightenEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RadiosityModifierEntityData {
    pub bounce_scale: f32,
    pub sun_scale: f32,
}

pub const RADIOSITYMODIFIERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BounceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityModifierEntityData, bounce_scale),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityModifierEntityData, sun_scale),
            },
        ],
    }),
    array_type: Some(RADIOSITYMODIFIERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityModifierEntityData {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMODIFIERENTITYDATA_TYPE_INFO
    }
}


pub const RADIOSITYMODIFIERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RadiosityModifierEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RadiosityMaterialOpacityTriggerEntityData {
    pub material_src_color: super::core::Vec3,
    pub material_src_opacity: f32,
    pub opacity_on_transparency: f32,
    pub opacity_on_color_multiplier: f32,
    pub opacity_off_transparency: f32,
    pub opacity_off_color_multiplier: f32,
    pub material_guid: super::core::Guid,
    pub opacity: f32,
    pub color: super::core::Vec3,
}

pub const RADIOSITYMATERIALOPACITYTRIGGERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialOpacityTriggerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaterialSrcColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialOpacityTriggerEntityData, material_src_color),
            },
            FieldInfoData {
                name: "MaterialSrcOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialOpacityTriggerEntityData, material_src_opacity),
            },
            FieldInfoData {
                name: "OpacityOnTransparency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialOpacityTriggerEntityData, opacity_on_transparency),
            },
            FieldInfoData {
                name: "OpacityOnColorMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialOpacityTriggerEntityData, opacity_on_color_multiplier),
            },
            FieldInfoData {
                name: "OpacityOffTransparency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialOpacityTriggerEntityData, opacity_off_transparency),
            },
            FieldInfoData {
                name: "OpacityOffColorMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialOpacityTriggerEntityData, opacity_off_color_multiplier),
            },
            FieldInfoData {
                name: "MaterialGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialOpacityTriggerEntityData, material_guid),
            },
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialOpacityTriggerEntityData, opacity),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialOpacityTriggerEntityData, color),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALOPACITYTRIGGERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityMaterialOpacityTriggerEntityData {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIALOPACITYTRIGGERENTITYDATA_TYPE_INFO
    }
}


pub const RADIOSITYMATERIALOPACITYTRIGGERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialOpacityTriggerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RadiosityMaterialOpacityTriggerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RadiosityMaterialInstanceEntityData {
    pub material_guid: super::core::Guid,
    pub material_update_mask: u8,
    pub color: super::core::Vec3,
    pub emissive_intensity: f32,
    pub opacity: f32,
    pub backface_type: super::render_base::RadiosityBackfaceType,
    pub field_flag_override0: u8,
}

pub const RADIOSITYMATERIALINSTANCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialInstanceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaterialGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialInstanceEntityData, material_guid),
            },
            FieldInfoData {
                name: "MaterialUpdateMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialInstanceEntityData, material_update_mask),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialInstanceEntityData, color),
            },
            FieldInfoData {
                name: "EmissiveIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialInstanceEntityData, emissive_intensity),
            },
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialInstanceEntityData, opacity),
            },
            FieldInfoData {
                name: "BackfaceType",
                flags: MemberInfoFlags::new(0),
                field_type: RADIOSITYBACKFACETYPE_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialInstanceEntityData, backface_type),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialInstanceEntityData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALINSTANCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityMaterialInstanceEntityData {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIALINSTANCEENTITYDATA_TYPE_INFO
    }
}


pub const RADIOSITYMATERIALINSTANCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialInstanceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RadiosityMaterialInstanceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RadiosityMaterialEntityData {
    pub material_guid: super::core::Guid,
    pub material_update_mask: u8,
    pub color: super::core::Vec3,
    pub emissive_intensity: f32,
    pub opacity: f32,
    pub backface_type: super::render_base::RadiosityBackfaceType,
    pub field_flag_override0: u8,
}

pub const RADIOSITYMATERIALENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaterialGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialEntityData, material_guid),
            },
            FieldInfoData {
                name: "MaterialUpdateMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialEntityData, material_update_mask),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialEntityData, color),
            },
            FieldInfoData {
                name: "EmissiveIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialEntityData, emissive_intensity),
            },
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialEntityData, opacity),
            },
            FieldInfoData {
                name: "BackfaceType",
                flags: MemberInfoFlags::new(0),
                field_type: RADIOSITYBACKFACETYPE_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialEntityData, backface_type),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterialEntityData, field_flag_override0),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIALENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityMaterialEntityData {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIALENTITYDATA_TYPE_INFO
    }
}


pub const RADIOSITYMATERIALENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RadiosityMaterialEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GroundHeightEntityData {
    pub data: super::world_base::GroundHeightData,
}

pub const GROUNDHEIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: GROUNDHEIGHTDATA_TYPE_INFO,
                rust_offset: offset_of!(GroundHeightEntityData, data),
            },
        ],
    }),
    array_type: Some(GROUNDHEIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GroundHeightEntityData {
    fn type_info() -> &'static TypeInfo {
        GROUNDHEIGHTENTITYDATA_TYPE_INFO
    }
}


pub const GROUNDHEIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("GroundHeightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RenderVolumeEntityData {
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub user_masks: super::core::Vec4,
    pub transform_type: super::world_base::RenderVolumeTransformType,
    pub enabled: bool,
}

pub const RENDERVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeEntityData, shader),
            },
            FieldInfoData {
                name: "UserMasks",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeEntityData, user_masks),
            },
            FieldInfoData {
                name: "TransformType",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERVOLUMETRANSFORMTYPE_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeEntityData, transform_type),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RenderVolumeEntityData, enabled),
            },
        ],
    }),
    array_type: Some(RENDERVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RenderVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        RENDERVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const RENDERVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RenderVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshProxyEntityData {
    pub mesh: super::render_base::MeshBaseAsset,
    pub base_pose_transforms: super::core::SparseTransformArray,
    pub static_instances: super::entity::StaticInstancingData,
}

pub const MESHPROXYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshProxyEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MeshProxyEntityData, mesh),
            },
            FieldInfoData {
                name: "BasePoseTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: SPARSETRANSFORMARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshProxyEntityData, base_pose_transforms),
            },
            FieldInfoData {
                name: "StaticInstances",
                flags: MemberInfoFlags::new(0),
                field_type: STATICINSTANCINGDATA_TYPE_INFO,
                rust_offset: offset_of!(MeshProxyEntityData, static_instances),
            },
        ],
    }),
    array_type: Some(MESHPROXYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MeshProxyEntityData {
    fn type_info() -> &'static TypeInfo {
        MESHPROXYENTITYDATA_TYPE_INFO
    }
}


pub const MESHPROXYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshProxyEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("MeshProxyEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ParticipatingMediaMaterialEntityData {
    pub specification_mode: super::world_base::ParticipatingMediaSpecificationMode,
    pub absorption: f32,
    pub scattering: super::core::Vec3,
    pub exctinction: f32,
    pub albedo: super::core::Vec3,
    pub emissive: super::core::Vec3,
    pub phase: f32,
}

pub const PARTICIPATINGMEDIAMATERIALENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaMaterialEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SpecificationMode",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICIPATINGMEDIASPECIFICATIONMODE_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialEntityData, specification_mode),
            },
            FieldInfoData {
                name: "Absorption",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialEntityData, absorption),
            },
            FieldInfoData {
                name: "Scattering",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialEntityData, scattering),
            },
            FieldInfoData {
                name: "Exctinction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialEntityData, exctinction),
            },
            FieldInfoData {
                name: "Albedo",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialEntityData, albedo),
            },
            FieldInfoData {
                name: "Emissive",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialEntityData, emissive),
            },
            FieldInfoData {
                name: "Phase",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ParticipatingMediaMaterialEntityData, phase),
            },
        ],
    }),
    array_type: Some(PARTICIPATINGMEDIAMATERIALENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ParticipatingMediaMaterialEntityData {
    fn type_info() -> &'static TypeInfo {
        PARTICIPATINGMEDIAMATERIALENTITYDATA_TYPE_INFO
    }
}


pub const PARTICIPATINGMEDIAMATERIALENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaMaterialEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ParticipatingMediaMaterialEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalFogVolumeEntityData {
    pub object_layers: u16,
    pub enabled: bool,
    pub participating_media: super::world_base::ParticipatingMedia,
    pub ambient_lighting_scale: f32,
    pub density_texture: super::render_base::TextureBaseAsset,
}

pub const LOCALFOGVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALVOLUMETRICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeEntityData, object_layers),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeEntityData, enabled),
            },
            FieldInfoData {
                name: "ParticipatingMedia",
                flags: MemberInfoFlags::new(0),
                field_type: PARTICIPATINGMEDIA_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeEntityData, participating_media),
            },
            FieldInfoData {
                name: "AmbientLightingScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeEntityData, ambient_lighting_scale),
            },
            FieldInfoData {
                name: "DensityTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(LocalFogVolumeEntityData, density_texture),
            },
        ],
    }),
    array_type: Some(LOCALFOGVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalFogVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALFOGVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const LOCALFOGVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalFogVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalVolumetricEntityData {
}

pub const LOCALVOLUMETRICENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalVolumetricEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALVOLUMETRICENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalVolumetricEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALVOLUMETRICENTITYDATA_TYPE_INFO
    }
}


pub const LOCALVOLUMETRICENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalVolumetricEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalVolumetricEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReflectionVolumeSynchronizerEntityData {
}

pub const REFLECTIONVOLUMESYNCHRONIZERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeSynchronizerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(REFLECTIONVOLUMESYNCHRONIZERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReflectionVolumeSynchronizerEntityData {
    fn type_info() -> &'static TypeInfo {
        REFLECTIONVOLUMESYNCHRONIZERENTITYDATA_TYPE_INFO
    }
}


pub const REFLECTIONVOLUMESYNCHRONIZERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeSynchronizerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ReflectionVolumeSynchronizerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalPlanarReflectionEntityData {
    pub far_plane: f32,
    pub clipping_offset: f32,
    pub clipping_enable: bool,
    pub distance_tolerance: f32,
    pub distance_falloff: f32,
    pub normal_tolerance: f32,
    pub normal_falloff: f32,
    pub enable: bool,
    pub terrain_reflections_enable: bool,
    pub sky_reflection_enable: bool,
}

pub const LOCALPLANARREFLECTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionEntityData, far_plane),
            },
            FieldInfoData {
                name: "ClippingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionEntityData, clipping_offset),
            },
            FieldInfoData {
                name: "ClippingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionEntityData, clipping_enable),
            },
            FieldInfoData {
                name: "DistanceTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionEntityData, distance_tolerance),
            },
            FieldInfoData {
                name: "DistanceFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionEntityData, distance_falloff),
            },
            FieldInfoData {
                name: "NormalTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionEntityData, normal_tolerance),
            },
            FieldInfoData {
                name: "NormalFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionEntityData, normal_falloff),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionEntityData, enable),
            },
            FieldInfoData {
                name: "TerrainReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionEntityData, terrain_reflections_enable),
            },
            FieldInfoData {
                name: "SkyReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalPlanarReflectionEntityData, sky_reflection_enable),
            },
        ],
    }),
    array_type: Some(LOCALPLANARREFLECTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocalPlanarReflectionEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALPLANARREFLECTIONENTITYDATA_TYPE_INFO
    }
}


pub const LOCALPLANARREFLECTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalPlanarReflectionEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrDistantReflectionVolumeEntityData {
    pub location_type: super::render_base::DistantIBLLocationType,
    pub local_offset: super::core::Vec3,
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
    pub enabled: bool,
    pub is_visible: bool,
}

pub const PBRDISTANTREFLECTIONVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrDistantReflectionVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PBRGENERICBOXREFLECTIONVOLUMEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LocationType",
                flags: MemberInfoFlags::new(0),
                field_type: DISTANTIBLLOCATIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, location_type),
            },
            FieldInfoData {
                name: "LocalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, local_offset),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIBLMODE_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, baked_texture),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrDistantReflectionVolumeEntityData, is_visible),
            },
        ],
    }),
    array_type: Some(PBRDISTANTREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrDistantReflectionVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        PBRDISTANTREFLECTIONVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const PBRDISTANTREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrDistantReflectionVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrDistantReflectionVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrBoxReflectionVolumeEntityData {
    pub influence_fade_normal: super::core::Vec3,
    pub side_fade_pos_x: f32,
    pub side_fade_neg_x: f32,
    pub side_fade_pos_y: f32,
    pub side_fade_neg_y: f32,
    pub side_fade_pos_z: f32,
    pub side_fade_neg_z: f32,
    pub local_offset: super::core::Vec3,
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
    pub enabled: bool,
    pub is_visible: bool,
}

pub const PBRBOXREFLECTIONVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrBoxReflectionVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PBRGENERICBOXREFLECTIONVOLUMEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InfluenceFadeNormal",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, influence_fade_normal),
            },
            FieldInfoData {
                name: "SideFadePosX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, side_fade_pos_x),
            },
            FieldInfoData {
                name: "SideFadeNegX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, side_fade_neg_x),
            },
            FieldInfoData {
                name: "SideFadePosY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, side_fade_pos_y),
            },
            FieldInfoData {
                name: "SideFadeNegY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, side_fade_neg_y),
            },
            FieldInfoData {
                name: "SideFadePosZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, side_fade_pos_z),
            },
            FieldInfoData {
                name: "SideFadeNegZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, side_fade_neg_z),
            },
            FieldInfoData {
                name: "LocalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, local_offset),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIBLMODE_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, baked_texture),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrBoxReflectionVolumeEntityData, is_visible),
            },
        ],
    }),
    array_type: Some(PBRBOXREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrBoxReflectionVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        PBRBOXREFLECTIONVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const PBRBOXREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrBoxReflectionVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrBoxReflectionVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrGenericBoxReflectionVolumeEntityData {
}

pub const PBRGENERICBOXREFLECTIONVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrGenericBoxReflectionVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PBRREFLECTIONVOLUMEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PBRGENERICBOXREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrGenericBoxReflectionVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        PBRGENERICBOXREFLECTIONVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const PBRGENERICBOXREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrGenericBoxReflectionVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrGenericBoxReflectionVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrSphereReflectionVolumeEntityData {
    pub influence_fade_normal: f32,
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
    pub enabled: bool,
    pub is_visible: bool,
}

pub const PBRSPHEREREFLECTIONVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereReflectionVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PBRREFLECTIONVOLUMEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InfluenceFadeNormal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, influence_fade_normal),
            },
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALIBLMODE_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, mode),
            },
            FieldInfoData {
                name: "CaptureDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, capture_distance),
            },
            FieldInfoData {
                name: "CaptureFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, capture_fade_distance),
            },
            FieldInfoData {
                name: "InfluenceExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, influence_expand_distance),
            },
            FieldInfoData {
                name: "InfluenceFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, influence_fade_distance),
            },
            FieldInfoData {
                name: "UpdateWhenMoving",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, update_when_moving),
            },
            FieldInfoData {
                name: "CaptureSky",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, capture_sky),
            },
            FieldInfoData {
                name: "CaptureSkyMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, capture_sky_mask),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsAO",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, use_sky_visibility_as_a_o),
            },
            FieldInfoData {
                name: "UseSkyVisibilityAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, use_sky_visibility_as_mask),
            },
            FieldInfoData {
                name: "SharpenSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, sharpen_sky_visibility),
            },
            FieldInfoData {
                name: "BiasSkyVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, bias_sky_visibility),
            },
            FieldInfoData {
                name: "UseProxyReprojection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, use_proxy_reprojection),
            },
            FieldInfoData {
                name: "CaptureFog",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, capture_fog),
            },
            FieldInfoData {
                name: "ObjectLayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, object_layers),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, baked_texture),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, enabled),
            },
            FieldInfoData {
                name: "IsVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereReflectionVolumeEntityData, is_visible),
            },
        ],
    }),
    array_type: Some(PBRSPHEREREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrSphereReflectionVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        PBRSPHEREREFLECTIONVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const PBRSPHEREREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereReflectionVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrSphereReflectionVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrReflectionVolumeEntityData {
}

pub const PBRREFLECTIONVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrReflectionVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BAKEABLETEXTUREENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PBRREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrReflectionVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        PBRREFLECTIONVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const PBRREFLECTIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrReflectionVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrReflectionVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrRectangularLightEntityData {
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
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
}

pub const PBRRECTANGULARLIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PBRANALYTICLIGHTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shape",
                flags: MemberInfoFlags::new(0),
                field_type: RECTANGULARLIGHTSHAPE_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shape),
            },
            FieldInfoData {
                name: "OuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, outer_angle),
            },
            FieldInfoData {
                name: "Aspect",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, aspect),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, height),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, texture),
            },
            FieldInfoData {
                name: "ShadowMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_max_angle),
            },
            FieldInfoData {
                name: "ShadowFadeOutRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_fade_out_range),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTUNITTYPE_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrRectangularLightEntityData, shadow_cache_update_counter),
            },
        ],
    }),
    array_type: Some(PBRRECTANGULARLIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrRectangularLightEntityData {
    fn type_info() -> &'static TypeInfo {
        PBRRECTANGULARLIGHTENTITYDATA_TYPE_INFO
    }
}


pub const PBRRECTANGULARLIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrRectangularLightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrTubeLightEntityData {
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
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
}

pub const PBRTUBELIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PBRANALYTICLIGHTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TubeRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, tube_radius),
            },
            FieldInfoData {
                name: "TubeWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, tube_width),
            },
            FieldInfoData {
                name: "OnlyHempishere",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, only_hempishere),
            },
            FieldInfoData {
                name: "IsCapsule",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, is_capsule),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTUNITTYPE_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrTubeLightEntityData, shadow_cache_update_counter),
            },
        ],
    }),
    array_type: Some(PBRTUBELIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrTubeLightEntityData {
    fn type_info() -> &'static TypeInfo {
        PBRTUBELIGHTENTITYDATA_TYPE_INFO
    }
}


pub const PBRTUBELIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrTubeLightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrSpotLightEntityData {
    pub disc_radius: f32,
    pub inner_angle: f32,
    pub outer_angle: f32,
    pub i_e_s_profile: super::world_base::IesProfileAsset,
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
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
}

pub const PBRSPOTLIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PBRANALYTICLIGHTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DiscRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, disc_radius),
            },
            FieldInfoData {
                name: "InnerAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, inner_angle),
            },
            FieldInfoData {
                name: "OuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, outer_angle),
            },
            FieldInfoData {
                name: "IESProfile",
                flags: MemberInfoFlags::new(0),
                field_type: IESPROFILEASSET_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, i_e_s_profile),
            },
            FieldInfoData {
                name: "UseIESProfileAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, use_i_e_s_profile_as_mask),
            },
            FieldInfoData {
                name: "IESMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, i_e_s_multiplier),
            },
            FieldInfoData {
                name: "ShadowMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_max_angle),
            },
            FieldInfoData {
                name: "ShadowFadeOutRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_fade_out_range),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTUNITTYPE_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSpotLightEntityData, shadow_cache_update_counter),
            },
        ],
    }),
    array_type: Some(PBRSPOTLIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrSpotLightEntityData {
    fn type_info() -> &'static TypeInfo {
        PBRSPOTLIGHTENTITYDATA_TYPE_INFO
    }
}


pub const PBRSPOTLIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrSpotLightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrSphereLightEntityData {
    pub sphere_radius: f32,
    pub only_hempishere: bool,
    pub i_e_s_profile: super::world_base::IesProfileAsset,
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
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
}

pub const PBRSPHERELIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PBRANALYTICLIGHTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SphereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, sphere_radius),
            },
            FieldInfoData {
                name: "OnlyHempishere",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, only_hempishere),
            },
            FieldInfoData {
                name: "IESProfile",
                flags: MemberInfoFlags::new(0),
                field_type: IESPROFILEASSET_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, i_e_s_profile),
            },
            FieldInfoData {
                name: "UseIESProfileAsMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, use_i_e_s_profile_as_mask),
            },
            FieldInfoData {
                name: "IESMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, i_e_s_multiplier),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, color),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, intensity),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, exposure_compensation),
            },
            FieldInfoData {
                name: "AttenuationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, attenuation_radius),
            },
            FieldInfoData {
                name: "EmissiveShapeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, emissive_shape_enable),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, attenuation_offset),
            },
            FieldInfoData {
                name: "LightUnit",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTUNITTYPE_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, light_unit),
            },
            FieldInfoData {
                name: "AffectDiffuse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, affect_diffuse),
            },
            FieldInfoData {
                name: "AffectSpecular",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, affect_specular),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, particle_color_scale),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, cast_volumetric),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowNearRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_near_radius),
            },
            FieldInfoData {
                name: "ShadowFarRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_far_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_dimmer),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "AffectRadiosity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, affect_radiosity),
            },
            FieldInfoData {
                name: "RadiosityColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, radiosity_color_scale),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, dimmer),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, cull_screen_area),
            },
            FieldInfoData {
                name: "FadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, fade_screen_area),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, cull_distance),
            },
            FieldInfoData {
                name: "FadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, fade_distance),
            },
            FieldInfoData {
                name: "ShadowCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_cull_screen_area),
            },
            FieldInfoData {
                name: "ShadowFadeScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_fade_screen_area),
            },
            FieldInfoData {
                name: "ShadowCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_cull_distance),
            },
            FieldInfoData {
                name: "ShadowFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_fade_distance),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PbrSphereLightEntityData, shadow_cache_update_counter),
            },
        ],
    }),
    array_type: Some(PBRSPHERELIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrSphereLightEntityData {
    fn type_info() -> &'static TypeInfo {
        PBRSPHERELIGHTENTITYDATA_TYPE_INFO
    }
}


pub const PBRSPHERELIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrSphereLightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PbrAnalyticLightEntityData {
}

pub const PBRANALYTICLIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrAnalyticLightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALLIGHTENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PBRANALYTICLIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PbrAnalyticLightEntityData {
    fn type_info() -> &'static TypeInfo {
        PBRANALYTICLIGHTENTITYDATA_TYPE_INFO
    }
}


pub const PBRANALYTICLIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrAnalyticLightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrAnalyticLightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpotLightEntityData {
    pub shape: super::world_base::SpotLightShape,
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
    pub enlighten_color_mode: super::world_base::EnlightenColorMode,
    pub enlighten_enable: bool,
    pub enlighten_color_scale: super::core::Vec3,
    pub particle_color_scale: super::core::Vec3,
    pub exposure_compensation: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
}

pub const SPOTLIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpotLightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALLIGHTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shape",
                flags: MemberInfoFlags::new(0),
                field_type: SPOTLIGHTSHAPE_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, shape),
            },
            FieldInfoData {
                name: "ConeInnerAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, cone_inner_angle),
            },
            FieldInfoData {
                name: "ConeOuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, cone_outer_angle),
            },
            FieldInfoData {
                name: "FrustumFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, frustum_fov),
            },
            FieldInfoData {
                name: "FrustumAspect",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, frustum_aspect),
            },
            FieldInfoData {
                name: "OrthoWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, ortho_width),
            },
            FieldInfoData {
                name: "OrthoHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, ortho_height),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, near_plane),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, texture),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, cast_shadows),
            },
            FieldInfoData {
                name: "CastVolumetricShadows",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, cast_volumetric_shadows),
            },
            FieldInfoData {
                name: "ShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, shadow_resolution),
            },
            FieldInfoData {
                name: "ShadowRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, shadow_radius),
            },
            FieldInfoData {
                name: "ShadowDimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, shadow_dimmer),
            },
            FieldInfoData {
                name: "FrustumAsCone",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, frustum_as_cone),
            },
            FieldInfoData {
                name: "FrustumAsConeAngle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, frustum_as_cone_angle),
            },
            FieldInfoData {
                name: "FrustumAsConeIntensityScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, frustum_as_cone_intensity_scale),
            },
            FieldInfoData {
                name: "CastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, cast_shadows_enable),
            },
            FieldInfoData {
                name: "CastVolumetricShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, cast_volumetric_shadows_enable),
            },
            FieldInfoData {
                name: "CastShadowsMinLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, cast_shadows_min_level),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, color),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, radius),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, intensity),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, attenuation_offset),
            },
            FieldInfoData {
                name: "DirectLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, direct_light_enable),
            },
            FieldInfoData {
                name: "SpecularEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, specular_enable),
            },
            FieldInfoData {
                name: "EnlightenColorMode",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENCOLORMODE_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, enlighten_color_mode),
            },
            FieldInfoData {
                name: "EnlightenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, enlighten_enable),
            },
            FieldInfoData {
                name: "EnlightenColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, enlighten_color_scale),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, particle_color_scale),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, exposure_compensation),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SpotLightEntityData, shadow_cache_update_counter),
            },
        ],
    }),
    array_type: Some(SPOTLIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SpotLightEntityData {
    fn type_info() -> &'static TypeInfo {
        SPOTLIGHTENTITYDATA_TYPE_INFO
    }
}


pub const SPOTLIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpotLightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SpotLightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PointLightEntityData {
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
    pub enlighten_color_mode: super::world_base::EnlightenColorMode,
    pub enlighten_enable: bool,
    pub enlighten_color_scale: super::core::Vec3,
    pub particle_color_scale: super::core::Vec3,
    pub exposure_compensation: f32,
    pub shadow_cache_enable: bool,
    pub shadow_cache_update_priority: f32,
    pub shadow_cache_update_counter: u32,
}

pub const POINTLIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PointLightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALLIGHTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, width),
            },
            FieldInfoData {
                name: "TranslucencyAmbient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, translucency_ambient),
            },
            FieldInfoData {
                name: "TranslucencyScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, translucency_scale),
            },
            FieldInfoData {
                name: "TranslucencyPower",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, translucency_power),
            },
            FieldInfoData {
                name: "TranslucencyDistortion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, translucency_distortion),
            },
            FieldInfoData {
                name: "DirectLightmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, direct_lightmap_enable),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, color),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, radius),
            },
            FieldInfoData {
                name: "Intensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, intensity),
            },
            FieldInfoData {
                name: "AttenuationOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, attenuation_offset),
            },
            FieldInfoData {
                name: "DirectLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, direct_light_enable),
            },
            FieldInfoData {
                name: "SpecularEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, specular_enable),
            },
            FieldInfoData {
                name: "EnlightenColorMode",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENCOLORMODE_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, enlighten_color_mode),
            },
            FieldInfoData {
                name: "EnlightenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, enlighten_enable),
            },
            FieldInfoData {
                name: "EnlightenColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, enlighten_color_scale),
            },
            FieldInfoData {
                name: "ParticleColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, particle_color_scale),
            },
            FieldInfoData {
                name: "ExposureCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, exposure_compensation),
            },
            FieldInfoData {
                name: "ShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, shadow_cache_enable),
            },
            FieldInfoData {
                name: "ShadowCacheUpdatePriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, shadow_cache_update_priority),
            },
            FieldInfoData {
                name: "ShadowCacheUpdateCounter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PointLightEntityData, shadow_cache_update_counter),
            },
        ],
    }),
    array_type: Some(POINTLIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PointLightEntityData {
    fn type_info() -> &'static TypeInfo {
        POINTLIGHTENTITYDATA_TYPE_INFO
    }
}


pub const POINTLIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PointLightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PointLightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocalLightEntityData {
    pub enabled: bool,
}

pub const LOCALLIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LocalLightEntityData, enabled),
            },
        ],
    }),
    array_type: Some(LOCALLIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalLightEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALLIGHTENTITYDATA_TYPE_INFO
    }
}


pub const LOCALLIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalLightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LensFlareReferenceObjectData {
}

pub const LENSFLAREREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LENSFLAREREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensFlareReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        LENSFLAREREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const LENSFLAREREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LensFlareReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LensFlareBlueprint {
    pub time_delta_type: super::entity::TimeDeltaType,
}

pub const LENSFLAREBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBJECTBLUEPRINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: TIMEDELTATYPE_TYPE_INFO,
                rust_offset: offset_of!(LensFlareBlueprint, time_delta_type),
            },
        ],
    }),
    array_type: Some(LENSFLAREBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LensFlareBlueprint {
    fn type_info() -> &'static TypeInfo {
        LENSFLAREBLUEPRINT_TYPE_INFO
    }
}


pub const LENSFLAREBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LensFlareBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LensFlareEntityData {
    pub debug_draw_occluder: bool,
    pub half_res: bool,
    pub occluder_size: f32,
    pub render_mode: super::world_base::LensFlareRenderMode,
    pub screen_clip: bool,
    pub depth_bias: f32,
    pub elements: Vec<super::world_base::LensFlareElement>,
    pub flare_direction_mode: super::world_base::FlareDirectionMode,
    pub visible: bool,
    pub dimmer: f32,
}

pub const LENSFLAREENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DebugDrawOccluder",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareEntityData, debug_draw_occluder),
            },
            FieldInfoData {
                name: "HalfRes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareEntityData, half_res),
            },
            FieldInfoData {
                name: "OccluderSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareEntityData, occluder_size),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: LENSFLARERENDERMODE_TYPE_INFO,
                rust_offset: offset_of!(LensFlareEntityData, render_mode),
            },
            FieldInfoData {
                name: "ScreenClip",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareEntityData, screen_clip),
            },
            FieldInfoData {
                name: "DepthBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareEntityData, depth_bias),
            },
            FieldInfoData {
                name: "Elements",
                flags: MemberInfoFlags::new(144),
                field_type: LENSFLAREELEMENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LensFlareEntityData, elements),
            },
            FieldInfoData {
                name: "FlareDirectionMode",
                flags: MemberInfoFlags::new(0),
                field_type: FLAREDIRECTIONMODE_TYPE_INFO,
                rust_offset: offset_of!(LensFlareEntityData, flare_direction_mode),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LensFlareEntityData, visible),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LensFlareEntityData, dimmer),
            },
        ],
    }),
    array_type: Some(LENSFLAREENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LensFlareEntityData {
    fn type_info() -> &'static TypeInfo {
        LENSFLAREENTITYDATA_TYPE_INFO
    }
}


pub const LENSFLAREENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LensFlareEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FogExclusionVolumeEntityData {
    pub fog_exclusion_volume_shape: super::world_base::FogExclusionVolumeShape,
    pub fog_volume_strength: f32,
    pub fade_out_start: f32,
    pub fade_out_end: f32,
    pub enabled: bool,
}

pub const FOGEXCLUSIONVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FogExclusionVolumeShape",
                flags: MemberInfoFlags::new(0),
                field_type: FOGEXCLUSIONVOLUMESHAPE_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeEntityData, fog_exclusion_volume_shape),
            },
            FieldInfoData {
                name: "FogVolumeStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeEntityData, fog_volume_strength),
            },
            FieldInfoData {
                name: "FadeOutStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeEntityData, fade_out_start),
            },
            FieldInfoData {
                name: "FadeOutEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeEntityData, fade_out_end),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FogExclusionVolumeEntityData, enabled),
            },
        ],
    }),
    array_type: Some(FOGEXCLUSIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FogExclusionVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        FOGEXCLUSIONVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const FOGEXCLUSIONVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("FogExclusionVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DistantShadowCacheVolumeEntityData {
    pub mode: super::render_base::ShadowCacheMode,
    pub resolution: u32,
    pub doublebuffer: bool,
    pub depth_bias: super::render_base::ShadowCacheDepthBias,
    pub dynamic_prod_priority: u32,
    pub tiles_per_side: u32,
    pub do_not_update_baked_texture: bool,
    pub baked_texture: super::render_base::TextureBaseAsset,
    pub enabled: bool,
}

pub const DISTANTSHADOWCACHEVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BAKEABLETEXTUREENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: SHADOWCACHEMODE_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheVolumeEntityData, mode),
            },
            FieldInfoData {
                name: "Resolution",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheVolumeEntityData, resolution),
            },
            FieldInfoData {
                name: "Doublebuffer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheVolumeEntityData, doublebuffer),
            },
            FieldInfoData {
                name: "DepthBias",
                flags: MemberInfoFlags::new(0),
                field_type: SHADOWCACHEDEPTHBIAS_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheVolumeEntityData, depth_bias),
            },
            FieldInfoData {
                name: "DynamicProdPriority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheVolumeEntityData, dynamic_prod_priority),
            },
            FieldInfoData {
                name: "TilesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheVolumeEntityData, tiles_per_side),
            },
            FieldInfoData {
                name: "DoNotUpdateBakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheVolumeEntityData, do_not_update_baked_texture),
            },
            FieldInfoData {
                name: "BakedTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheVolumeEntityData, baked_texture),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistantShadowCacheVolumeEntityData, enabled),
            },
        ],
    }),
    array_type: Some(DISTANTSHADOWCACHEVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DistantShadowCacheVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        DISTANTSHADOWCACHEVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const DISTANTSHADOWCACHEVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DistantShadowCacheVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SimpleVolumetricsEntity {
}

pub const SIMPLEVOLUMETRICSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SIMPLEVOLUMETRICSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SimpleVolumetricsEntity {
    fn type_info() -> &'static TypeInfo {
        SIMPLEVOLUMETRICSENTITY_TYPE_INFO
    }
}


pub const SIMPLEVOLUMETRICSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleVolumetricsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SimpleVolumetricsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScreenshotCaptureEntity {
}

pub const SCREENSHOTCAPTUREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotCaptureEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SCREENSHOTCAPTUREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ScreenshotCaptureEntity {
    fn type_info() -> &'static TypeInfo {
        SCREENSHOTCAPTUREENTITY_TYPE_INFO
    }
}


pub const SCREENSHOTCAPTUREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenshotCaptureEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ScreenshotCaptureEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderVolumeEntity {
}

pub const RENDERVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RENDERVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RenderVolumeEntity {
    fn type_info() -> &'static TypeInfo {
        RENDERVOLUMEENTITY_TYPE_INFO
    }
}


pub const RENDERVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RenderVolumeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RadiosityModifierEntity {
}

pub const RADIOSITYMODIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RADIOSITYMODIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RadiosityModifierEntity {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMODIFIERENTITY_TYPE_INFO
    }
}


pub const RADIOSITYMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityModifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RadiosityModifierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RadiosityMaterialOpacityTriggerEntity {
}

pub const RADIOSITYMATERIALOPACITYTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialOpacityTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RADIOSITYMATERIALOPACITYTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RadiosityMaterialOpacityTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIALOPACITYTRIGGERENTITY_TYPE_INFO
    }
}


pub const RADIOSITYMATERIALOPACITYTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialOpacityTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RadiosityMaterialOpacityTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RadiosityMaterialInstanceEntity {
}

pub const RADIOSITYMATERIALINSTANCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialInstanceEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RADIOSITYMATERIALINSTANCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RadiosityMaterialInstanceEntity {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIALINSTANCEENTITY_TYPE_INFO
    }
}


pub const RADIOSITYMATERIALINSTANCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialInstanceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RadiosityMaterialInstanceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RadiosityMaterialEntity {
}

pub const RADIOSITYMATERIALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RADIOSITYMATERIALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RadiosityMaterialEntity {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIALENTITY_TYPE_INFO
    }
}


pub const RADIOSITYMATERIALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterialEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RadiosityMaterialEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlanarReflectionLocatorEntity {
}

pub const PLANARREFLECTIONLOCATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PLANARREFLECTIONLOCATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PlanarReflectionLocatorEntity {
    fn type_info() -> &'static TypeInfo {
        PLANARREFLECTIONLOCATORENTITY_TYPE_INFO
    }
}


pub const PLANARREFLECTIONLOCATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionLocatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PlanarReflectionLocatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OccluderVolumeEntity {
}

pub const OCCLUDERVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OCCLUDERVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OccluderVolumeEntity {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERVOLUMEENTITY_TYPE_INFO
    }
}


pub const OCCLUDERVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("OccluderVolumeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OccluderPlaneEntity {
}

pub const OCCLUDERPLANEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OCCLUDERPLANEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OccluderPlaneEntity {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERPLANEENTITY_TYPE_INFO
    }
}


pub const OCCLUDERPLANEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderPlaneEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("OccluderPlaneEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OccluderMeshEntity {
}

pub const OCCLUDERMESHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OCCLUDERMESHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OccluderMeshEntity {
    fn type_info() -> &'static TypeInfo {
        OCCLUDERMESHENTITY_TYPE_INFO
    }
}


pub const OCCLUDERMESHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMeshEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("OccluderMeshEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshProxyEntity {
}

pub const MESHPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshProxyEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshProxyEntity {
    fn type_info() -> &'static TypeInfo {
        MESHPROXYENTITY_TYPE_INFO
    }
}


pub const MESHPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshProxyEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("MeshProxyEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ParticipatingMediaMaterialEntity {
}

pub const PARTICIPATINGMEDIAMATERIALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaMaterialEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PARTICIPATINGMEDIAMATERIALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ParticipatingMediaMaterialEntity {
    fn type_info() -> &'static TypeInfo {
        PARTICIPATINGMEDIAMATERIALENTITY_TYPE_INFO
    }
}


pub const PARTICIPATINGMEDIAMATERIALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParticipatingMediaMaterialEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ParticipatingMediaMaterialEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalFogVolumeEntity {
}

pub const LOCALFOGVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALFOGVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalFogVolumeEntity {
    fn type_info() -> &'static TypeInfo {
        LOCALFOGVOLUMEENTITY_TYPE_INFO
    }
}


pub const LOCALFOGVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalFogVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalFogVolumeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalPlanarReflectionEntity {
}

pub const LOCALPLANARREFLECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALPLANARREFLECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalPlanarReflectionEntity {
    fn type_info() -> &'static TypeInfo {
        LOCALPLANARREFLECTIONENTITY_TYPE_INFO
    }
}


pub const LOCALPLANARREFLECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlanarReflectionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalPlanarReflectionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReflectionVolumeSynchronizerEntity {
}

pub const REFLECTIONVOLUMESYNCHRONIZERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeSynchronizerEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(REFLECTIONVOLUMESYNCHRONIZERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ReflectionVolumeSynchronizerEntity {
    fn type_info() -> &'static TypeInfo {
        REFLECTIONVOLUMESYNCHRONIZERENTITY_TYPE_INFO
    }
}


pub const REFLECTIONVOLUMESYNCHRONIZERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionVolumeSynchronizerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ReflectionVolumeSynchronizerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DistantIBLEntity {
}

pub const DISTANTIBLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DISTANTIBLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DistantIBLEntity {
    fn type_info() -> &'static TypeInfo {
        DISTANTIBLENTITY_TYPE_INFO
    }
}


pub const DISTANTIBLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantIBLEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DistantIBLEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalBoxIBLEntity {
}

pub const LOCALBOXIBLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalBoxIBLEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALBOXIBLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalBoxIBLEntity {
    fn type_info() -> &'static TypeInfo {
        LOCALBOXIBLENTITY_TYPE_INFO
    }
}


pub const LOCALBOXIBLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalBoxIBLEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalBoxIBLEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalSphereIBLEntity {
}

pub const LOCALSPHEREIBLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalSphereIBLEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALSPHEREIBLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalSphereIBLEntity {
    fn type_info() -> &'static TypeInfo {
        LOCALSPHEREIBLENTITY_TYPE_INFO
    }
}


pub const LOCALSPHEREIBLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalSphereIBLEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalSphereIBLEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PbrRectangularLightEntity {
}

pub const PBRRECTANGULARLIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALLIGHTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PBRRECTANGULARLIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PbrRectangularLightEntity {
    fn type_info() -> &'static TypeInfo {
        PBRRECTANGULARLIGHTENTITY_TYPE_INFO
    }
}


pub const PBRRECTANGULARLIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrRectangularLightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrRectangularLightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PbrTubeLightEntity {
}

pub const PBRTUBELIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALLIGHTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PBRTUBELIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PbrTubeLightEntity {
    fn type_info() -> &'static TypeInfo {
        PBRTUBELIGHTENTITY_TYPE_INFO
    }
}


pub const PBRTUBELIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrTubeLightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrTubeLightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PbrSpotLightEntity {
}

pub const PBRSPOTLIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALLIGHTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PBRSPOTLIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PbrSpotLightEntity {
    fn type_info() -> &'static TypeInfo {
        PBRSPOTLIGHTENTITY_TYPE_INFO
    }
}


pub const PBRSPOTLIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSpotLightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrSpotLightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PbrSphereLightEntity {
}

pub const PBRSPHERELIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALLIGHTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PBRSPHERELIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PbrSphereLightEntity {
    fn type_info() -> &'static TypeInfo {
        PBRSPHERELIGHTENTITY_TYPE_INFO
    }
}


pub const PBRSPHERELIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PbrSphereLightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PbrSphereLightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OldSpotLightEntity {
}

pub const OLDSPOTLIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldSpotLightEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALLIGHTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OLDSPOTLIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OldSpotLightEntity {
    fn type_info() -> &'static TypeInfo {
        OLDSPOTLIGHTENTITY_TYPE_INFO
    }
}


pub const OLDSPOTLIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldSpotLightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("OldSpotLightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OldPointLightEntity {
}

pub const OLDPOINTLIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldPointLightEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALLIGHTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OLDPOINTLIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OldPointLightEntity {
    fn type_info() -> &'static TypeInfo {
        OLDPOINTLIGHTENTITY_TYPE_INFO
    }
}


pub const OLDPOINTLIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OldPointLightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("OldPointLightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalLightEntity {
}

pub const LOCALLIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLightEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALLIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalLightEntity {
    fn type_info() -> &'static TypeInfo {
        LOCALLIGHTENTITY_TYPE_INFO
    }
}


pub const LOCALLIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LocalLightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LensFlareEntity {
}

pub const LENSFLAREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LENSFLAREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LensFlareEntity {
    fn type_info() -> &'static TypeInfo {
        LENSFLAREENTITY_TYPE_INFO
    }
}


pub const LENSFLAREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensFlareEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LensFlareEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GroundHeightEntity {
}

pub const GROUNDHEIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GROUNDHEIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GroundHeightEntity {
    fn type_info() -> &'static TypeInfo {
        GROUNDHEIGHTENTITY_TYPE_INFO
    }
}


pub const GROUNDHEIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroundHeightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("GroundHeightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FogExclusionVolumeEntity {
}

pub const FOGEXCLUSIONVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FOGEXCLUSIONVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FogExclusionVolumeEntity {
    fn type_info() -> &'static TypeInfo {
        FOGEXCLUSIONVOLUMEENTITY_TYPE_INFO
    }
}


pub const FOGEXCLUSIONVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogExclusionVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("FogExclusionVolumeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicEnlightenEntity {
}

pub const DYNAMICENLIGHTENENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DYNAMICENLIGHTENENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DynamicEnlightenEntity {
    fn type_info() -> &'static TypeInfo {
        DYNAMICENLIGHTENENTITY_TYPE_INFO
    }
}


pub const DYNAMICENLIGHTENENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnlightenEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DynamicEnlightenEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DistantShadowCacheVolumeEntity {
}

pub const DISTANTSHADOWCACHEVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DISTANTSHADOWCACHEVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DistantShadowCacheVolumeEntity {
    fn type_info() -> &'static TypeInfo {
        DISTANTSHADOWCACHEVOLUMEENTITY_TYPE_INFO
    }
}


pub const DISTANTSHADOWCACHEVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistantShadowCacheVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DistantShadowCacheVolumeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BakeableTextureEntityData {
}

pub const BAKEABLETEXTUREENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BakeableTextureEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BAKEABLETEXTUREENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BakeableTextureEntityData {
    fn type_info() -> &'static TypeInfo {
        BAKEABLETEXTUREENTITYDATA_TYPE_INFO
    }
}


pub const BAKEABLETEXTUREENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BakeableTextureEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("BakeableTextureEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindForceSphereEntity {
}

pub const WINDFORCESPHEREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceSphereEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WINDFORCEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDFORCESPHEREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindForceSphereEntity {
    fn type_info() -> &'static TypeInfo {
        WINDFORCESPHEREENTITY_TYPE_INFO
    }
}


pub const WINDFORCESPHEREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceSphereEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindForceSphereEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindForceSphereComponent {
}

pub const WINDFORCESPHERECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceSphereComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WINDFORCECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDFORCESPHERECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindForceSphereComponent {
    fn type_info() -> &'static TypeInfo {
        WINDFORCESPHERECOMPONENT_TYPE_INFO
    }
}


pub const WINDFORCESPHERECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceSphereComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindForceSphereComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindForceSamplerEntity {
}

pub const WINDFORCESAMPLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceSamplerEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDFORCESAMPLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindForceSamplerEntity {
    fn type_info() -> &'static TypeInfo {
        WINDFORCESAMPLERENTITY_TYPE_INFO
    }
}


pub const WINDFORCESAMPLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceSamplerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindForceSamplerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindForceEntity {
}

pub const WINDFORCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDFORCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindForceEntity {
    fn type_info() -> &'static TypeInfo {
        WINDFORCEENTITY_TYPE_INFO
    }
}


pub const WINDFORCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindForceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindForceConeEntity {
}

pub const WINDFORCECONEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceConeEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WINDFORCEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDFORCECONEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindForceConeEntity {
    fn type_info() -> &'static TypeInfo {
        WINDFORCECONEENTITY_TYPE_INFO
    }
}


pub const WINDFORCECONEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceConeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindForceConeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindForceConeComponent {
}

pub const WINDFORCECONECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceConeComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WINDFORCECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDFORCECONECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindForceConeComponent {
    fn type_info() -> &'static TypeInfo {
        WINDFORCECONECOMPONENT_TYPE_INFO
    }
}


pub const WINDFORCECONECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceConeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindForceConeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindForceComponent {
}

pub const WINDFORCECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDFORCECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindForceComponent {
    fn type_info() -> &'static TypeInfo {
        WINDFORCECOMPONENT_TYPE_INFO
    }
}


pub const WINDFORCECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindForceComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindForceBaked3DAs2x2DTexEntity {
}

pub const WINDFORCEBAKED3DAS2X2DTEXENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceBaked3DAs2x2DTexEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WINDFORCEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDFORCEBAKED3DAS2X2DTEXENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindForceBaked3DAs2x2DTexEntity {
    fn type_info() -> &'static TypeInfo {
        WINDFORCEBAKED3DAS2X2DTEXENTITY_TYPE_INFO
    }
}


pub const WINDFORCEBAKED3DAS2X2DTEXENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceBaked3DAs2x2DTexEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindForceBaked3DAs2x2DTexEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindForceBaked3DAs2x2DTexComponent {
}

pub const WINDFORCEBAKED3DAS2X2DTEXCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceBaked3DAs2x2DTexComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WINDFORCECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDFORCEBAKED3DAS2X2DTEXCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindForceBaked3DAs2x2DTexComponent {
    fn type_info() -> &'static TypeInfo {
        WINDFORCEBAKED3DAS2X2DTEXCOMPONENT_TYPE_INFO
    }
}


pub const WINDFORCEBAKED3DAS2X2DTEXCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindForceBaked3DAs2x2DTexComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindForceBaked3DAs2x2DTexComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VisualEnvironmentEntity {
}

pub const VISUALENVIRONMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VISUALENVIRONMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VisualEnvironmentEntity {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTENTITY_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("VisualEnvironmentEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RaytraceReflectionComponent {
}

pub const RAYTRACEREFLECTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaytraceReflectionComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RAYTRACEREFLECTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RaytraceReflectionComponent {
    fn type_info() -> &'static TypeInfo {
        RAYTRACEREFLECTIONCOMPONENT_TYPE_INFO
    }
}


pub const RAYTRACEREFLECTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaytraceReflectionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RaytraceReflectionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SonarParamsComponent {
}

pub const SONARPARAMSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SonarParamsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SONARPARAMSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SonarParamsComponent {
    fn type_info() -> &'static TypeInfo {
        SONARPARAMSCOMPONENT_TYPE_INFO
    }
}


pub const SONARPARAMSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SonarParamsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SonarParamsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HologramParamsComponent {
}

pub const HOLOGRAMPARAMSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramParamsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(HOLOGRAMPARAMSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HologramParamsComponent {
    fn type_info() -> &'static TypeInfo {
        HOLOGRAMPARAMSCOMPONENT_TYPE_INFO
    }
}


pub const HOLOGRAMPARAMSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HologramParamsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("HologramParamsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ThreatAlertHighlightParamsComponent {
}

pub const THREATALERTHIGHLIGHTPARAMSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreatAlertHighlightParamsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(THREATALERTHIGHLIGHTPARAMSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ThreatAlertHighlightParamsComponent {
    fn type_info() -> &'static TypeInfo {
        THREATALERTHIGHLIGHTPARAMSCOMPONENT_TYPE_INFO
    }
}


pub const THREATALERTHIGHLIGHTPARAMSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreatAlertHighlightParamsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ThreatAlertHighlightParamsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectHighlightParamsComponent {
}

pub const OBJECTHIGHLIGHTPARAMSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightParamsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBJECTHIGHLIGHTPARAMSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectHighlightParamsComponent {
    fn type_info() -> &'static TypeInfo {
        OBJECTHIGHLIGHTPARAMSCOMPONENT_TYPE_INFO
    }
}


pub const OBJECTHIGHLIGHTPARAMSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHighlightParamsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ObjectHighlightParamsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntiAliasComponent {
}

pub const ANTIALIASCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiAliasComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTIALIASCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AntiAliasComponent {
    fn type_info() -> &'static TypeInfo {
        ANTIALIASCOMPONENT_TYPE_INFO
    }
}


pub const ANTIALIASCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntiAliasComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("AntiAliasComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScreenSpaceRaytraceComponent {
}

pub const SCREENSPACERAYTRACECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenSpaceRaytraceComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SCREENSPACERAYTRACECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ScreenSpaceRaytraceComponent {
    fn type_info() -> &'static TypeInfo {
        SCREENSPACERAYTRACECOMPONENT_TYPE_INFO
    }
}


pub const SCREENSPACERAYTRACECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenSpaceRaytraceComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ScreenSpaceRaytraceComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DebugComponent {
}

pub const DEBUGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DEBUGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DebugComponent {
    fn type_info() -> &'static TypeInfo {
        DEBUGCOMPONENT_TYPE_INFO
    }
}


pub const DEBUGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DebugComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LensReflectionComponent {
}

pub const LENSREFLECTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LENSREFLECTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LensReflectionComponent {
    fn type_info() -> &'static TypeInfo {
        LENSREFLECTIONCOMPONENT_TYPE_INFO
    }
}


pub const LENSREFLECTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensReflectionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LensReflectionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SkyCelestialRotationComponent {
}

pub const SKYCELESTIALROTATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialRotationComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SKYCELESTIALROTATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SkyCelestialRotationComponent {
    fn type_info() -> &'static TypeInfo {
        SKYCELESTIALROTATIONCOMPONENT_TYPE_INFO
    }
}


pub const SKYCELESTIALROTATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialRotationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SkyCelestialRotationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SkyCelestialMultiComponent {
}

pub const SKYCELESTIALMULTICOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialMultiComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SKYCELESTIALMULTICOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SkyCelestialMultiComponent {
    fn type_info() -> &'static TypeInfo {
        SKYCELESTIALMULTICOMPONENT_TYPE_INFO
    }
}


pub const SKYCELESTIALMULTICOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialMultiComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SkyCelestialMultiComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SkyCelestialSingleComponent {
}

pub const SKYCELESTIALSINGLECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialSingleComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SKYCELESTIALSINGLECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SkyCelestialSingleComponent {
    fn type_info() -> &'static TypeInfo {
        SKYCELESTIALSINGLECOMPONENT_TYPE_INFO
    }
}


pub const SKYCELESTIALSINGLECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyCelestialSingleComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SkyCelestialSingleComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterParamGlobalComponent {
}

pub const EMITTERPARAMGLOBALCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EMITTERPARAMGLOBALCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EmitterParamGlobalComponent {
    fn type_info() -> &'static TypeInfo {
        EMITTERPARAMGLOBALCOMPONENT_TYPE_INFO
    }
}


pub const EMITTERPARAMGLOBALCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamGlobalComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("EmitterParamGlobalComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterParamComponent {
}

pub const EMITTERPARAMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EMITTERPARAMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EmitterParamComponent {
    fn type_info() -> &'static TypeInfo {
        EMITTERPARAMCOMPONENT_TYPE_INFO
    }
}


pub const EMITTERPARAMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParamComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("EmitterParamComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderParamsComponent {
}

pub const SHADERPARAMSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParamsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADERPARAMSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderParamsComponent {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMSCOMPONENT_TYPE_INFO
    }
}


pub const SHADERPARAMSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParamsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ShaderParamsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MotionBlurComponent {
}

pub const MOTIONBLURCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionBlurComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MOTIONBLURCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MotionBlurComponent {
    fn type_info() -> &'static TypeInfo {
        MOTIONBLURCOMPONENT_TYPE_INFO
    }
}


pub const MOTIONBLURCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionBlurComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("MotionBlurComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubSurfaceScatteringComponent {
}

pub const SUBSURFACESCATTERINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceScatteringComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUBSURFACESCATTERINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubSurfaceScatteringComponent {
    fn type_info() -> &'static TypeInfo {
        SUBSURFACESCATTERINGCOMPONENT_TYPE_INFO
    }
}


pub const SUBSURFACESCATTERINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceScatteringComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SubSurfaceScatteringComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CharacterLightingComponent {
}

pub const CHARACTERLIGHTINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CHARACTERLIGHTINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CharacterLightingComponent {
    fn type_info() -> &'static TypeInfo {
        CHARACTERLIGHTINGCOMPONENT_TYPE_INFO
    }
}


pub const CHARACTERLIGHTINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterLightingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("CharacterLightingComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicEnvmapComponent {
}

pub const DYNAMICENVMAPCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnvmapComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DYNAMICENVMAPCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DynamicEnvmapComponent {
    fn type_info() -> &'static TypeInfo {
        DYNAMICENVMAPCOMPONENT_TYPE_INFO
    }
}


pub const DYNAMICENVMAPCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEnvmapComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DynamicEnvmapComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlanarReflectionComponent {
}

pub const PLANARREFLECTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PLANARREFLECTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PlanarReflectionComponent {
    fn type_info() -> &'static TypeInfo {
        PLANARREFLECTIONCOMPONENT_TYPE_INFO
    }
}


pub const PLANARREFLECTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlanarReflectionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("PlanarReflectionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DamageEffectComponent {
}

pub const DAMAGEEFFECTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageEffectComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DAMAGEEFFECTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DamageEffectComponent {
    fn type_info() -> &'static TypeInfo {
        DAMAGEEFFECTCOMPONENT_TYPE_INFO
    }
}


pub const DAMAGEEFFECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DamageEffectComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DamageEffectComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScreenEffectComponent {
}

pub const SCREENEFFECTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SCREENEFFECTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ScreenEffectComponent {
    fn type_info() -> &'static TypeInfo {
        SCREENEFFECTCOMPONENT_TYPE_INFO
    }
}


pub const SCREENEFFECTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScreenEffectComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ScreenEffectComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShadowsComponent {
}

pub const SHADOWSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADOWSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShadowsComponent {
    fn type_info() -> &'static TypeInfo {
        SHADOWSCOMPONENT_TYPE_INFO
    }
}


pub const SHADOWSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ShadowsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshSettingsComponent {
}

pub const MESHSETTINGSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSettingsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHSETTINGSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshSettingsComponent {
    fn type_info() -> &'static TypeInfo {
        MESHSETTINGSCOMPONENT_TYPE_INFO
    }
}


pub const MESHSETTINGSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSettingsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("MeshSettingsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CameraParamsComponent {
}

pub const CAMERAPARAMSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraParamsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAMERAPARAMSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CameraParamsComponent {
    fn type_info() -> &'static TypeInfo {
        CAMERAPARAMSCOMPONENT_TYPE_INFO
    }
}


pub const CAMERAPARAMSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CameraParamsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("CameraParamsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LensScopeComponent {
}

pub const LENSSCOPECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensScopeComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LENSSCOPECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LensScopeComponent {
    fn type_info() -> &'static TypeInfo {
        LENSSCOPECOMPONENT_TYPE_INFO
    }
}


pub const LENSSCOPECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LensScopeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("LensScopeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilmGrainComponent {
}

pub const FILMGRAINCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmGrainComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FILMGRAINCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FilmGrainComponent {
    fn type_info() -> &'static TypeInfo {
        FILMGRAINCOMPONENT_TYPE_INFO
    }
}


pub const FILMGRAINCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmGrainComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("FilmGrainComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VignetteComponent {
}

pub const VIGNETTECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignetteComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VIGNETTECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VignetteComponent {
    fn type_info() -> &'static TypeInfo {
        VIGNETTECOMPONENT_TYPE_INFO
    }
}


pub const VIGNETTECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VignetteComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("VignetteComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DofComponent {
}

pub const DOFCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DOFCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DofComponent {
    fn type_info() -> &'static TypeInfo {
        DOFCOMPONENT_TYPE_INFO
    }
}


pub const DOFCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DofComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DofComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilmicEffectsComponent {
}

pub const FILMICEFFECTSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmicEffectsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FILMICEFFECTSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FilmicEffectsComponent {
    fn type_info() -> &'static TypeInfo {
        FILMICEFFECTSCOMPONENT_TYPE_INFO
    }
}


pub const FILMICEFFECTSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FilmicEffectsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("FilmicEffectsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicAOComponent {
}

pub const DYNAMICAOCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicAOComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DYNAMICAOCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DynamicAOComponent {
    fn type_info() -> &'static TypeInfo {
        DYNAMICAOCOMPONENT_TYPE_INFO
    }
}


pub const DYNAMICAOCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicAOComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("DynamicAOComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SunFlareComponent {
}

pub const SUNFLARECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUNFLARECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SunFlareComponent {
    fn type_info() -> &'static TypeInfo {
        SUNFLARECOMPONENT_TYPE_INFO
    }
}


pub const SUNFLARECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SunFlareComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SunFlareComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CloudComponent {
}

pub const CLOUDCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLOUDCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CloudComponent {
    fn type_info() -> &'static TypeInfo {
        CLOUDCOMPONENT_TYPE_INFO
    }
}


pub const CLOUDCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CloudComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("CloudComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindComponent {
}

pub const WINDCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WINDCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WindComponent {
    fn type_info() -> &'static TypeInfo {
        WINDCOMPONENT_TYPE_INFO
    }
}


pub const WINDCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("WindComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FogComponent {
}

pub const FOGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FOGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FogComponent {
    fn type_info() -> &'static TypeInfo {
        FOGCOMPONENT_TYPE_INFO
    }
}


pub const FOGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("FogComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColorCorrectionComponent {
}

pub const COLORCORRECTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorCorrectionComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COLORCORRECTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ColorCorrectionComponent {
    fn type_info() -> &'static TypeInfo {
        COLORCORRECTIONCOMPONENT_TYPE_INFO
    }
}


pub const COLORCORRECTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorCorrectionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ColorCorrectionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TonemapComponent {
}

pub const TONEMAPCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TONEMAPCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TonemapComponent {
    fn type_info() -> &'static TypeInfo {
        TONEMAPCOMPONENT_TYPE_INFO
    }
}


pub const TONEMAPCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TonemapComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("TonemapComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnlightenComponent {
}

pub const ENLIGHTENCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENLIGHTENCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnlightenComponent {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENCOMPONENT_TYPE_INFO
    }
}


pub const ENLIGHTENCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("EnlightenComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OutdoorLightComponent {
}

pub const OUTDOORLIGHTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OUTDOORLIGHTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OutdoorLightComponent {
    fn type_info() -> &'static TypeInfo {
        OUTDOORLIGHTCOMPONENT_TYPE_INFO
    }
}


pub const OUTDOORLIGHTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("OutdoorLightComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IndirectSpecularComponent {
}

pub const INDIRECTSPECULARCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndirectSpecularComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INDIRECTSPECULARCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IndirectSpecularComponent {
    fn type_info() -> &'static TypeInfo {
        INDIRECTSPECULARCOMPONENT_TYPE_INFO
    }
}


pub const INDIRECTSPECULARCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IndirectSpecularComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("IndirectSpecularComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SkyComponent {
}

pub const SKYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyComponent",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SKYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SkyComponent {
    fn type_info() -> &'static TypeInfo {
        SKYCOMPONENT_TYPE_INFO
    }
}


pub const SKYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("SkyComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderFramesTrack {
}

pub const RENDERFRAMESTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFramesTrack",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RENDERFRAMESTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RenderFramesTrack {
    fn type_info() -> &'static TypeInfo {
        RENDERFRAMESTRACK_TYPE_INFO
    }
}


pub const RENDERFRAMESTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFramesTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("RenderFramesTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerVisibleAreaEntity {
}

pub const SERVERVISIBLEAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVisibleAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERVISIBLEAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerVisibleAreaEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERVISIBLEAREAENTITY_TYPE_INFO
    }
}


pub const SERVERVISIBLEAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerVisibleAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ServerVisibleAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientVisibleAreaEntity {
}

pub const CLIENTVISIBLEAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVisibleAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTVISIBLEAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientVisibleAreaEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTVISIBLEAREAENTITY_TYPE_INFO
    }
}


pub const CLIENTVISIBLEAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientVisibleAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("ClientVisibleAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticEnlightenEntity {
}

pub const STATICENLIGHTENENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenEntity",
    flags: MemberInfoFlags::new(101),
    module: "WorldSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STATICENLIGHTENENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StaticEnlightenEntity {
    fn type_info() -> &'static TypeInfo {
        STATICENLIGHTENENTITY_TYPE_INFO
    }
}


pub const STATICENLIGHTENENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldSim",
    data: TypeInfoData::Array("StaticEnlightenEntity-Array"),
    array_type: None,
    alignment: 8,
};


