use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_entity_types(registry: &mut TypeRegistry) {
    registry.register_type(WORLDPARTINCLUSION_TYPE_INFO);
    registry.register_type(WORLDPARTINCLUSION_ARRAY_TYPE_INFO);
    registry.register_type(WORLDPARTINCLUSIONCRITERION_TYPE_INFO);
    registry.register_type(WORLDPARTINCLUSIONCRITERION_ARRAY_TYPE_INFO);
    registry.register_type(WORLDPARTINCLUSIONSETTING_TYPE_INFO);
    registry.register_type(WORLDPARTINCLUSIONSETTING_ARRAY_TYPE_INFO);
    registry.register_type(WORLDPARTINCLUSIONSETTINGS_TYPE_INFO);
    registry.register_type(WORLDPARTINCLUSIONSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(ASSEMBLYDESCRIPTORDATA_TYPE_INFO);
    registry.register_type(ASSEMBLYDESCRIPTORDATA_ARRAY_TYPE_INFO);
    registry.register_type(WORLDOBJECTLISTDATA_TYPE_INFO);
    registry.register_type(WORLDOBJECTLISTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WORLDPARTREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(WORLDPARTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(READONLYLAYERREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(READONLYLAYERREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LAYERREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(LAYERREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SUBWORLDREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(SUBWORLDREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DETACHEDSUBWORLDDATA_TYPE_INFO);
    registry.register_type(DETACHEDSUBWORLDDATA_ARRAY_TYPE_INFO);
    registry.register_type(WORLDDATA_TYPE_INFO);
    registry.register_type(WORLDDATA_ARRAY_TYPE_INFO);
    registry.register_type(SUBWORLDDATA_TYPE_INFO);
    registry.register_type(SUBWORLDDATA_ARRAY_TYPE_INFO);
    registry.register_type(SUBWORLDDATACOMPONENT_TYPE_INFO);
    registry.register_type(SUBWORLDDATACOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WORLDPARTDATA_TYPE_INFO);
    registry.register_type(WORLDPARTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LAYERDATA_TYPE_INFO);
    registry.register_type(LAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(SUBWORLDINCLUSIONSETTINGS_TYPE_INFO);
    registry.register_type(SUBWORLDINCLUSIONSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(SUBWORLDINCLUSIONSETTING_TYPE_INFO);
    registry.register_type(SUBWORLDINCLUSIONSETTING_ARRAY_TYPE_INFO);
    registry.register_type(SUBWORLDINCLUSION_TYPE_INFO);
    registry.register_type(SUBWORLDINCLUSION_ARRAY_TYPE_INFO);
    registry.register_type(SUBWORLDINCLUSIONCRITERION_TYPE_INFO);
    registry.register_type(SUBWORLDINCLUSIONCRITERION_ARRAY_TYPE_INFO);
    registry.register_type(LEVELSETUP_TYPE_INFO);
    registry.register_type(LEVELSETUP_ARRAY_TYPE_INFO);
    registry.register_type(LEVELSETUPOPTION_TYPE_INFO);
    registry.register_type(LEVELSETUPOPTION_ARRAY_TYPE_INFO);
    registry.register_type(CREATESCHEMATICSINSTANCEENTITY_TYPE_INFO);
    registry.register_type(CREATESCHEMATICSINSTANCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(GETENTITYBUSENTITY_TYPE_INFO);
    registry.register_type(GETENTITYBUSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(GETENTITYENTITY_TYPE_INFO);
    registry.register_type(GETENTITYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CALLFUNCTIONENTITY_TYPE_INFO);
    registry.register_type(CALLFUNCTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYTRANSFORMSPACE_TYPE_INFO);
    registry.register_type(ENTITYTRANSFORMSPACE_ARRAY_TYPE_INFO);
    registry.register_type(ENTITY_TYPE_INFO);
    registry.register_type(ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPONENTENTITY_TYPE_INFO);
    registry.register_type(COMPONENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPONENT_TYPE_INFO);
    registry.register_type(COMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREAXISALIGNEDBOXENTITY_TYPE_INFO);
    registry.register_type(COMPAREAXISALIGNEDBOXENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREUINTRANGEENTITY_TYPE_INFO);
    registry.register_type(COMPAREUINTRANGEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREINTRANGEENTITY_TYPE_INFO);
    registry.register_type(COMPAREINTRANGEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREVEC4ENTITY_TYPE_INFO);
    registry.register_type(COMPAREVEC4ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREVEC3ENTITY_TYPE_INFO);
    registry.register_type(COMPAREVEC3ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREFLOATRANGEENTITY_TYPE_INFO);
    registry.register_type(COMPAREFLOATRANGEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPARESTRINGENTITY_TYPE_INFO);
    registry.register_type(COMPARESTRINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREFLOATENTITY_TYPE_INFO);
    registry.register_type(COMPAREFLOATENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREUINTENTITY_TYPE_INFO);
    registry.register_type(COMPAREUINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREINTENTITY_TYPE_INFO);
    registry.register_type(COMPAREINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREBOOLENTITY_TYPE_INFO);
    registry.register_type(COMPAREBOOLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDILATIONENTITY_TYPE_INFO);
    registry.register_type(SERVERDILATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBALANCEDDILATIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTBALANCEDDILATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ANIMTRACKDATA_TYPE_INFO);
    registry.register_type(ANIMTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(SEQUENCEENTITY_TYPE_INFO);
    registry.register_type(SEQUENCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(STRINGBUILDERENTITY_TYPE_INFO);
    registry.register_type(STRINGBUILDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC4CHANGEDENTITY_TYPE_INFO);
    registry.register_type(VEC4CHANGEDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC3CHANGEDENTITY_TYPE_INFO);
    registry.register_type(VEC3CHANGEDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC2CHANGEDENTITY_TYPE_INFO);
    registry.register_type(VEC2CHANGEDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(INTCHANGEDENTITY_TYPE_INFO);
    registry.register_type(INTCHANGEDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FLOATCHANGEDENTITY_TYPE_INFO);
    registry.register_type(FLOATCHANGEDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BOOLCHANGEDENTITY_TYPE_INFO);
    registry.register_type(BOOLCHANGEDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPARETRANSFORMENTITY_TYPE_INFO);
    registry.register_type(COMPARETRANSFORMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(GETSCHEMATICSINSTANCE_SCHEMATICSINSTANCE_ENTITY__TYPE_INFO);
    registry.register_type(GETSCHEMATICSINSTANCE_SCHEMATICSINSTANCE_ENTITYBUS__TYPE_INFO);
    registry.register_type(DESTROYENTITY_ENTITY__TYPE_INFO);
    registry.register_type(CREATEENTITY_ENTITY_ENTITYBUS_OBJECTBLUEPRINT_LINEARTRANSFORM__TYPE_INFO);
    registry.register_type(CASTTOSPATIAL_SPATIALENTITY_ENTITY__TYPE_INFO);
    registry.register_type(GETTRANSFORM_SPATIALENTITY_LINEARTRANSFORM__TYPE_INFO);
    registry.register_type(SETTRANSFORM_SPATIALENTITY_LINEARTRANSFORM__TYPE_INFO);
    registry.register_type(UPDATEPASSMASK_TYPE_INFO);
    registry.register_type(UPDATEPASSMASK_ARRAY_TYPE_INFO);
    registry.register_type(UPDATEPASS_TYPE_INFO);
    registry.register_type(UPDATEPASS_ARRAY_TYPE_INFO);
    registry.register_type(GAMEDATACONTAINERASSET_TYPE_INFO);
    registry.register_type(GAMEDATACONTAINERASSET_ARRAY_TYPE_INFO);
    registry.register_type(DATACONTAINERASSET_TYPE_INFO);
    registry.register_type(DATACONTAINERASSET_ARRAY_TYPE_INFO);
    registry.register_type(STATICINSTANCINGDATA_TYPE_INFO);
    registry.register_type(STATICINSTANCINGDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTDATA_TYPE_INFO);
    registry.register_type(PARTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTSTATE_TYPE_INFO);
    registry.register_type(PARTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(BONECOMPONENTDATA_TYPE_INFO);
    registry.register_type(BONECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMECOMPONENTDATA_TYPE_INFO);
    registry.register_type(GAMECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPONENTDATA_TYPE_INFO);
    registry.register_type(COMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMECOMPONENTENTITYDATA_TYPE_INFO);
    registry.register_type(GAMECOMPONENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPONENTENTITYDATA_TYPE_INFO);
    registry.register_type(COMPONENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SCRIPTENTITYDATA_TYPE_INFO);
    registry.register_type(SCRIPTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPATIALENTITYDATA_TYPE_INFO);
    registry.register_type(SPATIALENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYDATA_TYPE_INFO);
    registry.register_type(ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYCREATORTYPE_TYPE_INFO);
    registry.register_type(ENTITYCREATORTYPE_ARRAY_TYPE_INFO);
    registry.register_type(GAMEOBJECTDATA_TYPE_INFO);
    registry.register_type(GAMEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SUBREALM_TYPE_INFO);
    registry.register_type(SUBREALM_ARRAY_TYPE_INFO);
    registry.register_type(TIMESHAPE_TYPE_INFO);
    registry.register_type(TIMESHAPE_ARRAY_TYPE_INFO);
    registry.register_type(DILATIONPRIORITY_TYPE_INFO);
    registry.register_type(DILATIONPRIORITY_ARRAY_TYPE_INFO);
    registry.register_type(TIMEDELTATYPE_TYPE_INFO);
    registry.register_type(TIMEDELTATYPE_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICCHANNELENTITYDATA_TYPE_INFO);
    registry.register_type(SCHEMATICCHANNELENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICCHANNELASSET_TYPE_INFO);
    registry.register_type(SCHEMATICCHANNELASSET_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYCHANNEL_TYPE_INFO);
    registry.register_type(PROPERTYCHANNEL_ARRAY_TYPE_INFO);
    registry.register_type(EVENTCHANNEL_TYPE_INFO);
    registry.register_type(EVENTCHANNEL_ARRAY_TYPE_INFO);
    registry.register_type(LINKCHANNEL_TYPE_INFO);
    registry.register_type(LINKCHANNEL_ARRAY_TYPE_INFO);
    registry.register_type(LOGICPREFABREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(LOGICPREFABREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPATIALPREFABREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(SPATIALPREFABREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(OBJECTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOGICREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(LOGICREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPATIALREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(SPATIALREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(REFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(REFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(STREAMREALM_TYPE_INFO);
    registry.register_type(STREAMREALM_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONCOLLECTION_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONCOLLECTION_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTVARIATION_TYPE_INFO);
    registry.register_type(OBJECTVARIATION_ARRAY_TYPE_INFO);
    registry.register_type(DATACONTAINERCOLLECTIONBLUEPRINT_TYPE_INFO);
    registry.register_type(DATACONTAINERCOLLECTIONBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(LOGICPREFABBLUEPRINT_TYPE_INFO);
    registry.register_type(LOGICPREFABBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(SPATIALPREFABBLUEPRINT_TYPE_INFO);
    registry.register_type(SPATIALPREFABBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(PREFABBLUEPRINT_TYPE_INFO);
    registry.register_type(PREFABBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTBLUEPRINT_TYPE_INFO);
    registry.register_type(OBJECTBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINT_TYPE_INFO);
    registry.register_type(BLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTPERSISTENCESETTING_TYPE_INFO);
    registry.register_type(BLUEPRINTPERSISTENCESETTING_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYBUSDATA_TYPE_INFO);
    registry.register_type(ENTITYBUSDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTERFACEDESCRIPTORDATA_TYPE_INFO);
    registry.register_type(INTERFACEDESCRIPTORDATA_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICLINK_TYPE_INFO);
    registry.register_type(DYNAMICLINK_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICEVENT_TYPE_INFO);
    registry.register_type(DYNAMICEVENT_ARRAY_TYPE_INFO);
    registry.register_type(EVENTCONNECTION_TYPE_INFO);
    registry.register_type(EVENTCONNECTION_ARRAY_TYPE_INFO);
    registry.register_type(EVENTCONNECTIONTARGETTYPE_TYPE_INFO);
    registry.register_type(EVENTCONNECTIONTARGETTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EVENTSPEC_TYPE_INFO);
    registry.register_type(EVENTSPEC_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREAXISALIGNEDBOXENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREAXISALIGNEDBOXENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREUINTRANGEENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREUINTRANGEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREINTRANGEENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREINTRANGEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREFLOATRANGEENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREFLOATRANGEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPARESTRINGENTITYDATA_TYPE_INFO);
    registry.register_type(COMPARESTRINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREVEC4ENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREVEC4ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREVEC3ENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREVEC3ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREFLOATENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREFLOATENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREUINTENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREUINTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREINTENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREINTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREBOOLENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREBOOLENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREENTITYBASEDATA_TYPE_INFO);
    registry.register_type(COMPAREENTITYBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPROFILERDATA_TYPE_INFO);
    registry.register_type(CLIENTPROFILERDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHILDEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(CHILDEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BUNDLETYPE_TYPE_INFO);
    registry.register_type(BUNDLETYPE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOASSETCOLLECTOR_TYPE_INFO);
    registry.register_type(AUTOASSETCOLLECTOR_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKEDOBJECTSASSET_TYPE_INFO);
    registry.register_type(NETWORKEDOBJECTSASSET_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKREGISTRYASSET_TYPE_INFO);
    registry.register_type(NETWORKREGISTRYASSET_ARRAY_TYPE_INFO);
    registry.register_type(BUNDLEHEAPINFO_TYPE_INFO);
    registry.register_type(BUNDLEHEAPINFO_ARRAY_TYPE_INFO);
    registry.register_type(BUNDLEHEAPTYPE_TYPE_INFO);
    registry.register_type(BUNDLEHEAPTYPE_ARRAY_TYPE_INFO);
    registry.register_type(BUNDLESETTINGSINFO_TYPE_INFO);
    registry.register_type(BUNDLESETTINGSINFO_ARRAY_TYPE_INFO);
    registry.register_type(MESHOPERATIONDATA_TYPE_INFO);
    registry.register_type(MESHOPERATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(SKELETONBASEASSET_TYPE_INFO);
    registry.register_type(SKELETONBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMPARTPROPERTYTRACKDATA_TYPE_INFO);
    registry.register_type(TRANSFORMPARTPROPERTYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMPARTPROPERTYKEY_TYPE_INFO);
    registry.register_type(TRANSFORMPARTPROPERTYKEY_ARRAY_TYPE_INFO);
    registry.register_type(ANIMTANGENTTYPE_TYPE_INFO);
    registry.register_type(ANIMTANGENTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CURVEINFINITYTYPE_TYPE_INFO);
    registry.register_type(CURVEINFINITYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMPART_TYPE_INFO);
    registry.register_type(TRANSFORMPART_ARRAY_TYPE_INFO);
    registry.register_type(VEC4PROPERTYTRACKDATA_TYPE_INFO);
    registry.register_type(VEC4PROPERTYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC3PROPERTYTRACKDATA_TYPE_INFO);
    registry.register_type(VEC3PROPERTYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC2PROPERTYTRACKDATA_TYPE_INFO);
    registry.register_type(VEC2PROPERTYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMPROPERTYTRACKDATA_TYPE_INFO);
    registry.register_type(TRANSFORMPROPERTYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTPROPERTYTRACKDATA_TYPE_INFO);
    registry.register_type(INTPROPERTYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATPROPERTYTRACKDATA_TYPE_INFO);
    registry.register_type(FLOATPROPERTYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOOLPROPERTYTRACKDATA_TYPE_INFO);
    registry.register_type(BOOLPROPERTYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLEPROPERTYTRACKDATA_TYPE_INFO);
    registry.register_type(SIMPLEPROPERTYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(SEQUENCEENTITYDATA_TYPE_INFO);
    registry.register_type(SEQUENCEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMSEQUENCETRACKDATA_TYPE_INFO);
    registry.register_type(CUSTOMSEQUENCETRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMSEQUENCETRACKPROPERTYMAPPING_TYPE_INFO);
    registry.register_type(CUSTOMSEQUENCETRACKPROPERTYMAPPING_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMSEQUENCETRACKEVENTMAPPING_TYPE_INFO);
    registry.register_type(CUSTOMSEQUENCETRACKEVENTMAPPING_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMSEQUENCETRACKLINKMAPPING_TYPE_INFO);
    registry.register_type(CUSTOMSEQUENCETRACKLINKMAPPING_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYTRACKDATA_TYPE_INFO);
    registry.register_type(PROPERTYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTERPOLATIONTYPE_TYPE_INFO);
    registry.register_type(INTERPOLATIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SEQUENCEEVENTDATA_TYPE_INFO);
    registry.register_type(SEQUENCEEVENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYCLASSINFOASSET_TYPE_INFO);
    registry.register_type(ENTITYCLASSINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUTOVERRIDEBASE_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUTOVERRIDEBASE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUTBASE_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUTBASE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUTSBASE_TYPE_INFO);
    registry.register_type(AUTOPAINTOUTPUTSBASE_ARRAY_TYPE_INFO);
    registry.register_type(RASTERTYPE_TYPE_INFO);
    registry.register_type(RASTERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RASTERFORMAT_TYPE_INFO);
    registry.register_type(RASTERFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(BASESHAPEDATABASE_TYPE_INFO);
    registry.register_type(BASESHAPEDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(BASEMATERIALDATAPAIR_TYPE_INFO);
    registry.register_type(BASEMATERIALDATAPAIR_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALGRIDDATA_TYPE_INFO);
    registry.register_type(MATERIALGRIDDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALINTERACTIONGRIDROW_TYPE_INFO);
    registry.register_type(MATERIALINTERACTIONGRIDROW_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONPROPERTYPAIR_TYPE_INFO);
    registry.register_type(MATERIALRELATIONPROPERTYPAIR_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO);
    registry.register_type(PHYSICSPROPERTYRELATIONPROPERTYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO);
    registry.register_type(PHYSICSMATERIALRELATIONPROPERTYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALRELATIONPROPERTYDATA_TYPE_INFO);
    registry.register_type(MATERIALRELATIONPROPERTYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALDECL_TYPE_INFO);
    registry.register_type(MATERIALDECL_ARRAY_TYPE_INFO);
    registry.register_type(STRINGBUILDERENTITYDATA_TYPE_INFO);
    registry.register_type(STRINGBUILDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC4CHANGEDENTITYDATA_TYPE_INFO);
    registry.register_type(VEC4CHANGEDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC3CHANGEDENTITYDATA_TYPE_INFO);
    registry.register_type(VEC3CHANGEDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC2CHANGEDENTITYDATA_TYPE_INFO);
    registry.register_type(VEC2CHANGEDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTCHANGEDENTITYDATA_TYPE_INFO);
    registry.register_type(INTCHANGEDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATCHANGEDENTITYDATA_TYPE_INFO);
    registry.register_type(FLOATCHANGEDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOOLCHANGEDENTITYDATA_TYPE_INFO);
    registry.register_type(BOOLCHANGEDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYCHANGEDENTITYDATA_TYPE_INFO);
    registry.register_type(PROPERTYCHANGEDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPARETRANSFORMENTITYDATA_TYPE_INFO);
    registry.register_type(COMPARETRANSFORMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SWITCHENUMENTITYDATA_TYPE_INFO);
    registry.register_type(SWITCHENUMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EVENTNAME_TYPE_INFO);
    registry.register_type(EVENTNAME_ARRAY_TYPE_INFO);
    registry.register_type(SETENUMENTITYDATA_TYPE_INFO);
    registry.register_type(SETENUMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SETENUMVALUECONTAINER_TYPE_INFO);
    registry.register_type(SETENUMVALUECONTAINER_ARRAY_TYPE_INFO);
    registry.register_type(INTTOENUMENTITYDATA_TYPE_INFO);
    registry.register_type(INTTOENUMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WRITEARRAYELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(WRITEARRAYELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(READARRAYSIZEENTITYDATA_TYPE_INFO);
    registry.register_type(READARRAYSIZEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(READARRAYELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(READARRAYELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FOREACHVARIABLEENTITYDATA_TYPE_INFO);
    registry.register_type(FOREACHVARIABLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLEARARRAYVARIABLEENTITYDATA_TYPE_INFO);
    registry.register_type(CLEARARRAYVARIABLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ADDTOARRAYVARIABLEENTITYDATA_TYPE_INFO);
    registry.register_type(ADDTOARRAYVARIABLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WRITEVARIABLEENTITYDATA_TYPE_INFO);
    registry.register_type(WRITEVARIABLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(READVARIABLEENTITYDATA_TYPE_INFO);
    registry.register_type(READVARIABLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WRITEVARIABLEBASEENTITYDATA_TYPE_INFO);
    registry.register_type(WRITEVARIABLEBASEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(READVARIABLEBASEENTITYDATA_TYPE_INFO);
    registry.register_type(READVARIABLEBASEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VARIABLEBASEENTITYDATA_TYPE_INFO);
    registry.register_type(VARIABLEBASEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(HIGHLIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(HIGHLIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TEXTSEQUENCEENTITYDATA_TYPE_INFO);
    registry.register_type(TEXTSEQUENCEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TEXTSEQUENCEITEM_TYPE_INFO);
    registry.register_type(TEXTSEQUENCEITEM_ARRAY_TYPE_INFO);
    registry.register_type(TREEBASE_TYPE_INFO);
    registry.register_type(TREEBASE_ARRAY_TYPE_INFO);
    registry.register_type(TREENODEBASE_TYPE_INFO);
    registry.register_type(TREENODEBASE_ARRAY_TYPE_INFO);
    registry.register_type(SUBLEVELDESTROYEDMESSAGE_TYPE_INFO);
    registry.register_type(SUBLEVELENTITIESCREATEDMESSAGE_TYPE_INFO);
    registry.register_type(REPORTINSTALLATIONPROGRESSENTITYDATA_TYPE_INFO);
    registry.register_type(REPORTINSTALLATIONPROGRESSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALPLAYERIDENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALPLAYERIDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATCACHEENTITYDATA_TYPE_INFO);
    registry.register_type(FLOATCACHEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MULTILINESTRINGENTITYDATA_TYPE_INFO);
    registry.register_type(MULTILINESTRINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STRINGENTITYDATA_TYPE_INFO);
    registry.register_type(STRINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AXISALIGNEDBOXENTITYDATA_TYPE_INFO);
    registry.register_type(AXISALIGNEDBOXENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VECTOR4ENTITYDATA_TYPE_INFO);
    registry.register_type(VECTOR4ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VECTOR3ENTITYDATA_TYPE_INFO);
    registry.register_type(VECTOR3ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATENTITYDATA_TYPE_INFO);
    registry.register_type(FLOATENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UINTENTITYDATA_TYPE_INFO);
    registry.register_type(UINTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTENTITYDATA_TYPE_INFO);
    registry.register_type(INTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOOLENTITYDATA_TYPE_INFO);
    registry.register_type(BOOLENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MASTERSKELETONASSET_TYPE_INFO);
    registry.register_type(MASTERSKELETONASSET_ARRAY_TYPE_INFO);
    registry.register_type(SUBSKELETON_TYPE_INFO);
    registry.register_type(SUBSKELETON_ARRAY_TYPE_INFO);
    registry.register_type(SKELETONASSET_TYPE_INFO);
    registry.register_type(SKELETONASSET_ARRAY_TYPE_INFO);
    registry.register_type(BONESELECTION_TYPE_INFO);
    registry.register_type(BONESELECTION_ARRAY_TYPE_INFO);
    registry.register_type(GAMEPLAYBONE_TYPE_INFO);
    registry.register_type(GAMEPLAYBONE_ARRAY_TYPE_INFO);
    registry.register_type(GAMEPLAYBONES_TYPE_INFO);
    registry.register_type(GAMEPLAYBONES_ARRAY_TYPE_INFO);
    registry.register_type(SHAREDBUNDLEREFERENCE_TYPE_INFO);
    registry.register_type(SHAREDBUNDLEREFERENCE_ARRAY_TYPE_INFO);
    registry.register_type(SHAREDBUNDLEBASEASSET_TYPE_INFO);
    registry.register_type(SHAREDBUNDLEBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(SCANLINEVOLUMESHAPEDATA_TYPE_INFO);
    registry.register_type(SCANLINEVOLUMESHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ZONEVECTORSHAPEDATA_TYPE_INFO);
    registry.register_type(ZONEVECTORSHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(VOLUMEVECTORSHAPEDATA_TYPE_INFO);
    registry.register_type(VOLUMEVECTORSHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMSPLINEDATA_TYPE_INFO);
    registry.register_type(CUSTOMSPLINEDATA_ARRAY_TYPE_INFO);
    registry.register_type(VECTORSHAPEDATA_TYPE_INFO);
    registry.register_type(VECTORSHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(OBBDATA_TYPE_INFO);
    registry.register_type(OBBDATA_ARRAY_TYPE_INFO);
    registry.register_type(AABBDATA_TYPE_INFO);
    registry.register_type(AABBDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPHEREDATA_TYPE_INFO);
    registry.register_type(SPHEREDATA_ARRAY_TYPE_INFO);
    registry.register_type(BASESHAPEDATA_TYPE_INFO);
    registry.register_type(BASESHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTVEC4ENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTVEC4ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTVEC3ENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTVEC3ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTVEC2ENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTVEC2ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTTRANSFORMENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTTRANSFORMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTBOOLENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTBOOLENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTSTRINGENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTSTRINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTINT64ENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTINT64ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTINTENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTINTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTFLOATENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTFLOATENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTPROPERTYENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTPROPERTYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGOBJECTCATEGORYASSET_TYPE_INFO);
    registry.register_type(PATHFINDINGOBJECTCATEGORYASSET_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTAREATRIGGERENTITYDATA_TYPE_INFO);
    registry.register_type(OBJECTAREATRIGGERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONTOINTENTITYDATA_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONTOINTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMSPACEENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMSPACEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BALANCEDDILATIONENTITYDATA_TYPE_INFO);
    registry.register_type(BALANCEDDILATIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DILATIONENTITYDATA_TYPE_INFO);
    registry.register_type(DILATIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATSELECTENTITYDATA_TYPE_INFO);
    registry.register_type(FLOATSELECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATCURVEENTITYDATA_TYPE_INFO);
    registry.register_type(FLOATCURVEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALPLAYERGATEENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALPLAYERGATEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMSELECTORENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMSELECTORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SETTINGENTITYDATA_TYPE_INFO);
    registry.register_type(SETTINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTEGRATORORDIFFERENTIATORENTITYDATA_TYPE_INFO);
    registry.register_type(INTEGRATORORDIFFERENTIATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEENTITYDATA_TYPE_INFO);
    registry.register_type(PROFILEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREOP_TYPE_INFO);
    registry.register_type(COMPAREOP_ARRAY_TYPE_INFO);
    registry.register_type(RUNNINGAVERAGEENTITYDATA_TYPE_INFO);
    registry.register_type(RUNNINGAVERAGEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ABSENTITYDATA_TYPE_INFO);
    registry.register_type(ABSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VECTORMATHOPENTITYDATA_TYPE_INFO);
    registry.register_type(VECTORMATHOPENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VECTORMATHOP_TYPE_INFO);
    registry.register_type(VECTORMATHOP_ARRAY_TYPE_INFO);
    registry.register_type(MATHOPENTITYDATA_TYPE_INFO);
    registry.register_type(MATHOPENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATHOP_TYPE_INFO);
    registry.register_type(MATHOP_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYCASTENTITYDATA_TYPE_INFO);
    registry.register_type(PROPERTYCASTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOOLTOEVENTENTITYDATA_TYPE_INFO);
    registry.register_type(BOOLTOEVENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SWITCHPROPERTYSTRINGENTITYDATA_TYPE_INFO);
    registry.register_type(SWITCHPROPERTYSTRINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYGATEENTITYDATA_TYPE_INFO);
    registry.register_type(PROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EVENTGATEENTITYDATA_TYPE_INFO);
    registry.register_type(EVENTGATEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STOPWATCHENTITYDATA_TYPE_INFO);
    registry.register_type(STOPWATCHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMTOROTATIONENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMTOROTATIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMMODIFIERENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMMODIFIERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MODIFIERAXIS_TYPE_INFO);
    registry.register_type(MODIFIERAXIS_ARRAY_TYPE_INFO);
    registry.register_type(ROTATIONTRANSFORMBUILDERENTITYDATA_TYPE_INFO);
    registry.register_type(ROTATIONTRANSFORMBUILDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SCALETRANSFORMBUILDERENTITYDATA_TYPE_INFO);
    registry.register_type(SCALETRANSFORMBUILDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMBUILDERENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMBUILDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ORTHONORMALMODE_TYPE_INFO);
    registry.register_type(ORTHONORMALMODE_ARRAY_TYPE_INFO);
    registry.register_type(VECBUILDERENTITYDATA_TYPE_INFO);
    registry.register_type(VECBUILDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC4BUILDERENTITYDATA_TYPE_INFO);
    registry.register_type(VEC4BUILDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC3BUILDERENTITYDATA_TYPE_INFO);
    registry.register_type(VEC3BUILDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC2BUILDERENTITYDATA_TYPE_INFO);
    registry.register_type(VEC2BUILDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VECSPLITTERENTITYDATA_TYPE_INFO);
    registry.register_type(VECSPLITTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC4SPLITTERENTITYDATA_TYPE_INFO);
    registry.register_type(VEC4SPLITTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC3SPLITTERENTITYDATA_TYPE_INFO);
    registry.register_type(VEC3SPLITTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC2SPLITTERENTITYDATA_TYPE_INFO);
    registry.register_type(VEC2SPLITTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EULERTRANSFORMENTITYDATA_TYPE_INFO);
    registry.register_type(EULERTRANSFORMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MODIFIEREULER_TYPE_INFO);
    registry.register_type(MODIFIEREULER_ARRAY_TYPE_INFO);
    registry.register_type(EULERTRANSFORMSPLITTERENTITYDATA_TYPE_INFO);
    registry.register_type(EULERTRANSFORMSPLITTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMSPLITTERENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMSPLITTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMBLENDENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMBLENDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMMULTIPLIERENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMMULTIPLIERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMHUBENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMHUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STRINGHUBENTITYDATA_TYPE_INFO);
    registry.register_type(STRINGHUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC4HUBENTITYDATA_TYPE_INFO);
    registry.register_type(VEC4HUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC3HUBENTITYDATA_TYPE_INFO);
    registry.register_type(VEC3HUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC2HUBENTITYDATA_TYPE_INFO);
    registry.register_type(VEC2HUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATHUBENTITYDATA_TYPE_INFO);
    registry.register_type(FLOATHUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(INT64HUBENTITYDATA_TYPE_INFO);
    registry.register_type(INT64HUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTHUBENTITYDATA_TYPE_INFO);
    registry.register_type(INTHUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOOLHUBENTITYDATA_TYPE_INFO);
    registry.register_type(BOOLHUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLEDOFPARAMCONVERTERENTITYDATA_TYPE_INFO);
    registry.register_type(SIMPLEDOFPARAMCONVERTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPRITEDOFPARAMCONVERTERENTITYDATA_TYPE_INFO);
    registry.register_type(SPRITEDOFPARAMCONVERTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONMULTISWITCHENTITYDATA_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONMULTISWITCHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONSWITCHENTITYDATA_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONSWITCHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMDELAYENTITYDATA_TYPE_INFO);
    registry.register_type(RANDOMDELAYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DELAYENTITYDATA_TYPE_INFO);
    registry.register_type(DELAYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EVENTORDERGATEENTITYDATA_TYPE_INFO);
    registry.register_type(EVENTORDERGATEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EVENTANDGATEENTITYDATA_TYPE_INFO);
    registry.register_type(EVENTANDGATEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTEVENTENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTEVENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EVENTSWITCHENTITYDATA_TYPE_INFO);
    registry.register_type(EVENTSWITCHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UNIQUEIDENTITYDATA_TYPE_INFO);
    registry.register_type(UNIQUEIDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMINTENTITYDATA_TYPE_INFO);
    registry.register_type(RANDOMINTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMFLOATENTITYDATA_TYPE_INFO);
    registry.register_type(RANDOMFLOATENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMMULTIEVENTENTITYDATA_TYPE_INFO);
    registry.register_type(RANDOMMULTIEVENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMEVENTENTITYDATA_TYPE_INFO);
    registry.register_type(RANDOMEVENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCATORENTITYDATA_TYPE_INFO);
    registry.register_type(LOCATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(COUNTDOWNENTITYDATA_TYPE_INFO);
    registry.register_type(COUNTDOWNENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATHENTITYDATA_TYPE_INFO);
    registry.register_type(MATHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATHENTITYASSEMBLY_TYPE_INFO);
    registry.register_type(MATHENTITYASSEMBLY_ARRAY_TYPE_INFO);
    registry.register_type(MATHENTITYTYPE_TYPE_INFO);
    registry.register_type(MATHENTITYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MATHENTITYFUNCTIONCALL_TYPE_INFO);
    registry.register_type(MATHENTITYFUNCTIONCALL_ARRAY_TYPE_INFO);
    registry.register_type(MATHENTITYINSTRUCTION_TYPE_INFO);
    registry.register_type(MATHENTITYINSTRUCTION_ARRAY_TYPE_INFO);
    registry.register_type(MATHOPCODE_TYPE_INFO);
    registry.register_type(MATHOPCODE_ARRAY_TYPE_INFO);
    registry.register_type(OR4ENTITYDATA_TYPE_INFO);
    registry.register_type(OR4ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(XORENTITYDATA_TYPE_INFO);
    registry.register_type(XORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ORENTITYDATA_TYPE_INFO);
    registry.register_type(ORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BITWISEANDENTITYDATA_TYPE_INFO);
    registry.register_type(BITWISEANDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANDENTITYDATA_TYPE_INFO);
    registry.register_type(ANDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(NOTENTITYDATA_TYPE_INFO);
    registry.register_type(NOTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATESCHEMATICSINSTANCEENTITYDATA_TYPE_INFO);
    registry.register_type(CREATESCHEMATICSINSTANCEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(GETENTITYBUSENTITYDATA_TYPE_INFO);
    registry.register_type(GETENTITYBUSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(GETENTITYENTITYDATA_TYPE_INFO);
    registry.register_type(GETENTITYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CALLFUNCTIONENTITYDATA_TYPE_INFO);
    registry.register_type(CALLFUNCTIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYOWNERUID_TYPE_INFO);
    registry.register_type(ENTITYOWNERUID_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYUID_TYPE_INFO);
    registry.register_type(ENTITYUID_ARRAY_TYPE_INFO);
    registry.register_type(SETTRANSFORMSPACELOCALTRANSFORM_ENTITYTRANSFORMSPACE_LINEARTRANSFORM__TYPE_INFO);
    registry.register_type(GETTRANSFORMSPACE_ENTITYTRANSFORMSPACE_ENTITY__TYPE_INFO);
    registry.register_type(TRANSFORMSPACECHILDCONTROLSETTING_TYPE_INFO);
    registry.register_type(TRANSFORMSPACECHILDCONTROLSETTING_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMSPACESIMULATIONSETTING_TYPE_INFO);
    registry.register_type(TRANSFORMSPACESIMULATIONSETTING_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYSETTINGS_TYPE_INFO);
    registry.register_type(ENTITYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(EXECUTIONMODETYPE_TYPE_INFO);
    registry.register_type(EXECUTIONMODETYPE_ARRAY_TYPE_INFO);
    registry.register_type(SWITCHENUMENTITY_TYPE_INFO);
    registry.register_type(SWITCHENUMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SETENUMENTITY_TYPE_INFO);
    registry.register_type(SETENUMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(INTTOENUMENTITY_TYPE_INFO);
    registry.register_type(INTTOENUMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ENUMTOSTRINGENTITY_TYPE_INFO);
    registry.register_type(ENUMTOSTRINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ENUMTOINTENTITY_TYPE_INFO);
    registry.register_type(ENUMTOINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ENUMPROPERTYGATEENTITY_TYPE_INFO);
    registry.register_type(ENUMPROPERTYGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ENUMLOGICENTITYWITHSINGLEINPUT_TYPE_INFO);
    registry.register_type(ENUMLOGICENTITYWITHSINGLEINPUT_ARRAY_TYPE_INFO);
    registry.register_type(ENUMINTSOURCEENTITY_TYPE_INFO);
    registry.register_type(ENUMINTSOURCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREENUMENTITY_TYPE_INFO);
    registry.register_type(COMPAREENUMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYBUSPEER_TYPE_INFO);
    registry.register_type(ENTITYBUSPEER_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYEVENT_TYPE_INFO);
    registry.register_type(ENTITYEVENT_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYBUS_TYPE_INFO);
    registry.register_type(ENTITYBUS_ARRAY_TYPE_INFO);
    registry.register_type(DATASOURCEQUERYENTITY_TYPE_INFO);
    registry.register_type(DATASOURCEQUERYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ENUMTOSTRINGDATA_TYPE_INFO);
    registry.register_type(ENUMTOSTRINGDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENUMTOINTDATA_TYPE_INFO);
    registry.register_type(ENUMTOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENUMPROPERTYGATEENTITYDATA_TYPE_INFO);
    registry.register_type(ENUMPROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENUMDEBUGENTITYDATA_TYPE_INFO);
    registry.register_type(ENUMDEBUGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(IMPLIEDENUMTYPELOGICENTITYDATA_TYPE_INFO);
    registry.register_type(IMPLIEDENUMTYPELOGICENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EXPLICITENUMTYPELOGICENTITYDATA_TYPE_INFO);
    registry.register_type(EXPLICITENUMTYPELOGICENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENUMLOGICENTITYBASEDATA_TYPE_INFO);
    registry.register_type(ENUMLOGICENTITYBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENUMINTSOURCEENTITYDATA_TYPE_INFO);
    registry.register_type(ENUMINTSOURCEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENUMINTSOURCEPROPERTY_TYPE_INFO);
    registry.register_type(ENUMINTSOURCEPROPERTY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREENUMENTITYDATA_TYPE_INFO);
    registry.register_type(COMPAREENUMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DATASOURCEQUERYENTITYDATA_TYPE_INFO);
    registry.register_type(DATASOURCEQUERYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DATASOURCEQUERYFIELDDEFINITION_TYPE_INFO);
    registry.register_type(DATASOURCEQUERYFIELDDEFINITION_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYWRAPPER_TYPE_INFO);
    registry.register_type(PROPERTYWRAPPER_ARRAY_TYPE_INFO);
    registry.register_type(BUNDLEMANAGERBUNDLEUNLOADMESSAGE_TYPE_INFO);
    registry.register_type(WRITEARRAYELEMENTENTITY_TYPE_INFO);
    registry.register_type(WRITEARRAYELEMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(READARRAYSIZEENTITY_TYPE_INFO);
    registry.register_type(READARRAYSIZEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(READARRAYELEMENTENTITY_TYPE_INFO);
    registry.register_type(READARRAYELEMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FOREACHVARIABLEENTITY_TYPE_INFO);
    registry.register_type(FOREACHVARIABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLEARARRAYVARIABLEENTITY_TYPE_INFO);
    registry.register_type(CLEARARRAYVARIABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ADDTOARRAYVARIABLEENTITY_TYPE_INFO);
    registry.register_type(ADDTOARRAYVARIABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WRITEVARIABLEENTITY_TYPE_INFO);
    registry.register_type(WRITEVARIABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(READVARIABLEENTITY_TYPE_INFO);
    registry.register_type(READVARIABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(HIGHLIGHTENTITY_TYPE_INFO);
    registry.register_type(HIGHLIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TEXTSEQUENCEENTITY_TYPE_INFO);
    registry.register_type(TEXTSEQUENCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SUBLEVELENTITY_TYPE_INFO);
    registry.register_type(SUBLEVELENTITY_ARRAY_TYPE_INFO);
    registry.register_type(REPORTINSTALLATIONPROGRESSENTITY_TYPE_INFO);
    registry.register_type(REPORTINSTALLATIONPROGRESSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOCALPLAYERIDENTITY_TYPE_INFO);
    registry.register_type(LOCALPLAYERIDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FLOATCACHEENTITY_TYPE_INFO);
    registry.register_type(FLOATCACHEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(MULTILINESTRINGENTITY_TYPE_INFO);
    registry.register_type(MULTILINESTRINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(STRINGENTITY_TYPE_INFO);
    registry.register_type(STRINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AXISALIGNEDBOXENTITY_TYPE_INFO);
    registry.register_type(AXISALIGNEDBOXENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VECTOR4ENTITY_TYPE_INFO);
    registry.register_type(VECTOR4ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VECTOR3ENTITY_TYPE_INFO);
    registry.register_type(VECTOR3ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FLOATENTITY_TYPE_INFO);
    registry.register_type(FLOATENTITY_ARRAY_TYPE_INFO);
    registry.register_type(UINTENTITY_TYPE_INFO);
    registry.register_type(UINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(INTENTITY_TYPE_INFO);
    registry.register_type(INTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BOOLENTITY_TYPE_INFO);
    registry.register_type(BOOLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SPATIALENTITY_TYPE_INFO);
    registry.register_type(SPATIALENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTVEC4ENTITY_TYPE_INFO);
    registry.register_type(SELECTVEC4ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTVEC3ENTITY_TYPE_INFO);
    registry.register_type(SELECTVEC3ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTVEC2ENTITY_TYPE_INFO);
    registry.register_type(SELECTVEC2ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTTRANSFORMENTITY_TYPE_INFO);
    registry.register_type(SELECTTRANSFORMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTBOOLENTITY_TYPE_INFO);
    registry.register_type(SELECTBOOLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTSTRINGENTITY_TYPE_INFO);
    registry.register_type(SELECTSTRINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTINT64ENTITY_TYPE_INFO);
    registry.register_type(SELECTINT64ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTINTENTITY_TYPE_INFO);
    registry.register_type(SELECTINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTFLOATENTITY_TYPE_INFO);
    registry.register_type(SELECTFLOATENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SCRIPTENTITY_TYPE_INFO);
    registry.register_type(SCRIPTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICCHANNELENTITY_TYPE_INFO);
    registry.register_type(SCHEMATICCHANNELENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PART_TYPE_INFO);
    registry.register_type(PART_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTAREATRIGGERENTITY_TYPE_INFO);
    registry.register_type(OBJECTAREATRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONTOINTENTITY_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONTOINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMSPACEENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMSPACEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FLOATSELECTENTITY_TYPE_INFO);
    registry.register_type(FLOATSELECTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FLOATCURVEENTITY_TYPE_INFO);
    registry.register_type(FLOATCURVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SETTINGENTITY_TYPE_INFO);
    registry.register_type(SETTINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(INTEGRATORORDIFFERENTIATORENTITY_TYPE_INFO);
    registry.register_type(INTEGRATORORDIFFERENTIATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PROFILEENTITY_TYPE_INFO);
    registry.register_type(PROFILEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPAREENTITY_TYPE_INFO);
    registry.register_type(COMPAREENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RUNNINGAVERAGEENTITY_TYPE_INFO);
    registry.register_type(RUNNINGAVERAGEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ABSENTITY_TYPE_INFO);
    registry.register_type(ABSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VECTORMATHOPENTITY_TYPE_INFO);
    registry.register_type(VECTORMATHOPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(MATHOPENTITY_TYPE_INFO);
    registry.register_type(MATHOPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYCASTENTITY_TYPE_INFO);
    registry.register_type(PROPERTYCASTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BOOLTOEVENTENTITY_TYPE_INFO);
    registry.register_type(BOOLTOEVENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SWITCHPROPERTYSTRINGENTITY_TYPE_INFO);
    registry.register_type(SWITCHPROPERTYSTRINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYGATEENTITY_TYPE_INFO);
    registry.register_type(PROPERTYGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(EVENTGATEENTITY_TYPE_INFO);
    registry.register_type(EVENTGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(STOPWATCHENTITY_TYPE_INFO);
    registry.register_type(STOPWATCHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMTOROTATIONENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMTOROTATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMMODIFIERENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMMODIFIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ROTATIONTRANSFORMBUILDERENTITY_TYPE_INFO);
    registry.register_type(ROTATIONTRANSFORMBUILDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SCALETRANSFORMBUILDERENTITY_TYPE_INFO);
    registry.register_type(SCALETRANSFORMBUILDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMBUILDERENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMBUILDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VECBUILDERENTITY_TYPE_INFO);
    registry.register_type(VECBUILDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC4BUILDERENTITY_TYPE_INFO);
    registry.register_type(VEC4BUILDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC3BUILDERENTITY_TYPE_INFO);
    registry.register_type(VEC3BUILDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC2BUILDERENTITY_TYPE_INFO);
    registry.register_type(VEC2BUILDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VECSPLITTERENTITY_TYPE_INFO);
    registry.register_type(VECSPLITTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC4SPLITTERENTITY_TYPE_INFO);
    registry.register_type(VEC4SPLITTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC3SPLITTERENTITY_TYPE_INFO);
    registry.register_type(VEC3SPLITTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC2SPLITTERENTITY_TYPE_INFO);
    registry.register_type(VEC2SPLITTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(EULERTRANSFORMENTITY_TYPE_INFO);
    registry.register_type(EULERTRANSFORMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(EULERTRANSFORMSPLITTERENTITY_TYPE_INFO);
    registry.register_type(EULERTRANSFORMSPLITTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMSPLITTERENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMSPLITTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMSELECTORENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMSELECTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMBLENDENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMBLENDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLEDOFPARAMCONVERTERENTITY_TYPE_INFO);
    registry.register_type(SIMPLEDOFPARAMCONVERTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SPRITEDOFPARAMCONVERTERENTITY_TYPE_INFO);
    registry.register_type(SPRITEDOFPARAMCONVERTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMMULTIPLIERENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMMULTIPLIERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMHUBENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMHUBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(STRINGHUBENTITY_TYPE_INFO);
    registry.register_type(STRINGHUBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC4HUBENTITY_TYPE_INFO);
    registry.register_type(VEC4HUBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC3HUBENTITY_TYPE_INFO);
    registry.register_type(VEC3HUBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC2HUBENTITY_TYPE_INFO);
    registry.register_type(VEC2HUBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FLOATHUBENTITY_TYPE_INFO);
    registry.register_type(FLOATHUBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(INT64HUBENTITY_TYPE_INFO);
    registry.register_type(INT64HUBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(INTHUBENTITY_TYPE_INFO);
    registry.register_type(INTHUBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BOOLHUBENTITY_TYPE_INFO);
    registry.register_type(BOOLHUBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONSWITCHENTITY_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONSWITCHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONMULTISWITCHENTITY_TYPE_INFO);
    registry.register_type(OBJECTVARIATIONMULTISWITCHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMDELAYENTITY_TYPE_INFO);
    registry.register_type(RANDOMDELAYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DELAYENTITY_TYPE_INFO);
    registry.register_type(DELAYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTEVENTENTITY_TYPE_INFO);
    registry.register_type(SELECTEVENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(EVENTORDERGATEENTITY_TYPE_INFO);
    registry.register_type(EVENTORDERGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(EVENTANDGATEENTITY_TYPE_INFO);
    registry.register_type(EVENTANDGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(EVENTSWITCHENTITY_TYPE_INFO);
    registry.register_type(EVENTSWITCHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(UNIQUEIDENTITY_TYPE_INFO);
    registry.register_type(UNIQUEIDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMINTENTITY_TYPE_INFO);
    registry.register_type(RANDOMINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMFLOATENTITY_TYPE_INFO);
    registry.register_type(RANDOMFLOATENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMMULTIEVENTENTITY_TYPE_INFO);
    registry.register_type(RANDOMMULTIEVENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RANDOMEVENTENTITY_TYPE_INFO);
    registry.register_type(RANDOMEVENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOCATORENTITY_TYPE_INFO);
    registry.register_type(LOCATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COUNTDOWNENTITY_TYPE_INFO);
    registry.register_type(COUNTDOWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(MATHENTITY_TYPE_INFO);
    registry.register_type(MATHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OR4ENTITY_TYPE_INFO);
    registry.register_type(OR4ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(XORENTITY_TYPE_INFO);
    registry.register_type(XORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ORENTITY_TYPE_INFO);
    registry.register_type(ORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BITWISEANDENTITY_TYPE_INFO);
    registry.register_type(BITWISEANDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ANDENTITY_TYPE_INFO);
    registry.register_type(ANDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(NOTENTITY_TYPE_INFO);
    registry.register_type(NOTENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WorldPartInclusion {
}

pub const WORLDPARTINCLUSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartInclusion",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDINCLUSION_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WORLDPARTINCLUSION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldPartInclusion {
    fn type_info() -> &'static TypeInfo {
        WORLDPARTINCLUSION_TYPE_INFO
    }
}


pub const WORLDPARTINCLUSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartInclusion-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WorldPartInclusion-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WorldPartInclusionCriterion {
}

pub const WORLDPARTINCLUSIONCRITERION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartInclusionCriterion",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDINCLUSIONCRITERION_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WORLDPARTINCLUSIONCRITERION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldPartInclusionCriterion {
    fn type_info() -> &'static TypeInfo {
        WORLDPARTINCLUSIONCRITERION_TYPE_INFO
    }
}


pub const WORLDPARTINCLUSIONCRITERION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartInclusionCriterion-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WorldPartInclusionCriterion-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WorldPartInclusionSetting {
}

pub const WORLDPARTINCLUSIONSETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartInclusionSetting",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDINCLUSIONSETTING_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WORLDPARTINCLUSIONSETTING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldPartInclusionSetting {
    fn type_info() -> &'static TypeInfo {
        WORLDPARTINCLUSIONSETTING_TYPE_INFO
    }
}


pub const WORLDPARTINCLUSIONSETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartInclusionSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WorldPartInclusionSetting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WorldPartInclusionSettings {
}

pub const WORLDPARTINCLUSIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartInclusionSettings",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDINCLUSIONSETTINGS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WORLDPARTINCLUSIONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldPartInclusionSettings {
    fn type_info() -> &'static TypeInfo {
        WORLDPARTINCLUSIONSETTINGS_TYPE_INFO
    }
}


pub const WORLDPARTINCLUSIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartInclusionSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WorldPartInclusionSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssemblyDescriptorData {
}

pub const ASSEMBLYDESCRIPTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssemblyDescriptorData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(INTERFACEDESCRIPTORDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ASSEMBLYDESCRIPTORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AssemblyDescriptorData {
    fn type_info() -> &'static TypeInfo {
        ASSEMBLYDESCRIPTORDATA_TYPE_INFO
    }
}


pub const ASSEMBLYDESCRIPTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssemblyDescriptorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AssemblyDescriptorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WorldObjectListData {
}

pub const WORLDOBJECTLISTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldObjectListData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WORLDOBJECTLISTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldObjectListData {
    fn type_info() -> &'static TypeInfo {
        WORLDOBJECTLISTDATA_TYPE_INFO
    }
}


pub const WORLDOBJECTLISTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldObjectListData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WorldObjectListData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WorldPartReferenceObjectData {
}

pub const WORLDPARTREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LAYERREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WORLDPARTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WorldPartReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        WORLDPARTREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const WORLDPARTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WorldPartReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ReadOnlyLayerReferenceObjectData {
}

pub const READONLYLAYERREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadOnlyLayerReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LAYERREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(READONLYLAYERREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ReadOnlyLayerReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        READONLYLAYERREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const READONLYLAYERREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadOnlyLayerReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReadOnlyLayerReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LayerReferenceObjectData {
}

pub const LAYERREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(REFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LAYERREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LayerReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        LAYERREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const LAYERREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LayerReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SubWorldReferenceObjectData {
    pub bundle_name: String,
    pub preloaded_bundle_names: Vec<String>,
    pub bundle_heap: BundleHeapInfo,
    pub inclusion_settings: SubWorldInclusionSettings,
    pub auto_load: bool,
    pub is_detached_sub_level: bool,
    pub is_win32_sub_level: bool,
    pub is_gen4a_sub_level: bool,
    pub is_gen4b_sub_level: bool,
    pub is_i_o_s_sub_level: bool,
    pub is_android_sub_level: bool,
    pub is_o_s_x_sub_level: bool,
    pub is_linux_sub_level: bool,
    pub on_level_load_fire_on_stream_in: bool,
    pub use_peer_filtering: bool,
    pub parents: Vec<SharedBundleReference>,
}

pub const SUBWORLDREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(REFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BundleName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, bundle_name),
            },
            FieldInfoData {
                name: "PreloadedBundleNames",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, preloaded_bundle_names),
            },
            FieldInfoData {
                name: "BundleHeap",
                flags: MemberInfoFlags::new(0),
                field_type: BUNDLEHEAPINFO_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, bundle_heap),
            },
            FieldInfoData {
                name: "InclusionSettings",
                flags: MemberInfoFlags::new(0),
                field_type: SUBWORLDINCLUSIONSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, inclusion_settings),
            },
            FieldInfoData {
                name: "AutoLoad",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, auto_load),
            },
            FieldInfoData {
                name: "IsDetachedSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, is_detached_sub_level),
            },
            FieldInfoData {
                name: "IsWin32SubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, is_win32_sub_level),
            },
            FieldInfoData {
                name: "IsGen4aSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, is_gen4a_sub_level),
            },
            FieldInfoData {
                name: "IsGen4bSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, is_gen4b_sub_level),
            },
            FieldInfoData {
                name: "IsIOSSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, is_i_o_s_sub_level),
            },
            FieldInfoData {
                name: "IsAndroidSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, is_android_sub_level),
            },
            FieldInfoData {
                name: "IsOSXSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, is_o_s_x_sub_level),
            },
            FieldInfoData {
                name: "IsLinuxSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, is_linux_sub_level),
            },
            FieldInfoData {
                name: "OnLevelLoadFireOnStreamIn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, on_level_load_fire_on_stream_in),
            },
            FieldInfoData {
                name: "UsePeerFiltering",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, use_peer_filtering),
            },
            FieldInfoData {
                name: "Parents",
                flags: MemberInfoFlags::new(144),
                field_type: SHAREDBUNDLEREFERENCE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SubWorldReferenceObjectData, parents),
            },
        ],
    }),
    array_type: Some(SUBWORLDREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SubWorldReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        SUBWORLDREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const SUBWORLDREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SubWorldReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DetachedSubWorldData {
    pub reset_destruction_state: bool,
}

pub const DETACHEDSUBWORLDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DetachedSubWorldData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WORLDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ResetDestructionState",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DetachedSubWorldData, reset_destruction_state),
            },
        ],
    }),
    array_type: Some(DETACHEDSUBWORLDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DetachedSubWorldData {
    fn type_info() -> &'static TypeInfo {
        DETACHEDSUBWORLDDATA_TYPE_INFO
    }
}


pub const DETACHEDSUBWORLDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DetachedSubWorldData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DetachedSubWorldData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WorldData {
    pub world_size_x_z: f32,
    pub world_size_y: f32,
    pub is_procedural_level: bool,
    pub is_dedicated_server_level: bool,
}

pub const WORLDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WorldSizeXZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldData, world_size_x_z),
            },
            FieldInfoData {
                name: "WorldSizeY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldData, world_size_y),
            },
            FieldInfoData {
                name: "IsProceduralLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WorldData, is_procedural_level),
            },
            FieldInfoData {
                name: "IsDedicatedServerLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WorldData, is_dedicated_server_level),
            },
        ],
    }),
    array_type: Some(WORLDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldData {
    fn type_info() -> &'static TypeInfo {
        WORLDDATA_TYPE_INFO
    }
}


pub const WORLDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WorldData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SubWorldData {
    pub runtime_material_grid: MaterialGridData,
    pub is_win32_sub_level: bool,
    pub is_gen4a_sub_level: bool,
    pub is_gen4b_sub_level: bool,
    pub is_android_sub_level: bool,
    pub is_i_o_s_sub_level: bool,
    pub is_o_s_x_sub_level: bool,
    pub is_linux_sub_level: bool,
    pub persistence_setting: BlueprintPersistenceSetting,
    pub auto_asset_collector: AutoAssetCollector,
    pub components: Vec<SubWorldDataComponent>,
}

pub const SUBWORLDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALPREFABBLUEPRINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RuntimeMaterialGrid",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALGRIDDATA_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, runtime_material_grid),
            },
            FieldInfoData {
                name: "IsWin32SubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, is_win32_sub_level),
            },
            FieldInfoData {
                name: "IsGen4aSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, is_gen4a_sub_level),
            },
            FieldInfoData {
                name: "IsGen4bSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, is_gen4b_sub_level),
            },
            FieldInfoData {
                name: "IsAndroidSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, is_android_sub_level),
            },
            FieldInfoData {
                name: "IsIOSSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, is_i_o_s_sub_level),
            },
            FieldInfoData {
                name: "IsOSXSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, is_o_s_x_sub_level),
            },
            FieldInfoData {
                name: "IsLinuxSubLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, is_linux_sub_level),
            },
            FieldInfoData {
                name: "PersistenceSetting",
                flags: MemberInfoFlags::new(0),
                field_type: BLUEPRINTPERSISTENCESETTING_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, persistence_setting),
            },
            FieldInfoData {
                name: "AutoAssetCollector",
                flags: MemberInfoFlags::new(0),
                field_type: AUTOASSETCOLLECTOR_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, auto_asset_collector),
            },
            FieldInfoData {
                name: "Components",
                flags: MemberInfoFlags::new(144),
                field_type: SUBWORLDDATACOMPONENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SubWorldData, components),
            },
        ],
    }),
    array_type: Some(SUBWORLDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubWorldData {
    fn type_info() -> &'static TypeInfo {
        SUBWORLDDATA_TYPE_INFO
    }
}


pub const SUBWORLDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SubWorldData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubWorldDataComponent {
}

pub const SUBWORLDDATACOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldDataComponent",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUBWORLDDATACOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubWorldDataComponent {
    fn type_info() -> &'static TypeInfo {
        SUBWORLDDATACOMPONENT_TYPE_INFO
    }
}


pub const SUBWORLDDATACOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldDataComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SubWorldDataComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WorldPartData {
}

pub const WORLDPARTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LAYERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WORLDPARTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldPartData {
    fn type_info() -> &'static TypeInfo {
        WORLDPARTDATA_TYPE_INFO
    }
}


pub const WORLDPARTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldPartData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WorldPartData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LayerData {
    pub enabled: bool,
    pub hack_to_solve_real_time_tweaking_issue: super::core::Guid,
}

pub const LAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PREFABBLUEPRINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LayerData, enabled),
            },
            FieldInfoData {
                name: "HackToSolveRealTimeTweakingIssue",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(LayerData, hack_to_solve_real_time_tweaking_issue),
            },
        ],
    }),
    array_type: Some(LAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LayerData {
    fn type_info() -> &'static TypeInfo {
        LAYERDATA_TYPE_INFO
    }
}


pub const LAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubWorldInclusionSettings {
    pub settings: Vec<SubWorldInclusionSetting>,
}

pub const SUBWORLDINCLUSIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldInclusionSettings",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Settings",
                flags: MemberInfoFlags::new(144),
                field_type: SUBWORLDINCLUSIONSETTING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SubWorldInclusionSettings, settings),
            },
        ],
    }),
    array_type: Some(SUBWORLDINCLUSIONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubWorldInclusionSettings {
    fn type_info() -> &'static TypeInfo {
        SUBWORLDINCLUSIONSETTINGS_TYPE_INFO
    }
}


pub const SUBWORLDINCLUSIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldInclusionSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SubWorldInclusionSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubWorldInclusionSetting {
    pub criterion: SubWorldInclusionCriterion,
    pub enabled_options: Vec<String>,
}

pub const SUBWORLDINCLUSIONSETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldInclusionSetting",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Criterion",
                flags: MemberInfoFlags::new(0),
                field_type: SUBWORLDINCLUSIONCRITERION_TYPE_INFO,
                rust_offset: offset_of!(SubWorldInclusionSetting, criterion),
            },
            FieldInfoData {
                name: "EnabledOptions",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SubWorldInclusionSetting, enabled_options),
            },
        ],
    }),
    array_type: Some(SUBWORLDINCLUSIONSETTING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubWorldInclusionSetting {
    fn type_info() -> &'static TypeInfo {
        SUBWORLDINCLUSIONSETTING_TYPE_INFO
    }
}


pub const SUBWORLDINCLUSIONSETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldInclusionSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SubWorldInclusionSetting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubWorldInclusion {
    pub criteria: Vec<SubWorldInclusionCriterion>,
}

pub const SUBWORLDINCLUSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldInclusion",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Criteria",
                flags: MemberInfoFlags::new(144),
                field_type: SUBWORLDINCLUSIONCRITERION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SubWorldInclusion, criteria),
            },
        ],
    }),
    array_type: Some(SUBWORLDINCLUSION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubWorldInclusion {
    fn type_info() -> &'static TypeInfo {
        SUBWORLDINCLUSION_TYPE_INFO
    }
}


pub const SUBWORLDINCLUSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldInclusion-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SubWorldInclusion-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubWorldInclusionCriterion {
    pub name: String,
    pub options: Vec<String>,
}

pub const SUBWORLDINCLUSIONCRITERION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldInclusionCriterion",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SubWorldInclusionCriterion, name),
            },
            FieldInfoData {
                name: "Options",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SubWorldInclusionCriterion, options),
            },
        ],
    }),
    array_type: Some(SUBWORLDINCLUSIONCRITERION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubWorldInclusionCriterion {
    fn type_info() -> &'static TypeInfo {
        SUBWORLDINCLUSIONCRITERION_TYPE_INFO
    }
}


pub const SUBWORLDINCLUSIONCRITERION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubWorldInclusionCriterion-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SubWorldInclusionCriterion-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelSetup {
    pub name: String,
    pub inclusion_options: Vec<LevelSetupOption>,
    pub difficulty_index: u32,
    pub start_point: String,
    pub is_save_game: bool,
    pub has_persistent_save: bool,
    pub force_reload_resources: bool,
    pub level_manager_initial_level: String,
    pub level_manager_start_point: String,
}

pub const LEVELSETUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelSetup",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelSetup, name),
            },
            FieldInfoData {
                name: "InclusionOptions",
                flags: MemberInfoFlags::new(144),
                field_type: LEVELSETUPOPTION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LevelSetup, inclusion_options),
            },
            FieldInfoData {
                name: "DifficultyIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LevelSetup, difficulty_index),
            },
            FieldInfoData {
                name: "StartPoint",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelSetup, start_point),
            },
            FieldInfoData {
                name: "IsSaveGame",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelSetup, is_save_game),
            },
            FieldInfoData {
                name: "HasPersistentSave",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelSetup, has_persistent_save),
            },
            FieldInfoData {
                name: "ForceReloadResources",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LevelSetup, force_reload_resources),
            },
            FieldInfoData {
                name: "LevelManagerInitialLevel",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelSetup, level_manager_initial_level),
            },
            FieldInfoData {
                name: "LevelManagerStartPoint",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelSetup, level_manager_start_point),
            },
        ],
    }),
    array_type: Some(LEVELSETUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelSetup {
    fn type_info() -> &'static TypeInfo {
        LEVELSETUP_TYPE_INFO
    }
}


pub const LEVELSETUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelSetup-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LevelSetup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelSetupOption {
    pub criterion: String,
    pub value: String,
}

pub const LEVELSETUPOPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelSetupOption",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Criterion",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelSetupOption, criterion),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LevelSetupOption, value),
            },
        ],
    }),
    array_type: Some(LEVELSETUPOPTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelSetupOption {
    fn type_info() -> &'static TypeInfo {
        LEVELSETUPOPTION_TYPE_INFO
    }
}


pub const LEVELSETUPOPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelSetupOption-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LevelSetupOption-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreateSchematicsInstanceEntity {
}

pub const CREATESCHEMATICSINSTANCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreateSchematicsInstanceEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATESCHEMATICSINSTANCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreateSchematicsInstanceEntity {
    fn type_info() -> &'static TypeInfo {
        CREATESCHEMATICSINSTANCEENTITY_TYPE_INFO
    }
}


pub const CREATESCHEMATICSINSTANCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreateSchematicsInstanceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CreateSchematicsInstanceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GetEntityBusEntity {
}

pub const GETENTITYBUSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetEntityBusEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GETENTITYBUSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GetEntityBusEntity {
    fn type_info() -> &'static TypeInfo {
        GETENTITYBUSENTITY_TYPE_INFO
    }
}


pub const GETENTITYBUSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetEntityBusEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("GetEntityBusEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GetEntityEntity {
}

pub const GETENTITYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetEntityEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GETENTITYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GetEntityEntity {
    fn type_info() -> &'static TypeInfo {
        GETENTITYENTITY_TYPE_INFO
    }
}


pub const GETENTITYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetEntityEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("GetEntityEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CallFunctionEntity {
}

pub const CALLFUNCTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CallFunctionEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CALLFUNCTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CallFunctionEntity {
    fn type_info() -> &'static TypeInfo {
        CALLFUNCTIONENTITY_TYPE_INFO
    }
}


pub const CALLFUNCTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CallFunctionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CallFunctionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityTransformSpace {
}

pub const ENTITYTRANSFORMSPACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTransformSpace",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ENTITYTRANSFORMSPACE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityTransformSpace {
    fn type_info() -> &'static TypeInfo {
        ENTITYTRANSFORMSPACE_TYPE_INFO
    }
}


pub const ENTITYTRANSFORMSPACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTransformSpace-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntityTransformSpace-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entity {
}

pub const ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Entity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYBUSPEER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Entity {
    fn type_info() -> &'static TypeInfo {
        ENTITY_TYPE_INFO
    }
}


pub const ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ComponentEntity {
}

pub const COMPONENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPONENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ComponentEntity {
    fn type_info() -> &'static TypeInfo {
        COMPONENTENTITY_TYPE_INFO
    }
}


pub const COMPONENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ComponentEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Component {
}

pub const COMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Component",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYBUSPEER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Component {
    fn type_info() -> &'static TypeInfo {
        COMPONENT_TYPE_INFO
    }
}


pub const COMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Component-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Component-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareAxisAlignedBoxEntity {
}

pub const COMPAREAXISALIGNEDBOXENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareAxisAlignedBoxEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREAXISALIGNEDBOXENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareAxisAlignedBoxEntity {
    fn type_info() -> &'static TypeInfo {
        COMPAREAXISALIGNEDBOXENTITY_TYPE_INFO
    }
}


pub const COMPAREAXISALIGNEDBOXENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareAxisAlignedBoxEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareAxisAlignedBoxEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareUintRangeEntity {
}

pub const COMPAREUINTRANGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareUintRangeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREUINTRANGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareUintRangeEntity {
    fn type_info() -> &'static TypeInfo {
        COMPAREUINTRANGEENTITY_TYPE_INFO
    }
}


pub const COMPAREUINTRANGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareUintRangeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareUintRangeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareIntRangeEntity {
}

pub const COMPAREINTRANGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareIntRangeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREINTRANGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareIntRangeEntity {
    fn type_info() -> &'static TypeInfo {
        COMPAREINTRANGEENTITY_TYPE_INFO
    }
}


pub const COMPAREINTRANGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareIntRangeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareIntRangeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareVec4Entity {
}

pub const COMPAREVEC4ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareVec4Entity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREVEC4ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareVec4Entity {
    fn type_info() -> &'static TypeInfo {
        COMPAREVEC4ENTITY_TYPE_INFO
    }
}


pub const COMPAREVEC4ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareVec4Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareVec4Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareVec3Entity {
}

pub const COMPAREVEC3ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareVec3Entity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREVEC3ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareVec3Entity {
    fn type_info() -> &'static TypeInfo {
        COMPAREVEC3ENTITY_TYPE_INFO
    }
}


pub const COMPAREVEC3ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareVec3Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareVec3Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareFloatRangeEntity {
}

pub const COMPAREFLOATRANGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareFloatRangeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREFLOATRANGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareFloatRangeEntity {
    fn type_info() -> &'static TypeInfo {
        COMPAREFLOATRANGEENTITY_TYPE_INFO
    }
}


pub const COMPAREFLOATRANGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareFloatRangeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareFloatRangeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareStringEntity {
}

pub const COMPARESTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareStringEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPARESTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareStringEntity {
    fn type_info() -> &'static TypeInfo {
        COMPARESTRINGENTITY_TYPE_INFO
    }
}


pub const COMPARESTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareStringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareStringEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareFloatEntity {
}

pub const COMPAREFLOATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareFloatEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREFLOATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareFloatEntity {
    fn type_info() -> &'static TypeInfo {
        COMPAREFLOATENTITY_TYPE_INFO
    }
}


pub const COMPAREFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareFloatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareFloatEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareUintEntity {
}

pub const COMPAREUINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareUintEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREUINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareUintEntity {
    fn type_info() -> &'static TypeInfo {
        COMPAREUINTENTITY_TYPE_INFO
    }
}


pub const COMPAREUINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareUintEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareUintEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareIntEntity {
}

pub const COMPAREINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareIntEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareIntEntity {
    fn type_info() -> &'static TypeInfo {
        COMPAREINTENTITY_TYPE_INFO
    }
}


pub const COMPAREINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareIntEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareBoolEntity {
}

pub const COMPAREBOOLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareBoolEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREBOOLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareBoolEntity {
    fn type_info() -> &'static TypeInfo {
        COMPAREBOOLENTITY_TYPE_INFO
    }
}


pub const COMPAREBOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareBoolEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareBoolEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDilationEntity {
}

pub const SERVERDILATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDilationEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDILATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDilationEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERDILATIONENTITY_TYPE_INFO
    }
}


pub const SERVERDILATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDilationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ServerDilationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBalancedDilationEntity {
}

pub const CLIENTBALANCEDDILATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBalancedDilationEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBALANCEDDILATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBalancedDilationEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTBALANCEDDILATIONENTITY_TYPE_INFO
    }
}


pub const CLIENTBALANCEDDILATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBalancedDilationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ClientBalancedDilationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AnimTrackData {
}

pub const ANIMTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ANIMTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AnimTrackData {
    fn type_info() -> &'static TypeInfo {
        ANIMTRACKDATA_TYPE_INFO
    }
}


pub const ANIMTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AnimTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SequenceEntity {
}

pub const SEQUENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SequenceEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SEQUENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SequenceEntity {
    fn type_info() -> &'static TypeInfo {
        SEQUENCEENTITY_TYPE_INFO
    }
}


pub const SEQUENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SequenceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SequenceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringBuilderEntity {
}

pub const STRINGBUILDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringBuilderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STRINGBUILDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StringBuilderEntity {
    fn type_info() -> &'static TypeInfo {
        STRINGBUILDERENTITY_TYPE_INFO
    }
}


pub const STRINGBUILDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringBuilderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("StringBuilderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec4ChangedEntity {
}

pub const VEC4CHANGEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4ChangedEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC4CHANGEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec4ChangedEntity {
    fn type_info() -> &'static TypeInfo {
        VEC4CHANGEDENTITY_TYPE_INFO
    }
}


pub const VEC4CHANGEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4ChangedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec4ChangedEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec3ChangedEntity {
}

pub const VEC3CHANGEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3ChangedEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC3CHANGEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec3ChangedEntity {
    fn type_info() -> &'static TypeInfo {
        VEC3CHANGEDENTITY_TYPE_INFO
    }
}


pub const VEC3CHANGEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3ChangedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec3ChangedEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec2ChangedEntity {
}

pub const VEC2CHANGEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2ChangedEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC2CHANGEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec2ChangedEntity {
    fn type_info() -> &'static TypeInfo {
        VEC2CHANGEDENTITY_TYPE_INFO
    }
}


pub const VEC2CHANGEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2ChangedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec2ChangedEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntChangedEntity {
}

pub const INTCHANGEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntChangedEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INTCHANGEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntChangedEntity {
    fn type_info() -> &'static TypeInfo {
        INTCHANGEDENTITY_TYPE_INFO
    }
}


pub const INTCHANGEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntChangedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntChangedEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatChangedEntity {
}

pub const FLOATCHANGEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatChangedEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATCHANGEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatChangedEntity {
    fn type_info() -> &'static TypeInfo {
        FLOATCHANGEDENTITY_TYPE_INFO
    }
}


pub const FLOATCHANGEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatChangedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatChangedEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolChangedEntity {
}

pub const BOOLCHANGEDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolChangedEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BOOLCHANGEDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BoolChangedEntity {
    fn type_info() -> &'static TypeInfo {
        BOOLCHANGEDENTITY_TYPE_INFO
    }
}


pub const BOOLCHANGEDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolChangedEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoolChangedEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareTransformEntity {
}

pub const COMPARETRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareTransformEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPARETRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareTransformEntity {
    fn type_info() -> &'static TypeInfo {
        COMPARETRANSFORMENTITY_TYPE_INFO
    }
}


pub const COMPARETRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareTransformEntity-Array"),
    array_type: None,
    alignment: 8,
};



pub const GETSCHEMATICSINSTANCE_SCHEMATICSINSTANCE_ENTITY__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetSchematicsInstance(SchematicsInstance,Entity)",
    flags: MemberInfoFlags::new(793),
    module: "Entity",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const GETSCHEMATICSINSTANCE_SCHEMATICSINSTANCE_ENTITYBUS__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetSchematicsInstance(SchematicsInstance,EntityBus)",
    flags: MemberInfoFlags::new(793),
    module: "Entity",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const DESTROYENTITY_ENTITY__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestroyEntity(Entity)",
    flags: MemberInfoFlags::new(793),
    module: "Entity",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const CREATEENTITY_ENTITY_ENTITYBUS_OBJECTBLUEPRINT_LINEARTRANSFORM__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreateEntity(Entity,EntityBus,ObjectBlueprint,LinearTransform)",
    flags: MemberInfoFlags::new(793),
    module: "Entity",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const CASTTOSPATIAL_SPATIALENTITY_ENTITY__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CastToSpatial(SpatialEntity,Entity)",
    flags: MemberInfoFlags::new(793),
    module: "Entity",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const GETTRANSFORM_SPATIALENTITY_LINEARTRANSFORM__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetTransform(SpatialEntity,LinearTransform)",
    flags: MemberInfoFlags::new(793),
    module: "Entity",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const SETTRANSFORM_SPATIALENTITY_LINEARTRANSFORM__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetTransform(SpatialEntity,LinearTransform)",
    flags: MemberInfoFlags::new(793),
    module: "Entity",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UpdatePassMask {
    #[default]
    EmptyUpdateMask = 0,
    PrePhysicsUpdateMask = 1,
    PostPhysicsUpdateMask = 2,
    PreAnimationUpdateMask = 4,
    PostAnimationUpdateMask = 8,
    PostFrameUpdateMask = 16,
    UpdatePassMaskCount = 5,
}

pub const UPDATEPASSMASK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdatePassMask",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(UPDATEPASSMASK_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UpdatePassMask {
    fn type_info() -> &'static TypeInfo {
        UPDATEPASSMASK_TYPE_INFO
    }
}


pub const UPDATEPASSMASK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdatePassMask-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("UpdatePassMask-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UpdatePass {
    #[default]
    UpdatePass_PreSim = 0,
    UpdatePass_PostSim = 1,
    UpdatePass_PostFrame = 2,
    UpdatePass_PreInput = 3,
    UpdatePass_PreFrame = 4,
    UpdatePass_PreAnimation = 5,
    UpdatePass_PostAnimation = 6,
    UpdatePass_Count = 9,
}

pub const UPDATEPASS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdatePass",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(UPDATEPASS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UpdatePass {
    fn type_info() -> &'static TypeInfo {
        UPDATEPASS_TYPE_INFO
    }
}


pub const UPDATEPASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UpdatePass-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("UpdatePass-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameDataContainerAsset {
    pub data: super::core::GameDataContainer,
}

pub const GAMEDATACONTAINERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameDataContainerAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEDATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(GameDataContainerAsset, data),
            },
        ],
    }),
    array_type: Some(GAMEDATACONTAINERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameDataContainerAsset {
    fn type_info() -> &'static TypeInfo {
        GAMEDATACONTAINERASSET_TYPE_INFO
    }
}


pub const GAMEDATACONTAINERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameDataContainerAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("GameDataContainerAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataContainerAsset {
    pub data: super::core::DataContainer,
}

pub const DATACONTAINERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(DataContainerAsset, data),
            },
        ],
    }),
    array_type: Some(DATACONTAINERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataContainerAsset {
    fn type_info() -> &'static TypeInfo {
        DATACONTAINERASSET_TYPE_INFO
    }
}


pub const DATACONTAINERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DataContainerAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StaticInstancingData {
    pub transforms: Vec<super::core::LinearTransform>,
    pub mesh_ids: Vec<i32>,
}

pub const STATICINSTANCINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticInstancingData",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transforms",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARTRANSFORM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StaticInstancingData, transforms),
            },
            FieldInfoData {
                name: "MeshIds",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StaticInstancingData, mesh_ids),
            },
        ],
    }),
    array_type: Some(STATICINSTANCINGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticInstancingData {
    fn type_info() -> &'static TypeInfo {
        STATICINSTANCINGDATA_TYPE_INFO
    }
}


pub const STATICINSTANCINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticInstancingData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("StaticInstancingData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PartData {
    pub part_states: Vec<PartState>,
}

pub const PARTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PartStates",
                flags: MemberInfoFlags::new(144),
                field_type: PARTSTATE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PartData, part_states),
            },
        ],
    }),
    array_type: Some(PARTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PartData {
    fn type_info() -> &'static TypeInfo {
        PARTDATA_TYPE_INFO
    }
}


pub const PARTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PartData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartState {
    pub part_index: u32,
    pub objects: Vec<GameObjectData>,
}

pub const PARTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartState",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PartState, part_index),
            },
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PartState, objects),
            },
        ],
    }),
    array_type: Some(PARTSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PartState {
    fn type_info() -> &'static TypeInfo {
        PARTSTATE_TYPE_INFO
    }
}


pub const PARTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PartState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BoneComponentData {
}

pub const BONECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BONECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoneComponentData {
    fn type_info() -> &'static TypeInfo {
        BONECOMPONENTDATA_TYPE_INFO
    }
}


pub const BONECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoneComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GameComponentData {
}

pub const GAMECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GameComponentData {
    fn type_info() -> &'static TypeInfo {
        GAMECOMPONENTDATA_TYPE_INFO
    }
}


pub const GAMECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("GameComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ComponentData {
    pub transform: super::core::LinearTransform,
    pub components: Vec<GameObjectData>,
    pub client_index: u8,
    pub server_index: u8,
    pub excluded: bool,
}

pub const COMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(ComponentData, transform),
            },
            FieldInfoData {
                name: "Components",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ComponentData, components),
            },
            FieldInfoData {
                name: "ClientIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ComponentData, client_index),
            },
            FieldInfoData {
                name: "ServerIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ComponentData, server_index),
            },
            FieldInfoData {
                name: "Excluded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ComponentData, excluded),
            },
        ],
    }),
    array_type: Some(COMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ComponentData {
    fn type_info() -> &'static TypeInfo {
        COMPONENTDATA_TYPE_INFO
    }
}


pub const COMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GameComponentEntityData {
    pub enabled: bool,
}

pub const GAMECOMPONENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponentEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameComponentEntityData, enabled),
            },
        ],
    }),
    array_type: Some(GAMECOMPONENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameComponentEntityData {
    fn type_info() -> &'static TypeInfo {
        GAMECOMPONENTENTITYDATA_TYPE_INFO
    }
}


pub const GAMECOMPONENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameComponentEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("GameComponentEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ComponentEntityData {
    pub components: Vec<GameObjectData>,
    pub part_bounding_boxes: Vec<super::core::AxisAlignedBox>,
    pub client_runtime_component_count: u8,
    pub server_runtime_component_count: u8,
    pub client_runtime_transformation_count: u8,
    pub server_runtime_transformation_count: u8,
}

pub const COMPONENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Components",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ComponentEntityData, components),
            },
            FieldInfoData {
                name: "PartBoundingBoxes",
                flags: MemberInfoFlags::new(144),
                field_type: AXISALIGNEDBOX_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ComponentEntityData, part_bounding_boxes),
            },
            FieldInfoData {
                name: "ClientRuntimeComponentCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ComponentEntityData, client_runtime_component_count),
            },
            FieldInfoData {
                name: "ServerRuntimeComponentCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ComponentEntityData, server_runtime_component_count),
            },
            FieldInfoData {
                name: "ClientRuntimeTransformationCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ComponentEntityData, client_runtime_transformation_count),
            },
            FieldInfoData {
                name: "ServerRuntimeTransformationCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(ComponentEntityData, server_runtime_transformation_count),
            },
        ],
    }),
    array_type: Some(COMPONENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ComponentEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPONENTENTITYDATA_TYPE_INFO
    }
}


pub const COMPONENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComponentEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ComponentEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScriptEntityData {
    pub code: String,
    pub realm: super::core::Realm,
}

pub const SCRIPTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScriptEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ScriptEntityData, code),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ScriptEntityData, realm),
            },
        ],
    }),
    array_type: Some(SCRIPTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScriptEntityData {
    fn type_info() -> &'static TypeInfo {
        SCRIPTENTITYDATA_TYPE_INFO
    }
}


pub const SCRIPTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScriptEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ScriptEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpatialEntityData {
    pub transform: super::core::LinearTransform,
}

pub const SPATIALENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SpatialEntityData, transform),
            },
        ],
    }),
    array_type: Some(SPATIALENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SpatialEntityData {
    fn type_info() -> &'static TypeInfo {
        SPATIALENTITYDATA_TYPE_INFO
    }
}


pub const SPATIALENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SpatialEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityData {
}

pub const ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityData {
    fn type_info() -> &'static TypeInfo {
        ENTITYDATA_TYPE_INFO
    }
}


pub const ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EntityCreatorType {
    #[default]
    EntityCreatorType_Unknown = 0,
    EntityCreatorType_Level = 1,
    EntityCreatorType_Spawner = 2,
    EntityCreatorType_Owner = 3,
    EntityCreatorType_Ghost = 4,
}

pub const ENTITYCREATORTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityCreatorType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(ENTITYCREATORTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntityCreatorType {
    fn type_info() -> &'static TypeInfo {
        ENTITYCREATORTYPE_TYPE_INFO
    }
}


pub const ENTITYCREATORTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityCreatorType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntityCreatorType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameObjectData {
}

pub const GAMEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATABUSPEER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameObjectData {
    fn type_info() -> &'static TypeInfo {
        GAMEOBJECTDATA_TYPE_INFO
    }
}


pub const GAMEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("GameObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SubRealm {
    #[default]
    SubRealm_All = 0,
    SubRealm_LocalPlayer = 1,
    SubRealm_RemotePlayer = 2,
}

pub const SUBREALM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubRealm",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(SUBREALM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SubRealm {
    fn type_info() -> &'static TypeInfo {
        SUBREALM_TYPE_INFO
    }
}


pub const SUBREALM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubRealm-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SubRealm-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TimeShape {
    #[default]
    TimeShape_Step = 0,
    TimeShape_Sine = 1,
    TimeShape_SemiCircle = 2,
    TimeShape_Triangle = 3,
    TimeShape_Ramps = 4,
}

pub const TIMESHAPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeShape",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(TIMESHAPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TimeShape {
    fn type_info() -> &'static TypeInfo {
        TIMESHAPE_TYPE_INFO
    }
}


pub const TIMESHAPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeShape-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TimeShape-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DilationPriority {
    #[default]
    DilationPriority_Highest = 0,
    DilationPriority_High = 1,
    DilationPriority_Normal = 2,
    DilationPriority_Low = 3,
    DilationPriority_Lowest = 4,
}

pub const DILATIONPRIORITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DilationPriority",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(DILATIONPRIORITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DilationPriority {
    fn type_info() -> &'static TypeInfo {
        DILATIONPRIORITY_TYPE_INFO
    }
}


pub const DILATIONPRIORITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DilationPriority-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DilationPriority-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TimeDeltaType {
    #[default]
    TimeDeltaType_None = 0,
    TimeDeltaType_Player = 1,
    TimeDeltaType_World = 2,
    TimeDeltaType_Extra1 = 3,
    TimeDeltaType_Extra2 = 4,
    TimeDeltaType_Extra3 = 5,
    TimeDeltaType_Extra4 = 6,
    TimeDeltaType_Extra5 = 7,
    TimeDeltaType_Count = 8,
}

pub const TIMEDELTATYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeDeltaType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(TIMEDELTATYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TimeDeltaType {
    fn type_info() -> &'static TypeInfo {
        TIMEDELTATYPE_TYPE_INFO
    }
}


pub const TIMEDELTATYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeDeltaType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TimeDeltaType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicChannelEntityData {
    pub realm: super::core::Realm,
    pub channel: SchematicChannelAsset,
    pub input_properties: Vec<i32>,
    pub input_ref_properties: Vec<i32>,
    pub output_properties: Vec<i32>,
    pub output_ref_properties: Vec<i32>,
}

pub const SCHEMATICCHANNELENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicChannelEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SchematicChannelEntityData, realm),
            },
            FieldInfoData {
                name: "Channel",
                flags: MemberInfoFlags::new(0),
                field_type: SCHEMATICCHANNELASSET_TYPE_INFO,
                rust_offset: offset_of!(SchematicChannelEntityData, channel),
            },
            FieldInfoData {
                name: "InputProperties",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicChannelEntityData, input_properties),
            },
            FieldInfoData {
                name: "InputRefProperties",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicChannelEntityData, input_ref_properties),
            },
            FieldInfoData {
                name: "OutputProperties",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicChannelEntityData, output_properties),
            },
            FieldInfoData {
                name: "OutputRefProperties",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicChannelEntityData, output_ref_properties),
            },
        ],
    }),
    array_type: Some(SCHEMATICCHANNELENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicChannelEntityData {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICCHANNELENTITYDATA_TYPE_INFO
    }
}


pub const SCHEMATICCHANNELENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicChannelEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SchematicChannelEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicChannelAsset {
    pub events: Vec<EventChannel>,
    pub links: Vec<LinkChannel>,
    pub properties: Vec<PropertyChannel>,
}

pub const SCHEMATICCHANNELASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicChannelAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Events",
                flags: MemberInfoFlags::new(144),
                field_type: EVENTCHANNEL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicChannelAsset, events),
            },
            FieldInfoData {
                name: "Links",
                flags: MemberInfoFlags::new(144),
                field_type: LINKCHANNEL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicChannelAsset, links),
            },
            FieldInfoData {
                name: "Properties",
                flags: MemberInfoFlags::new(144),
                field_type: PROPERTYCHANNEL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SchematicChannelAsset, properties),
            },
        ],
    }),
    array_type: Some(SCHEMATICCHANNELASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SchematicChannelAsset {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICCHANNELASSET_TYPE_INFO
    }
}


pub const SCHEMATICCHANNELASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicChannelAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SchematicChannelAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyChannel {
    pub realm: super::core::Realm,
    pub id: i32,
    pub field_type_hash: i32,
}

pub const PROPERTYCHANNEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyChannel",
    flags: MemberInfoFlags::new(32841),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PropertyChannel, realm),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyChannel, id),
            },
            FieldInfoData {
                name: "FieldTypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyChannel, field_type_hash),
            },
        ],
    }),
    array_type: Some(PROPERTYCHANNEL_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PropertyChannel {
    fn type_info() -> &'static TypeInfo {
        PROPERTYCHANNEL_TYPE_INFO
    }
}


pub const PROPERTYCHANNEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyChannel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PropertyChannel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventChannel {
    pub realm: super::core::Realm,
}

pub const EVENTCHANNEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventChannel",
    flags: MemberInfoFlags::new(32841),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(EventChannel, realm),
            },
        ],
    }),
    array_type: Some(EVENTCHANNEL_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EventChannel {
    fn type_info() -> &'static TypeInfo {
        EVENTCHANNEL_TYPE_INFO
    }
}


pub const EVENTCHANNEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventChannel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventChannel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinkChannel {
    pub realm: super::core::Realm,
    pub id: i32,
    pub link_type_hash: i32,
}

pub const LINKCHANNEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkChannel",
    flags: MemberInfoFlags::new(32841),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LinkChannel, realm),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinkChannel, id),
            },
            FieldInfoData {
                name: "LinkTypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinkChannel, link_type_hash),
            },
        ],
    }),
    array_type: Some(LINKCHANNEL_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LinkChannel {
    fn type_info() -> &'static TypeInfo {
        LINKCHANNEL_TYPE_INFO
    }
}


pub const LINKCHANNEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkChannel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LinkChannel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogicPrefabReferenceObjectData {
}

pub const LOGICPREFABREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicPrefabReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOGICREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGICPREFABREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogicPrefabReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        LOGICPREFABREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const LOGICPREFABREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicPrefabReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LogicPrefabReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpatialPrefabReferenceObjectData {
    pub persistence_setting: BlueprintPersistenceSetting,
}

pub const SPATIALPREFABREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialPrefabReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PersistenceSetting",
                flags: MemberInfoFlags::new(0),
                field_type: BLUEPRINTPERSISTENCESETTING_TYPE_INFO,
                rust_offset: offset_of!(SpatialPrefabReferenceObjectData, persistence_setting),
            },
        ],
    }),
    array_type: Some(SPATIALPREFABREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SpatialPrefabReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        SPATIALPREFABREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const SPATIALPREFABREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialPrefabReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SpatialPrefabReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ObjectReferenceObjectData {
}

pub const OBJECTREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBJECTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ObjectReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        OBJECTREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const OBJECTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LogicReferenceObjectData {
    pub local_player_id: super::core::LocalPlayerId,
    pub sub_realm: SubRealm,
}

pub const LOGICREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(REFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LocalPlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(LogicReferenceObjectData, local_player_id),
            },
            FieldInfoData {
                name: "SubRealm",
                flags: MemberInfoFlags::new(0),
                field_type: SUBREALM_TYPE_INFO,
                rust_offset: offset_of!(LogicReferenceObjectData, sub_realm),
            },
        ],
    }),
    array_type: Some(LOGICREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LogicReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        LOGICREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const LOGICREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LogicReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpatialReferenceObjectData {
    pub local_player_id: super::core::LocalPlayerId,
}

pub const SPATIALREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(REFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LocalPlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(SpatialReferenceObjectData, local_player_id),
            },
        ],
    }),
    array_type: Some(SPATIALREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SpatialReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        SPATIALREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const SPATIALREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SpatialReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ReferenceObjectData {
    pub blueprint_transform: super::core::LinearTransform,
    pub blueprint: Blueprint,
    pub object_variation: ObjectVariation,
    pub stream_realm: StreamRealm,
    pub radiosity_type_override: super::core::RadiosityTypeOverride,
    pub lightmap_resolution_scale: u32,
    pub lightmap_scale_with_size: bool,
    pub rendering_overrides: super::core::RenderingOverrides,
    pub excluded: bool,
    pub create_indestructible_entity: bool,
}

pub const REFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BlueprintTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(ReferenceObjectData, blueprint_transform),
            },
            FieldInfoData {
                name: "Blueprint",
                flags: MemberInfoFlags::new(0),
                field_type: BLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(ReferenceObjectData, blueprint),
            },
            FieldInfoData {
                name: "ObjectVariation",
                flags: MemberInfoFlags::new(0),
                field_type: OBJECTVARIATION_TYPE_INFO,
                rust_offset: offset_of!(ReferenceObjectData, object_variation),
            },
            FieldInfoData {
                name: "StreamRealm",
                flags: MemberInfoFlags::new(0),
                field_type: STREAMREALM_TYPE_INFO,
                rust_offset: offset_of!(ReferenceObjectData, stream_realm),
            },
            FieldInfoData {
                name: "RadiosityTypeOverride",
                flags: MemberInfoFlags::new(0),
                field_type: RADIOSITYTYPEOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(ReferenceObjectData, radiosity_type_override),
            },
            FieldInfoData {
                name: "LightmapResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReferenceObjectData, lightmap_resolution_scale),
            },
            FieldInfoData {
                name: "LightmapScaleWithSize",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ReferenceObjectData, lightmap_scale_with_size),
            },
            FieldInfoData {
                name: "RenderingOverrides",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERINGOVERRIDES_TYPE_INFO,
                rust_offset: offset_of!(ReferenceObjectData, rendering_overrides),
            },
            FieldInfoData {
                name: "Excluded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ReferenceObjectData, excluded),
            },
            FieldInfoData {
                name: "CreateIndestructibleEntity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ReferenceObjectData, create_indestructible_entity),
            },
        ],
    }),
    array_type: Some(REFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        REFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const REFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum StreamRealm {
    #[default]
    StreamRealm_None = 0,
    StreamRealm_Client = 1,
    StreamRealm_Both = 2,
}

pub const STREAMREALM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamRealm",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(STREAMREALM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StreamRealm {
    fn type_info() -> &'static TypeInfo {
        STREAMREALM_TYPE_INFO
    }
}


pub const STREAMREALM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamRealm-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("StreamRealm-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectVariationCollection {
    pub variations: Vec<ObjectVariation>,
}

pub const OBJECTVARIATIONCOLLECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationCollection",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Variations",
                flags: MemberInfoFlags::new(144),
                field_type: OBJECTVARIATION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ObjectVariationCollection, variations),
            },
        ],
    }),
    array_type: Some(OBJECTVARIATIONCOLLECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObjectVariationCollection {
    fn type_info() -> &'static TypeInfo {
        OBJECTVARIATIONCOLLECTION_TYPE_INFO
    }
}


pub const OBJECTVARIATIONCOLLECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationCollection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectVariationCollection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectVariation {
    pub name_hash: u32,
}

pub const OBJECTVARIATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariation",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "NameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectVariation, name_hash),
            },
        ],
    }),
    array_type: Some(OBJECTVARIATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObjectVariation {
    fn type_info() -> &'static TypeInfo {
        OBJECTVARIATION_TYPE_INFO
    }
}


pub const OBJECTVARIATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectVariation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataContainerCollectionBlueprint {
    pub data_containers: Vec<super::core::DataContainer>,
}

pub const DATACONTAINERCOLLECTIONBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerCollectionBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DataContainers",
                flags: MemberInfoFlags::new(144),
                field_type: DATACONTAINER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DataContainerCollectionBlueprint, data_containers),
            },
        ],
    }),
    array_type: Some(DATACONTAINERCOLLECTIONBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataContainerCollectionBlueprint {
    fn type_info() -> &'static TypeInfo {
        DATACONTAINERCOLLECTIONBLUEPRINT_TYPE_INFO
    }
}


pub const DATACONTAINERCOLLECTIONBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerCollectionBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DataContainerCollectionBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogicPrefabBlueprint {
}

pub const LOGICPREFABBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicPrefabBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PREFABBLUEPRINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGICPREFABBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LogicPrefabBlueprint {
    fn type_info() -> &'static TypeInfo {
        LOGICPREFABBLUEPRINT_TYPE_INFO
    }
}


pub const LOGICPREFABBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogicPrefabBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LogicPrefabBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpatialPrefabBlueprint {
}

pub const SPATIALPREFABBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialPrefabBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PREFABBLUEPRINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SPATIALPREFABBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpatialPrefabBlueprint {
    fn type_info() -> &'static TypeInfo {
        SPATIALPREFABBLUEPRINT_TYPE_INFO
    }
}


pub const SPATIALPREFABBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialPrefabBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SpatialPrefabBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PrefabBlueprint {
    pub time_delta_type: TimeDeltaType,
}

pub const PREFABBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrefabBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: TIMEDELTATYPE_TYPE_INFO,
                rust_offset: offset_of!(PrefabBlueprint, time_delta_type),
            },
        ],
    }),
    array_type: Some(PREFABBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PrefabBlueprint {
    fn type_info() -> &'static TypeInfo {
        PREFABBLUEPRINT_TYPE_INFO
    }
}


pub const PREFABBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrefabBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PrefabBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectBlueprint {
    pub object: EntityData,
}

pub const OBJECTBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Object",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(ObjectBlueprint, object),
            },
        ],
    }),
    array_type: Some(OBJECTBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObjectBlueprint {
    fn type_info() -> &'static TypeInfo {
        OBJECTBLUEPRINT_TYPE_INFO
    }
}


pub const OBJECTBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Blueprint {
    pub objects: Vec<GameObjectData>,
    pub schematics: super::schematics::SchematicsBaseAsset,
}

pub const BLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Blueprint",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYBUSDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Blueprint, objects),
            },
            FieldInfoData {
                name: "Schematics",
                flags: MemberInfoFlags::new(0),
                field_type: SCHEMATICSBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(Blueprint, schematics),
            },
        ],
    }),
    array_type: Some(BLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Blueprint {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINT_TYPE_INFO
    }
}


pub const BLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Blueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Blueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BlueprintPersistenceSetting {
    #[default]
    BlueprintPersistenceSetting_InheritFromParent = 0,
    BlueprintPersistenceSetting_SavedWhenInMemory = 1,
    BlueprintPersistenceSetting_AlwaysSaved = 2,
    BlueprintPersistenceSetting_NeverSaved = 3,
}

pub const BLUEPRINTPERSISTENCESETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintPersistenceSetting",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(BLUEPRINTPERSISTENCESETTING_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BlueprintPersistenceSetting {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTPERSISTENCESETTING_TYPE_INFO
    }
}


pub const BLUEPRINTPERSISTENCESETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintPersistenceSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BlueprintPersistenceSetting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityBusData {
    pub event_connections: Vec<EventConnection>,
}

pub const ENTITYBUSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityBusData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATABUSDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EventConnections",
                flags: MemberInfoFlags::new(144),
                field_type: EVENTCONNECTION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EntityBusData, event_connections),
            },
        ],
    }),
    array_type: Some(ENTITYBUSDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityBusData {
    fn type_info() -> &'static TypeInfo {
        ENTITYBUSDATA_TYPE_INFO
    }
}


pub const ENTITYBUSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityBusData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntityBusData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InterfaceDescriptorData {
    pub input_events: Vec<DynamicEvent>,
    pub output_events: Vec<DynamicEvent>,
    pub input_links: Vec<DynamicLink>,
    pub output_links: Vec<DynamicLink>,
}

pub const INTERFACEDESCRIPTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InterfaceDescriptorData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DYNAMICDATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InputEvents",
                flags: MemberInfoFlags::new(144),
                field_type: DYNAMICEVENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(InterfaceDescriptorData, input_events),
            },
            FieldInfoData {
                name: "OutputEvents",
                flags: MemberInfoFlags::new(144),
                field_type: DYNAMICEVENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(InterfaceDescriptorData, output_events),
            },
            FieldInfoData {
                name: "InputLinks",
                flags: MemberInfoFlags::new(144),
                field_type: DYNAMICLINK_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(InterfaceDescriptorData, input_links),
            },
            FieldInfoData {
                name: "OutputLinks",
                flags: MemberInfoFlags::new(144),
                field_type: DYNAMICLINK_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(InterfaceDescriptorData, output_links),
            },
        ],
    }),
    array_type: Some(INTERFACEDESCRIPTORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InterfaceDescriptorData {
    fn type_info() -> &'static TypeInfo {
        INTERFACEDESCRIPTORDATA_TYPE_INFO
    }
}


pub const INTERFACEDESCRIPTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InterfaceDescriptorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("InterfaceDescriptorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicLink {
    pub id: i32,
}

pub const DYNAMICLINK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicLink",
    flags: MemberInfoFlags::new(32841),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicLink, id),
            },
        ],
    }),
    array_type: Some(DYNAMICLINK_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DynamicLink {
    fn type_info() -> &'static TypeInfo {
        DYNAMICLINK_TYPE_INFO
    }
}


pub const DYNAMICLINK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicLink-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DynamicLink-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicEvent {
    pub id: i32,
}

pub const DYNAMICEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEvent",
    flags: MemberInfoFlags::new(32841),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicEvent, id),
            },
        ],
    }),
    array_type: Some(DYNAMICEVENT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DynamicEvent {
    fn type_info() -> &'static TypeInfo {
        DYNAMICEVENT_TYPE_INFO
    }
}


pub const DYNAMICEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DynamicEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventConnection {
    pub source: super::core::DataContainer,
    pub target: super::core::DataContainer,
    pub source_event: EventSpec,
    pub target_event: EventSpec,
    pub target_type: EventConnectionTargetType,
}

pub const EVENTCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventConnection",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Source",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(EventConnection, source),
            },
            FieldInfoData {
                name: "Target",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(EventConnection, target),
            },
            FieldInfoData {
                name: "SourceEvent",
                flags: MemberInfoFlags::new(0),
                field_type: EVENTSPEC_TYPE_INFO,
                rust_offset: offset_of!(EventConnection, source_event),
            },
            FieldInfoData {
                name: "TargetEvent",
                flags: MemberInfoFlags::new(0),
                field_type: EVENTSPEC_TYPE_INFO,
                rust_offset: offset_of!(EventConnection, target_event),
            },
            FieldInfoData {
                name: "TargetType",
                flags: MemberInfoFlags::new(0),
                field_type: EVENTCONNECTIONTARGETTYPE_TYPE_INFO,
                rust_offset: offset_of!(EventConnection, target_type),
            },
        ],
    }),
    array_type: Some(EVENTCONNECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventConnection {
    fn type_info() -> &'static TypeInfo {
        EVENTCONNECTION_TYPE_INFO
    }
}


pub const EVENTCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventConnection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EventConnectionTargetType {
    #[default]
    EventConnectionTargetType_Invalid = 0,
    EventConnectionTargetType_ClientAndServer = 1,
    EventConnectionTargetType_Client = 2,
    EventConnectionTargetType_Server = 3,
    EventConnectionTargetType_NetworkedClient = 4,
    EventConnectionTargetType_NetworkedClientAndServer = 5,
}

pub const EVENTCONNECTIONTARGETTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventConnectionTargetType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(EVENTCONNECTIONTARGETTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EventConnectionTargetType {
    fn type_info() -> &'static TypeInfo {
        EVENTCONNECTIONTARGETTYPE_TYPE_INFO
    }
}


pub const EVENTCONNECTIONTARGETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventConnectionTargetType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventConnectionTargetType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventSpec {
    pub id: i32,
}

pub const EVENTSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSpec",
    flags: MemberInfoFlags::new(32841),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EventSpec, id),
            },
        ],
    }),
    array_type: Some(EVENTSPEC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EventSpec {
    fn type_info() -> &'static TypeInfo {
        EVENTSPEC_TYPE_INFO
    }
}


pub const EVENTSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventSpec-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompareAxisAlignedBoxEntityData {
    pub a: super::core::AxisAlignedBox,
    pub b: super::core::AxisAlignedBox,
}

pub const COMPAREAXISALIGNEDBOXENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareAxisAlignedBoxEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "A",
                flags: MemberInfoFlags::new(0),
                field_type: AXISALIGNEDBOX_TYPE_INFO,
                rust_offset: offset_of!(CompareAxisAlignedBoxEntityData, a),
            },
            FieldInfoData {
                name: "B",
                flags: MemberInfoFlags::new(0),
                field_type: AXISALIGNEDBOX_TYPE_INFO,
                rust_offset: offset_of!(CompareAxisAlignedBoxEntityData, b),
            },
        ],
    }),
    array_type: Some(COMPAREAXISALIGNEDBOXENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CompareAxisAlignedBoxEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREAXISALIGNEDBOXENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREAXISALIGNEDBOXENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareAxisAlignedBoxEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareAxisAlignedBoxEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareUintRangeEntityData {
    pub value: u32,
    pub min: u32,
    pub max: u32,
}

pub const COMPAREUINTRANGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareUintRangeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CompareUintRangeEntityData, value),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CompareUintRangeEntityData, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CompareUintRangeEntityData, max),
            },
        ],
    }),
    array_type: Some(COMPAREUINTRANGEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareUintRangeEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREUINTRANGEENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREUINTRANGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareUintRangeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareUintRangeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareIntRangeEntityData {
    pub value: i32,
    pub min: i32,
    pub max: i32,
}

pub const COMPAREINTRANGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareIntRangeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CompareIntRangeEntityData, value),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CompareIntRangeEntityData, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CompareIntRangeEntityData, max),
            },
        ],
    }),
    array_type: Some(COMPAREINTRANGEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareIntRangeEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREINTRANGEENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREINTRANGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareIntRangeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareIntRangeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompareFloatRangeEntityData {
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

pub const COMPAREFLOATRANGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareFloatRangeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompareFloatRangeEntityData, value),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompareFloatRangeEntityData, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompareFloatRangeEntityData, max),
            },
        ],
    }),
    array_type: Some(COMPAREFLOATRANGEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareFloatRangeEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREFLOATRANGEENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREFLOATRANGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareFloatRangeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareFloatRangeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareStringEntityData {
    pub a: String,
    pub b: String,
}

pub const COMPARESTRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareStringEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "A",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CompareStringEntityData, a),
            },
            FieldInfoData {
                name: "B",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CompareStringEntityData, b),
            },
        ],
    }),
    array_type: Some(COMPARESTRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareStringEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPARESTRINGENTITYDATA_TYPE_INFO
    }
}


pub const COMPARESTRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareStringEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareStringEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompareVec4EntityData {
    pub value_a: super::core::Vec4,
    pub value_b: super::core::Vec4,
    pub tolerance: f32,
}

pub const COMPAREVEC4ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareVec4EntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ValueA",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(CompareVec4EntityData, value_a),
            },
            FieldInfoData {
                name: "ValueB",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(CompareVec4EntityData, value_b),
            },
            FieldInfoData {
                name: "Tolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompareVec4EntityData, tolerance),
            },
        ],
    }),
    array_type: Some(COMPAREVEC4ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CompareVec4EntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREVEC4ENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREVEC4ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareVec4EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareVec4EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompareVec3EntityData {
    pub value_a: super::core::Vec3,
    pub value_b: super::core::Vec3,
    pub tolerance: f32,
}

pub const COMPAREVEC3ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareVec3EntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ValueA",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CompareVec3EntityData, value_a),
            },
            FieldInfoData {
                name: "ValueB",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CompareVec3EntityData, value_b),
            },
            FieldInfoData {
                name: "Tolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompareVec3EntityData, tolerance),
            },
        ],
    }),
    array_type: Some(COMPAREVEC3ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CompareVec3EntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREVEC3ENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREVEC3ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareVec3EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareVec3EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompareFloatEntityData {
    pub a: f32,
    pub b: f32,
}

pub const COMPAREFLOATENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareFloatEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "A",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompareFloatEntityData, a),
            },
            FieldInfoData {
                name: "B",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompareFloatEntityData, b),
            },
        ],
    }),
    array_type: Some(COMPAREFLOATENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareFloatEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREFLOATENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREFLOATENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareFloatEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareFloatEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareUintEntityData {
    pub a: u32,
    pub b: u32,
}

pub const COMPAREUINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareUintEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "A",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CompareUintEntityData, a),
            },
            FieldInfoData {
                name: "B",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CompareUintEntityData, b),
            },
        ],
    }),
    array_type: Some(COMPAREUINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareUintEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREUINTENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREUINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareUintEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareUintEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareIntEntityData {
    pub a: i32,
    pub b: i32,
}

pub const COMPAREINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareIntEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "A",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CompareIntEntityData, a),
            },
            FieldInfoData {
                name: "B",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CompareIntEntityData, b),
            },
        ],
    }),
    array_type: Some(COMPAREINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareIntEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREINTENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareIntEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareIntEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareBoolEntityData {
    pub bool: bool,
}

pub const COMPAREBOOLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareBoolEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPAREENTITYBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Bool",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompareBoolEntityData, bool),
            },
        ],
    }),
    array_type: Some(COMPAREBOOLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareBoolEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREBOOLENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREBOOLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareBoolEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareBoolEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareEntityBaseData {
    pub realm: super::core::Realm,
    pub trigger_on_property_change: bool,
    pub trigger_on_start: bool,
    pub always_send: bool,
}

pub const COMPAREENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareEntityBaseData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CompareEntityBaseData, realm),
            },
            FieldInfoData {
                name: "TriggerOnPropertyChange",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompareEntityBaseData, trigger_on_property_change),
            },
            FieldInfoData {
                name: "TriggerOnStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompareEntityBaseData, trigger_on_start),
            },
            FieldInfoData {
                name: "AlwaysSend",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompareEntityBaseData, always_send),
            },
        ],
    }),
    array_type: Some(COMPAREENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareEntityBaseData {
    fn type_info() -> &'static TypeInfo {
        COMPAREENTITYBASEDATA_TYPE_INFO
    }
}


pub const COMPAREENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareEntityBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareEntityBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientProfilerData {
    pub profiler_name: String,
}

pub const CLIENTPROFILERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProfilerData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ProfilerName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ClientProfilerData, profiler_name),
            },
        ],
    }),
    array_type: Some(CLIENTPROFILERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClientProfilerData {
    fn type_info() -> &'static TypeInfo {
        CLIENTPROFILERDATA_TYPE_INFO
    }
}


pub const CLIENTPROFILERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProfilerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ClientProfilerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ChildEffectEntityData {
    pub components: Vec<GameObjectData>,
    pub start_delay: f32,
    pub attach_to_spawn_surface: bool,
    pub enable: super::core::QualityScalableBool,
    pub override_draw_order: bool,
    pub keep_alive: bool,
}

pub const CHILDEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChildEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Components",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ChildEffectEntityData, components),
            },
            FieldInfoData {
                name: "StartDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ChildEffectEntityData, start_delay),
            },
            FieldInfoData {
                name: "AttachToSpawnSurface",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ChildEffectEntityData, attach_to_spawn_surface),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(ChildEffectEntityData, enable),
            },
            FieldInfoData {
                name: "OverrideDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ChildEffectEntityData, override_draw_order),
            },
            FieldInfoData {
                name: "KeepAlive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ChildEffectEntityData, keep_alive),
            },
        ],
    }),
    array_type: Some(CHILDEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ChildEffectEntityData {
    fn type_info() -> &'static TypeInfo {
        CHILDEFFECTENTITYDATA_TYPE_INFO
    }
}


pub const CHILDEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChildEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ChildEffectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BundleType {
    #[default]
    BundleType_SubLevel = 0,
    BundleType_BlueprintBundle = 1,
    BundleType_SharedBundle = 2,
}

pub const BUNDLETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(BUNDLETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BundleType {
    fn type_info() -> &'static TypeInfo {
        BUNDLETYPE_TYPE_INFO
    }
}


pub const BUNDLETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BundleType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoAssetCollector {
    pub assets: Vec<super::core::Asset>,
    pub auto_collect_minimum_usage_percentage: f32,
}

pub const AUTOASSETCOLLECTOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoAssetCollector",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Assets",
                flags: MemberInfoFlags::new(144),
                field_type: ASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AutoAssetCollector, assets),
            },
            FieldInfoData {
                name: "AutoCollectMinimumUsagePercentage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoAssetCollector, auto_collect_minimum_usage_percentage),
            },
        ],
    }),
    array_type: Some(AUTOASSETCOLLECTOR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoAssetCollector {
    fn type_info() -> &'static TypeInfo {
        AUTOASSETCOLLECTOR_TYPE_INFO
    }
}


pub const AUTOASSETCOLLECTOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoAssetCollector-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AutoAssetCollector-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkedObjectsAsset {
    pub objects: Vec<super::core::DataContainer>,
}

pub const NETWORKEDOBJECTSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkedObjectsAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: DATACONTAINER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(NetworkedObjectsAsset, objects),
            },
        ],
    }),
    array_type: Some(NETWORKEDOBJECTSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkedObjectsAsset {
    fn type_info() -> &'static TypeInfo {
        NETWORKEDOBJECTSASSET_TYPE_INFO
    }
}


pub const NETWORKEDOBJECTSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkedObjectsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("NetworkedObjectsAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkRegistryAsset {
    pub objects: Vec<super::core::DataContainer>,
    pub checksum: u32,
}

pub const NETWORKREGISTRYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkRegistryAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: DATACONTAINER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(NetworkRegistryAsset, objects),
            },
            FieldInfoData {
                name: "Checksum",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkRegistryAsset, checksum),
            },
        ],
    }),
    array_type: Some(NETWORKREGISTRYASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkRegistryAsset {
    fn type_info() -> &'static TypeInfo {
        NETWORKREGISTRYASSET_TYPE_INFO
    }
}


pub const NETWORKREGISTRYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkRegistryAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("NetworkRegistryAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BundleHeapInfo {
    pub heap_type: BundleHeapType,
    pub initial_size: u32,
    pub allow_grow: bool,
}

pub const BUNDLEHEAPINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleHeapInfo",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "HeapType",
                flags: MemberInfoFlags::new(0),
                field_type: BUNDLEHEAPTYPE_TYPE_INFO,
                rust_offset: offset_of!(BundleHeapInfo, heap_type),
            },
            FieldInfoData {
                name: "InitialSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BundleHeapInfo, initial_size),
            },
            FieldInfoData {
                name: "AllowGrow",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BundleHeapInfo, allow_grow),
            },
        ],
    }),
    array_type: Some(BUNDLEHEAPINFO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BundleHeapInfo {
    fn type_info() -> &'static TypeInfo {
        BUNDLEHEAPINFO_TYPE_INFO
    }
}


pub const BUNDLEHEAPINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleHeapInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BundleHeapInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BundleHeapType {
    #[default]
    BundleHeapType_OwnWithParentSmallblock = 0,
    BundleHeapType_OwnWithSmallblock = 1,
    BundleHeapType_OwnWithoutSmallblock = 2,
    BundleHeapType_Parent = 3,
    BundleHeapType_Level = 4,
    BundleHeapType_Global = 5,
    BundleHeapType_Null = 6,
}

pub const BUNDLEHEAPTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleHeapType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(BUNDLEHEAPTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BundleHeapType {
    fn type_info() -> &'static TypeInfo {
        BUNDLEHEAPTYPE_TYPE_INFO
    }
}


pub const BUNDLEHEAPTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleHeapType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BundleHeapType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BundleSettingsInfo {
    pub group_identifier: u32,
    pub group_name: String,
}

pub const BUNDLESETTINGSINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleSettingsInfo",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "GroupIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BundleSettingsInfo, group_identifier),
            },
            FieldInfoData {
                name: "GroupName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BundleSettingsInfo, group_name),
            },
        ],
    }),
    array_type: Some(BUNDLESETTINGSINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BundleSettingsInfo {
    fn type_info() -> &'static TypeInfo {
        BUNDLESETTINGSINFO_TYPE_INFO
    }
}


pub const BUNDLESETTINGSINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleSettingsInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BundleSettingsInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshOperationData {
}

pub const MESHOPERATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshOperationData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHOPERATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshOperationData {
    fn type_info() -> &'static TypeInfo {
        MESHOPERATIONDATA_TYPE_INFO
    }
}


pub const MESHOPERATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshOperationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MeshOperationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SkeletonBaseAsset {
}

pub const SKELETONBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkeletonBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SKELETONBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SkeletonBaseAsset {
    fn type_info() -> &'static TypeInfo {
        SKELETONBASEASSET_TYPE_INFO
    }
}


pub const SKELETONBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkeletonBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SkeletonBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformPartPropertyTrackData {
    pub transform_part: TransformPart,
    pub pre_infinity: CurveInfinityType,
    pub post_infinity: CurveInfinityType,
    pub values: Vec<TransformPartPropertyKey>,
    pub evaluator_fps: f32,
    pub weighted: bool,
    pub key_start_index: u32,
    pub key_count: u32,
    pub is_static: bool,
    pub resource: super::core::ResourceRef,
}

pub const TRANSFORMPARTPROPERTYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformPartPropertyTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TransformPart",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMPART_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyTrackData, transform_part),
            },
            FieldInfoData {
                name: "PreInfinity",
                flags: MemberInfoFlags::new(0),
                field_type: CURVEINFINITYTYPE_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyTrackData, pre_infinity),
            },
            FieldInfoData {
                name: "PostInfinity",
                flags: MemberInfoFlags::new(0),
                field_type: CURVEINFINITYTYPE_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyTrackData, post_infinity),
            },
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: TRANSFORMPARTPROPERTYKEY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyTrackData, values),
            },
            FieldInfoData {
                name: "EvaluatorFps",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyTrackData, evaluator_fps),
            },
            FieldInfoData {
                name: "Weighted",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyTrackData, weighted),
            },
            FieldInfoData {
                name: "KeyStartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyTrackData, key_start_index),
            },
            FieldInfoData {
                name: "KeyCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyTrackData, key_count),
            },
            FieldInfoData {
                name: "IsStatic",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyTrackData, is_static),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyTrackData, resource),
            },
        ],
    }),
    array_type: Some(TRANSFORMPARTPROPERTYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TransformPartPropertyTrackData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMPARTPROPERTYTRACKDATA_TYPE_INFO
    }
}


pub const TRANSFORMPARTPROPERTYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformPartPropertyTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformPartPropertyTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformPartPropertyKey {
    pub value: f32,
    pub in_anim_tangent_type: AnimTangentType,
    pub in_angle: f32,
    pub in_weight: f32,
    pub out_anim_tangent_type: AnimTangentType,
    pub out_angle: f32,
    pub out_weight: f32,
}

pub const TRANSFORMPARTPROPERTYKEY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformPartPropertyKey",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyKey, value),
            },
            FieldInfoData {
                name: "InAnimTangentType",
                flags: MemberInfoFlags::new(0),
                field_type: ANIMTANGENTTYPE_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyKey, in_anim_tangent_type),
            },
            FieldInfoData {
                name: "InAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyKey, in_angle),
            },
            FieldInfoData {
                name: "InWeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyKey, in_weight),
            },
            FieldInfoData {
                name: "OutAnimTangentType",
                flags: MemberInfoFlags::new(0),
                field_type: ANIMTANGENTTYPE_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyKey, out_anim_tangent_type),
            },
            FieldInfoData {
                name: "OutAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyKey, out_angle),
            },
            FieldInfoData {
                name: "OutWeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TransformPartPropertyKey, out_weight),
            },
        ],
    }),
    array_type: Some(TRANSFORMPARTPROPERTYKEY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TransformPartPropertyKey {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMPARTPROPERTYKEY_TYPE_INFO
    }
}


pub const TRANSFORMPARTPROPERTYKEY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformPartPropertyKey-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformPartPropertyKey-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AnimTangentType {
    #[default]
    kTangentFixed = 0,
    kTangentLinear = 1,
    kTangentFlat = 2,
    kTangentStep = 3,
    kTangentStepNext = 4,
    kTangentSlow = 5,
    kTangentFast = 6,
    kTangentSmooth = 7,
    kTangentClamped = 8,
}

pub const ANIMTANGENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimTangentType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(ANIMTANGENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AnimTangentType {
    fn type_info() -> &'static TypeInfo {
        ANIMTANGENTTYPE_TYPE_INFO
    }
}


pub const ANIMTANGENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimTangentType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AnimTangentType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CurveInfinityType {
    #[default]
    CurveInfinityType_Constant = 0,
    CurveInfinityType_Linear = 1,
    CurveInfinityType_Cycle = 2,
    CurveInfinityType_CycleWithOffset = 3,
    CurveInfinityType_Oscillate = 4,
}

pub const CURVEINFINITYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveInfinityType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(CURVEINFINITYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CurveInfinityType {
    fn type_info() -> &'static TypeInfo {
        CURVEINFINITYTYPE_TYPE_INFO
    }
}


pub const CURVEINFINITYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveInfinityType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CurveInfinityType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TransformPart {
    #[default]
    TransformPart_TranslationX = 0,
    TransformPart_TranslationY = 1,
    TransformPart_TranslationZ = 2,
    TransformPart_RotationX = 3,
    TransformPart_RotationY = 4,
    TransformPart_RotationZ = 5,
}

pub const TRANSFORMPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformPart",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(TRANSFORMPART_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TransformPart {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMPART_TYPE_INFO
    }
}


pub const TRANSFORMPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformPart-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformPart-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec4PropertyTrackData {
    pub values: Vec<super::core::Vec4>,
}

pub const VEC4PROPERTYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4PropertyTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SIMPLEPROPERTYTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: VEC4_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Vec4PropertyTrackData, values),
            },
        ],
    }),
    array_type: Some(VEC4PROPERTYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec4PropertyTrackData {
    fn type_info() -> &'static TypeInfo {
        VEC4PROPERTYTRACKDATA_TYPE_INFO
    }
}


pub const VEC4PROPERTYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4PropertyTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec4PropertyTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec3PropertyTrackData {
    pub values: Vec<super::core::Vec3>,
}

pub const VEC3PROPERTYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3PropertyTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SIMPLEPROPERTYTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: VEC3_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Vec3PropertyTrackData, values),
            },
        ],
    }),
    array_type: Some(VEC3PROPERTYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec3PropertyTrackData {
    fn type_info() -> &'static TypeInfo {
        VEC3PROPERTYTRACKDATA_TYPE_INFO
    }
}


pub const VEC3PROPERTYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3PropertyTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec3PropertyTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec2PropertyTrackData {
    pub values: Vec<super::core::Vec2>,
}

pub const VEC2PROPERTYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2PropertyTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SIMPLEPROPERTYTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: VEC2_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Vec2PropertyTrackData, values),
            },
        ],
    }),
    array_type: Some(VEC2PROPERTYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec2PropertyTrackData {
    fn type_info() -> &'static TypeInfo {
        VEC2PROPERTYTRACKDATA_TYPE_INFO
    }
}


pub const VEC2PROPERTYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2PropertyTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec2PropertyTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformPropertyTrackData {
    pub values: Vec<super::core::LinearTransform>,
}

pub const TRANSFORMPROPERTYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformPropertyTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SIMPLEPROPERTYTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARTRANSFORM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TransformPropertyTrackData, values),
            },
        ],
    }),
    array_type: Some(TRANSFORMPROPERTYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TransformPropertyTrackData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMPROPERTYTRACKDATA_TYPE_INFO
    }
}


pub const TRANSFORMPROPERTYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformPropertyTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformPropertyTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntPropertyTrackData {
    pub values: Vec<i32>,
}

pub const INTPROPERTYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntPropertyTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SIMPLEPROPERTYTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(IntPropertyTrackData, values),
            },
        ],
    }),
    array_type: Some(INTPROPERTYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntPropertyTrackData {
    fn type_info() -> &'static TypeInfo {
        INTPROPERTYTRACKDATA_TYPE_INFO
    }
}


pub const INTPROPERTYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntPropertyTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntPropertyTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatPropertyTrackData {
    pub values: Vec<f32>,
}

pub const FLOATPROPERTYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPropertyTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SIMPLEPROPERTYTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FloatPropertyTrackData, values),
            },
        ],
    }),
    array_type: Some(FLOATPROPERTYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatPropertyTrackData {
    fn type_info() -> &'static TypeInfo {
        FLOATPROPERTYTRACKDATA_TYPE_INFO
    }
}


pub const FLOATPROPERTYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatPropertyTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatPropertyTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolPropertyTrackData {
    pub values: Vec<bool>,
}

pub const BOOLPROPERTYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolPropertyTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SIMPLEPROPERTYTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: BOOLEAN_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(BoolPropertyTrackData, values),
            },
        ],
    }),
    array_type: Some(BOOLPROPERTYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoolPropertyTrackData {
    fn type_info() -> &'static TypeInfo {
        BOOLPROPERTYTRACKDATA_TYPE_INFO
    }
}


pub const BOOLPROPERTYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolPropertyTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoolPropertyTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SimplePropertyTrackData {
    pub interpolation_type: InterpolationType,
}

pub const SIMPLEPROPERTYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimplePropertyTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InterpolationType",
                flags: MemberInfoFlags::new(0),
                field_type: INTERPOLATIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(SimplePropertyTrackData, interpolation_type),
            },
        ],
    }),
    array_type: Some(SIMPLEPROPERTYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SimplePropertyTrackData {
    fn type_info() -> &'static TypeInfo {
        SIMPLEPROPERTYTRACKDATA_TYPE_INFO
    }
}


pub const SIMPLEPROPERTYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimplePropertyTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SimplePropertyTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SequenceEntityData {
    pub events: Vec<SequenceEventData>,
    pub sequence_start_time: i32,
    pub sequence_length: i32,
    pub looping: bool,
    pub auto_start: bool,
    pub auto_play_first_frame: bool,
    pub play_in_reverse: bool,
    pub realm: super::core::Realm,
    pub property_tracks: Vec<PropertyTrackData>,
    pub custom_sequence_tracks: Vec<CustomSequenceTrackData>,
    pub client_update_pass: UpdatePass,
    pub server_update_pass: UpdatePass,
    pub external_time: f32,
    pub playback_speed: f32,
}

pub const SEQUENCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SequenceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Events",
                flags: MemberInfoFlags::new(144),
                field_type: SEQUENCEEVENTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, events),
            },
            FieldInfoData {
                name: "SequenceStartTime",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, sequence_start_time),
            },
            FieldInfoData {
                name: "SequenceLength",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, sequence_length),
            },
            FieldInfoData {
                name: "Looping",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, looping),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, auto_start),
            },
            FieldInfoData {
                name: "AutoPlayFirstFrame",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, auto_play_first_frame),
            },
            FieldInfoData {
                name: "PlayInReverse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, play_in_reverse),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, realm),
            },
            FieldInfoData {
                name: "PropertyTracks",
                flags: MemberInfoFlags::new(144),
                field_type: PROPERTYTRACKDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, property_tracks),
            },
            FieldInfoData {
                name: "CustomSequenceTracks",
                flags: MemberInfoFlags::new(144),
                field_type: CUSTOMSEQUENCETRACKDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, custom_sequence_tracks),
            },
            FieldInfoData {
                name: "ClientUpdatePass",
                flags: MemberInfoFlags::new(0),
                field_type: UPDATEPASS_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, client_update_pass),
            },
            FieldInfoData {
                name: "ServerUpdatePass",
                flags: MemberInfoFlags::new(0),
                field_type: UPDATEPASS_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, server_update_pass),
            },
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, external_time),
            },
            FieldInfoData {
                name: "PlaybackSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SequenceEntityData, playback_speed),
            },
        ],
    }),
    array_type: Some(SEQUENCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SequenceEntityData {
    fn type_info() -> &'static TypeInfo {
        SEQUENCEENTITYDATA_TYPE_INFO
    }
}


pub const SEQUENCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SequenceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SequenceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomSequenceTrackData {
    pub track_name: String,
    pub input_link_map: Vec<CustomSequenceTrackLinkMapping>,
    pub output_link_map: Vec<CustomSequenceTrackLinkMapping>,
    pub event_map: Vec<CustomSequenceTrackEventMapping>,
    pub source_property_map: Vec<CustomSequenceTrackPropertyMapping>,
    pub target_property_map: Vec<CustomSequenceTrackPropertyMapping>,
    pub source_and_target_property_map: Vec<CustomSequenceTrackPropertyMapping>,
}

pub const CUSTOMSEQUENCETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomSequenceTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TrackName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackData, track_name),
            },
            FieldInfoData {
                name: "InputLinkMap",
                flags: MemberInfoFlags::new(144),
                field_type: CUSTOMSEQUENCETRACKLINKMAPPING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackData, input_link_map),
            },
            FieldInfoData {
                name: "OutputLinkMap",
                flags: MemberInfoFlags::new(144),
                field_type: CUSTOMSEQUENCETRACKLINKMAPPING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackData, output_link_map),
            },
            FieldInfoData {
                name: "EventMap",
                flags: MemberInfoFlags::new(144),
                field_type: CUSTOMSEQUENCETRACKEVENTMAPPING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackData, event_map),
            },
            FieldInfoData {
                name: "SourcePropertyMap",
                flags: MemberInfoFlags::new(144),
                field_type: CUSTOMSEQUENCETRACKPROPERTYMAPPING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackData, source_property_map),
            },
            FieldInfoData {
                name: "TargetPropertyMap",
                flags: MemberInfoFlags::new(144),
                field_type: CUSTOMSEQUENCETRACKPROPERTYMAPPING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackData, target_property_map),
            },
            FieldInfoData {
                name: "SourceAndTargetPropertyMap",
                flags: MemberInfoFlags::new(144),
                field_type: CUSTOMSEQUENCETRACKPROPERTYMAPPING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackData, source_and_target_property_map),
            },
        ],
    }),
    array_type: Some(CUSTOMSEQUENCETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomSequenceTrackData {
    fn type_info() -> &'static TypeInfo {
        CUSTOMSEQUENCETRACKDATA_TYPE_INFO
    }
}


pub const CUSTOMSEQUENCETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomSequenceTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CustomSequenceTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomSequenceTrackPropertyMapping {
    pub track_property_i_d: i32,
    pub sequence_property_i_d: i32,
}

pub const CUSTOMSEQUENCETRACKPROPERTYMAPPING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomSequenceTrackPropertyMapping",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TrackPropertyID",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackPropertyMapping, track_property_i_d),
            },
            FieldInfoData {
                name: "SequencePropertyID",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackPropertyMapping, sequence_property_i_d),
            },
        ],
    }),
    array_type: Some(CUSTOMSEQUENCETRACKPROPERTYMAPPING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomSequenceTrackPropertyMapping {
    fn type_info() -> &'static TypeInfo {
        CUSTOMSEQUENCETRACKPROPERTYMAPPING_TYPE_INFO
    }
}


pub const CUSTOMSEQUENCETRACKPROPERTYMAPPING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomSequenceTrackPropertyMapping-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CustomSequenceTrackPropertyMapping-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomSequenceTrackEventMapping {
    pub track_event_i_d: i32,
    pub sequence_event_i_d: i32,
}

pub const CUSTOMSEQUENCETRACKEVENTMAPPING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomSequenceTrackEventMapping",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TrackEventID",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackEventMapping, track_event_i_d),
            },
            FieldInfoData {
                name: "SequenceEventID",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackEventMapping, sequence_event_i_d),
            },
        ],
    }),
    array_type: Some(CUSTOMSEQUENCETRACKEVENTMAPPING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomSequenceTrackEventMapping {
    fn type_info() -> &'static TypeInfo {
        CUSTOMSEQUENCETRACKEVENTMAPPING_TYPE_INFO
    }
}


pub const CUSTOMSEQUENCETRACKEVENTMAPPING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomSequenceTrackEventMapping-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CustomSequenceTrackEventMapping-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomSequenceTrackLinkMapping {
    pub track_link_i_d: i32,
    pub sequence_link_i_d: i32,
}

pub const CUSTOMSEQUENCETRACKLINKMAPPING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomSequenceTrackLinkMapping",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TrackLinkID",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackLinkMapping, track_link_i_d),
            },
            FieldInfoData {
                name: "SequenceLinkID",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CustomSequenceTrackLinkMapping, sequence_link_i_d),
            },
        ],
    }),
    array_type: Some(CUSTOMSEQUENCETRACKLINKMAPPING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomSequenceTrackLinkMapping {
    fn type_info() -> &'static TypeInfo {
        CUSTOMSEQUENCETRACKLINKMAPPING_TYPE_INFO
    }
}


pub const CUSTOMSEQUENCETRACKLINKMAPPING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomSequenceTrackLinkMapping-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CustomSequenceTrackLinkMapping-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyTrackData {
    pub id: i32,
    pub times: Vec<i32>,
}

pub const PROPERTYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyTrackData, id),
            },
            FieldInfoData {
                name: "Times",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PropertyTrackData, times),
            },
        ],
    }),
    array_type: Some(PROPERTYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertyTrackData {
    fn type_info() -> &'static TypeInfo {
        PROPERTYTRACKDATA_TYPE_INFO
    }
}


pub const PROPERTYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PropertyTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum InterpolationType {
    #[default]
    InterpolationType_None = 0,
    InterpolationType_Linear = 1,
    InterpolationType_CatmullRom = 2,
    InterpolationType_Curves = 3,
}

pub const INTERPOLATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InterpolationType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(INTERPOLATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InterpolationType {
    fn type_info() -> &'static TypeInfo {
        INTERPOLATIONTYPE_TYPE_INFO
    }
}


pub const INTERPOLATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InterpolationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("InterpolationType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SequenceEventData {
    pub event: EventSpec,
    pub time: i32,
}

pub const SEQUENCEEVENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SequenceEventData",
    flags: MemberInfoFlags::new(32841),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Event",
                flags: MemberInfoFlags::new(0),
                field_type: EVENTSPEC_TYPE_INFO,
                rust_offset: offset_of!(SequenceEventData, event),
            },
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SequenceEventData, time),
            },
        ],
    }),
    array_type: Some(SEQUENCEEVENTDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SequenceEventData {
    fn type_info() -> &'static TypeInfo {
        SEQUENCEEVENTDATA_TYPE_INFO
    }
}


pub const SEQUENCEEVENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SequenceEventData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SequenceEventData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityClassInfoAsset {
}

pub const ENTITYCLASSINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityClassInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLASSINFOASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENTITYCLASSINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityClassInfoAsset {
    fn type_info() -> &'static TypeInfo {
        ENTITYCLASSINFOASSET_TYPE_INFO
    }
}


pub const ENTITYCLASSINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityClassInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntityClassInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutopaintOutputOverrideBase {
}

pub const AUTOPAINTOUTPUTOVERRIDEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputOverrideBase",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTOUTPUTOVERRIDEBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutopaintOutputOverrideBase {
    fn type_info() -> &'static TypeInfo {
        AUTOPAINTOUTPUTOVERRIDEBASE_TYPE_INFO
    }
}


pub const AUTOPAINTOUTPUTOVERRIDEBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputOverrideBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AutopaintOutputOverrideBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutopaintOutputBase {
}

pub const AUTOPAINTOUTPUTBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputBase",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPAINTOUTPUTBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutopaintOutputBase {
    fn type_info() -> &'static TypeInfo {
        AUTOPAINTOUTPUTBASE_TYPE_INFO
    }
}


pub const AUTOPAINTOUTPUTBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AutopaintOutputBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutopaintOutputsBase {
    pub outputs: Vec<AutopaintOutputBase>,
}

pub const AUTOPAINTOUTPUTSBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputsBase",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Outputs",
                flags: MemberInfoFlags::new(144),
                field_type: AUTOPAINTOUTPUTBASE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AutopaintOutputsBase, outputs),
            },
        ],
    }),
    array_type: Some(AUTOPAINTOUTPUTSBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutopaintOutputsBase {
    fn type_info() -> &'static TypeInfo {
        AUTOPAINTOUTPUTSBASE_TYPE_INFO
    }
}


pub const AUTOPAINTOUTPUTSBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutopaintOutputsBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AutopaintOutputsBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RasterType {
    #[default]
    RasterType_Null = 0,
    RasterType_ColorMap = 1,
    RasterType_DestructionDepth = 2,
    RasterType_Density = 3,
    RasterType_EnlightenAlbedo = 4,
    RasterType_Heightfield = 5,
    RasterType_NonPhysic = 6,
    RasterType_PhysicsMaterials = 7,
    RasterType_ResolutionLayout = 8,
    RasterType_LayerMask = 9,
    RasterType_FlowMap = 10,
    RasterType_Biome = 11,
    RasterType_Count = 12,
}

pub const RASTERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(RASTERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RasterType {
    fn type_info() -> &'static TypeInfo {
        RASTERTYPE_TYPE_INFO
    }
}


pub const RASTERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RasterType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RasterFormat {
    #[default]
    RasterFormat_Null = 0,
    RasterFormat_R8_UNORM = 1,
    RasterFormat_R8_UINT = 2,
    RasterFormat_R8G8B8A8_UNORM = 3,
    RasterFormat_R32_FLOAT = 4,
    RasterFormat_Count = 5,
}

pub const RASTERFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(RASTERFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RasterFormat {
    fn type_info() -> &'static TypeInfo {
        RASTERFORMAT_TYPE_INFO
    }
}


pub const RASTERFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RasterFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RasterFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BaseShapeDataBase {
}

pub const BASESHAPEDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShapeDataBase",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BASESHAPEDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BaseShapeDataBase {
    fn type_info() -> &'static TypeInfo {
        BASESHAPEDATABASE_TYPE_INFO
    }
}


pub const BASESHAPEDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShapeDataBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BaseShapeDataBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BaseMaterialDataPair {
}

pub const BASEMATERIALDATAPAIR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseMaterialDataPair",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BASEMATERIALDATAPAIR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BaseMaterialDataPair {
    fn type_info() -> &'static TypeInfo {
        BASEMATERIALDATAPAIR_TYPE_INFO
    }
}


pub const BASEMATERIALDATAPAIR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseMaterialDataPair-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BaseMaterialDataPair-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialGridData {
    pub default_material: MaterialDecl,
    pub material_pairs: Vec<MaterialDecl>,
    pub material_index_map: Vec<u32>,
    pub default_material_index: u32,
    pub material_properties: Vec<MaterialRelationPropertyPair>,
    pub interaction_grid: Vec<MaterialInteractionGridRow>,
}

pub const MATERIALGRIDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialGridData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DefaultMaterial",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(MaterialGridData, default_material),
            },
            FieldInfoData {
                name: "MaterialPairs",
                flags: MemberInfoFlags::new(144),
                field_type: MATERIALDECL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialGridData, material_pairs),
            },
            FieldInfoData {
                name: "MaterialIndexMap",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialGridData, material_index_map),
            },
            FieldInfoData {
                name: "DefaultMaterialIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialGridData, default_material_index),
            },
            FieldInfoData {
                name: "MaterialProperties",
                flags: MemberInfoFlags::new(144),
                field_type: MATERIALRELATIONPROPERTYPAIR_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialGridData, material_properties),
            },
            FieldInfoData {
                name: "InteractionGrid",
                flags: MemberInfoFlags::new(144),
                field_type: MATERIALINTERACTIONGRIDROW_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialGridData, interaction_grid),
            },
        ],
    }),
    array_type: Some(MATERIALGRIDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialGridData {
    fn type_info() -> &'static TypeInfo {
        MATERIALGRIDDATA_TYPE_INFO
    }
}


pub const MATERIALGRIDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialGridData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MaterialGridData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialInteractionGridRow {
    pub items: Vec<MaterialRelationPropertyPair>,
}

pub const MATERIALINTERACTIONGRIDROW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialInteractionGridRow",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Items",
                flags: MemberInfoFlags::new(144),
                field_type: MATERIALRELATIONPROPERTYPAIR_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialInteractionGridRow, items),
            },
        ],
    }),
    array_type: Some(MATERIALINTERACTIONGRIDROW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialInteractionGridRow {
    fn type_info() -> &'static TypeInfo {
        MATERIALINTERACTIONGRIDROW_TYPE_INFO
    }
}


pub const MATERIALINTERACTIONGRIDROW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialInteractionGridRow-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MaterialInteractionGridRow-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialRelationPropertyPair {
    pub physics_material_properties: Vec<PhysicsMaterialRelationPropertyData>,
    pub physics_property_properties: Vec<PhysicsPropertyRelationPropertyData>,
}

pub const MATERIALRELATIONPROPERTYPAIR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationPropertyPair",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PhysicsMaterialProperties",
                flags: MemberInfoFlags::new(144),
                field_type: PHYSICSMATERIALRELATIONPROPERTYDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationPropertyPair, physics_material_properties),
            },
            FieldInfoData {
                name: "PhysicsPropertyProperties",
                flags: MemberInfoFlags::new(144),
                field_type: PHYSICSPROPERTYRELATIONPROPERTYDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MaterialRelationPropertyPair, physics_property_properties),
            },
        ],
    }),
    array_type: Some(MATERIALRELATIONPROPERTYPAIR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationPropertyPair {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONPROPERTYPAIR_TYPE_INFO
    }
}


pub const MATERIALRELATIONPROPERTYPAIR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationPropertyPair-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MaterialRelationPropertyPair-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsPropertyRelationPropertyData {
}

pub const PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPropertyRelationPropertyData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSPROPERTYRELATIONPROPERTYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsPropertyRelationPropertyData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSPROPERTYRELATIONPROPERTYDATA_TYPE_INFO
    }
}


pub const PHYSICSPROPERTYRELATIONPROPERTYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsPropertyRelationPropertyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PhysicsPropertyRelationPropertyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsMaterialRelationPropertyData {
}

pub const PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsMaterialRelationPropertyData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MATERIALRELATIONPROPERTYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSMATERIALRELATIONPROPERTYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PhysicsMaterialRelationPropertyData {
    fn type_info() -> &'static TypeInfo {
        PHYSICSMATERIALRELATIONPROPERTYDATA_TYPE_INFO
    }
}


pub const PHYSICSMATERIALRELATIONPROPERTYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsMaterialRelationPropertyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PhysicsMaterialRelationPropertyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialRelationPropertyData {
}

pub const MATERIALRELATIONPROPERTYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationPropertyData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MATERIALRELATIONPROPERTYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MaterialRelationPropertyData {
    fn type_info() -> &'static TypeInfo {
        MATERIALRELATIONPROPERTYDATA_TYPE_INFO
    }
}


pub const MATERIALRELATIONPROPERTYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialRelationPropertyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MaterialRelationPropertyData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MaterialDecl {
    pub packed: u32,
}

pub const MATERIALDECL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialDecl",
    flags: MemberInfoFlags::new(32841),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Packed",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MaterialDecl, packed),
            },
        ],
    }),
    array_type: Some(MATERIALDECL_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MaterialDecl {
    fn type_info() -> &'static TypeInfo {
        MATERIALDECL_TYPE_INFO
    }
}


pub const MATERIALDECL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialDecl-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MaterialDecl-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringBuilderEntityData {
    pub realm: super::core::Realm,
    pub num_strings: i32,
}

pub const STRINGBUILDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringBuilderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(StringBuilderEntityData, realm),
            },
            FieldInfoData {
                name: "NumStrings",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StringBuilderEntityData, num_strings),
            },
        ],
    }),
    array_type: Some(STRINGBUILDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringBuilderEntityData {
    fn type_info() -> &'static TypeInfo {
        STRINGBUILDERENTITYDATA_TYPE_INFO
    }
}


pub const STRINGBUILDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringBuilderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("StringBuilderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec4ChangedEntityData {
    pub realm: super::core::Realm,
    pub value: super::core::Vec4,
    pub always_send: bool,
}

pub const VEC4CHANGEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4ChangedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYCHANGEDENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec4ChangedEntityData, realm),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(Vec4ChangedEntityData, value),
            },
            FieldInfoData {
                name: "AlwaysSend",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Vec4ChangedEntityData, always_send),
            },
        ],
    }),
    array_type: Some(VEC4CHANGEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec4ChangedEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC4CHANGEDENTITYDATA_TYPE_INFO
    }
}


pub const VEC4CHANGEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4ChangedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec4ChangedEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec3ChangedEntityData {
    pub realm: super::core::Realm,
    pub value: super::core::Vec3,
    pub always_send: bool,
}

pub const VEC3CHANGEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3ChangedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYCHANGEDENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec3ChangedEntityData, realm),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(Vec3ChangedEntityData, value),
            },
            FieldInfoData {
                name: "AlwaysSend",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Vec3ChangedEntityData, always_send),
            },
        ],
    }),
    array_type: Some(VEC3CHANGEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec3ChangedEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC3CHANGEDENTITYDATA_TYPE_INFO
    }
}


pub const VEC3CHANGEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3ChangedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec3ChangedEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec2ChangedEntityData {
    pub realm: super::core::Realm,
    pub value: super::core::Vec2,
    pub always_send: bool,
}

pub const VEC2CHANGEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2ChangedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYCHANGEDENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec2ChangedEntityData, realm),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(Vec2ChangedEntityData, value),
            },
            FieldInfoData {
                name: "AlwaysSend",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Vec2ChangedEntityData, always_send),
            },
        ],
    }),
    array_type: Some(VEC2CHANGEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec2ChangedEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC2CHANGEDENTITYDATA_TYPE_INFO
    }
}


pub const VEC2CHANGEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2ChangedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec2ChangedEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntChangedEntityData {
    pub realm: super::core::Realm,
    pub value: i32,
    pub always_send: bool,
}

pub const INTCHANGEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntChangedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYCHANGEDENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(IntChangedEntityData, realm),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IntChangedEntityData, value),
            },
            FieldInfoData {
                name: "AlwaysSend",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IntChangedEntityData, always_send),
            },
        ],
    }),
    array_type: Some(INTCHANGEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntChangedEntityData {
    fn type_info() -> &'static TypeInfo {
        INTCHANGEDENTITYDATA_TYPE_INFO
    }
}


pub const INTCHANGEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntChangedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntChangedEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatChangedEntityData {
    pub realm: super::core::Realm,
    pub value: f32,
    pub always_send: bool,
}

pub const FLOATCHANGEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatChangedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYCHANGEDENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(FloatChangedEntityData, realm),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatChangedEntityData, value),
            },
            FieldInfoData {
                name: "AlwaysSend",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FloatChangedEntityData, always_send),
            },
        ],
    }),
    array_type: Some(FLOATCHANGEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatChangedEntityData {
    fn type_info() -> &'static TypeInfo {
        FLOATCHANGEDENTITYDATA_TYPE_INFO
    }
}


pub const FLOATCHANGEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatChangedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatChangedEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolChangedEntityData {
    pub realm: super::core::Realm,
    pub value: bool,
    pub always_send: bool,
}

pub const BOOLCHANGEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolChangedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYCHANGEDENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(BoolChangedEntityData, realm),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoolChangedEntityData, value),
            },
            FieldInfoData {
                name: "AlwaysSend",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoolChangedEntityData, always_send),
            },
        ],
    }),
    array_type: Some(BOOLCHANGEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoolChangedEntityData {
    fn type_info() -> &'static TypeInfo {
        BOOLCHANGEDENTITYDATA_TYPE_INFO
    }
}


pub const BOOLCHANGEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolChangedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoolChangedEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyChangedEntityData {
}

pub const PROPERTYCHANGEDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyChangedEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROPERTYCHANGEDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertyChangedEntityData {
    fn type_info() -> &'static TypeInfo {
        PROPERTYCHANGEDENTITYDATA_TYPE_INFO
    }
}


pub const PROPERTYCHANGEDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyChangedEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PropertyChangedEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompareTransformEntityData {
    pub realm: super::core::Realm,
    pub a: super::core::LinearTransform,
    pub b: super::core::LinearTransform,
    pub trigger_on_property_change: bool,
    pub trigger_on_start: bool,
}

pub const COMPARETRANSFORMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareTransformEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CompareTransformEntityData, realm),
            },
            FieldInfoData {
                name: "A",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(CompareTransformEntityData, a),
            },
            FieldInfoData {
                name: "B",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(CompareTransformEntityData, b),
            },
            FieldInfoData {
                name: "TriggerOnPropertyChange",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompareTransformEntityData, trigger_on_property_change),
            },
            FieldInfoData {
                name: "TriggerOnStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompareTransformEntityData, trigger_on_start),
            },
        ],
    }),
    array_type: Some(COMPARETRANSFORMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CompareTransformEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPARETRANSFORMENTITYDATA_TYPE_INFO
    }
}


pub const COMPARETRANSFORMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareTransformEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareTransformEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwitchEnumEntityData {
    pub fire_on_property_changed: bool,
    pub output_events: Vec<EventName>,
}

pub const SWITCHENUMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwitchEnumEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMPLIEDENUMTYPELOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FireOnPropertyChanged",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SwitchEnumEntityData, fire_on_property_changed),
            },
            FieldInfoData {
                name: "OutputEvents",
                flags: MemberInfoFlags::new(144),
                field_type: EVENTNAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SwitchEnumEntityData, output_events),
            },
        ],
    }),
    array_type: Some(SWITCHENUMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SwitchEnumEntityData {
    fn type_info() -> &'static TypeInfo {
        SWITCHENUMENTITYDATA_TYPE_INFO
    }
}


pub const SWITCHENUMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwitchEnumEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SwitchEnumEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventName {
    pub enum_value: i32,
    pub hash: u32,
    pub is_connected: bool,
}

pub const EVENTNAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventName",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EnumValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EventName, enum_value),
            },
            FieldInfoData {
                name: "Hash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EventName, hash),
            },
            FieldInfoData {
                name: "IsConnected",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EventName, is_connected),
            },
        ],
    }),
    array_type: Some(EVENTNAME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EventName {
    fn type_info() -> &'static TypeInfo {
        EVENTNAME_TYPE_INFO
    }
}


pub const EVENTNAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventName-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventName-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SetEnumEntityData {
    pub input_events: Vec<SetEnumValueContainer>,
}

pub const SETENUMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetEnumEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EXPLICITENUMTYPELOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InputEvents",
                flags: MemberInfoFlags::new(144),
                field_type: SETENUMVALUECONTAINER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SetEnumEntityData, input_events),
            },
        ],
    }),
    array_type: Some(SETENUMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SetEnumEntityData {
    fn type_info() -> &'static TypeInfo {
        SETENUMENTITYDATA_TYPE_INFO
    }
}


pub const SETENUMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetEnumEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SetEnumEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SetEnumValueContainer {
    pub value: i32,
    pub input_event_hash: u32,
}

pub const SETENUMVALUECONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetEnumValueContainer",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SetEnumValueContainer, value),
            },
            FieldInfoData {
                name: "InputEventHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SetEnumValueContainer, input_event_hash),
            },
        ],
    }),
    array_type: Some(SETENUMVALUECONTAINER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SetEnumValueContainer {
    fn type_info() -> &'static TypeInfo {
        SETENUMVALUECONTAINER_TYPE_INFO
    }
}


pub const SETENUMVALUECONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetEnumValueContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SetEnumValueContainer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntToEnumEntityData {
    pub r#in: i32,
}

pub const INTTOENUMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntToEnumEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EXPLICITENUMTYPELOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IntToEnumEntityData, r#in),
            },
        ],
    }),
    array_type: Some(INTTOENUMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntToEnumEntityData {
    fn type_info() -> &'static TypeInfo {
        INTTOENUMENTITYDATA_TYPE_INFO
    }
}


pub const INTTOENUMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntToEnumEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntToEnumEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WriteArrayElementEntityData {
    pub element_index: i32,
}

pub const WRITEARRAYELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriteArrayElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WRITEVARIABLEBASEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ElementIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WriteArrayElementEntityData, element_index),
            },
        ],
    }),
    array_type: Some(WRITEARRAYELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WriteArrayElementEntityData {
    fn type_info() -> &'static TypeInfo {
        WRITEARRAYELEMENTENTITYDATA_TYPE_INFO
    }
}


pub const WRITEARRAYELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriteArrayElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WriteArrayElementEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReadArraySizeEntityData {
}

pub const READARRAYSIZEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadArraySizeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(READVARIABLEBASEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(READARRAYSIZEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReadArraySizeEntityData {
    fn type_info() -> &'static TypeInfo {
        READARRAYSIZEENTITYDATA_TYPE_INFO
    }
}


pub const READARRAYSIZEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadArraySizeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReadArraySizeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReadArrayElementEntityData {
    pub element_index: i32,
}

pub const READARRAYELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadArrayElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(READVARIABLEBASEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ElementIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ReadArrayElementEntityData, element_index),
            },
        ],
    }),
    array_type: Some(READARRAYELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReadArrayElementEntityData {
    fn type_info() -> &'static TypeInfo {
        READARRAYELEMENTENTITYDATA_TYPE_INFO
    }
}


pub const READARRAYELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadArrayElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReadArrayElementEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForEachVariableEntityData {
}

pub const FOREACHVARIABLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForEachVariableEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(READVARIABLEBASEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FOREACHVARIABLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ForEachVariableEntityData {
    fn type_info() -> &'static TypeInfo {
        FOREACHVARIABLEENTITYDATA_TYPE_INFO
    }
}


pub const FOREACHVARIABLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForEachVariableEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ForEachVariableEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClearArrayVariableEntityData {
}

pub const CLEARARRAYVARIABLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClearArrayVariableEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WRITEVARIABLEBASEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLEARARRAYVARIABLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClearArrayVariableEntityData {
    fn type_info() -> &'static TypeInfo {
        CLEARARRAYVARIABLEENTITYDATA_TYPE_INFO
    }
}


pub const CLEARARRAYVARIABLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClearArrayVariableEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ClearArrayVariableEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AddToArrayVariableEntityData {
}

pub const ADDTOARRAYVARIABLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AddToArrayVariableEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WRITEVARIABLEBASEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ADDTOARRAYVARIABLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AddToArrayVariableEntityData {
    fn type_info() -> &'static TypeInfo {
        ADDTOARRAYVARIABLEENTITYDATA_TYPE_INFO
    }
}


pub const ADDTOARRAYVARIABLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AddToArrayVariableEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AddToArrayVariableEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WriteVariableEntityData {
}

pub const WRITEVARIABLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriteVariableEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WRITEVARIABLEBASEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WRITEVARIABLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WriteVariableEntityData {
    fn type_info() -> &'static TypeInfo {
        WRITEVARIABLEENTITYDATA_TYPE_INFO
    }
}


pub const WRITEVARIABLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriteVariableEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WriteVariableEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReadVariableEntityData {
}

pub const READVARIABLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadVariableEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(READVARIABLEBASEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(READVARIABLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReadVariableEntityData {
    fn type_info() -> &'static TypeInfo {
        READVARIABLEENTITYDATA_TYPE_INFO
    }
}


pub const READVARIABLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadVariableEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReadVariableEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WriteVariableBaseEntityData {
}

pub const WRITEVARIABLEBASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriteVariableBaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VARIABLEBASEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WRITEVARIABLEBASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WriteVariableBaseEntityData {
    fn type_info() -> &'static TypeInfo {
        WRITEVARIABLEBASEENTITYDATA_TYPE_INFO
    }
}


pub const WRITEVARIABLEBASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriteVariableBaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WriteVariableBaseEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReadVariableBaseEntityData {
}

pub const READVARIABLEBASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadVariableBaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VARIABLEBASEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(READVARIABLEBASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReadVariableBaseEntityData {
    fn type_info() -> &'static TypeInfo {
        READVARIABLEBASEENTITYDATA_TYPE_INFO
    }
}


pub const READVARIABLEBASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadVariableBaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReadVariableBaseEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VariableBaseEntityData {
    pub realm: super::core::Realm,
    pub field_name_hash: u32,
    pub field_type: super::core::TypeRef,
    pub field_offset: u32,
    pub use_data: bool,
}

pub const VARIABLEBASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VariableBaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(VariableBaseEntityData, realm),
            },
            FieldInfoData {
                name: "FieldNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VariableBaseEntityData, field_name_hash),
            },
            FieldInfoData {
                name: "FieldType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(VariableBaseEntityData, field_type),
            },
            FieldInfoData {
                name: "FieldOffset",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VariableBaseEntityData, field_offset),
            },
            FieldInfoData {
                name: "UseData",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VariableBaseEntityData, use_data),
            },
        ],
    }),
    array_type: Some(VARIABLEBASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VariableBaseEntityData {
    fn type_info() -> &'static TypeInfo {
        VARIABLEBASEENTITYDATA_TYPE_INFO
    }
}


pub const VARIABLEBASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VariableBaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("VariableBaseEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HighlightEntityData {
    pub realm: super::core::Realm,
    pub color: super::core::Vec3,
    pub alpha: u8,
    pub top_left: super::core::Vec2,
    pub bottom_right: super::core::Vec2,
}

pub const HIGHLIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HighlightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(HighlightEntityData, realm),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HighlightEntityData, color),
            },
            FieldInfoData {
                name: "Alpha",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(HighlightEntityData, alpha),
            },
            FieldInfoData {
                name: "TopLeft",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(HighlightEntityData, top_left),
            },
            FieldInfoData {
                name: "BottomRight",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(HighlightEntityData, bottom_right),
            },
        ],
    }),
    array_type: Some(HIGHLIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HighlightEntityData {
    fn type_info() -> &'static TypeInfo {
        HIGHLIGHTENTITYDATA_TYPE_INFO
    }
}


pub const HIGHLIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HighlightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("HighlightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TextSequenceEntityData {
    pub realm: super::core::Realm,
    pub events: Vec<String>,
    pub items: Vec<TextSequenceItem>,
    pub screen_position: super::core::Vec2,
}

pub const TEXTSEQUENCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextSequenceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TextSequenceEntityData, realm),
            },
            FieldInfoData {
                name: "Events",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TextSequenceEntityData, events),
            },
            FieldInfoData {
                name: "Items",
                flags: MemberInfoFlags::new(144),
                field_type: TEXTSEQUENCEITEM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TextSequenceEntityData, items),
            },
            FieldInfoData {
                name: "ScreenPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(TextSequenceEntityData, screen_position),
            },
        ],
    }),
    array_type: Some(TEXTSEQUENCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextSequenceEntityData {
    fn type_info() -> &'static TypeInfo {
        TEXTSEQUENCEENTITYDATA_TYPE_INFO
    }
}


pub const TEXTSEQUENCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextSequenceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TextSequenceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TextSequenceItem {
    pub text: String,
    pub text_color: super::core::Vec3,
    pub screen_position: super::core::Vec2,
    pub use_entity_screen_position: bool,
    pub time_to_show: f32,
    pub trigger_event: String,
}

pub const TEXTSEQUENCEITEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextSequenceItem",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Text",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TextSequenceItem, text),
            },
            FieldInfoData {
                name: "TextColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TextSequenceItem, text_color),
            },
            FieldInfoData {
                name: "ScreenPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(TextSequenceItem, screen_position),
            },
            FieldInfoData {
                name: "UseEntityScreenPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextSequenceItem, use_entity_screen_position),
            },
            FieldInfoData {
                name: "TimeToShow",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TextSequenceItem, time_to_show),
            },
            FieldInfoData {
                name: "TriggerEvent",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TextSequenceItem, trigger_event),
            },
        ],
    }),
    array_type: Some(TEXTSEQUENCEITEM_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TextSequenceItem {
    fn type_info() -> &'static TypeInfo {
        TEXTSEQUENCEITEM_TYPE_INFO
    }
}


pub const TEXTSEQUENCEITEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextSequenceItem-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TextSequenceItem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TreeBase {
}

pub const TREEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TreeBase",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TREEBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TreeBase {
    fn type_info() -> &'static TypeInfo {
        TREEBASE_TYPE_INFO
    }
}


pub const TREEBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TreeBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TreeBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TreeNodeBase {
}

pub const TREENODEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TreeNodeBase",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TREENODEBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TreeNodeBase {
    fn type_info() -> &'static TypeInfo {
        TREENODEBASE_TYPE_INFO
    }
}


pub const TREENODEBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TreeNodeBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TreeNodeBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelDestroyedMessage {
}

pub const SUBLEVELDESTROYEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelDestroyedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelDestroyedMessage {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELDESTROYEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelEntitiesCreatedMessage {
}

pub const SUBLEVELENTITIESCREATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelEntitiesCreatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SubLevelEntitiesCreatedMessage {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELENTITIESCREATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReportInstallationProgressEntityData {
    pub install_group_name: String,
}

pub const REPORTINSTALLATIONPROGRESSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReportInstallationProgressEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InstallGroupName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ReportInstallationProgressEntityData, install_group_name),
            },
        ],
    }),
    array_type: Some(REPORTINSTALLATIONPROGRESSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReportInstallationProgressEntityData {
    fn type_info() -> &'static TypeInfo {
        REPORTINSTALLATIONPROGRESSENTITYDATA_TYPE_INFO
    }
}


pub const REPORTINSTALLATIONPROGRESSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReportInstallationProgressEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReportInstallationProgressEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalPlayerIdEntityData {
    pub realm: super::core::Realm,
    pub default_value: super::core::LocalPlayerId,
    pub r#in: super::core::LocalPlayerId,
}

pub const LOCALPLAYERIDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerIdEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LocalPlayerIdEntityData, realm),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(LocalPlayerIdEntityData, default_value),
            },
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(LocalPlayerIdEntityData, r#in),
            },
        ],
    }),
    array_type: Some(LOCALPLAYERIDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalPlayerIdEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALPLAYERIDENTITYDATA_TYPE_INFO
    }
}


pub const LOCALPLAYERIDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerIdEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LocalPlayerIdEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatCacheEntityData {
    pub realm: super::core::Realm,
    pub load_on_create: bool,
    pub in_value: f32,
}

pub const FLOATCACHEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCacheEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(FloatCacheEntityData, realm),
            },
            FieldInfoData {
                name: "LoadOnCreate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FloatCacheEntityData, load_on_create),
            },
            FieldInfoData {
                name: "InValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatCacheEntityData, in_value),
            },
        ],
    }),
    array_type: Some(FLOATCACHEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatCacheEntityData {
    fn type_info() -> &'static TypeInfo {
        FLOATCACHEENTITYDATA_TYPE_INFO
    }
}


pub const FLOATCACHEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCacheEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatCacheEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MultilineStringEntityData {
    pub realm: super::core::Realm,
    pub value: String,
}

pub const MULTILINESTRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultilineStringEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(MultilineStringEntityData, realm),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MultilineStringEntityData, value),
            },
        ],
    }),
    array_type: Some(MULTILINESTRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MultilineStringEntityData {
    fn type_info() -> &'static TypeInfo {
        MULTILINESTRINGENTITYDATA_TYPE_INFO
    }
}


pub const MULTILINESTRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultilineStringEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MultilineStringEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringEntityData {
    pub realm: super::core::Realm,
    pub default_string: String,
}

pub const STRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(StringEntityData, realm),
            },
            FieldInfoData {
                name: "DefaultString",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(StringEntityData, default_string),
            },
        ],
    }),
    array_type: Some(STRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringEntityData {
    fn type_info() -> &'static TypeInfo {
        STRINGENTITYDATA_TYPE_INFO
    }
}


pub const STRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("StringEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformEntityData {
    pub realm: super::core::Realm,
    pub default_transform: super::core::LinearTransform,
}

pub const TRANSFORMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransformEntityData, realm),
            },
            FieldInfoData {
                name: "DefaultTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformEntityData, default_transform),
            },
        ],
    }),
    array_type: Some(TRANSFORMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AxisAlignedBoxEntityData {
    pub realm: super::core::Realm,
    pub aabb_min: super::core::Vec3,
    pub aabb_max: super::core::Vec3,
}

pub const AXISALIGNEDBOXENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxisAlignedBoxEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(AxisAlignedBoxEntityData, realm),
            },
            FieldInfoData {
                name: "AabbMin",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AxisAlignedBoxEntityData, aabb_min),
            },
            FieldInfoData {
                name: "AabbMax",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AxisAlignedBoxEntityData, aabb_max),
            },
        ],
    }),
    array_type: Some(AXISALIGNEDBOXENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AxisAlignedBoxEntityData {
    fn type_info() -> &'static TypeInfo {
        AXISALIGNEDBOXENTITYDATA_TYPE_INFO
    }
}


pub const AXISALIGNEDBOXENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxisAlignedBoxEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AxisAlignedBoxEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vector4EntityData {
    pub realm: super::core::Realm,
    pub default_vec4: super::core::Vec4,
}

pub const VECTOR4ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vector4EntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vector4EntityData, realm),
            },
            FieldInfoData {
                name: "DefaultVec4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(Vector4EntityData, default_vec4),
            },
        ],
    }),
    array_type: Some(VECTOR4ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vector4EntityData {
    fn type_info() -> &'static TypeInfo {
        VECTOR4ENTITYDATA_TYPE_INFO
    }
}


pub const VECTOR4ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vector4EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vector4EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vector3EntityData {
    pub realm: super::core::Realm,
    pub default_vec3: super::core::Vec3,
}

pub const VECTOR3ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vector3EntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vector3EntityData, realm),
            },
            FieldInfoData {
                name: "DefaultVec3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(Vector3EntityData, default_vec3),
            },
        ],
    }),
    array_type: Some(VECTOR3ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vector3EntityData {
    fn type_info() -> &'static TypeInfo {
        VECTOR3ENTITYDATA_TYPE_INFO
    }
}


pub const VECTOR3ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vector3EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vector3EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatEntityData {
    pub realm: super::core::Realm,
    pub default_value: f32,
    pub inc_dec_value: f32,
}

pub const FLOATENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(FloatEntityData, realm),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatEntityData, default_value),
            },
            FieldInfoData {
                name: "IncDecValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatEntityData, inc_dec_value),
            },
        ],
    }),
    array_type: Some(FLOATENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatEntityData {
    fn type_info() -> &'static TypeInfo {
        FLOATENTITYDATA_TYPE_INFO
    }
}


pub const FLOATENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIntEntityData {
    pub realm: super::core::Realm,
    pub default_value: u32,
    pub inc_dec_value: u32,
}

pub const UINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIntEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(UIntEntityData, realm),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UIntEntityData, default_value),
            },
            FieldInfoData {
                name: "IncDecValue",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UIntEntityData, inc_dec_value),
            },
        ],
    }),
    array_type: Some(UINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIntEntityData {
    fn type_info() -> &'static TypeInfo {
        UINTENTITYDATA_TYPE_INFO
    }
}


pub const UINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIntEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("UIntEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntEntityData {
    pub realm: super::core::Realm,
    pub default_value: i32,
    pub inc_dec_value: i32,
}

pub const INTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(IntEntityData, realm),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IntEntityData, default_value),
            },
            FieldInfoData {
                name: "IncDecValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IntEntityData, inc_dec_value),
            },
        ],
    }),
    array_type: Some(INTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntEntityData {
    fn type_info() -> &'static TypeInfo {
        INTENTITYDATA_TYPE_INFO
    }
}


pub const INTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolEntityData {
    pub realm: super::core::Realm,
    pub default_value: bool,
}

pub const BOOLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(BoolEntityData, realm),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoolEntityData, default_value),
            },
        ],
    }),
    array_type: Some(BOOLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoolEntityData {
    fn type_info() -> &'static TypeInfo {
        BOOLENTITYDATA_TYPE_INFO
    }
}


pub const BOOLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoolEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MasterSkeletonAsset {
    pub master_skeleton: SkeletonAsset,
    pub sub_skeletons: Vec<SubSkeleton>,
}

pub const MASTERSKELETONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MasterSkeletonAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MasterSkeleton",
                flags: MemberInfoFlags::new(0),
                field_type: SKELETONASSET_TYPE_INFO,
                rust_offset: offset_of!(MasterSkeletonAsset, master_skeleton),
            },
            FieldInfoData {
                name: "SubSkeletons",
                flags: MemberInfoFlags::new(144),
                field_type: SUBSKELETON_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MasterSkeletonAsset, sub_skeletons),
            },
        ],
    }),
    array_type: Some(MASTERSKELETONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MasterSkeletonAsset {
    fn type_info() -> &'static TypeInfo {
        MASTERSKELETONASSET_TYPE_INFO
    }
}


pub const MASTERSKELETONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MasterSkeletonAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MasterSkeletonAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SubSkeleton {
    pub skeleton: SkeletonAsset,
    pub bone_map: Vec<i32>,
    pub transform_map: Vec<super::core::LinearTransform>,
}

pub const SUBSKELETON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSkeleton",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Skeleton",
                flags: MemberInfoFlags::new(0),
                field_type: SKELETONASSET_TYPE_INFO,
                rust_offset: offset_of!(SubSkeleton, skeleton),
            },
            FieldInfoData {
                name: "BoneMap",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SubSkeleton, bone_map),
            },
            FieldInfoData {
                name: "TransformMap",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARTRANSFORM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SubSkeleton, transform_map),
            },
        ],
    }),
    array_type: Some(SUBSKELETON_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SubSkeleton {
    fn type_info() -> &'static TypeInfo {
        SUBSKELETON_TYPE_INFO
    }
}


pub const SUBSKELETON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSkeleton-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SubSkeleton-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkeletonAsset {
    pub bone_names: Vec<String>,
    pub bone_name_hashes: Vec<u32>,
    pub hierarchy: Vec<i32>,
    pub local_pose: Vec<super::core::LinearTransform>,
    pub model_pose: Vec<super::core::LinearTransform>,
    pub inverse_model_pose: Vec<super::core::LinearTransform>,
    pub server_skeleton_to_skeleton_map: Vec<i32>,
    pub skeleton_to_server_skeleton_map: Vec<i32>,
    pub server_hierarchy: Vec<i32>,
    pub gameplay_bones_to_skeleton: Vec<i32>,
    pub gameplay_bones_to_server_skeleton: Vec<i32>,
}

pub const SKELETONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkeletonAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SKELETONBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BoneNames",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, bone_names),
            },
            FieldInfoData {
                name: "BoneNameHashes",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, bone_name_hashes),
            },
            FieldInfoData {
                name: "Hierarchy",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, hierarchy),
            },
            FieldInfoData {
                name: "LocalPose",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARTRANSFORM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, local_pose),
            },
            FieldInfoData {
                name: "ModelPose",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARTRANSFORM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, model_pose),
            },
            FieldInfoData {
                name: "InverseModelPose",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARTRANSFORM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, inverse_model_pose),
            },
            FieldInfoData {
                name: "ServerSkeletonToSkeletonMap",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, server_skeleton_to_skeleton_map),
            },
            FieldInfoData {
                name: "SkeletonToServerSkeletonMap",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, skeleton_to_server_skeleton_map),
            },
            FieldInfoData {
                name: "ServerHierarchy",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, server_hierarchy),
            },
            FieldInfoData {
                name: "GameplayBonesToSkeleton",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, gameplay_bones_to_skeleton),
            },
            FieldInfoData {
                name: "GameplayBonesToServerSkeleton",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkeletonAsset, gameplay_bones_to_server_skeleton),
            },
        ],
    }),
    array_type: Some(SKELETONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SkeletonAsset {
    fn type_info() -> &'static TypeInfo {
        SKELETONASSET_TYPE_INFO
    }
}


pub const SKELETONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkeletonAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SkeletonAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoneSelection {
    pub bone_name_hashes: Vec<u32>,
}

pub const BONESELECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneSelection",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BoneNameHashes",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(BoneSelection, bone_name_hashes),
            },
        ],
    }),
    array_type: Some(BONESELECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoneSelection {
    fn type_info() -> &'static TypeInfo {
        BONESELECTION_TYPE_INFO
    }
}


pub const BONESELECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneSelection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoneSelection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameplayBone {
    pub name: String,
    pub bone: GameplayBones,
}

pub const GAMEPLAYBONE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayBone",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameplayBone, name),
            },
            FieldInfoData {
                name: "Bone",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLAYBONES_TYPE_INFO,
                rust_offset: offset_of!(GameplayBone, bone),
            },
        ],
    }),
    array_type: Some(GAMEPLAYBONE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameplayBone {
    fn type_info() -> &'static TypeInfo {
        GAMEPLAYBONE_TYPE_INFO
    }
}


pub const GAMEPLAYBONE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayBone-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("GameplayBone-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GameplayBones {
    #[default]
    GameplayBones_UndefinedBone = 4294967295,
    GameplayBones_RootBone = 0,
    GameplayBones_RootMeshBone = 1,
    GameplayBones_ConnectBone = 2,
    GameplayBones_HeadBone = 3,
    GameplayBones_CameraBone = 4,
    GameplayBones_AimBone = 5,
    GameplayBones_WeaponBone = 6,
    GameplayBones_WeaponBone2 = 7,
    GameplayBones_WeaponAux1Bone = 8,
    GameplayBones_LeftArmBone = 9,
    GameplayBones_RightArmBone = 10,
    GameplayBones_SpineBone = 11,
    GameplayBones_WeaponMuzzleBone = 12,
    GameplayBones_WeaponShellEjectBone = 13,
    GameplayBones_LeftHandBone = 14,
    GameplayBones_RightHandBone = 15,
    GameplayBones_LeftFootBone = 16,
    GameplayBones_RightFootBone = 17,
    GameplayBones_BackPack = 18,
    GameplayBones_PleaseDeleteMeHackDroidBone = 19,
    GameplayBones_WeaponMuzzleBone2 = 20,
    GameplayBones_WeaponMuzzleBone3 = 21,
    GameplayBones_WeaponMuzzleBone4 = 22,
    GameplayBones_WeaponMuzzleBone5 = 23,
    GameplayBones_Count = 24,
}

pub const GAMEPLAYBONES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayBones",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(GAMEPLAYBONES_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GameplayBones {
    fn type_info() -> &'static TypeInfo {
        GAMEPLAYBONES_TYPE_INFO
    }
}


pub const GAMEPLAYBONES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameplayBones-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("GameplayBones-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SharedBundleReference {
    pub name: String,
    pub heap: BundleHeapInfo,
}

pub const SHAREDBUNDLEREFERENCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedBundleReference",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SharedBundleReference, name),
            },
            FieldInfoData {
                name: "Heap",
                flags: MemberInfoFlags::new(0),
                field_type: BUNDLEHEAPINFO_TYPE_INFO,
                rust_offset: offset_of!(SharedBundleReference, heap),
            },
        ],
    }),
    array_type: Some(SHAREDBUNDLEREFERENCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SharedBundleReference {
    fn type_info() -> &'static TypeInfo {
        SHAREDBUNDLEREFERENCE_TYPE_INFO
    }
}


pub const SHAREDBUNDLEREFERENCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedBundleReference-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SharedBundleReference-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SharedBundleBaseAsset {
}

pub const SHAREDBUNDLEBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedBundleBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHAREDBUNDLEBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SharedBundleBaseAsset {
    fn type_info() -> &'static TypeInfo {
        SHAREDBUNDLEBASEASSET_TYPE_INFO
    }
}


pub const SHAREDBUNDLEBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedBundleBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SharedBundleBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ScanlineVolumeShapeData {
    pub x_step: f32,
    pub y_step: f32,
    pub z_step: f32,
}

pub const SCANLINEVOLUMESHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScanlineVolumeShapeData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VOLUMEVECTORSHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "xStep",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScanlineVolumeShapeData, x_step),
            },
            FieldInfoData {
                name: "yStep",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScanlineVolumeShapeData, y_step),
            },
            FieldInfoData {
                name: "zStep",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ScanlineVolumeShapeData, z_step),
            },
        ],
    }),
    array_type: Some(SCANLINEVOLUMESHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScanlineVolumeShapeData {
    fn type_info() -> &'static TypeInfo {
        SCANLINEVOLUMESHAPEDATA_TYPE_INFO
    }
}


pub const SCANLINEVOLUMESHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScanlineVolumeShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ScanlineVolumeShapeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ZoneVectorShapeData {
}

pub const ZONEVECTORSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneVectorShapeData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VECTORSHAPEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ZONEVECTORSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneVectorShapeData {
    fn type_info() -> &'static TypeInfo {
        ZONEVECTORSHAPEDATA_TYPE_INFO
    }
}


pub const ZONEVECTORSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneVectorShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ZoneVectorShapeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VolumeVectorShapeData {
    pub height: f32,
}

pub const VOLUMEVECTORSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumeVectorShapeData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VECTORSHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VolumeVectorShapeData, height),
            },
        ],
    }),
    array_type: Some(VOLUMEVECTORSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VolumeVectorShapeData {
    fn type_info() -> &'static TypeInfo {
        VOLUMEVECTORSHAPEDATA_TYPE_INFO
    }
}


pub const VOLUMEVECTORSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VolumeVectorShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("VolumeVectorShapeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CustomSplineData {
}

pub const CUSTOMSPLINEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomSplineData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VECTORSHAPEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CUSTOMSPLINEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomSplineData {
    fn type_info() -> &'static TypeInfo {
        CUSTOMSPLINEDATA_TYPE_INFO
    }
}


pub const CUSTOMSPLINEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomSplineData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CustomSplineData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VectorShapeData {
    pub points: Vec<super::core::Vec3>,
    pub tension: f32,
    pub is_closed: bool,
    pub allow_roll: bool,
    pub allow_yaw_pitch: bool,
}

pub const VECTORSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorShapeData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Points",
                flags: MemberInfoFlags::new(144),
                field_type: VEC3_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VectorShapeData, points),
            },
            FieldInfoData {
                name: "Tension",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VectorShapeData, tension),
            },
            FieldInfoData {
                name: "IsClosed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VectorShapeData, is_closed),
            },
            FieldInfoData {
                name: "AllowRoll",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VectorShapeData, allow_roll),
            },
            FieldInfoData {
                name: "AllowYawPitch",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VectorShapeData, allow_yaw_pitch),
            },
        ],
    }),
    array_type: Some(VECTORSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VectorShapeData {
    fn type_info() -> &'static TypeInfo {
        VECTORSHAPEDATA_TYPE_INFO
    }
}


pub const VECTORSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("VectorShapeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OBBData {
    pub transform: super::core::LinearTransform,
    pub half_extents: super::core::Vec3,
}

pub const OBBDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OBBData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(OBBData, transform),
            },
            FieldInfoData {
                name: "HalfExtents",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(OBBData, half_extents),
            },
        ],
    }),
    array_type: Some(OBBDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OBBData {
    fn type_info() -> &'static TypeInfo {
        OBBDATA_TYPE_INFO
    }
}


pub const OBBDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OBBData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("OBBData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AABBData {
    pub position: super::core::Vec3,
    pub half_extents: super::core::Vec3,
}

pub const AABBDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AABBData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AABBData, position),
            },
            FieldInfoData {
                name: "HalfExtents",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AABBData, half_extents),
            },
        ],
    }),
    array_type: Some(AABBDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AABBData {
    fn type_info() -> &'static TypeInfo {
        AABBDATA_TYPE_INFO
    }
}


pub const AABBDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AABBData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AABBData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SphereData {
    pub position: super::core::Vec3,
    pub radius: f32,
}

pub const SPHEREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SphereData, position),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereData, radius),
            },
        ],
    }),
    array_type: Some(SPHEREDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SphereData {
    fn type_info() -> &'static TypeInfo {
        SPHEREDATA_TYPE_INFO
    }
}


pub const SPHEREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SphereData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BaseShapeData {
}

pub const BASESHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShapeData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESHAPEDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BASESHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BaseShapeData {
    fn type_info() -> &'static TypeInfo {
        BASESHAPEDATA_TYPE_INFO
    }
}


pub const BASESHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BaseShapeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectVec4EntityData {
}

pub const SELECTVEC4ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec4EntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SELECTPROPERTYENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTVEC4ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectVec4EntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTVEC4ENTITYDATA_TYPE_INFO
    }
}


pub const SELECTVEC4ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec4EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectVec4EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectVec3EntityData {
}

pub const SELECTVEC3ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec3EntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SELECTPROPERTYENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTVEC3ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectVec3EntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTVEC3ENTITYDATA_TYPE_INFO
    }
}


pub const SELECTVEC3ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec3EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectVec3EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectVec2EntityData {
}

pub const SELECTVEC2ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec2EntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SELECTPROPERTYENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTVEC2ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectVec2EntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTVEC2ENTITYDATA_TYPE_INFO
    }
}


pub const SELECTVEC2ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec2EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectVec2EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectTransformEntityData {
}

pub const SELECTTRANSFORMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectTransformEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SELECTPROPERTYENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTTRANSFORMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectTransformEntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTTRANSFORMENTITYDATA_TYPE_INFO
    }
}


pub const SELECTTRANSFORMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectTransformEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectTransformEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectBoolEntityData {
}

pub const SELECTBOOLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectBoolEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SELECTPROPERTYENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTBOOLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectBoolEntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTBOOLENTITYDATA_TYPE_INFO
    }
}


pub const SELECTBOOLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectBoolEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectBoolEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectStringEntityData {
}

pub const SELECTSTRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectStringEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SELECTPROPERTYENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTSTRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectStringEntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTSTRINGENTITYDATA_TYPE_INFO
    }
}


pub const SELECTSTRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectStringEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectStringEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectInt64EntityData {
}

pub const SELECTINT64ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectInt64EntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SELECTPROPERTYENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTINT64ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectInt64EntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTINT64ENTITYDATA_TYPE_INFO
    }
}


pub const SELECTINT64ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectInt64EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectInt64EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectIntEntityData {
}

pub const SELECTINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectIntEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SELECTPROPERTYENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectIntEntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTINTENTITYDATA_TYPE_INFO
    }
}


pub const SELECTINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectIntEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectIntEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectFloatEntityData {
}

pub const SELECTFLOATENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectFloatEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SELECTPROPERTYENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTFLOATENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectFloatEntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTFLOATENTITYDATA_TYPE_INFO
    }
}


pub const SELECTFLOATENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectFloatEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectFloatEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectPropertyEntityData {
    pub realm: super::core::Realm,
    pub inputs: Vec<String>,
    pub input_select: i32,
}

pub const SELECTPROPERTYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectPropertyEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SelectPropertyEntityData, realm),
            },
            FieldInfoData {
                name: "Inputs",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SelectPropertyEntityData, inputs),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SelectPropertyEntityData, input_select),
            },
        ],
    }),
    array_type: Some(SELECTPROPERTYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectPropertyEntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTPROPERTYENTITYDATA_TYPE_INFO
    }
}


pub const SELECTPROPERTYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectPropertyEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectPropertyEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingObjectCategoryAsset {
}

pub const PATHFINDINGOBJECTCATEGORYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingObjectCategoryAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PATHFINDINGOBJECTCATEGORYASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingObjectCategoryAsset {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGOBJECTCATEGORYASSET_TYPE_INFO
    }
}


pub const PATHFINDINGOBJECTCATEGORYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingObjectCategoryAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PathfindingObjectCategoryAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ObjectAreaTriggerEntityData {
    pub realm: super::core::Realm,
    pub time_out: f32,
}

pub const OBJECTAREATRIGGERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectAreaTriggerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ObjectAreaTriggerEntityData, realm),
            },
            FieldInfoData {
                name: "TimeOut",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectAreaTriggerEntityData, time_out),
            },
        ],
    }),
    array_type: Some(OBJECTAREATRIGGERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObjectAreaTriggerEntityData {
    fn type_info() -> &'static TypeInfo {
        OBJECTAREATRIGGERENTITYDATA_TYPE_INFO
    }
}


pub const OBJECTAREATRIGGERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectAreaTriggerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectAreaTriggerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectVariationToIntEntityData {
    pub realm: super::core::Realm,
}

pub const OBJECTVARIATIONTOINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationToIntEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ObjectVariationToIntEntityData, realm),
            },
        ],
    }),
    array_type: Some(OBJECTVARIATIONTOINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObjectVariationToIntEntityData {
    fn type_info() -> &'static TypeInfo {
        OBJECTVARIATIONTOINTENTITYDATA_TYPE_INFO
    }
}


pub const OBJECTVARIATIONTOINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationToIntEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectVariationToIntEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformSpaceEntityData {
    pub realm: super::core::Realm,
}

pub const TRANSFORMSPACEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransformSpaceEntityData, realm),
            },
        ],
    }),
    array_type: Some(TRANSFORMSPACEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TransformSpaceEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMSPACEENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMSPACEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformSpaceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BalancedDilationEntityData {
    pub realm: super::core::Realm,
    pub dilation_value: f32,
    pub real_time_duration: f32,
    pub recovery_time: f32,
    pub dilation_shape: TimeShape,
    pub recovery_shape: TimeShape,
}

pub const BALANCEDDILATIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BalancedDilationEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(BalancedDilationEntityData, realm),
            },
            FieldInfoData {
                name: "DilationValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BalancedDilationEntityData, dilation_value),
            },
            FieldInfoData {
                name: "RealTimeDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BalancedDilationEntityData, real_time_duration),
            },
            FieldInfoData {
                name: "RecoveryTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BalancedDilationEntityData, recovery_time),
            },
            FieldInfoData {
                name: "DilationShape",
                flags: MemberInfoFlags::new(0),
                field_type: TIMESHAPE_TYPE_INFO,
                rust_offset: offset_of!(BalancedDilationEntityData, dilation_shape),
            },
            FieldInfoData {
                name: "RecoveryShape",
                flags: MemberInfoFlags::new(0),
                field_type: TIMESHAPE_TYPE_INFO,
                rust_offset: offset_of!(BalancedDilationEntityData, recovery_shape),
            },
        ],
    }),
    array_type: Some(BALANCEDDILATIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BalancedDilationEntityData {
    fn type_info() -> &'static TypeInfo {
        BALANCEDDILATIONENTITYDATA_TYPE_INFO
    }
}


pub const BALANCEDDILATIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BalancedDilationEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BalancedDilationEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DilationEntityData {
    pub fade_in_time: f32,
    pub fade_out_time: f32,
    pub real_time_duration: f32,
    pub priority: DilationPriority,
    pub dilation_value: f32,
    pub time_delta_type: TimeDeltaType,
}

pub const DILATIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DilationEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FadeInTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DilationEntityData, fade_in_time),
            },
            FieldInfoData {
                name: "FadeOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DilationEntityData, fade_out_time),
            },
            FieldInfoData {
                name: "RealTimeDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DilationEntityData, real_time_duration),
            },
            FieldInfoData {
                name: "Priority",
                flags: MemberInfoFlags::new(0),
                field_type: DILATIONPRIORITY_TYPE_INFO,
                rust_offset: offset_of!(DilationEntityData, priority),
            },
            FieldInfoData {
                name: "DilationValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DilationEntityData, dilation_value),
            },
            FieldInfoData {
                name: "TimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: TIMEDELTATYPE_TYPE_INFO,
                rust_offset: offset_of!(DilationEntityData, time_delta_type),
            },
        ],
    }),
    array_type: Some(DILATIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DilationEntityData {
    fn type_info() -> &'static TypeInfo {
        DILATIONENTITYDATA_TYPE_INFO
    }
}


pub const DILATIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DilationEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DilationEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatSelectEntityData {
    pub realm: super::core::Realm,
    pub in_pos_or_zero: f32,
    pub in_neg: f32,
    pub select: f32,
}

pub const FLOATSELECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatSelectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(FloatSelectEntityData, realm),
            },
            FieldInfoData {
                name: "InPosOrZero",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatSelectEntityData, in_pos_or_zero),
            },
            FieldInfoData {
                name: "InNeg",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatSelectEntityData, in_neg),
            },
            FieldInfoData {
                name: "Select",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatSelectEntityData, select),
            },
        ],
    }),
    array_type: Some(FLOATSELECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatSelectEntityData {
    fn type_info() -> &'static TypeInfo {
        FLOATSELECTENTITYDATA_TYPE_INFO
    }
}


pub const FLOATSELECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatSelectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatSelectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatCurveEntityData {
    pub realm: super::core::Realm,
    pub r#in: f32,
    pub output_integral: bool,
    pub curve: super::core::FloatCurve,
}

pub const FLOATCURVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(FloatCurveEntityData, realm),
            },
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatCurveEntityData, r#in),
            },
            FieldInfoData {
                name: "OutputIntegral",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FloatCurveEntityData, output_integral),
            },
            FieldInfoData {
                name: "Curve",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVE_TYPE_INFO,
                rust_offset: offset_of!(FloatCurveEntityData, curve),
            },
        ],
    }),
    array_type: Some(FLOATCURVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatCurveEntityData {
    fn type_info() -> &'static TypeInfo {
        FLOATCURVEENTITYDATA_TYPE_INFO
    }
}


pub const FLOATCURVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatCurveEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalPlayerGateEntityData {
    pub realm: super::core::Realm,
}

pub const LOCALPLAYERGATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerGateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LocalPlayerGateEntityData, realm),
            },
        ],
    }),
    array_type: Some(LOCALPLAYERGATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalPlayerGateEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCALPLAYERGATEENTITYDATA_TYPE_INFO
    }
}


pub const LOCALPLAYERGATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerGateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LocalPlayerGateEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformSelectorEntityData {
    pub realm: super::core::Realm,
    pub in1: super::core::LinearTransform,
    pub in2: super::core::LinearTransform,
    pub selection: bool,
}

pub const TRANSFORMSELECTORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSelectorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransformSelectorEntityData, realm),
            },
            FieldInfoData {
                name: "In1",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformSelectorEntityData, in1),
            },
            FieldInfoData {
                name: "In2",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformSelectorEntityData, in2),
            },
            FieldInfoData {
                name: "Selection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransformSelectorEntityData, selection),
            },
        ],
    }),
    array_type: Some(TRANSFORMSELECTORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformSelectorEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMSELECTORENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMSELECTORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSelectorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformSelectorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SettingEntityData {
    pub realm: super::core::Realm,
    pub bool_setting_name: String,
    pub int_setting_name: String,
    pub float_setting_name: String,
    pub uint_setting_name: String,
}

pub const SETTINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SettingEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SettingEntityData, realm),
            },
            FieldInfoData {
                name: "BoolSettingName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SettingEntityData, bool_setting_name),
            },
            FieldInfoData {
                name: "IntSettingName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SettingEntityData, int_setting_name),
            },
            FieldInfoData {
                name: "FloatSettingName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SettingEntityData, float_setting_name),
            },
            FieldInfoData {
                name: "UintSettingName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SettingEntityData, uint_setting_name),
            },
        ],
    }),
    array_type: Some(SETTINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SettingEntityData {
    fn type_info() -> &'static TypeInfo {
        SETTINGENTITYDATA_TYPE_INFO
    }
}


pub const SETTINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SettingEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SettingEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IntegratorOrDifferentiatorEntityData {
    pub realm: super::core::Realm,
    pub start_value: f32,
    pub input: f32,
    pub bounded: bool,
    pub max_value: f32,
    pub min_value: f32,
}

pub const INTEGRATORORDIFFERENTIATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegratorOrDifferentiatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(IntegratorOrDifferentiatorEntityData, realm),
            },
            FieldInfoData {
                name: "StartValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IntegratorOrDifferentiatorEntityData, start_value),
            },
            FieldInfoData {
                name: "Input",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IntegratorOrDifferentiatorEntityData, input),
            },
            FieldInfoData {
                name: "Bounded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(IntegratorOrDifferentiatorEntityData, bounded),
            },
            FieldInfoData {
                name: "MaxValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IntegratorOrDifferentiatorEntityData, max_value),
            },
            FieldInfoData {
                name: "MinValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IntegratorOrDifferentiatorEntityData, min_value),
            },
        ],
    }),
    array_type: Some(INTEGRATORORDIFFERENTIATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntegratorOrDifferentiatorEntityData {
    fn type_info() -> &'static TypeInfo {
        INTEGRATORORDIFFERENTIATORENTITYDATA_TYPE_INFO
    }
}


pub const INTEGRATORORDIFFERENTIATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegratorOrDifferentiatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntegratorOrDifferentiatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileEntityData {
    pub realm: super::core::Realm,
    pub profile_name: String,
}

pub const PROFILEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ProfileEntityData, realm),
            },
            FieldInfoData {
                name: "ProfileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ProfileEntityData, profile_name),
            },
        ],
    }),
    array_type: Some(PROFILEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProfileEntityData {
    fn type_info() -> &'static TypeInfo {
        PROFILEENTITYDATA_TYPE_INFO
    }
}


pub const PROFILEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ProfileEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompareEntityData {
    pub realm: super::core::Realm,
    pub operator: CompareOp,
    pub float_in0: f32,
    pub float_in1: f32,
    pub int_in0: i32,
    pub int_in1: i32,
}

pub const COMPAREENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CompareEntityData, realm),
            },
            FieldInfoData {
                name: "Operator",
                flags: MemberInfoFlags::new(0),
                field_type: COMPAREOP_TYPE_INFO,
                rust_offset: offset_of!(CompareEntityData, operator),
            },
            FieldInfoData {
                name: "FloatIn0",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompareEntityData, float_in0),
            },
            FieldInfoData {
                name: "FloatIn1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompareEntityData, float_in1),
            },
            FieldInfoData {
                name: "IntIn0",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CompareEntityData, int_in0),
            },
            FieldInfoData {
                name: "IntIn1",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CompareEntityData, int_in1),
            },
        ],
    }),
    array_type: Some(COMPAREENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CompareOp {
    #[default]
    CompareOp_Equal = 0,
    CompareOp_NotEqual = 1,
    CompareOp_Greater = 2,
    CompareOp_Less = 3,
    CompareOp_GreaterOrEqual = 4,
    CompareOp_LessOrEqual = 5,
}

pub const COMPAREOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareOp",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(COMPAREOP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CompareOp {
    fn type_info() -> &'static TypeInfo {
        COMPAREOP_TYPE_INFO
    }
}


pub const COMPAREOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareOp-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RunningAverageEntityData {
    pub realm: super::core::Realm,
    pub number_of_values: u32,
    pub r#in: f32,
}

pub const RUNNINGAVERAGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RunningAverageEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(RunningAverageEntityData, realm),
            },
            FieldInfoData {
                name: "NumberOfValues",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RunningAverageEntityData, number_of_values),
            },
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RunningAverageEntityData, r#in),
            },
        ],
    }),
    array_type: Some(RUNNINGAVERAGEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RunningAverageEntityData {
    fn type_info() -> &'static TypeInfo {
        RUNNINGAVERAGEENTITYDATA_TYPE_INFO
    }
}


pub const RUNNINGAVERAGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RunningAverageEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RunningAverageEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AbsEntityData {
    pub realm: super::core::Realm,
    pub r#in: f32,
}

pub const ABSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AbsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(AbsEntityData, realm),
            },
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AbsEntityData, r#in),
            },
        ],
    }),
    array_type: Some(ABSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AbsEntityData {
    fn type_info() -> &'static TypeInfo {
        ABSENTITYDATA_TYPE_INFO
    }
}


pub const ABSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AbsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AbsEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VectorMathOpEntityData {
    pub realm: super::core::Realm,
    pub math_operator: VectorMathOp,
}

pub const VECTORMATHOPENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorMathOpEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(VectorMathOpEntityData, realm),
            },
            FieldInfoData {
                name: "MathOperator",
                flags: MemberInfoFlags::new(0),
                field_type: VECTORMATHOP_TYPE_INFO,
                rust_offset: offset_of!(VectorMathOpEntityData, math_operator),
            },
        ],
    }),
    array_type: Some(VECTORMATHOPENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VectorMathOpEntityData {
    fn type_info() -> &'static TypeInfo {
        VECTORMATHOPENTITYDATA_TYPE_INFO
    }
}


pub const VECTORMATHOPENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorMathOpEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("VectorMathOpEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VectorMathOp {
    #[default]
    VectorMathOp_Add = 0,
    VectorMathOp_Subtract = 1,
    VectorMathOp_MultiplyByFloat = 2,
    VectorMathOp_DivideByFloat = 3,
    VectorMathOp_Cross = 4,
    VectorMathOp_Dot = 5,
    VectorMathOp_Length = 6,
}

pub const VECTORMATHOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorMathOp",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(VECTORMATHOP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VectorMathOp {
    fn type_info() -> &'static TypeInfo {
        VECTORMATHOP_TYPE_INFO
    }
}


pub const VECTORMATHOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorMathOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("VectorMathOp-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MathOpEntityData {
    pub realm: super::core::Realm,
    pub operators: Vec<MathOp>,
}

pub const MATHOPENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathOpEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(MathOpEntityData, realm),
            },
            FieldInfoData {
                name: "Operators",
                flags: MemberInfoFlags::new(144),
                field_type: MATHOP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MathOpEntityData, operators),
            },
        ],
    }),
    array_type: Some(MATHOPENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MathOpEntityData {
    fn type_info() -> &'static TypeInfo {
        MATHOPENTITYDATA_TYPE_INFO
    }
}


pub const MATHOPENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathOpEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MathOpEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MathOp {
    #[default]
    MathOp_Add = 0,
    MathOp_Subtract = 1,
    MathOp_Multiply = 2,
    MathOp_Divide = 3,
    MathOp_Min = 4,
    MathOp_Max = 5,
    MathOp_Modulo = 6,
    MathOp_Exponent = 7,
}

pub const MATHOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathOp",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(MATHOP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MathOp {
    fn type_info() -> &'static TypeInfo {
        MATHOP_TYPE_INFO
    }
}


pub const MATHOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MathOp-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PropertyCastEntityData {
    pub realm: super::core::Realm,
    pub precision: i32,
    pub bool_value: bool,
    pub float_value: f32,
    pub int_value: i32,
    pub uint_value: u32,
    pub string_value: String,
}

pub const PROPERTYCASTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyCastEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PropertyCastEntityData, realm),
            },
            FieldInfoData {
                name: "Precision",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyCastEntityData, precision),
            },
            FieldInfoData {
                name: "BoolValue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropertyCastEntityData, bool_value),
            },
            FieldInfoData {
                name: "FloatValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyCastEntityData, float_value),
            },
            FieldInfoData {
                name: "IntValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyCastEntityData, int_value),
            },
            FieldInfoData {
                name: "UintValue",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyCastEntityData, uint_value),
            },
            FieldInfoData {
                name: "StringValue",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PropertyCastEntityData, string_value),
            },
        ],
    }),
    array_type: Some(PROPERTYCASTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertyCastEntityData {
    fn type_info() -> &'static TypeInfo {
        PROPERTYCASTENTITYDATA_TYPE_INFO
    }
}


pub const PROPERTYCASTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyCastEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PropertyCastEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolToEventEntityData {
    pub realm: super::core::Realm,
    pub value: bool,
    pub inital_event: bool,
}

pub const BOOLTOEVENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolToEventEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(BoolToEventEntityData, realm),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoolToEventEntityData, value),
            },
            FieldInfoData {
                name: "InitalEvent",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoolToEventEntityData, inital_event),
            },
        ],
    }),
    array_type: Some(BOOLTOEVENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoolToEventEntityData {
    fn type_info() -> &'static TypeInfo {
        BOOLTOEVENTENTITYDATA_TYPE_INFO
    }
}


pub const BOOLTOEVENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolToEventEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoolToEventEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwitchPropertyStringEntityData {
    pub string_properties: Vec<String>,
    pub realm: super::core::Realm,
}

pub const SWITCHPROPERTYSTRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwitchPropertyStringEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StringProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SwitchPropertyStringEntityData, string_properties),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SwitchPropertyStringEntityData, realm),
            },
        ],
    }),
    array_type: Some(SWITCHPROPERTYSTRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SwitchPropertyStringEntityData {
    fn type_info() -> &'static TypeInfo {
        SWITCHPROPERTYSTRINGENTITYDATA_TYPE_INFO
    }
}


pub const SWITCHPROPERTYSTRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwitchPropertyStringEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SwitchPropertyStringEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PropertyGateEntityData {
    pub default: bool,
    pub write_property_on_open_gate: bool,
    pub realm: super::core::Realm,
    pub bool_in: bool,
    pub int_in: i32,
    pub float_in: f32,
    pub vec3_in: super::core::Vec3,
    pub vec4_in: super::core::Vec4,
    pub transform_in: super::core::LinearTransform,
}

pub const PROPERTYGATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyGateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Default",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropertyGateEntityData, default),
            },
            FieldInfoData {
                name: "WritePropertyOnOpenGate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropertyGateEntityData, write_property_on_open_gate),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PropertyGateEntityData, realm),
            },
            FieldInfoData {
                name: "BoolIn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PropertyGateEntityData, bool_in),
            },
            FieldInfoData {
                name: "IntIn",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyGateEntityData, int_in),
            },
            FieldInfoData {
                name: "FloatIn",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyGateEntityData, float_in),
            },
            FieldInfoData {
                name: "Vec3In",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PropertyGateEntityData, vec3_in),
            },
            FieldInfoData {
                name: "Vec4In",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(PropertyGateEntityData, vec4_in),
            },
            FieldInfoData {
                name: "TransformIn",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(PropertyGateEntityData, transform_in),
            },
        ],
    }),
    array_type: Some(PROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PropertyGateEntityData {
    fn type_info() -> &'static TypeInfo {
        PROPERTYGATEENTITYDATA_TYPE_INFO
    }
}


pub const PROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyGateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PropertyGateEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventGateEntityData {
    pub default: bool,
    pub realm: super::core::Realm,
}

pub const EVENTGATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Default",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EventGateEntityData, default),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(EventGateEntityData, realm),
            },
        ],
    }),
    array_type: Some(EVENTGATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventGateEntityData {
    fn type_info() -> &'static TypeInfo {
        EVENTGATEENTITYDATA_TYPE_INFO
    }
}


pub const EVENTGATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventGateEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StopWatchEntityData {
    pub realm: super::core::Realm,
    pub update_pass: UpdatePass,
    pub multiplier: f32,
    pub trigger_on_time: f32,
    pub use_real_delta_time: bool,
}

pub const STOPWATCHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopWatchEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(StopWatchEntityData, realm),
            },
            FieldInfoData {
                name: "UpdatePass",
                flags: MemberInfoFlags::new(0),
                field_type: UPDATEPASS_TYPE_INFO,
                rust_offset: offset_of!(StopWatchEntityData, update_pass),
            },
            FieldInfoData {
                name: "Multiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StopWatchEntityData, multiplier),
            },
            FieldInfoData {
                name: "TriggerOnTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StopWatchEntityData, trigger_on_time),
            },
            FieldInfoData {
                name: "UseRealDeltaTime",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StopWatchEntityData, use_real_delta_time),
            },
        ],
    }),
    array_type: Some(STOPWATCHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StopWatchEntityData {
    fn type_info() -> &'static TypeInfo {
        STOPWATCHENTITYDATA_TYPE_INFO
    }
}


pub const STOPWATCHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopWatchEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("StopWatchEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformToRotationEntityData {
    pub realm: super::core::Realm,
    pub r#in: super::core::LinearTransform,
    pub degrees: bool,
}

pub const TRANSFORMTOROTATIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformToRotationEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransformToRotationEntityData, realm),
            },
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformToRotationEntityData, r#in),
            },
            FieldInfoData {
                name: "Degrees",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransformToRotationEntityData, degrees),
            },
        ],
    }),
    array_type: Some(TRANSFORMTOROTATIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformToRotationEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMTOROTATIONENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMTOROTATIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformToRotationEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformToRotationEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformModifierEntityData {
    pub realm: super::core::Realm,
    pub r#in: super::core::LinearTransform,
    pub left: ModifierAxis,
    pub up: ModifierAxis,
    pub forward: ModifierAxis,
    pub invert_full_transform: bool,
    pub invert_left: bool,
    pub invert_up: bool,
    pub invert_forward: bool,
    pub invert_translation: bool,
}

pub const TRANSFORMMODIFIERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierEntityData, realm),
            },
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierEntityData, r#in),
            },
            FieldInfoData {
                name: "Left",
                flags: MemberInfoFlags::new(0),
                field_type: MODIFIERAXIS_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierEntityData, left),
            },
            FieldInfoData {
                name: "Up",
                flags: MemberInfoFlags::new(0),
                field_type: MODIFIERAXIS_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierEntityData, up),
            },
            FieldInfoData {
                name: "Forward",
                flags: MemberInfoFlags::new(0),
                field_type: MODIFIERAXIS_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierEntityData, forward),
            },
            FieldInfoData {
                name: "InvertFullTransform",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierEntityData, invert_full_transform),
            },
            FieldInfoData {
                name: "InvertLeft",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierEntityData, invert_left),
            },
            FieldInfoData {
                name: "InvertUp",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierEntityData, invert_up),
            },
            FieldInfoData {
                name: "InvertForward",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierEntityData, invert_forward),
            },
            FieldInfoData {
                name: "InvertTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransformModifierEntityData, invert_translation),
            },
        ],
    }),
    array_type: Some(TRANSFORMMODIFIERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformModifierEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMMODIFIERENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMMODIFIERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformModifierEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ModifierAxis {
    #[default]
    maLeft = 0,
    maUp = 1,
    maForward = 2,
}

pub const MODIFIERAXIS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModifierAxis",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(MODIFIERAXIS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ModifierAxis {
    fn type_info() -> &'static TypeInfo {
        MODIFIERAXIS_TYPE_INFO
    }
}


pub const MODIFIERAXIS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModifierAxis-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ModifierAxis-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RotationTransformBuilderEntityData {
    pub realm: super::core::Realm,
    pub rotation: super::core::Vec3,
}

pub const ROTATIONTRANSFORMBUILDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationTransformBuilderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(RotationTransformBuilderEntityData, realm),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RotationTransformBuilderEntityData, rotation),
            },
        ],
    }),
    array_type: Some(ROTATIONTRANSFORMBUILDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RotationTransformBuilderEntityData {
    fn type_info() -> &'static TypeInfo {
        ROTATIONTRANSFORMBUILDERENTITYDATA_TYPE_INFO
    }
}


pub const ROTATIONTRANSFORMBUILDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationTransformBuilderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RotationTransformBuilderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ScaleTransformBuilderEntityData {
    pub realm: super::core::Realm,
    pub scale: super::core::Vec3,
}

pub const SCALETRANSFORMBUILDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScaleTransformBuilderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ScaleTransformBuilderEntityData, realm),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ScaleTransformBuilderEntityData, scale),
            },
        ],
    }),
    array_type: Some(SCALETRANSFORMBUILDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ScaleTransformBuilderEntityData {
    fn type_info() -> &'static TypeInfo {
        SCALETRANSFORMBUILDERENTITYDATA_TYPE_INFO
    }
}


pub const SCALETRANSFORMBUILDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScaleTransformBuilderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ScaleTransformBuilderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformBuilderEntityData {
    pub realm: super::core::Realm,
    pub ortho_normal_mode: OrthoNormalMode,
    pub left: super::core::Vec3,
    pub up: super::core::Vec3,
    pub forward: super::core::Vec3,
    pub trans: super::core::Vec3,
    pub is_world_space: bool,
}

pub const TRANSFORMBUILDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformBuilderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransformBuilderEntityData, realm),
            },
            FieldInfoData {
                name: "OrthoNormalMode",
                flags: MemberInfoFlags::new(0),
                field_type: ORTHONORMALMODE_TYPE_INFO,
                rust_offset: offset_of!(TransformBuilderEntityData, ortho_normal_mode),
            },
            FieldInfoData {
                name: "Left",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TransformBuilderEntityData, left),
            },
            FieldInfoData {
                name: "Up",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TransformBuilderEntityData, up),
            },
            FieldInfoData {
                name: "Forward",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TransformBuilderEntityData, forward),
            },
            FieldInfoData {
                name: "Trans",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TransformBuilderEntityData, trans),
            },
            FieldInfoData {
                name: "IsWorldSpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransformBuilderEntityData, is_world_space),
            },
        ],
    }),
    array_type: Some(TRANSFORMBUILDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformBuilderEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMBUILDERENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMBUILDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformBuilderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformBuilderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum OrthoNormalMode {
    #[default]
    OrthoNormalMode_LeftHanded = 0,
    OrthoNormalMode_RightHanded = 1,
    OrthoNormalMode_None = 2,
}

pub const ORTHONORMALMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrthoNormalMode",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(ORTHONORMALMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OrthoNormalMode {
    fn type_info() -> &'static TypeInfo {
        ORTHONORMALMODE_TYPE_INFO
    }
}


pub const ORTHONORMALMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrthoNormalMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("OrthoNormalMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VecBuilderEntityData {
    pub realm: super::core::Realm,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub const VECBUILDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VecBuilderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(VecBuilderEntityData, realm),
            },
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VecBuilderEntityData, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VecBuilderEntityData, y),
            },
            FieldInfoData {
                name: "Z",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VecBuilderEntityData, z),
            },
            FieldInfoData {
                name: "W",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VecBuilderEntityData, w),
            },
        ],
    }),
    array_type: Some(VECBUILDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VecBuilderEntityData {
    fn type_info() -> &'static TypeInfo {
        VECBUILDERENTITYDATA_TYPE_INFO
    }
}


pub const VECBUILDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VecBuilderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("VecBuilderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec4BuilderEntityData {
    pub realm: super::core::Realm,
}

pub const VEC4BUILDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4BuilderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec4BuilderEntityData, realm),
            },
        ],
    }),
    array_type: Some(VEC4BUILDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec4BuilderEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC4BUILDERENTITYDATA_TYPE_INFO
    }
}


pub const VEC4BUILDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4BuilderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec4BuilderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec3BuilderEntityData {
    pub realm: super::core::Realm,
}

pub const VEC3BUILDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3BuilderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec3BuilderEntityData, realm),
            },
        ],
    }),
    array_type: Some(VEC3BUILDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec3BuilderEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC3BUILDERENTITYDATA_TYPE_INFO
    }
}


pub const VEC3BUILDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3BuilderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec3BuilderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec2BuilderEntityData {
    pub realm: super::core::Realm,
}

pub const VEC2BUILDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2BuilderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec2BuilderEntityData, realm),
            },
        ],
    }),
    array_type: Some(VEC2BUILDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec2BuilderEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC2BUILDERENTITYDATA_TYPE_INFO
    }
}


pub const VEC2BUILDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2BuilderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec2BuilderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VecSplitterEntityData {
    pub realm: super::core::Realm,
    pub vec2: super::core::Vec2,
    pub vec3: super::core::Vec3,
    pub vec4: super::core::Vec4,
}

pub const VECSPLITTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VecSplitterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(VecSplitterEntityData, realm),
            },
            FieldInfoData {
                name: "Vec2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(VecSplitterEntityData, vec2),
            },
            FieldInfoData {
                name: "Vec3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(VecSplitterEntityData, vec3),
            },
            FieldInfoData {
                name: "Vec4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(VecSplitterEntityData, vec4),
            },
        ],
    }),
    array_type: Some(VECSPLITTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VecSplitterEntityData {
    fn type_info() -> &'static TypeInfo {
        VECSPLITTERENTITYDATA_TYPE_INFO
    }
}


pub const VECSPLITTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VecSplitterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("VecSplitterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec4SplitterEntityData {
    pub realm: super::core::Realm,
}

pub const VEC4SPLITTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4SplitterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec4SplitterEntityData, realm),
            },
        ],
    }),
    array_type: Some(VEC4SPLITTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec4SplitterEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC4SPLITTERENTITYDATA_TYPE_INFO
    }
}


pub const VEC4SPLITTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4SplitterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec4SplitterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec3SplitterEntityData {
    pub realm: super::core::Realm,
}

pub const VEC3SPLITTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3SplitterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec3SplitterEntityData, realm),
            },
        ],
    }),
    array_type: Some(VEC3SPLITTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec3SplitterEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC3SPLITTERENTITYDATA_TYPE_INFO
    }
}


pub const VEC3SPLITTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3SplitterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec3SplitterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec2SplitterEntityData {
    pub realm: super::core::Realm,
}

pub const VEC2SPLITTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2SplitterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec2SplitterEntityData, realm),
            },
        ],
    }),
    array_type: Some(VEC2SPLITTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec2SplitterEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC2SPLITTERENTITYDATA_TYPE_INFO
    }
}


pub const VEC2SPLITTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2SplitterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec2SplitterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EulerTransformEntityData {
    pub realm: super::core::Realm,
    pub rotation: f32,
    pub trans: super::core::Vec3,
    pub euler: ModifierEuler,
}

pub const EULERTRANSFORMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EulerTransformEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(EulerTransformEntityData, realm),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EulerTransformEntityData, rotation),
            },
            FieldInfoData {
                name: "Trans",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EulerTransformEntityData, trans),
            },
            FieldInfoData {
                name: "Euler",
                flags: MemberInfoFlags::new(0),
                field_type: MODIFIEREULER_TYPE_INFO,
                rust_offset: offset_of!(EulerTransformEntityData, euler),
            },
        ],
    }),
    array_type: Some(EULERTRANSFORMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EulerTransformEntityData {
    fn type_info() -> &'static TypeInfo {
        EULERTRANSFORMENTITYDATA_TYPE_INFO
    }
}


pub const EULERTRANSFORMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EulerTransformEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EulerTransformEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ModifierEuler {
    #[default]
    Roll = 0,
    Pitch = 1,
    Yaw = 2,
    Trans = 3,
}

pub const MODIFIEREULER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModifierEuler",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(MODIFIEREULER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ModifierEuler {
    fn type_info() -> &'static TypeInfo {
        MODIFIEREULER_TYPE_INFO
    }
}


pub const MODIFIEREULER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModifierEuler-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ModifierEuler-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EulerTransformSplitterEntityData {
    pub realm: super::core::Realm,
    pub transform: super::core::LinearTransform,
}

pub const EULERTRANSFORMSPLITTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EulerTransformSplitterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(EulerTransformSplitterEntityData, realm),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(EulerTransformSplitterEntityData, transform),
            },
        ],
    }),
    array_type: Some(EULERTRANSFORMSPLITTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EulerTransformSplitterEntityData {
    fn type_info() -> &'static TypeInfo {
        EULERTRANSFORMSPLITTERENTITYDATA_TYPE_INFO
    }
}


pub const EULERTRANSFORMSPLITTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EulerTransformSplitterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EulerTransformSplitterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformSplitterEntityData {
    pub realm: super::core::Realm,
    pub transform: super::core::LinearTransform,
}

pub const TRANSFORMSPLITTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSplitterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransformSplitterEntityData, realm),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformSplitterEntityData, transform),
            },
        ],
    }),
    array_type: Some(TRANSFORMSPLITTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformSplitterEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMSPLITTERENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMSPLITTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSplitterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformSplitterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformBlendEntityData {
    pub realm: super::core::Realm,
    pub blend_value: f32,
    pub blend_value2: f32,
    pub in1: super::core::LinearTransform,
    pub in2: super::core::LinearTransform,
}

pub const TRANSFORMBLENDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformBlendEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransformBlendEntityData, realm),
            },
            FieldInfoData {
                name: "BlendValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TransformBlendEntityData, blend_value),
            },
            FieldInfoData {
                name: "BlendValue2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TransformBlendEntityData, blend_value2),
            },
            FieldInfoData {
                name: "In1",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformBlendEntityData, in1),
            },
            FieldInfoData {
                name: "In2",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformBlendEntityData, in2),
            },
        ],
    }),
    array_type: Some(TRANSFORMBLENDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformBlendEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMBLENDENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMBLENDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformBlendEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformBlendEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformMultiplierEntityData {
    pub realm: super::core::Realm,
    pub in1: super::core::LinearTransform,
    pub in2: super::core::LinearTransform,
}

pub const TRANSFORMMULTIPLIERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformMultiplierEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransformMultiplierEntityData, realm),
            },
            FieldInfoData {
                name: "In1",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformMultiplierEntityData, in1),
            },
            FieldInfoData {
                name: "In2",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TransformMultiplierEntityData, in2),
            },
        ],
    }),
    array_type: Some(TRANSFORMMULTIPLIERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformMultiplierEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMMULTIPLIERENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMMULTIPLIERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformMultiplierEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformMultiplierEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformHubEntityData {
    pub realm: super::core::Realm,
    pub input_count: i32,
    pub input_select: i32,
    pub hashed_input: Vec<u32>,
}

pub const TRANSFORMHUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformHubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransformHubEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TransformHubEntityData, input_count),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TransformHubEntityData, input_select),
            },
            FieldInfoData {
                name: "HashedInput",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TransformHubEntityData, hashed_input),
            },
        ],
    }),
    array_type: Some(TRANSFORMHUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TransformHubEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMHUBENTITYDATA_TYPE_INFO
    }
}


pub const TRANSFORMHUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformHubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformHubEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringHubEntityData {
    pub realm: super::core::Realm,
    pub input_count: i32,
    pub input_select: i32,
    pub hashed_input: Vec<u32>,
}

pub const STRINGHUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringHubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(StringHubEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StringHubEntityData, input_count),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StringHubEntityData, input_select),
            },
            FieldInfoData {
                name: "HashedInput",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StringHubEntityData, hashed_input),
            },
        ],
    }),
    array_type: Some(STRINGHUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringHubEntityData {
    fn type_info() -> &'static TypeInfo {
        STRINGHUBENTITYDATA_TYPE_INFO
    }
}


pub const STRINGHUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringHubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("StringHubEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec4HubEntityData {
    pub realm: super::core::Realm,
    pub input_count: i32,
    pub input_select: i32,
    pub hashed_input: Vec<u32>,
}

pub const VEC4HUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4HubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec4HubEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Vec4HubEntityData, input_count),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Vec4HubEntityData, input_select),
            },
            FieldInfoData {
                name: "HashedInput",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Vec4HubEntityData, hashed_input),
            },
        ],
    }),
    array_type: Some(VEC4HUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec4HubEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC4HUBENTITYDATA_TYPE_INFO
    }
}


pub const VEC4HUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4HubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec4HubEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec3HubEntityData {
    pub realm: super::core::Realm,
    pub input_count: i32,
    pub input_select: i32,
    pub hashed_input: Vec<u32>,
}

pub const VEC3HUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3HubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec3HubEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Vec3HubEntityData, input_count),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Vec3HubEntityData, input_select),
            },
            FieldInfoData {
                name: "HashedInput",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Vec3HubEntityData, hashed_input),
            },
        ],
    }),
    array_type: Some(VEC3HUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec3HubEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC3HUBENTITYDATA_TYPE_INFO
    }
}


pub const VEC3HUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3HubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec3HubEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec2HubEntityData {
    pub realm: super::core::Realm,
    pub input_count: i32,
    pub input_select: i32,
    pub hashed_input: Vec<u32>,
}

pub const VEC2HUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2HubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Vec2HubEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Vec2HubEntityData, input_count),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Vec2HubEntityData, input_select),
            },
            FieldInfoData {
                name: "HashedInput",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Vec2HubEntityData, hashed_input),
            },
        ],
    }),
    array_type: Some(VEC2HUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec2HubEntityData {
    fn type_info() -> &'static TypeInfo {
        VEC2HUBENTITYDATA_TYPE_INFO
    }
}


pub const VEC2HUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2HubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec2HubEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatHubEntityData {
    pub realm: super::core::Realm,
    pub input_count: i32,
    pub input_select: i32,
    pub hashed_input: Vec<u32>,
}

pub const FLOATHUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatHubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(FloatHubEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FloatHubEntityData, input_count),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FloatHubEntityData, input_select),
            },
            FieldInfoData {
                name: "HashedInput",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FloatHubEntityData, hashed_input),
            },
        ],
    }),
    array_type: Some(FLOATHUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatHubEntityData {
    fn type_info() -> &'static TypeInfo {
        FLOATHUBENTITYDATA_TYPE_INFO
    }
}


pub const FLOATHUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatHubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatHubEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Int64HubEntityData {
    pub realm: super::core::Realm,
    pub input_count: i32,
    pub input_select: i32,
    pub hashed_input: Vec<u32>,
}

pub const INT64HUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int64HubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Int64HubEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Int64HubEntityData, input_count),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Int64HubEntityData, input_select),
            },
            FieldInfoData {
                name: "HashedInput",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Int64HubEntityData, hashed_input),
            },
        ],
    }),
    array_type: Some(INT64HUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Int64HubEntityData {
    fn type_info() -> &'static TypeInfo {
        INT64HUBENTITYDATA_TYPE_INFO
    }
}


pub const INT64HUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int64HubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Int64HubEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntHubEntityData {
    pub realm: super::core::Realm,
    pub input_count: i32,
    pub input_select: i32,
    pub hashed_input: Vec<u32>,
}

pub const INTHUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntHubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(IntHubEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IntHubEntityData, input_count),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IntHubEntityData, input_select),
            },
            FieldInfoData {
                name: "HashedInput",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(IntHubEntityData, hashed_input),
            },
        ],
    }),
    array_type: Some(INTHUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntHubEntityData {
    fn type_info() -> &'static TypeInfo {
        INTHUBENTITYDATA_TYPE_INFO
    }
}


pub const INTHUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntHubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntHubEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolHubEntityData {
    pub realm: super::core::Realm,
    pub input_select: i32,
    pub hashed_input: Vec<u32>,
}

pub const BOOLHUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolHubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(BoolHubEntityData, realm),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BoolHubEntityData, input_select),
            },
            FieldInfoData {
                name: "HashedInput",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(BoolHubEntityData, hashed_input),
            },
        ],
    }),
    array_type: Some(BOOLHUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoolHubEntityData {
    fn type_info() -> &'static TypeInfo {
        BOOLHUBENTITYDATA_TYPE_INFO
    }
}


pub const BOOLHUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolHubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoolHubEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SimpleDofParamConverterEntityData {
    pub focus_distance: f32,
    pub blur_filter_deviation: f32,
    pub near_distance_scale: f32,
    pub far_distance_scale: f32,
    pub scale: f32,
}

pub const SIMPLEDOFPARAMCONVERTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleDofParamConverterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FocusDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleDofParamConverterEntityData, focus_distance),
            },
            FieldInfoData {
                name: "BlurFilterDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleDofParamConverterEntityData, blur_filter_deviation),
            },
            FieldInfoData {
                name: "NearDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleDofParamConverterEntityData, near_distance_scale),
            },
            FieldInfoData {
                name: "FarDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleDofParamConverterEntityData, far_distance_scale),
            },
            FieldInfoData {
                name: "Scale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SimpleDofParamConverterEntityData, scale),
            },
        ],
    }),
    array_type: Some(SIMPLEDOFPARAMCONVERTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SimpleDofParamConverterEntityData {
    fn type_info() -> &'static TypeInfo {
        SIMPLEDOFPARAMCONVERTERENTITYDATA_TYPE_INFO
    }
}


pub const SIMPLEDOFPARAMCONVERTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleDofParamConverterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SimpleDofParamConverterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SpriteDofParamConverterEntityData {
    pub sprite_dof_out_focus_start: f32,
    pub sprite_dof_out_focus_end: f32,
    pub sprite_dof_max_blur_factor: f32,
}

pub const SPRITEDOFPARAMCONVERTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpriteDofParamConverterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SpriteDofOutFocusStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpriteDofParamConverterEntityData, sprite_dof_out_focus_start),
            },
            FieldInfoData {
                name: "SpriteDofOutFocusEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpriteDofParamConverterEntityData, sprite_dof_out_focus_end),
            },
            FieldInfoData {
                name: "SpriteDofMaxBlurFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SpriteDofParamConverterEntityData, sprite_dof_max_blur_factor),
            },
        ],
    }),
    array_type: Some(SPRITEDOFPARAMCONVERTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SpriteDofParamConverterEntityData {
    fn type_info() -> &'static TypeInfo {
        SPRITEDOFPARAMCONVERTERENTITYDATA_TYPE_INFO
    }
}


pub const SPRITEDOFPARAMCONVERTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpriteDofParamConverterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SpriteDofParamConverterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectVariationMultiSwitchEntityData {
    pub variations: Vec<ObjectVariation>,
    pub current_index: i32,
    pub set_on_init: bool,
}

pub const OBJECTVARIATIONMULTISWITCHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationMultiSwitchEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Variations",
                flags: MemberInfoFlags::new(144),
                field_type: OBJECTVARIATION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ObjectVariationMultiSwitchEntityData, variations),
            },
            FieldInfoData {
                name: "CurrentIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ObjectVariationMultiSwitchEntityData, current_index),
            },
            FieldInfoData {
                name: "SetOnInit",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ObjectVariationMultiSwitchEntityData, set_on_init),
            },
        ],
    }),
    array_type: Some(OBJECTVARIATIONMULTISWITCHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObjectVariationMultiSwitchEntityData {
    fn type_info() -> &'static TypeInfo {
        OBJECTVARIATIONMULTISWITCHENTITYDATA_TYPE_INFO
    }
}


pub const OBJECTVARIATIONMULTISWITCHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationMultiSwitchEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectVariationMultiSwitchEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectVariationSwitchEntityData {
    pub variation1: ObjectVariation,
    pub variation2: ObjectVariation,
}

pub const OBJECTVARIATIONSWITCHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationSwitchEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Variation1",
                flags: MemberInfoFlags::new(0),
                field_type: OBJECTVARIATION_TYPE_INFO,
                rust_offset: offset_of!(ObjectVariationSwitchEntityData, variation1),
            },
            FieldInfoData {
                name: "Variation2",
                flags: MemberInfoFlags::new(0),
                field_type: OBJECTVARIATION_TYPE_INFO,
                rust_offset: offset_of!(ObjectVariationSwitchEntityData, variation2),
            },
        ],
    }),
    array_type: Some(OBJECTVARIATIONSWITCHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObjectVariationSwitchEntityData {
    fn type_info() -> &'static TypeInfo {
        OBJECTVARIATIONSWITCHENTITYDATA_TYPE_INFO
    }
}


pub const OBJECTVARIATIONSWITCHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationSwitchEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectVariationSwitchEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RandomDelayEntityData {
    pub min_delay: f32,
    pub max_delay: f32,
    pub realm: super::core::Realm,
    pub auto_start: bool,
    pub run_once: bool,
    pub true_random: bool,
    pub time_delta_type: TimeDeltaType,
}

pub const RANDOMDELAYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomDelayEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MinDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomDelayEntityData, min_delay),
            },
            FieldInfoData {
                name: "MaxDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomDelayEntityData, max_delay),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(RandomDelayEntityData, realm),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomDelayEntityData, auto_start),
            },
            FieldInfoData {
                name: "RunOnce",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomDelayEntityData, run_once),
            },
            FieldInfoData {
                name: "TrueRandom",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomDelayEntityData, true_random),
            },
            FieldInfoData {
                name: "TimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: TIMEDELTATYPE_TYPE_INFO,
                rust_offset: offset_of!(RandomDelayEntityData, time_delta_type),
            },
        ],
    }),
    array_type: Some(RANDOMDELAYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomDelayEntityData {
    fn type_info() -> &'static TypeInfo {
        RANDOMDELAYENTITYDATA_TYPE_INFO
    }
}


pub const RANDOMDELAYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomDelayEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RandomDelayEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DelayEntityData {
    pub delay: f32,
    pub realm: super::core::Realm,
    pub auto_start: bool,
    pub run_once: bool,
    pub remove_duplicate_events: bool,
    pub time_delta_type: TimeDeltaType,
}

pub const DELAYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DelayEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Delay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DelayEntityData, delay),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(DelayEntityData, realm),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DelayEntityData, auto_start),
            },
            FieldInfoData {
                name: "RunOnce",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DelayEntityData, run_once),
            },
            FieldInfoData {
                name: "RemoveDuplicateEvents",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DelayEntityData, remove_duplicate_events),
            },
            FieldInfoData {
                name: "TimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: TIMEDELTATYPE_TYPE_INFO,
                rust_offset: offset_of!(DelayEntityData, time_delta_type),
            },
        ],
    }),
    array_type: Some(DELAYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DelayEntityData {
    fn type_info() -> &'static TypeInfo {
        DELAYENTITYDATA_TYPE_INFO
    }
}


pub const DELAYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DelayEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DelayEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventOrderGateEntityData {
    pub realm: super::core::Realm,
    pub event_count: u32,
    pub enabled: bool,
    pub disable_on_open: bool,
    pub event_countdowns: Vec<u32>,
    pub source_hashes: Vec<i32>,
}

pub const EVENTORDERGATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventOrderGateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(EventOrderGateEntityData, realm),
            },
            FieldInfoData {
                name: "EventCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EventOrderGateEntityData, event_count),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EventOrderGateEntityData, enabled),
            },
            FieldInfoData {
                name: "DisableOnOpen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EventOrderGateEntityData, disable_on_open),
            },
            FieldInfoData {
                name: "EventCountdowns",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EventOrderGateEntityData, event_countdowns),
            },
            FieldInfoData {
                name: "SourceHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EventOrderGateEntityData, source_hashes),
            },
        ],
    }),
    array_type: Some(EVENTORDERGATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventOrderGateEntityData {
    fn type_info() -> &'static TypeInfo {
        EVENTORDERGATEENTITYDATA_TYPE_INFO
    }
}


pub const EVENTORDERGATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventOrderGateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventOrderGateEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventAndGateEntityData {
    pub realm: super::core::Realm,
    pub event_count: u32,
}

pub const EVENTANDGATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventAndGateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(EventAndGateEntityData, realm),
            },
            FieldInfoData {
                name: "EventCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EventAndGateEntityData, event_count),
            },
        ],
    }),
    array_type: Some(EVENTANDGATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventAndGateEntityData {
    fn type_info() -> &'static TypeInfo {
        EVENTANDGATEENTITYDATA_TYPE_INFO
    }
}


pub const EVENTANDGATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventAndGateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventAndGateEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectEventEntityData {
    pub realm: super::core::Realm,
    pub selected: i32,
    pub events: Vec<String>,
}

pub const SELECTEVENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectEventEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SelectEventEntityData, realm),
            },
            FieldInfoData {
                name: "Selected",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SelectEventEntityData, selected),
            },
            FieldInfoData {
                name: "Events",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SelectEventEntityData, events),
            },
        ],
    }),
    array_type: Some(SELECTEVENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectEventEntityData {
    fn type_info() -> &'static TypeInfo {
        SELECTEVENTENTITYDATA_TYPE_INFO
    }
}


pub const SELECTEVENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectEventEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectEventEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventSwitchEntityData {
    pub realm: super::core::Realm,
    pub out_events: u32,
    pub auto_increment: bool,
}

pub const EVENTSWITCHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSwitchEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(EventSwitchEntityData, realm),
            },
            FieldInfoData {
                name: "OutEvents",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EventSwitchEntityData, out_events),
            },
            FieldInfoData {
                name: "AutoIncrement",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EventSwitchEntityData, auto_increment),
            },
        ],
    }),
    array_type: Some(EVENTSWITCHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventSwitchEntityData {
    fn type_info() -> &'static TypeInfo {
        EVENTSWITCHENTITYDATA_TYPE_INFO
    }
}


pub const EVENTSWITCHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSwitchEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventSwitchEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UniqueIdEntityData {
    pub realm: super::core::Realm,
}

pub const UNIQUEIDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UniqueIdEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(UniqueIdEntityData, realm),
            },
        ],
    }),
    array_type: Some(UNIQUEIDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UniqueIdEntityData {
    fn type_info() -> &'static TypeInfo {
        UNIQUEIDENTITYDATA_TYPE_INFO
    }
}


pub const UNIQUEIDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UniqueIdEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("UniqueIdEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RandomIntEntityData {
    pub realm: super::core::Realm,
    pub start: i32,
    pub count: i32,
    pub true_random: bool,
}

pub const RANDOMINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomIntEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(RandomIntEntityData, realm),
            },
            FieldInfoData {
                name: "Start",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RandomIntEntityData, start),
            },
            FieldInfoData {
                name: "Count",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RandomIntEntityData, count),
            },
            FieldInfoData {
                name: "TrueRandom",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomIntEntityData, true_random),
            },
        ],
    }),
    array_type: Some(RANDOMINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomIntEntityData {
    fn type_info() -> &'static TypeInfo {
        RANDOMINTENTITYDATA_TYPE_INFO
    }
}


pub const RANDOMINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomIntEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RandomIntEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RandomFloatEntityData {
    pub realm: super::core::Realm,
    pub min: f32,
    pub max: f32,
    pub true_random: bool,
}

pub const RANDOMFLOATENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomFloatEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(RandomFloatEntityData, realm),
            },
            FieldInfoData {
                name: "Min",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomFloatEntityData, min),
            },
            FieldInfoData {
                name: "Max",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RandomFloatEntityData, max),
            },
            FieldInfoData {
                name: "TrueRandom",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomFloatEntityData, true_random),
            },
        ],
    }),
    array_type: Some(RANDOMFLOATENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomFloatEntityData {
    fn type_info() -> &'static TypeInfo {
        RANDOMFLOATENTITYDATA_TYPE_INFO
    }
}


pub const RANDOMFLOATENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomFloatEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RandomFloatEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RandomMultiEventEntityData {
    pub realm: super::core::Realm,
    pub uniform_distribution: bool,
    pub disable_output_on_trigger: bool,
    pub reset_outputs_when_all_has_triggered: bool,
    pub random_event_weight: Vec<f32>,
    pub true_random: bool,
}

pub const RANDOMMULTIEVENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomMultiEventEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(RandomMultiEventEntityData, realm),
            },
            FieldInfoData {
                name: "UniformDistribution",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomMultiEventEntityData, uniform_distribution),
            },
            FieldInfoData {
                name: "DisableOutputOnTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomMultiEventEntityData, disable_output_on_trigger),
            },
            FieldInfoData {
                name: "ResetOutputsWhenAllHasTriggered",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomMultiEventEntityData, reset_outputs_when_all_has_triggered),
            },
            FieldInfoData {
                name: "RandomEventWeight",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RandomMultiEventEntityData, random_event_weight),
            },
            FieldInfoData {
                name: "TrueRandom",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomMultiEventEntityData, true_random),
            },
        ],
    }),
    array_type: Some(RANDOMMULTIEVENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomMultiEventEntityData {
    fn type_info() -> &'static TypeInfo {
        RANDOMMULTIEVENTENTITYDATA_TYPE_INFO
    }
}


pub const RANDOMMULTIEVENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomMultiEventEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RandomMultiEventEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RandomEventEntityData {
    pub realm: super::core::Realm,
    pub probability: i32,
    pub randomize_first_only: bool,
    pub auto_start: bool,
}

pub const RANDOMEVENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomEventEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(RandomEventEntityData, realm),
            },
            FieldInfoData {
                name: "Probability",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RandomEventEntityData, probability),
            },
            FieldInfoData {
                name: "RandomizeFirstOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomEventEntityData, randomize_first_only),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RandomEventEntityData, auto_start),
            },
        ],
    }),
    array_type: Some(RANDOMEVENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RandomEventEntityData {
    fn type_info() -> &'static TypeInfo {
        RANDOMEVENTENTITYDATA_TYPE_INFO
    }
}


pub const RANDOMEVENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomEventEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RandomEventEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocatorEntityData {
    pub realm: super::core::Realm,
}

pub const LOCATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LocatorEntityData, realm),
            },
        ],
    }),
    array_type: Some(LOCATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocatorEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCATORENTITYDATA_TYPE_INFO
    }
}


pub const LOCATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LocatorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CountDownEntityData {
    pub start_value: i32,
    pub run_once: bool,
    pub realm: super::core::Realm,
}

pub const COUNTDOWNENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CountDownEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StartValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CountDownEntityData, start_value),
            },
            FieldInfoData {
                name: "RunOnce",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CountDownEntityData, run_once),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CountDownEntityData, realm),
            },
        ],
    }),
    array_type: Some(COUNTDOWNENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CountDownEntityData {
    fn type_info() -> &'static TypeInfo {
        COUNTDOWNENTITYDATA_TYPE_INFO
    }
}


pub const COUNTDOWNENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CountDownEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CountDownEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MathEntityData {
    pub realm: super::core::Realm,
    pub evaluate_on_event: bool,
    pub assembly: MathEntityAssembly,
}

pub const MATHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(MathEntityData, realm),
            },
            FieldInfoData {
                name: "EvaluateOnEvent",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MathEntityData, evaluate_on_event),
            },
            FieldInfoData {
                name: "Assembly",
                flags: MemberInfoFlags::new(0),
                field_type: MATHENTITYASSEMBLY_TYPE_INFO,
                rust_offset: offset_of!(MathEntityData, assembly),
            },
        ],
    }),
    array_type: Some(MATHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MathEntityData {
    fn type_info() -> &'static TypeInfo {
        MATHENTITYDATA_TYPE_INFO
    }
}


pub const MATHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MathEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MathEntityAssembly {
    pub function_calls: Vec<MathEntityFunctionCall>,
    pub instructions: Vec<MathEntityInstruction>,
}

pub const MATHENTITYASSEMBLY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntityAssembly",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FunctionCalls",
                flags: MemberInfoFlags::new(144),
                field_type: MATHENTITYFUNCTIONCALL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MathEntityAssembly, function_calls),
            },
            FieldInfoData {
                name: "Instructions",
                flags: MemberInfoFlags::new(144),
                field_type: MATHENTITYINSTRUCTION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MathEntityAssembly, instructions),
            },
        ],
    }),
    array_type: Some(MATHENTITYASSEMBLY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MathEntityAssembly {
    fn type_info() -> &'static TypeInfo {
        MATHENTITYASSEMBLY_TYPE_INFO
    }
}


pub const MATHENTITYASSEMBLY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntityAssembly-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MathEntityAssembly-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MathEntityType {
    #[default]
    MathEntityType_Bool = 1,
    MathEntityType_Int = 2,
    MathEntityType_Float = 4,
    MathEntityType_Vec2 = 8,
    MathEntityType_Vec3 = 16,
    MathEntityType_Vec4 = 32,
    MathEntityType_Transform = 64,
}

pub const MATHENTITYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntityType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(MATHENTITYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MathEntityType {
    fn type_info() -> &'static TypeInfo {
        MATHENTITYTYPE_TYPE_INFO
    }
}


pub const MATHENTITYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntityType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MathEntityType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MathEntityFunctionCall {
    pub parameters: Vec<u32>,
}

pub const MATHENTITYFUNCTIONCALL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntityFunctionCall",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Parameters",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MathEntityFunctionCall, parameters),
            },
        ],
    }),
    array_type: Some(MATHENTITYFUNCTIONCALL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MathEntityFunctionCall {
    fn type_info() -> &'static TypeInfo {
        MATHENTITYFUNCTIONCALL_TYPE_INFO
    }
}


pub const MATHENTITYFUNCTIONCALL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntityFunctionCall-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MathEntityFunctionCall-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MathEntityInstruction {
    pub code: MathOpCode,
    pub result: i32,
    pub param1: i32,
    pub param2: i32,
}

pub const MATHENTITYINSTRUCTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntityInstruction",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Code",
                flags: MemberInfoFlags::new(0),
                field_type: MATHOPCODE_TYPE_INFO,
                rust_offset: offset_of!(MathEntityInstruction, code),
            },
            FieldInfoData {
                name: "Result",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MathEntityInstruction, result),
            },
            FieldInfoData {
                name: "Param1",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MathEntityInstruction, param1),
            },
            FieldInfoData {
                name: "Param2",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MathEntityInstruction, param2),
            },
        ],
    }),
    array_type: Some(MATHENTITYINSTRUCTION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MathEntityInstruction {
    fn type_info() -> &'static TypeInfo {
        MATHENTITYINSTRUCTION_TYPE_INFO
    }
}


pub const MATHENTITYINSTRUCTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntityInstruction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MathEntityInstruction-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MathOpCode {
    #[default]
    MathOpCode_ConstB = 0,
    MathOpCode_ConstI = 1,
    MathOpCode_ConstF = 2,
    MathOpCode_InputB = 3,
    MathOpCode_InputI = 4,
    MathOpCode_InputF = 5,
    MathOpCode_InputV2 = 6,
    MathOpCode_InputV3 = 7,
    MathOpCode_InputV4 = 8,
    MathOpCode_InputT = 9,
    MathOpCode_OrB = 10,
    MathOpCode_AndB = 11,
    MathOpCode_GreaterI = 12,
    MathOpCode_GreaterF = 13,
    MathOpCode_GreaterEqI = 14,
    MathOpCode_GreaterEqF = 15,
    MathOpCode_LessI = 16,
    MathOpCode_LessF = 17,
    MathOpCode_LessEqI = 18,
    MathOpCode_LessEqF = 19,
    MathOpCode_NotEqI = 20,
    MathOpCode_NotEqF = 21,
    MathOpCode_NotEqB = 22,
    MathOpCode_EqI = 23,
    MathOpCode_EqF = 24,
    MathOpCode_EqB = 25,
    MathOpCode_AddI = 26,
    MathOpCode_AddF = 27,
    MathOpCode_AddV2 = 28,
    MathOpCode_AddV3 = 29,
    MathOpCode_AddV4 = 30,
    MathOpCode_SubI = 31,
    MathOpCode_SubF = 32,
    MathOpCode_SubV2 = 33,
    MathOpCode_SubV3 = 34,
    MathOpCode_SubV4 = 35,
    MathOpCode_MulF = 36,
    MathOpCode_MulI = 37,
    MathOpCode_MulV2F = 38,
    MathOpCode_MulV3F = 39,
    MathOpCode_MulV4F = 40,
    MathOpCode_MulV2I = 41,
    MathOpCode_MulV3I = 42,
    MathOpCode_MulV4I = 43,
    MathOpCode_MulT = 44,
    MathOpCode_DivI = 45,
    MathOpCode_DivF = 46,
    MathOpCode_DivV2F = 47,
    MathOpCode_DivV3F = 48,
    MathOpCode_DivV4F = 49,
    MathOpCode_DivV2I = 50,
    MathOpCode_DivV3I = 51,
    MathOpCode_DivV4I = 52,
    MathOpCode_ModI = 53,
    MathOpCode_ModF = 54,
    MathOpCode_NegI = 55,
    MathOpCode_NegF = 56,
    MathOpCode_NegV2 = 57,
    MathOpCode_NegV3 = 58,
    MathOpCode_NegV4 = 59,
    MathOpCode_NotB = 60,
    MathOpCode_PowI = 61,
    MathOpCode_PowF = 62,
    MathOpCode_FieldV2 = 63,
    MathOpCode_FieldV3 = 64,
    MathOpCode_FieldV4 = 65,
    MathOpCode_FieldT = 66,
    MathOpCode_Func = 67,
    MathOpCode_Return = 68,
}

pub const MATHOPCODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathOpCode",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(MATHOPCODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MathOpCode {
    fn type_info() -> &'static TypeInfo {
        MATHOPCODE_TYPE_INFO
    }
}


pub const MATHOPCODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathOpCode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MathOpCode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Or4EntityData {
    pub realm: super::core::Realm,
    pub in1: bool,
    pub in2: bool,
    pub in3: bool,
    pub in4: bool,
}

pub const OR4ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Or4EntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(Or4EntityData, realm),
            },
            FieldInfoData {
                name: "In1",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Or4EntityData, in1),
            },
            FieldInfoData {
                name: "In2",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Or4EntityData, in2),
            },
            FieldInfoData {
                name: "In3",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Or4EntityData, in3),
            },
            FieldInfoData {
                name: "In4",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Or4EntityData, in4),
            },
        ],
    }),
    array_type: Some(OR4ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Or4EntityData {
    fn type_info() -> &'static TypeInfo {
        OR4ENTITYDATA_TYPE_INFO
    }
}


pub const OR4ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Or4EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Or4EntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct XorEntityData {
    pub realm: super::core::Realm,
    pub in1: bool,
    pub in2: bool,
}

pub const XORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "XorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(XorEntityData, realm),
            },
            FieldInfoData {
                name: "In1",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(XorEntityData, in1),
            },
            FieldInfoData {
                name: "In2",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(XorEntityData, in2),
            },
        ],
    }),
    array_type: Some(XORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for XorEntityData {
    fn type_info() -> &'static TypeInfo {
        XORENTITYDATA_TYPE_INFO
    }
}


pub const XORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "XorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("XorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OrEntityData {
    pub realm: super::core::Realm,
    pub input_count: u32,
}

pub const ORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(OrEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OrEntityData, input_count),
            },
        ],
    }),
    array_type: Some(ORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OrEntityData {
    fn type_info() -> &'static TypeInfo {
        ORENTITYDATA_TYPE_INFO
    }
}


pub const ORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("OrEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BitwiseAndEntityData {
    pub realm: super::core::Realm,
    pub value: i32,
    pub bit_shift_value: bool,
    pub and_value: i32,
    pub bit_shift_and_value: bool,
}

pub const BITWISEANDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BitwiseAndEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(BitwiseAndEntityData, realm),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BitwiseAndEntityData, value),
            },
            FieldInfoData {
                name: "BitShiftValue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BitwiseAndEntityData, bit_shift_value),
            },
            FieldInfoData {
                name: "AndValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BitwiseAndEntityData, and_value),
            },
            FieldInfoData {
                name: "BitShiftAndValue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BitwiseAndEntityData, bit_shift_and_value),
            },
        ],
    }),
    array_type: Some(BITWISEANDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BitwiseAndEntityData {
    fn type_info() -> &'static TypeInfo {
        BITWISEANDENTITYDATA_TYPE_INFO
    }
}


pub const BITWISEANDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BitwiseAndEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BitwiseAndEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AndEntityData {
    pub realm: super::core::Realm,
    pub input_count: u32,
}

pub const ANDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AndEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(AndEntityData, realm),
            },
            FieldInfoData {
                name: "InputCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AndEntityData, input_count),
            },
        ],
    }),
    array_type: Some(ANDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AndEntityData {
    fn type_info() -> &'static TypeInfo {
        ANDENTITYDATA_TYPE_INFO
    }
}


pub const ANDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AndEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AndEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NotEntityData {
    pub realm: super::core::Realm,
    pub r#in: bool,
}

pub const NOTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NotEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(NotEntityData, realm),
            },
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NotEntityData, r#in),
            },
        ],
    }),
    array_type: Some(NOTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NotEntityData {
    fn type_info() -> &'static TypeInfo {
        NOTENTITYDATA_TYPE_INFO
    }
}


pub const NOTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NotEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("NotEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreateSchematicsInstanceEntityData {
    pub realm: super::core::Realm,
    pub asset: super::schematics::SchematicsBaseAsset,
}

pub const CREATESCHEMATICSINSTANCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreateSchematicsInstanceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CreateSchematicsInstanceEntityData, realm),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: SCHEMATICSBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(CreateSchematicsInstanceEntityData, asset),
            },
        ],
    }),
    array_type: Some(CREATESCHEMATICSINSTANCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreateSchematicsInstanceEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATESCHEMATICSINSTANCEENTITYDATA_TYPE_INFO
    }
}


pub const CREATESCHEMATICSINSTANCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreateSchematicsInstanceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CreateSchematicsInstanceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GetEntityBusEntityData {
    pub realm: super::core::Realm,
}

pub const GETENTITYBUSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetEntityBusEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(GetEntityBusEntityData, realm),
            },
        ],
    }),
    array_type: Some(GETENTITYBUSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GetEntityBusEntityData {
    fn type_info() -> &'static TypeInfo {
        GETENTITYBUSENTITYDATA_TYPE_INFO
    }
}


pub const GETENTITYBUSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetEntityBusEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("GetEntityBusEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GetEntityEntityData {
    pub realm: super::core::Realm,
    pub runtime_type: super::core::TypeRef,
}

pub const GETENTITYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetEntityEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(GetEntityEntityData, realm),
            },
            FieldInfoData {
                name: "RuntimeType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(GetEntityEntityData, runtime_type),
            },
        ],
    }),
    array_type: Some(GETENTITYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GetEntityEntityData {
    fn type_info() -> &'static TypeInfo {
        GETENTITYENTITYDATA_TYPE_INFO
    }
}


pub const GETENTITYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetEntityEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("GetEntityEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CallFunctionEntityData {
    pub realm: super::core::Realm,
    pub function_type: super::core::TypeRef,
}

pub const CALLFUNCTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CallFunctionEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CallFunctionEntityData, realm),
            },
            FieldInfoData {
                name: "FunctionType",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(CallFunctionEntityData, function_type),
            },
        ],
    }),
    array_type: Some(CALLFUNCTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CallFunctionEntityData {
    fn type_info() -> &'static TypeInfo {
        CALLFUNCTIONENTITYDATA_TYPE_INFO
    }
}


pub const CALLFUNCTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CallFunctionEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CallFunctionEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityOwnerUid {
    pub id: u32,
}

pub const ENTITYOWNERUID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityOwnerUid",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EntityOwnerUid, id),
            },
        ],
    }),
    array_type: Some(ENTITYOWNERUID_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EntityOwnerUid {
    fn type_info() -> &'static TypeInfo {
        ENTITYOWNERUID_TYPE_INFO
    }
}


pub const ENTITYOWNERUID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityOwnerUid-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntityOwnerUid-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityUid {
    pub id: u32,
}

pub const ENTITYUID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityUid",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EntityUid, id),
            },
        ],
    }),
    array_type: Some(ENTITYUID_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EntityUid {
    fn type_info() -> &'static TypeInfo {
        ENTITYUID_TYPE_INFO
    }
}


pub const ENTITYUID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityUid-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntityUid-Array"),
    array_type: None,
    alignment: 8,
};



pub const SETTRANSFORMSPACELOCALTRANSFORM_ENTITYTRANSFORMSPACE_LINEARTRANSFORM__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetTransformSpaceLocalTransform(EntityTransformSpace,LinearTransform)",
    flags: MemberInfoFlags::new(793),
    module: "Entity",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const GETTRANSFORMSPACE_ENTITYTRANSFORMSPACE_ENTITY__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetTransformSpace(EntityTransformSpace,Entity)",
    flags: MemberInfoFlags::new(793),
    module: "Entity",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TransformSpaceChildControlSetting {
    #[default]
    TransformSpaceChildControlSetting_NoChildControl = 0,
    TransformSpaceChildControlSetting_ControlChildren = 1,
    TransformSpaceChildControlSetting_ControlChildrenAndLockTransforms = 2,
    TransformSpaceChildControlSetting_ControlDescendents = 3,
}

pub const TRANSFORMSPACECHILDCONTROLSETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceChildControlSetting",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(TRANSFORMSPACECHILDCONTROLSETTING_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TransformSpaceChildControlSetting {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMSPACECHILDCONTROLSETTING_TYPE_INFO
    }
}


pub const TRANSFORMSPACECHILDCONTROLSETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceChildControlSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformSpaceChildControlSetting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TransformSpaceSimulationSetting {
    #[default]
    TransformSpaceSimulationSetting_Default = 0,
    TransformSpaceSimulationSetting_Keyframed = 1,
}

pub const TRANSFORMSPACESIMULATIONSETTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceSimulationSetting",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(TRANSFORMSPACESIMULATIONSETTING_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TransformSpaceSimulationSetting {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMSPACESIMULATIONSETTING_TYPE_INFO
    }
}


pub const TRANSFORMSPACESIMULATIONSETTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceSimulationSetting-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformSpaceSimulationSetting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EntitySettings {
    pub editor_game_view_enable: bool,
    pub spawn_sub_levels_from_logic: bool,
    pub execution_mode: ExecutionModeType,
    pub out_of_entity_bus_network_id_threshold: u32,
    pub world_limit: f32,
    pub destroy_queued_entities_time: f32,
    pub sub_level_manager_create_sublevels_time: f32,
    pub sub_level_manager_destroy_sublevels_time: f32,
}

pub const ENTITYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitySettings",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EditorGameViewEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntitySettings, editor_game_view_enable),
            },
            FieldInfoData {
                name: "SpawnSubLevelsFromLogic",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntitySettings, spawn_sub_levels_from_logic),
            },
            FieldInfoData {
                name: "ExecutionMode",
                flags: MemberInfoFlags::new(0),
                field_type: EXECUTIONMODETYPE_TYPE_INFO,
                rust_offset: offset_of!(EntitySettings, execution_mode),
            },
            FieldInfoData {
                name: "OutOfEntityBusNetworkIdThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EntitySettings, out_of_entity_bus_network_id_threshold),
            },
            FieldInfoData {
                name: "WorldLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EntitySettings, world_limit),
            },
            FieldInfoData {
                name: "DestroyQueuedEntitiesTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EntitySettings, destroy_queued_entities_time),
            },
            FieldInfoData {
                name: "SubLevelManagerCreateSublevelsTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EntitySettings, sub_level_manager_create_sublevels_time),
            },
            FieldInfoData {
                name: "SubLevelManagerDestroySublevelsTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EntitySettings, sub_level_manager_destroy_sublevels_time),
            },
        ],
    }),
    array_type: Some(ENTITYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitySettings {
    fn type_info() -> &'static TypeInfo {
        ENTITYSETTINGS_TYPE_INFO
    }
}


pub const ENTITYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntitySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ExecutionModeType {
    #[default]
    ExecutionMode_Play = 0,
    ExecutionMode_GameView = 1,
    ExecutionMode_PlayFromHere = 2,
    ExecutionMode_FrostEdPlay = 3,
}

pub const EXECUTIONMODETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExecutionModeType",
    flags: MemberInfoFlags::new(49429),
    module: "Entity",
    data: TypeInfoData::Enum,
    array_type: Some(EXECUTIONMODETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ExecutionModeType {
    fn type_info() -> &'static TypeInfo {
        EXECUTIONMODETYPE_TYPE_INFO
    }
}


pub const EXECUTIONMODETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExecutionModeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ExecutionModeType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwitchEnumEntity {
}

pub const SWITCHENUMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwitchEnumEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENUMLOGICENTITYWITHSINGLEINPUT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SWITCHENUMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SwitchEnumEntity {
    fn type_info() -> &'static TypeInfo {
        SWITCHENUMENTITY_TYPE_INFO
    }
}


pub const SWITCHENUMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwitchEnumEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SwitchEnumEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SetEnumEntity {
}

pub const SETENUMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetEnumEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENUMLOGICENTITYWITHSINGLEINPUT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SETENUMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SetEnumEntity {
    fn type_info() -> &'static TypeInfo {
        SETENUMENTITY_TYPE_INFO
    }
}


pub const SETENUMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetEnumEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SetEnumEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntToEnumEntity {
}

pub const INTTOENUMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntToEnumEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INTTOENUMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntToEnumEntity {
    fn type_info() -> &'static TypeInfo {
        INTTOENUMENTITY_TYPE_INFO
    }
}


pub const INTTOENUMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntToEnumEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntToEnumEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumToStringEntity {
}

pub const ENUMTOSTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumToStringEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENUMLOGICENTITYWITHSINGLEINPUT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENUMTOSTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnumToStringEntity {
    fn type_info() -> &'static TypeInfo {
        ENUMTOSTRINGENTITY_TYPE_INFO
    }
}


pub const ENUMTOSTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumToStringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumToStringEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumToIntEntity {
}

pub const ENUMTOINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumToIntEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENUMLOGICENTITYWITHSINGLEINPUT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENUMTOINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnumToIntEntity {
    fn type_info() -> &'static TypeInfo {
        ENUMTOINTENTITY_TYPE_INFO
    }
}


pub const ENUMTOINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumToIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumToIntEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumPropertyGateEntity {
}

pub const ENUMPROPERTYGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumPropertyGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENUMLOGICENTITYWITHSINGLEINPUT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENUMPROPERTYGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnumPropertyGateEntity {
    fn type_info() -> &'static TypeInfo {
        ENUMPROPERTYGATEENTITY_TYPE_INFO
    }
}


pub const ENUMPROPERTYGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumPropertyGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumPropertyGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumLogicEntityWithSingleInput {
}

pub const ENUMLOGICENTITYWITHSINGLEINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumLogicEntityWithSingleInput",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENUMLOGICENTITYWITHSINGLEINPUT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnumLogicEntityWithSingleInput {
    fn type_info() -> &'static TypeInfo {
        ENUMLOGICENTITYWITHSINGLEINPUT_TYPE_INFO
    }
}


pub const ENUMLOGICENTITYWITHSINGLEINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumLogicEntityWithSingleInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumLogicEntityWithSingleInput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumIntSourceEntity {
}

pub const ENUMINTSOURCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumIntSourceEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENUMINTSOURCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnumIntSourceEntity {
    fn type_info() -> &'static TypeInfo {
        ENUMINTSOURCEENTITY_TYPE_INFO
    }
}


pub const ENUMINTSOURCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumIntSourceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumIntSourceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareEnumEntity {
}

pub const COMPAREENUMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareEnumEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREENUMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareEnumEntity {
    fn type_info() -> &'static TypeInfo {
        COMPAREENUMENTITY_TYPE_INFO
    }
}


pub const COMPAREENUMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareEnumEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareEnumEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityBusPeer {
}

pub const ENTITYBUSPEER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityBusPeer",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ENTITYBUSPEER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityBusPeer {
    fn type_info() -> &'static TypeInfo {
        ENTITYBUSPEER_TYPE_INFO
    }
}


pub const ENTITYBUSPEER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityBusPeer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntityBusPeer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityEvent {
}

pub const ENTITYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityEvent",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ENTITYEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityEvent {
    fn type_info() -> &'static TypeInfo {
        ENTITYEVENT_TYPE_INFO
    }
}


pub const ENTITYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntityEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityBus {
}

pub const ENTITYBUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityBus",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ENTITYBUS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityBus {
    fn type_info() -> &'static TypeInfo {
        ENTITYBUS_TYPE_INFO
    }
}


pub const ENTITYBUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityBus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EntityBus-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataSourceQueryEntity {
}

pub const DATASOURCEQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataSourceQueryEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DATASOURCEQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DataSourceQueryEntity {
    fn type_info() -> &'static TypeInfo {
        DATASOURCEQUERYENTITY_TYPE_INFO
    }
}


pub const DATASOURCEQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataSourceQueryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DataSourceQueryEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumToStringData {
}

pub const ENUMTOSTRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumToStringData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMPLIEDENUMTYPELOGICENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENUMTOSTRINGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnumToStringData {
    fn type_info() -> &'static TypeInfo {
        ENUMTOSTRINGDATA_TYPE_INFO
    }
}


pub const ENUMTOSTRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumToStringData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumToStringData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumToIntData {
}

pub const ENUMTOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumToIntData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMPLIEDENUMTYPELOGICENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENUMTOINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnumToIntData {
    fn type_info() -> &'static TypeInfo {
        ENUMTOINTDATA_TYPE_INFO
    }
}


pub const ENUMTOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumToIntData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumToIntData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumPropertyGateEntityData {
    pub default: bool,
    pub write_property_on_open_gate: bool,
}

pub const ENUMPROPERTYGATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumPropertyGateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMPLIEDENUMTYPELOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Default",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnumPropertyGateEntityData, default),
            },
            FieldInfoData {
                name: "WritePropertyOnOpenGate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnumPropertyGateEntityData, write_property_on_open_gate),
            },
        ],
    }),
    array_type: Some(ENUMPROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnumPropertyGateEntityData {
    fn type_info() -> &'static TypeInfo {
        ENUMPROPERTYGATEENTITYDATA_TYPE_INFO
    }
}


pub const ENUMPROPERTYGATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumPropertyGateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumPropertyGateEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EnumDebugEntityData {
    pub text_color: super::core::Vec3,
    pub screen_position: super::core::Vec2,
    pub world_position: super::core::Vec3,
    pub value_prefix: String,
    pub multiline: bool,
    pub show_transform_in_world: bool,
    pub show_transform_coordinates: bool,
    pub default_visible: bool,
    pub text_scale: f32,
}

pub const ENUMDEBUGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumDebugEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMPLIEDENUMTYPELOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TextColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnumDebugEntityData, text_color),
            },
            FieldInfoData {
                name: "ScreenPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(EnumDebugEntityData, screen_position),
            },
            FieldInfoData {
                name: "WorldPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnumDebugEntityData, world_position),
            },
            FieldInfoData {
                name: "ValuePrefix",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EnumDebugEntityData, value_prefix),
            },
            FieldInfoData {
                name: "Multiline",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnumDebugEntityData, multiline),
            },
            FieldInfoData {
                name: "ShowTransformInWorld",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnumDebugEntityData, show_transform_in_world),
            },
            FieldInfoData {
                name: "ShowTransformCoordinates",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnumDebugEntityData, show_transform_coordinates),
            },
            FieldInfoData {
                name: "DefaultVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnumDebugEntityData, default_visible),
            },
            FieldInfoData {
                name: "TextScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnumDebugEntityData, text_scale),
            },
        ],
    }),
    array_type: Some(ENUMDEBUGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnumDebugEntityData {
    fn type_info() -> &'static TypeInfo {
        ENUMDEBUGENTITYDATA_TYPE_INFO
    }
}


pub const ENUMDEBUGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumDebugEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumDebugEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImpliedEnumTypeLogicEntityData {
}

pub const IMPLIEDENUMTYPELOGICENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ImpliedEnumTypeLogicEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENUMLOGICENTITYBASEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(IMPLIEDENUMTYPELOGICENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ImpliedEnumTypeLogicEntityData {
    fn type_info() -> &'static TypeInfo {
        IMPLIEDENUMTYPELOGICENTITYDATA_TYPE_INFO
    }
}


pub const IMPLIEDENUMTYPELOGICENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ImpliedEnumTypeLogicEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ImpliedEnumTypeLogicEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExplicitEnumTypeLogicEntityData {
}

pub const EXPLICITENUMTYPELOGICENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExplicitEnumTypeLogicEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENUMLOGICENTITYBASEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EXPLICITENUMTYPELOGICENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExplicitEnumTypeLogicEntityData {
    fn type_info() -> &'static TypeInfo {
        EXPLICITENUMTYPELOGICENTITYDATA_TYPE_INFO
    }
}


pub const EXPLICITENUMTYPELOGICENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExplicitEnumTypeLogicEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ExplicitEnumTypeLogicEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumLogicEntityBaseData {
    pub realm: super::core::Realm,
    pub type_name_hash: u32,
}

pub const ENUMLOGICENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumLogicEntityBaseData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(EnumLogicEntityBaseData, realm),
            },
            FieldInfoData {
                name: "TypeNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnumLogicEntityBaseData, type_name_hash),
            },
        ],
    }),
    array_type: Some(ENUMLOGICENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnumLogicEntityBaseData {
    fn type_info() -> &'static TypeInfo {
        ENUMLOGICENTITYBASEDATA_TYPE_INFO
    }
}


pub const ENUMLOGICENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumLogicEntityBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumLogicEntityBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumIntSourceEntityData {
    pub enum_int_output_properties: Vec<EnumIntSourceProperty>,
}

pub const ENUMINTSOURCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumIntSourceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EXPLICITENUMTYPELOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EnumIntOutputProperties",
                flags: MemberInfoFlags::new(144),
                field_type: ENUMINTSOURCEPROPERTY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EnumIntSourceEntityData, enum_int_output_properties),
            },
        ],
    }),
    array_type: Some(ENUMINTSOURCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnumIntSourceEntityData {
    fn type_info() -> &'static TypeInfo {
        ENUMINTSOURCEENTITYDATA_TYPE_INFO
    }
}


pub const ENUMINTSOURCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumIntSourceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumIntSourceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumIntSourceProperty {
    pub enum_value: i32,
    pub output_property_hash: u32,
}

pub const ENUMINTSOURCEPROPERTY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumIntSourceProperty",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EnumValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EnumIntSourceProperty, enum_value),
            },
            FieldInfoData {
                name: "OutputPropertyHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnumIntSourceProperty, output_property_hash),
            },
        ],
    }),
    array_type: Some(ENUMINTSOURCEPROPERTY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EnumIntSourceProperty {
    fn type_info() -> &'static TypeInfo {
        ENUMINTSOURCEPROPERTY_TYPE_INFO
    }
}


pub const ENUMINTSOURCEPROPERTY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumIntSourceProperty-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EnumIntSourceProperty-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareEnumEntityData {
    pub trigger_on_property_change: bool,
    pub trigger_on_start: bool,
    pub always_send: bool,
    pub enum_default_value: i32,
}

pub const COMPAREENUMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareEnumEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMPLIEDENUMTYPELOGICENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TriggerOnPropertyChange",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompareEnumEntityData, trigger_on_property_change),
            },
            FieldInfoData {
                name: "TriggerOnStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompareEnumEntityData, trigger_on_start),
            },
            FieldInfoData {
                name: "AlwaysSend",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompareEnumEntityData, always_send),
            },
            FieldInfoData {
                name: "EnumDefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CompareEnumEntityData, enum_default_value),
            },
        ],
    }),
    array_type: Some(COMPAREENUMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompareEnumEntityData {
    fn type_info() -> &'static TypeInfo {
        COMPAREENUMENTITYDATA_TYPE_INFO
    }
}


pub const COMPAREENUMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareEnumEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareEnumEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataSourceQueryEntityData {
    pub realm: super::core::Realm,
    pub in_data: super::core::DataContainer,
    pub array_index: i32,
}

pub const DATASOURCEQUERYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataSourceQueryEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(DataSourceQueryEntityData, realm),
            },
            FieldInfoData {
                name: "InData",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(DataSourceQueryEntityData, in_data),
            },
            FieldInfoData {
                name: "ArrayIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DataSourceQueryEntityData, array_index),
            },
        ],
    }),
    array_type: Some(DATASOURCEQUERYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataSourceQueryEntityData {
    fn type_info() -> &'static TypeInfo {
        DATASOURCEQUERYENTITYDATA_TYPE_INFO
    }
}


pub const DATASOURCEQUERYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataSourceQueryEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DataSourceQueryEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataSourceQueryFieldDefinition {
    pub name: String,
    pub type_name_hash: u32,
    pub is_array: bool,
    pub editor_property_type: String,
}

pub const DATASOURCEQUERYFIELDDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataSourceQueryFieldDefinition",
    flags: MemberInfoFlags::new(73),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DataSourceQueryFieldDefinition, name),
            },
            FieldInfoData {
                name: "TypeNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DataSourceQueryFieldDefinition, type_name_hash),
            },
            FieldInfoData {
                name: "IsArray",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DataSourceQueryFieldDefinition, is_array),
            },
            FieldInfoData {
                name: "EditorPropertyType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DataSourceQueryFieldDefinition, editor_property_type),
            },
        ],
    }),
    array_type: Some(DATASOURCEQUERYFIELDDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataSourceQueryFieldDefinition {
    fn type_info() -> &'static TypeInfo {
        DATASOURCEQUERYFIELDDEFINITION_TYPE_INFO
    }
}


pub const DATASOURCEQUERYFIELDDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataSourceQueryFieldDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DataSourceQueryFieldDefinition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyWrapper {
}

pub const PROPERTYWRAPPER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyWrapper",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROPERTYWRAPPER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertyWrapper {
    fn type_info() -> &'static TypeInfo {
        PROPERTYWRAPPER_TYPE_INFO
    }
}


pub const PROPERTYWRAPPER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyWrapper-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PropertyWrapper-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BundleManagerBundleUnloadMessage {
}

pub const BUNDLEMANAGERBUNDLEUNLOADMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleManagerBundleUnloadMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Entity",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for BundleManagerBundleUnloadMessage {
    fn type_info() -> &'static TypeInfo {
        BUNDLEMANAGERBUNDLEUNLOADMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WriteArrayElementEntity {
}

pub const WRITEARRAYELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriteArrayElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WRITEARRAYELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WriteArrayElementEntity {
    fn type_info() -> &'static TypeInfo {
        WRITEARRAYELEMENTENTITY_TYPE_INFO
    }
}


pub const WRITEARRAYELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriteArrayElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WriteArrayElementEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReadArraySizeEntity {
}

pub const READARRAYSIZEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadArraySizeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(READARRAYSIZEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ReadArraySizeEntity {
    fn type_info() -> &'static TypeInfo {
        READARRAYSIZEENTITY_TYPE_INFO
    }
}


pub const READARRAYSIZEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadArraySizeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReadArraySizeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReadArrayElementEntity {
}

pub const READARRAYELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadArrayElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(READARRAYELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ReadArrayElementEntity {
    fn type_info() -> &'static TypeInfo {
        READARRAYELEMENTENTITY_TYPE_INFO
    }
}


pub const READARRAYELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadArrayElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReadArrayElementEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForEachVariableEntity {
}

pub const FOREACHVARIABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForEachVariableEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FOREACHVARIABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ForEachVariableEntity {
    fn type_info() -> &'static TypeInfo {
        FOREACHVARIABLEENTITY_TYPE_INFO
    }
}


pub const FOREACHVARIABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForEachVariableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ForEachVariableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClearArrayVariableEntity {
}

pub const CLEARARRAYVARIABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClearArrayVariableEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLEARARRAYVARIABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClearArrayVariableEntity {
    fn type_info() -> &'static TypeInfo {
        CLEARARRAYVARIABLEENTITY_TYPE_INFO
    }
}


pub const CLEARARRAYVARIABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClearArrayVariableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ClearArrayVariableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AddToArrayVariableEntity {
}

pub const ADDTOARRAYVARIABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AddToArrayVariableEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ADDTOARRAYVARIABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AddToArrayVariableEntity {
    fn type_info() -> &'static TypeInfo {
        ADDTOARRAYVARIABLEENTITY_TYPE_INFO
    }
}


pub const ADDTOARRAYVARIABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AddToArrayVariableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AddToArrayVariableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WriteVariableEntity {
}

pub const WRITEVARIABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriteVariableEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WRITEVARIABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WriteVariableEntity {
    fn type_info() -> &'static TypeInfo {
        WRITEVARIABLEENTITY_TYPE_INFO
    }
}


pub const WRITEVARIABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriteVariableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("WriteVariableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReadVariableEntity {
}

pub const READVARIABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadVariableEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(READVARIABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ReadVariableEntity {
    fn type_info() -> &'static TypeInfo {
        READVARIABLEENTITY_TYPE_INFO
    }
}


pub const READVARIABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReadVariableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReadVariableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HighlightEntity {
}

pub const HIGHLIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HighlightEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(HIGHLIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HighlightEntity {
    fn type_info() -> &'static TypeInfo {
        HIGHLIGHTENTITY_TYPE_INFO
    }
}


pub const HIGHLIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HighlightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("HighlightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextSequenceEntity {
}

pub const TEXTSEQUENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextSequenceEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TEXTSEQUENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TextSequenceEntity {
    fn type_info() -> &'static TypeInfo {
        TEXTSEQUENCEENTITY_TYPE_INFO
    }
}


pub const TEXTSEQUENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextSequenceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TextSequenceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelEntity {
}

pub const SUBLEVELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubLevelEntity {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELENTITY_TYPE_INFO
    }
}


pub const SUBLEVELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SubLevelEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReportInstallationProgressEntity {
}

pub const REPORTINSTALLATIONPROGRESSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReportInstallationProgressEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(REPORTINSTALLATIONPROGRESSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ReportInstallationProgressEntity {
    fn type_info() -> &'static TypeInfo {
        REPORTINSTALLATIONPROGRESSENTITY_TYPE_INFO
    }
}


pub const REPORTINSTALLATIONPROGRESSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReportInstallationProgressEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ReportInstallationProgressEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalPlayerIdEntity {
}

pub const LOCALPLAYERIDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerIdEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALPLAYERIDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalPlayerIdEntity {
    fn type_info() -> &'static TypeInfo {
        LOCALPLAYERIDENTITY_TYPE_INFO
    }
}


pub const LOCALPLAYERIDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerIdEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LocalPlayerIdEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatCacheEntity {
}

pub const FLOATCACHEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCacheEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATCACHEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatCacheEntity {
    fn type_info() -> &'static TypeInfo {
        FLOATCACHEENTITY_TYPE_INFO
    }
}


pub const FLOATCACHEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCacheEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatCacheEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MultilineStringEntity {
}

pub const MULTILINESTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultilineStringEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MULTILINESTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MultilineStringEntity {
    fn type_info() -> &'static TypeInfo {
        MULTILINESTRINGENTITY_TYPE_INFO
    }
}


pub const MULTILINESTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultilineStringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MultilineStringEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringEntity {
}

pub const STRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StringEntity {
    fn type_info() -> &'static TypeInfo {
        STRINGENTITY_TYPE_INFO
    }
}


pub const STRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("StringEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AxisAlignedBoxEntity {
}

pub const AXISALIGNEDBOXENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxisAlignedBoxEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AXISALIGNEDBOXENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AxisAlignedBoxEntity {
    fn type_info() -> &'static TypeInfo {
        AXISALIGNEDBOXENTITY_TYPE_INFO
    }
}


pub const AXISALIGNEDBOXENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxisAlignedBoxEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AxisAlignedBoxEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vector4Entity {
}

pub const VECTOR4ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vector4Entity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VECTOR4ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vector4Entity {
    fn type_info() -> &'static TypeInfo {
        VECTOR4ENTITY_TYPE_INFO
    }
}


pub const VECTOR4ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vector4Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vector4Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vector3Entity {
}

pub const VECTOR3ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vector3Entity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VECTOR3ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vector3Entity {
    fn type_info() -> &'static TypeInfo {
        VECTOR3ENTITY_TYPE_INFO
    }
}


pub const VECTOR3ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vector3Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vector3Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformEntity {
}

pub const TRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMENTITY_TYPE_INFO
    }
}


pub const TRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatEntity {
}

pub const FLOATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatEntity {
    fn type_info() -> &'static TypeInfo {
        FLOATENTITY_TYPE_INFO
    }
}


pub const FLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIntEntity {
}

pub const UINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIntEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIntEntity {
    fn type_info() -> &'static TypeInfo {
        UINTENTITY_TYPE_INFO
    }
}


pub const UINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("UIntEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntEntity {
}

pub const INTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntEntity {
    fn type_info() -> &'static TypeInfo {
        INTENTITY_TYPE_INFO
    }
}


pub const INTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolEntity {
}

pub const BOOLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BOOLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BoolEntity {
    fn type_info() -> &'static TypeInfo {
        BOOLENTITY_TYPE_INFO
    }
}


pub const BOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoolEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpatialEntity {
}

pub const SPATIALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SPATIALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SpatialEntity {
    fn type_info() -> &'static TypeInfo {
        SPATIALENTITY_TYPE_INFO
    }
}


pub const SPATIALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpatialEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SpatialEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectVec4Entity {
}

pub const SELECTVEC4ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec4Entity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTVEC4ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectVec4Entity {
    fn type_info() -> &'static TypeInfo {
        SELECTVEC4ENTITY_TYPE_INFO
    }
}


pub const SELECTVEC4ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec4Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectVec4Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectVec3Entity {
}

pub const SELECTVEC3ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec3Entity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTVEC3ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectVec3Entity {
    fn type_info() -> &'static TypeInfo {
        SELECTVEC3ENTITY_TYPE_INFO
    }
}


pub const SELECTVEC3ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec3Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectVec3Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectVec2Entity {
}

pub const SELECTVEC2ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec2Entity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTVEC2ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectVec2Entity {
    fn type_info() -> &'static TypeInfo {
        SELECTVEC2ENTITY_TYPE_INFO
    }
}


pub const SELECTVEC2ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectVec2Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectVec2Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectTransformEntity {
}

pub const SELECTTRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectTransformEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTTRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectTransformEntity {
    fn type_info() -> &'static TypeInfo {
        SELECTTRANSFORMENTITY_TYPE_INFO
    }
}


pub const SELECTTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectTransformEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectBoolEntity {
}

pub const SELECTBOOLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectBoolEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTBOOLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectBoolEntity {
    fn type_info() -> &'static TypeInfo {
        SELECTBOOLENTITY_TYPE_INFO
    }
}


pub const SELECTBOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectBoolEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectBoolEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectStringEntity {
}

pub const SELECTSTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectStringEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTSTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectStringEntity {
    fn type_info() -> &'static TypeInfo {
        SELECTSTRINGENTITY_TYPE_INFO
    }
}


pub const SELECTSTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectStringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectStringEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectInt64Entity {
}

pub const SELECTINT64ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectInt64Entity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTINT64ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectInt64Entity {
    fn type_info() -> &'static TypeInfo {
        SELECTINT64ENTITY_TYPE_INFO
    }
}


pub const SELECTINT64ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectInt64Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectInt64Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectIntEntity {
}

pub const SELECTINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectIntEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectIntEntity {
    fn type_info() -> &'static TypeInfo {
        SELECTINTENTITY_TYPE_INFO
    }
}


pub const SELECTINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectIntEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectFloatEntity {
}

pub const SELECTFLOATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectFloatEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTFLOATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectFloatEntity {
    fn type_info() -> &'static TypeInfo {
        SELECTFLOATENTITY_TYPE_INFO
    }
}


pub const SELECTFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectFloatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectFloatEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScriptEntity {
}

pub const SCRIPTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScriptEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SCRIPTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ScriptEntity {
    fn type_info() -> &'static TypeInfo {
        SCRIPTENTITY_TYPE_INFO
    }
}


pub const SCRIPTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScriptEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ScriptEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicChannelEntity {
}

pub const SCHEMATICCHANNELENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicChannelEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICCHANNELENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicChannelEntity {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICCHANNELENTITY_TYPE_INFO
    }
}


pub const SCHEMATICCHANNELENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicChannelEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SchematicChannelEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Part {
}

pub const PART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Part",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Part {
    fn type_info() -> &'static TypeInfo {
        PART_TYPE_INFO
    }
}


pub const PART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Part-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Part-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectAreaTriggerEntity {
}

pub const OBJECTAREATRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectAreaTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBJECTAREATRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectAreaTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        OBJECTAREATRIGGERENTITY_TYPE_INFO
    }
}


pub const OBJECTAREATRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectAreaTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectAreaTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectVariationToIntEntity {
}

pub const OBJECTVARIATIONTOINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationToIntEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBJECTVARIATIONTOINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectVariationToIntEntity {
    fn type_info() -> &'static TypeInfo {
        OBJECTVARIATIONTOINTENTITY_TYPE_INFO
    }
}


pub const OBJECTVARIATIONTOINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationToIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectVariationToIntEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformSpaceEntity {
}

pub const TRANSFORMSPACEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMSPACEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformSpaceEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMSPACEENTITY_TYPE_INFO
    }
}


pub const TRANSFORMSPACEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformSpaceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatSelectEntity {
}

pub const FLOATSELECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatSelectEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATSELECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatSelectEntity {
    fn type_info() -> &'static TypeInfo {
        FLOATSELECTENTITY_TYPE_INFO
    }
}


pub const FLOATSELECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatSelectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatSelectEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatCurveEntity {
}

pub const FLOATCURVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATCURVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatCurveEntity {
    fn type_info() -> &'static TypeInfo {
        FLOATCURVEENTITY_TYPE_INFO
    }
}


pub const FLOATCURVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatCurveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SettingEntity {
}

pub const SETTINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SettingEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SETTINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SettingEntity {
    fn type_info() -> &'static TypeInfo {
        SETTINGENTITY_TYPE_INFO
    }
}


pub const SETTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SettingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SettingEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntegratorOrDifferentiatorEntity {
}

pub const INTEGRATORORDIFFERENTIATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegratorOrDifferentiatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INTEGRATORORDIFFERENTIATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntegratorOrDifferentiatorEntity {
    fn type_info() -> &'static TypeInfo {
        INTEGRATORORDIFFERENTIATORENTITY_TYPE_INFO
    }
}


pub const INTEGRATORORDIFFERENTIATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegratorOrDifferentiatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntegratorOrDifferentiatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProfileEntity {
}

pub const PROFILEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROFILEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ProfileEntity {
    fn type_info() -> &'static TypeInfo {
        PROFILEENTITY_TYPE_INFO
    }
}


pub const PROFILEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProfileEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ProfileEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompareEntity {
}

pub const COMPAREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPAREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompareEntity {
    fn type_info() -> &'static TypeInfo {
        COMPAREENTITY_TYPE_INFO
    }
}


pub const COMPAREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompareEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CompareEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RunningAverageEntity {
}

pub const RUNNINGAVERAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RunningAverageEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RUNNINGAVERAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RunningAverageEntity {
    fn type_info() -> &'static TypeInfo {
        RUNNINGAVERAGEENTITY_TYPE_INFO
    }
}


pub const RUNNINGAVERAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RunningAverageEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RunningAverageEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AbsEntity {
}

pub const ABSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AbsEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ABSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AbsEntity {
    fn type_info() -> &'static TypeInfo {
        ABSENTITY_TYPE_INFO
    }
}


pub const ABSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AbsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AbsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VectorMathOpEntity {
}

pub const VECTORMATHOPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorMathOpEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VECTORMATHOPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VectorMathOpEntity {
    fn type_info() -> &'static TypeInfo {
        VECTORMATHOPENTITY_TYPE_INFO
    }
}


pub const VECTORMATHOPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VectorMathOpEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("VectorMathOpEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MathOpEntity {
}

pub const MATHOPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathOpEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MATHOPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MathOpEntity {
    fn type_info() -> &'static TypeInfo {
        MATHOPENTITY_TYPE_INFO
    }
}


pub const MATHOPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathOpEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MathOpEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyCastEntity {
}

pub const PROPERTYCASTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyCastEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROPERTYCASTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PropertyCastEntity {
    fn type_info() -> &'static TypeInfo {
        PROPERTYCASTENTITY_TYPE_INFO
    }
}


pub const PROPERTYCASTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyCastEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PropertyCastEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolToEventEntity {
}

pub const BOOLTOEVENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolToEventEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BOOLTOEVENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BoolToEventEntity {
    fn type_info() -> &'static TypeInfo {
        BOOLTOEVENTENTITY_TYPE_INFO
    }
}


pub const BOOLTOEVENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolToEventEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoolToEventEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwitchPropertyStringEntity {
}

pub const SWITCHPROPERTYSTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwitchPropertyStringEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SWITCHPROPERTYSTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SwitchPropertyStringEntity {
    fn type_info() -> &'static TypeInfo {
        SWITCHPROPERTYSTRINGENTITY_TYPE_INFO
    }
}


pub const SWITCHPROPERTYSTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SwitchPropertyStringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SwitchPropertyStringEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyGateEntity {
}

pub const PROPERTYGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROPERTYGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PropertyGateEntity {
    fn type_info() -> &'static TypeInfo {
        PROPERTYGATEENTITY_TYPE_INFO
    }
}


pub const PROPERTYGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("PropertyGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventGateEntity {
}

pub const EVENTGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EVENTGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventGateEntity {
    fn type_info() -> &'static TypeInfo {
        EVENTGATEENTITY_TYPE_INFO
    }
}


pub const EVENTGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StopWatchEntity {
}

pub const STOPWATCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopWatchEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STOPWATCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StopWatchEntity {
    fn type_info() -> &'static TypeInfo {
        STOPWATCHENTITY_TYPE_INFO
    }
}


pub const STOPWATCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopWatchEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("StopWatchEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformToRotationEntity {
}

pub const TRANSFORMTOROTATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformToRotationEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMTOROTATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformToRotationEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMTOROTATIONENTITY_TYPE_INFO
    }
}


pub const TRANSFORMTOROTATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformToRotationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformToRotationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformModifierEntity {
}

pub const TRANSFORMMODIFIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMMODIFIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformModifierEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMMODIFIERENTITY_TYPE_INFO
    }
}


pub const TRANSFORMMODIFIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformModifierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformModifierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RotationTransformBuilderEntity {
}

pub const ROTATIONTRANSFORMBUILDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationTransformBuilderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ROTATIONTRANSFORMBUILDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RotationTransformBuilderEntity {
    fn type_info() -> &'static TypeInfo {
        ROTATIONTRANSFORMBUILDERENTITY_TYPE_INFO
    }
}


pub const ROTATIONTRANSFORMBUILDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RotationTransformBuilderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RotationTransformBuilderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScaleTransformBuilderEntity {
}

pub const SCALETRANSFORMBUILDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScaleTransformBuilderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SCALETRANSFORMBUILDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ScaleTransformBuilderEntity {
    fn type_info() -> &'static TypeInfo {
        SCALETRANSFORMBUILDERENTITY_TYPE_INFO
    }
}


pub const SCALETRANSFORMBUILDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScaleTransformBuilderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ScaleTransformBuilderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformBuilderEntity {
}

pub const TRANSFORMBUILDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformBuilderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMBUILDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformBuilderEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMBUILDERENTITY_TYPE_INFO
    }
}


pub const TRANSFORMBUILDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformBuilderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformBuilderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VecBuilderEntity {
}

pub const VECBUILDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VecBuilderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VECBUILDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VecBuilderEntity {
    fn type_info() -> &'static TypeInfo {
        VECBUILDERENTITY_TYPE_INFO
    }
}


pub const VECBUILDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VecBuilderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("VecBuilderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec4BuilderEntity {
}

pub const VEC4BUILDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4BuilderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC4BUILDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec4BuilderEntity {
    fn type_info() -> &'static TypeInfo {
        VEC4BUILDERENTITY_TYPE_INFO
    }
}


pub const VEC4BUILDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4BuilderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec4BuilderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec3BuilderEntity {
}

pub const VEC3BUILDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3BuilderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC3BUILDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec3BuilderEntity {
    fn type_info() -> &'static TypeInfo {
        VEC3BUILDERENTITY_TYPE_INFO
    }
}


pub const VEC3BUILDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3BuilderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec3BuilderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec2BuilderEntity {
}

pub const VEC2BUILDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2BuilderEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC2BUILDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec2BuilderEntity {
    fn type_info() -> &'static TypeInfo {
        VEC2BUILDERENTITY_TYPE_INFO
    }
}


pub const VEC2BUILDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2BuilderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec2BuilderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VecSplitterEntity {
}

pub const VECSPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VecSplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VECSPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VecSplitterEntity {
    fn type_info() -> &'static TypeInfo {
        VECSPLITTERENTITY_TYPE_INFO
    }
}


pub const VECSPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VecSplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("VecSplitterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec4SplitterEntity {
}

pub const VEC4SPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4SplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC4SPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec4SplitterEntity {
    fn type_info() -> &'static TypeInfo {
        VEC4SPLITTERENTITY_TYPE_INFO
    }
}


pub const VEC4SPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4SplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec4SplitterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec3SplitterEntity {
}

pub const VEC3SPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3SplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC3SPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec3SplitterEntity {
    fn type_info() -> &'static TypeInfo {
        VEC3SPLITTERENTITY_TYPE_INFO
    }
}


pub const VEC3SPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3SplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec3SplitterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec2SplitterEntity {
}

pub const VEC2SPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2SplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC2SPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec2SplitterEntity {
    fn type_info() -> &'static TypeInfo {
        VEC2SPLITTERENTITY_TYPE_INFO
    }
}


pub const VEC2SPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2SplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec2SplitterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EulerTransformEntity {
}

pub const EULERTRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EulerTransformEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EULERTRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EulerTransformEntity {
    fn type_info() -> &'static TypeInfo {
        EULERTRANSFORMENTITY_TYPE_INFO
    }
}


pub const EULERTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EulerTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EulerTransformEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EulerTransformSplitterEntity {
}

pub const EULERTRANSFORMSPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EulerTransformSplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EULERTRANSFORMSPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EulerTransformSplitterEntity {
    fn type_info() -> &'static TypeInfo {
        EULERTRANSFORMSPLITTERENTITY_TYPE_INFO
    }
}


pub const EULERTRANSFORMSPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EulerTransformSplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EulerTransformSplitterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformSplitterEntity {
}

pub const TRANSFORMSPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMSPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformSplitterEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMSPLITTERENTITY_TYPE_INFO
    }
}


pub const TRANSFORMSPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformSplitterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformSelectorEntity {
}

pub const TRANSFORMSELECTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSelectorEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMSELECTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformSelectorEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMSELECTORENTITY_TYPE_INFO
    }
}


pub const TRANSFORMSELECTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSelectorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformSelectorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformBlendEntity {
}

pub const TRANSFORMBLENDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformBlendEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMBLENDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformBlendEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMBLENDENTITY_TYPE_INFO
    }
}


pub const TRANSFORMBLENDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformBlendEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformBlendEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SimpleDofParamConverterEntity {
}

pub const SIMPLEDOFPARAMCONVERTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleDofParamConverterEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SIMPLEDOFPARAMCONVERTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SimpleDofParamConverterEntity {
    fn type_info() -> &'static TypeInfo {
        SIMPLEDOFPARAMCONVERTERENTITY_TYPE_INFO
    }
}


pub const SIMPLEDOFPARAMCONVERTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleDofParamConverterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SimpleDofParamConverterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpriteDofParamConverterEntity {
}

pub const SPRITEDOFPARAMCONVERTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpriteDofParamConverterEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SPRITEDOFPARAMCONVERTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SpriteDofParamConverterEntity {
    fn type_info() -> &'static TypeInfo {
        SPRITEDOFPARAMCONVERTERENTITY_TYPE_INFO
    }
}


pub const SPRITEDOFPARAMCONVERTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpriteDofParamConverterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SpriteDofParamConverterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformMultiplierEntity {
}

pub const TRANSFORMMULTIPLIERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformMultiplierEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMMULTIPLIERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformMultiplierEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMMULTIPLIERENTITY_TYPE_INFO
    }
}


pub const TRANSFORMMULTIPLIERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformMultiplierEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformMultiplierEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformHubEntity {
}

pub const TRANSFORMHUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformHubEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMHUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformHubEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMHUBENTITY_TYPE_INFO
    }
}


pub const TRANSFORMHUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformHubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("TransformHubEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringHubEntity {
}

pub const STRINGHUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringHubEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STRINGHUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StringHubEntity {
    fn type_info() -> &'static TypeInfo {
        STRINGHUBENTITY_TYPE_INFO
    }
}


pub const STRINGHUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringHubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("StringHubEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec4HubEntity {
}

pub const VEC4HUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4HubEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC4HUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec4HubEntity {
    fn type_info() -> &'static TypeInfo {
        VEC4HUBENTITY_TYPE_INFO
    }
}


pub const VEC4HUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4HubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec4HubEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec3HubEntity {
}

pub const VEC3HUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3HubEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC3HUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec3HubEntity {
    fn type_info() -> &'static TypeInfo {
        VEC3HUBENTITY_TYPE_INFO
    }
}


pub const VEC3HUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3HubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec3HubEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec2HubEntity {
}

pub const VEC2HUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2HubEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC2HUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec2HubEntity {
    fn type_info() -> &'static TypeInfo {
        VEC2HUBENTITY_TYPE_INFO
    }
}


pub const VEC2HUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2HubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Vec2HubEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatHubEntity {
}

pub const FLOATHUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatHubEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATHUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatHubEntity {
    fn type_info() -> &'static TypeInfo {
        FLOATHUBENTITY_TYPE_INFO
    }
}


pub const FLOATHUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatHubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("FloatHubEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Int64HubEntity {
}

pub const INT64HUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int64HubEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INT64HUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Int64HubEntity {
    fn type_info() -> &'static TypeInfo {
        INT64HUBENTITY_TYPE_INFO
    }
}


pub const INT64HUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int64HubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Int64HubEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntHubEntity {
}

pub const INTHUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntHubEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INTHUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntHubEntity {
    fn type_info() -> &'static TypeInfo {
        INTHUBENTITY_TYPE_INFO
    }
}


pub const INTHUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntHubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("IntHubEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolHubEntity {
}

pub const BOOLHUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolHubEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BOOLHUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BoolHubEntity {
    fn type_info() -> &'static TypeInfo {
        BOOLHUBENTITY_TYPE_INFO
    }
}


pub const BOOLHUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolHubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BoolHubEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectVariationSwitchEntity {
}

pub const OBJECTVARIATIONSWITCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationSwitchEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBJECTVARIATIONSWITCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectVariationSwitchEntity {
    fn type_info() -> &'static TypeInfo {
        OBJECTVARIATIONSWITCHENTITY_TYPE_INFO
    }
}


pub const OBJECTVARIATIONSWITCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationSwitchEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectVariationSwitchEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectVariationMultiSwitchEntity {
}

pub const OBJECTVARIATIONMULTISWITCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationMultiSwitchEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBJECTVARIATIONMULTISWITCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectVariationMultiSwitchEntity {
    fn type_info() -> &'static TypeInfo {
        OBJECTVARIATIONMULTISWITCHENTITY_TYPE_INFO
    }
}


pub const OBJECTVARIATIONMULTISWITCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectVariationMultiSwitchEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("ObjectVariationMultiSwitchEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RandomDelayEntity {
}

pub const RANDOMDELAYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomDelayEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RANDOMDELAYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RandomDelayEntity {
    fn type_info() -> &'static TypeInfo {
        RANDOMDELAYENTITY_TYPE_INFO
    }
}


pub const RANDOMDELAYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomDelayEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RandomDelayEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DelayEntity {
}

pub const DELAYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DelayEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DELAYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DelayEntity {
    fn type_info() -> &'static TypeInfo {
        DELAYENTITY_TYPE_INFO
    }
}


pub const DELAYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DelayEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("DelayEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectEventEntity {
}

pub const SELECTEVENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectEventEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTEVENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectEventEntity {
    fn type_info() -> &'static TypeInfo {
        SELECTEVENTENTITY_TYPE_INFO
    }
}


pub const SELECTEVENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectEventEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("SelectEventEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventOrderGateEntity {
}

pub const EVENTORDERGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventOrderGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EVENTORDERGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventOrderGateEntity {
    fn type_info() -> &'static TypeInfo {
        EVENTORDERGATEENTITY_TYPE_INFO
    }
}


pub const EVENTORDERGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventOrderGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventOrderGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventAndGateEntity {
}

pub const EVENTANDGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventAndGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EVENTANDGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventAndGateEntity {
    fn type_info() -> &'static TypeInfo {
        EVENTANDGATEENTITY_TYPE_INFO
    }
}


pub const EVENTANDGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventAndGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventAndGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventSwitchEntity {
}

pub const EVENTSWITCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSwitchEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EVENTSWITCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventSwitchEntity {
    fn type_info() -> &'static TypeInfo {
        EVENTSWITCHENTITY_TYPE_INFO
    }
}


pub const EVENTSWITCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSwitchEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("EventSwitchEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UniqueIdEntity {
}

pub const UNIQUEIDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UniqueIdEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UNIQUEIDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UniqueIdEntity {
    fn type_info() -> &'static TypeInfo {
        UNIQUEIDENTITY_TYPE_INFO
    }
}


pub const UNIQUEIDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UniqueIdEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("UniqueIdEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RandomIntEntity {
}

pub const RANDOMINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomIntEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RANDOMINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RandomIntEntity {
    fn type_info() -> &'static TypeInfo {
        RANDOMINTENTITY_TYPE_INFO
    }
}


pub const RANDOMINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RandomIntEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RandomFloatEntity {
}

pub const RANDOMFLOATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomFloatEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RANDOMFLOATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RandomFloatEntity {
    fn type_info() -> &'static TypeInfo {
        RANDOMFLOATENTITY_TYPE_INFO
    }
}


pub const RANDOMFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomFloatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RandomFloatEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RandomMultiEventEntity {
}

pub const RANDOMMULTIEVENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomMultiEventEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RANDOMMULTIEVENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RandomMultiEventEntity {
    fn type_info() -> &'static TypeInfo {
        RANDOMMULTIEVENTENTITY_TYPE_INFO
    }
}


pub const RANDOMMULTIEVENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomMultiEventEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RandomMultiEventEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RandomEventEntity {
}

pub const RANDOMEVENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomEventEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RANDOMEVENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RandomEventEntity {
    fn type_info() -> &'static TypeInfo {
        RANDOMEVENTENTITY_TYPE_INFO
    }
}


pub const RANDOMEVENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RandomEventEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("RandomEventEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocatorEntity {
}

pub const LOCATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocatorEntity {
    fn type_info() -> &'static TypeInfo {
        LOCATORENTITY_TYPE_INFO
    }
}


pub const LOCATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("LocatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CountDownEntity {
}

pub const COUNTDOWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CountDownEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COUNTDOWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CountDownEntity {
    fn type_info() -> &'static TypeInfo {
        COUNTDOWNENTITY_TYPE_INFO
    }
}


pub const COUNTDOWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CountDownEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("CountDownEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MathEntity {
}

pub const MATHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MATHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MathEntity {
    fn type_info() -> &'static TypeInfo {
        MATHENTITY_TYPE_INFO
    }
}


pub const MATHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("MathEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Or4Entity {
}

pub const OR4ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Or4Entity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OR4ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Or4Entity {
    fn type_info() -> &'static TypeInfo {
        OR4ENTITY_TYPE_INFO
    }
}


pub const OR4ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Or4Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("Or4Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct XorEntity {
}

pub const XORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "XorEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(XORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for XorEntity {
    fn type_info() -> &'static TypeInfo {
        XORENTITY_TYPE_INFO
    }
}


pub const XORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "XorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("XorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OrEntity {
}

pub const ORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OrEntity {
    fn type_info() -> &'static TypeInfo {
        ORENTITY_TYPE_INFO
    }
}


pub const ORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("OrEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BitwiseAndEntity {
}

pub const BITWISEANDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BitwiseAndEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BITWISEANDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BitwiseAndEntity {
    fn type_info() -> &'static TypeInfo {
        BITWISEANDENTITY_TYPE_INFO
    }
}


pub const BITWISEANDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BitwiseAndEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("BitwiseAndEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AndEntity {
}

pub const ANDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AndEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AndEntity {
    fn type_info() -> &'static TypeInfo {
        ANDENTITY_TYPE_INFO
    }
}


pub const ANDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AndEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("AndEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NotEntity {
}

pub const NOTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NotEntity",
    flags: MemberInfoFlags::new(101),
    module: "Entity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(NOTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for NotEntity {
    fn type_info() -> &'static TypeInfo {
        NOTENTITY_TYPE_INFO
    }
}


pub const NOTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NotEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entity",
    data: TypeInfoData::Array("NotEntity-Array"),
    array_type: None,
    alignment: 8,
};


